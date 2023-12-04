use clap::Parser;
use std::collections::HashSet;
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
    let sum: u32 = input
        .lines()
        .map(|l| l.unwrap())
        .map(|line| {
            let (_, numbers) = line.split_once(':').unwrap();
            let (winning_numbers, our_numbers) = numbers.split_once('|').unwrap();
            let winning_numbers: HashSet<u32> = winning_numbers
                .trim()
                .split_ascii_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            let our_numbers: HashSet<u32> = our_numbers
                .trim()
                .split_ascii_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            let our_winning = winning_numbers.intersection(&our_numbers);
            let nb_winning = our_winning.count() as u32;
            if nb_winning == 0 {
                0
            } else {
                2_u32.pow(nb_winning - 1)
            }
        })
        .sum();

    println!("{}", sum);
    Ok(())
}

#[derive(Default)]
struct DefaultValueVec(Vec<u32>);

impl DefaultValueVec {
    pub fn add(&mut self, index: usize, value: u32) {
        for _ in self.0.len()..index + 1 {
            self.0.push(1);
        }
        self.0[index] += value;
    }
    pub fn get(&mut self, index: usize) -> u32 {
        for _ in self.0.len()..index + 1 {
            self.0.push(1);
        }
        self.0[index]
    }
}

fn main2(input: impl BufRead) -> Result<()> {
    let mut nb_copies = DefaultValueVec::default();
    let sum: u32 = input
        .lines()
        .map(|l| l.unwrap())
        .map(|line| {
            let (_, numbers) = line.split_once(':').unwrap();
            let (winning_numbers, our_numbers) = numbers.split_once('|').unwrap();
            let winning_numbers: HashSet<u32> = winning_numbers
                .trim()
                .split_ascii_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            let our_numbers: HashSet<u32> = our_numbers
                .trim()
                .split_ascii_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            let our_winning = winning_numbers.intersection(&our_numbers);
            our_winning.count() as u32
        })
        .enumerate()
        .map(|(card_nb, winning_nb)| {
            let current_copies = nb_copies.get(card_nb);
            for i in 0..winning_nb {
                nb_copies.add(i as usize + 1 + card_nb, current_copies)
            }
            current_copies
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
