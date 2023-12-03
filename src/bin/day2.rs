use clap::Parser;
use regex::Regex;
use std::cmp::max;
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

struct Config {
    r: u32,
    g: u32,
    b: u32,
}

const BAG_CONFIG: Config = Config {
    r: 12,
    g: 13,
    b: 14,
};

struct Round {
    r: u32,
    g: u32,
    b: u32,
}

impl Round {
    fn new(input: &str) -> Result<Self> {
        let mut r = 0;
        let mut g = 0;
        let mut b = 0;
        for info in input.split(',') {
            let info = info.trim();
            let (nb_str, color) = info.split_once(' ').unwrap();
            match color {
                "red" => r = nb_str.parse::<u32>()?,
                "green" => g = nb_str.parse::<u32>()?,
                "blue" => b = nb_str.parse::<u32>()?,
                _ => unreachable!(),
            }
        }
        Ok(Self { r, g, b })
    }

    fn is_valid(&self, cfg: &Config) -> bool {
        self.r <= cfg.r && self.g <= cfg.g && self.b <= cfg.b
    }
}

struct Game {
    r: u32,
    g: u32,
    b: u32,
}

impl Game {
    fn new() -> Self {
        Self { r: 0, g: 0, b: 0 }
    }

    fn update(&mut self, round: &Round) {
        self.r = max(self.r, round.r);
        self.g = max(self.g, round.g);
        self.b = max(self.b, round.b);
    }

    fn power(&self) -> u32 {
        self.r * self.b * self.g
    }
}

fn main1(input: impl BufRead) -> Result<()> {
    let game_re: Regex = Regex::new(r"Game (\d+)").unwrap();
    let sum: Result<u32> = input
        .lines()
        .filter_map(|line| -> Option<Result<u32>> {
            let line = line.unwrap();
            let (game_str, rounds_str) = line.split_once(':').unwrap();
            let valid = rounds_str.trim().split(';').all(|round_str| {
                let round = Round::new(round_str).unwrap();
                round.is_valid(&BAG_CONFIG)
            });
            if valid {
                Some(Ok(game_re
                    .captures(game_str)
                    .unwrap()
                    .get(1)?
                    .as_str()
                    .parse::<u32>()
                    .unwrap()))
            } else {
                None
            }
        })
        .sum();

    println!("{}", sum?);
    Ok(())
}

fn main2(input: impl BufRead) -> Result<()> {
    let sum: u32 = input
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let (_game_str, rounds_str) = line.split_once(':').unwrap();
            let mut game = Game::new();
            rounds_str.trim().split(';').for_each(|round_str| {
                let round = Round::new(round_str).unwrap();
                game.update(&round);
            });
            game.power()
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
