use clap::Parser;
use regex::{Regex, RegexSet};
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
    let sum: Result<u32> = input
        .lines()
        .map(|line| {
            let numbers: Vec<_> = line?.chars().filter(|c| c.is_ascii_digit()).collect();
            let val = format!("{}{}", numbers.first().unwrap(), numbers.last().unwrap());
            Ok(val.parse::<u32>()?)
        })
        .sum();

    println!("{}", sum?);
    Ok(())
}

fn main2(input: impl BufRead) -> Result<()> {
    let re = Regex::new(r"one|1|two|2|three|3|four|4|five|5|six|6|seven|7|eight|8|nine|9")?;
    let re_set = RegexSet::new(&[
        r"one|1", r"two|2", r"three|3", r"four|4", r"five|5", r"six|6", r"seven|7", r"eight|8",
        r"nine|9",
    ])?;

    let sum: Result<u32> = input
        .lines()
        .map(|line| {
            let line = line?;
            let mut numbers: Vec<u32> = vec![];
            let mut start = 0;
            while let Some(m) = re.find_at(&line, start) {
                let number = re_set.matches(m.as_str()).into_iter().next().unwrap() + 1;
                numbers.push(number as u32);
                start += 1;
            }
            let val = format!("{}{}", numbers.first().unwrap(), numbers.last().unwrap());
            Ok(val.parse::<u32>()?)
        })
        .sum();

    println!("{}", sum?);
    Ok(())
}

fn main() -> Result<()> {
    let (day, input) = get_input()?;

    match day {
        1 => main1(input),
        2 => main2(input),
        _ => Ok({
            println!("Oups");
        }),
    }
}
