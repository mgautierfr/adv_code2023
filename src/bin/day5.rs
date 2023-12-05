use clap::Parser;
use rayon::prelude::*;

use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
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

#[derive(Default)]
struct Range {
    source_start: i64,
    source_end: i64,
    delta: i64,
}

impl Range {
    fn new(input: &str) -> Self {
        let (dest_start, source_start, length) = match &input
            .split_ascii_whitespace()
            .map(|s| s.parse().unwrap())
            .collect::<Vec<_>>()[..]
        {
            &[a, b, c] => (a, b, c),
            _ => unreachable!(),
        };
        Self {
            source_start,
            source_end: source_start + length,
            delta: dest_start - source_start,
        }
    }

    fn apply(&self, input: i64) -> Option<i64> {
        if self.source_start <= input && input < self.source_end {
            Some(input + self.delta)
        } else {
            None
        }
    }
}

#[derive(Default)]
struct RangeMap {
    ranges: Vec<Range>,
}

impl RangeMap {
    fn feed(&mut self, input: &str) {
        self.ranges.push(Range::new(input));
    }

    fn apply(&self, input: i64) -> i64 {
        for range in &self.ranges {
            if let Some(output) = range.apply(input) {
                return output;
            }
        }
        input
    }
}

#[derive(Default)]
struct Almanac {
    seed_to_soil: RangeMap,
    soil_to_fertilizer: RangeMap,
    fertilizer_to_water: RangeMap,
    water_to_ligth: RangeMap,
    ligth_to_temperature: RangeMap,
    temperature_to_humidity: RangeMap,
    humidity_to_location: RangeMap,
}

impl Almanac {
    fn new<R: BufRead>(input: Lines<R>) -> Self {
        let mut output = Almanac::default();
        let mut current_map = None;
        for line in input {
            let line = line.unwrap();
            match line.as_str() {
                "" => (),
                "seed-to-soil map:" => current_map = Some(&mut output.seed_to_soil),
                "soil-to-fertilizer map:" => current_map = Some(&mut output.soil_to_fertilizer),
                "fertilizer-to-water map:" => current_map = Some(&mut output.fertilizer_to_water),
                "water-to-light map:" => current_map = Some(&mut output.water_to_ligth),
                "light-to-temperature map:" => current_map = Some(&mut output.ligth_to_temperature),
                "temperature-to-humidity map:" => {
                    current_map = Some(&mut output.temperature_to_humidity)
                }
                "humidity-to-location map:" => current_map = Some(&mut output.humidity_to_location),
                _ => current_map.as_mut().unwrap().feed(&line),
            }
        }
        output
    }

    fn resolve(&self, seeds: Vec<i64>) -> i64 {
        seeds
            .par_iter()
            .map(|seed| self.seed_to_soil.apply(*seed))
            .map(|soil| self.soil_to_fertilizer.apply(soil))
            .map(|fertilizer| self.fertilizer_to_water.apply(fertilizer))
            .map(|water| self.water_to_ligth.apply(water))
            .map(|ligth| self.ligth_to_temperature.apply(ligth))
            .map(|temperature| self.temperature_to_humidity.apply(temperature))
            .map(|humidity| self.humidity_to_location.apply(humidity))
            .min()
            .unwrap()
    }
}

fn main1(input: impl BufRead) -> Result<()> {
    let mut lines = input.lines();
    let first_line = lines.next().unwrap().unwrap();
    assert!(first_line.starts_with("seeds:"));
    let seeds = first_line
        .split_once(':')
        .unwrap()
        .1
        .trim()
        .split_ascii_whitespace()
        .map(|s| s.parse().unwrap())
        .collect::<Vec<_>>();
    let almanac = Almanac::new(lines);
    let min = almanac.resolve(seeds);
    println!("{}", min);
    Ok(())
}

fn main2(input: impl BufRead) -> Result<()> {
    let mut lines = input.lines();
    let first_line = lines.next().unwrap().unwrap();
    assert!(first_line.starts_with("seeds:"));
    let mut seeds = Vec::<i64>::new();
    let mut seeds_iter = first_line
        .split_once(':')
        .unwrap()
        .1
        .trim()
        .split_ascii_whitespace()
        .map(|s| s.parse().unwrap());
    while let Some(range_start) = seeds_iter.next() {
        for i in 0..seeds_iter.next().unwrap() {
            seeds.push(range_start + i);
        }
    }
    let almanac = Almanac::new(lines);
    let min = almanac.resolve(seeds);
    println!("{}", min);
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
