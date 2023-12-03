use clap::Parser;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

#[derive(Parser)]
#[clap(name = "day1")]
struct Cli {
    #[clap(value_parser)]
    day: u32,

    #[clap(value_parser)]
    input: PathBuf,
}

fn get_input() -> std::io::Result<(u32, impl BufRead)> {
    let args = Cli::parse();

    Ok((args.day, BufReader::new(File::open(args.input)?)))
}

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main1(input: impl BufRead) -> Result<()> {
    let number_re = Regex::new(r"\d+")?;
    let symbol_re = Regex::new(r"[^\d.]")?;
    let mut numbers = Vec::new();
    let mut symbols = Vec::new();
    input.lines().enumerate().for_each(|(lnb, line)| {
        let line = line.unwrap();
        number_re
            .find_iter(&line)
            .for_each(|m| numbers.push((lnb, m.start(), m.as_str().to_string())));
        symbol_re
            .find_iter(&line)
            .for_each(|m| symbols.push((lnb, m.start())));
    });

    let sum: u32 = numbers
        .into_iter()
        .filter_map(|(nl, nc, ns)| {
            if symbols.iter().any(|(sl, sc)| {
                if *sl == nl {
                    nc == sc + 1 || nc + ns.len() == *sc
                } else if sl.abs_diff(nl) == 1 {
                    nc + ns.len() >= *sc && nc <= sc + 1
                } else {
                    false
                }
            }) {
                Some(ns.parse::<u32>().unwrap())
            } else {
                None
            }
        })
        .sum();
    println!("{}", sum);
    Ok(())
}

fn main2(input: impl BufRead) -> Result<()> {
    let number_re = Regex::new(r"\d+")?;
    let gear_re = Regex::new(r"\*")?;
    let mut numbers = Vec::new();
    let mut gears = Vec::new();
    input.lines().enumerate().for_each(|(lnb, line)| {
        let line = line.unwrap();
        number_re
            .find_iter(&line)
            .for_each(|m| numbers.push((lnb, m.start(), m.as_str().to_string())));
        gear_re
            .find_iter(&line)
            .for_each(|m| gears.push((lnb, m.start())));
    });

    let sum: u32 = gears
        .into_iter()
        .filter_map(|(gl, gc)| {
            let adjacent_numbers: Vec<_> = numbers
                .iter()
                .filter_map(|(nl, nc, ns)| {
                    if gl == *nl {
                        if *nc == gc + 1 || nc + ns.len() == gc {
                            Some(ns.parse::<u32>().unwrap())
                        } else {
                            None
                        }
                    } else if gl.abs_diff(*nl) == 1 {
                        if nc + ns.len() >= gc && *nc <= gc + 1 {
                            Some(ns.parse::<u32>().unwrap())
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
                .collect();
            if adjacent_numbers.len() != 2 {
                None
            } else {
                Some(adjacent_numbers[0] * adjacent_numbers[1])
            }
        })
        .sum();
    println!("{}", sum);
    Ok(())
}

fn main() -> Result<()> {
    let (day, input) = get_input()?;

    match day {
        1 => main1(input),
        2 => main2(input),
        _ => {
            println!("Oups");
            Ok(())
        }
    }
}
