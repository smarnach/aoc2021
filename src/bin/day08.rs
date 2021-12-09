#![feature(io_read_to_string)]

use anyhow::{Context, Result};
use lazy_static::lazy_static;
use std::{
    collections::HashMap,
    io::{read_to_string, stdin},
};

fn main() -> Result<()> {
    let input = read_to_string(&mut stdin())?;
    let displays = parse_input(&input)?;

    let mut easy_digits = 0;
    let mut total = 0;
    for (patterns, digits) in displays {
        let counts = SegmentCounts::new(&patterns);
        let decoded = counts.decode(&digits).context("inconsistent input line")?;
        easy_digits += decoded.iter().filter(|d| [1, 4, 7, 8].contains(d)).count();
        total += decoded.into_iter().fold(0, |acc, d| 10 * acc + d);
    }
    println!("{}", easy_digits);
    println!("{}", total);

    Ok(())
}

struct SegmentCounts(HashMap<char, u32>);

impl SegmentCounts {
    fn new(patterns: &[&str]) -> Self {
        let mut counts = HashMap::new();
        for &p in patterns {
            for c in p.chars() {
                *counts.entry(c).or_default() += 1;
            }
        }
        Self(counts)
    }

    fn discriminant(&self, digit: &str) -> Option<u32> {
        digit.chars().map(|c| self.0.get(&c)).sum()
    }

    fn decode(&self, digits: &[&str]) -> Option<Vec<u32>> {
        digits
            .iter()
            .map(|d| self.discriminant(d).and_then(discriminant_to_digit))
            .collect()
    }
}

fn discriminant_to_digit(disc: u32) -> Option<u32> {
    const PATTERNS: [&str; 10] = [
        "abcefg", "cf", "acdeg", "acdfg", "bcdf", "abdfg", "abdefg", "acf", "abcdefg", "abcdfg",
    ];
    lazy_static! {
        static ref KEY: Vec<u32> = {
            let counts = SegmentCounts::new(&PATTERNS);
            let map = PATTERNS
                .iter()
                .map(|d| counts.discriminant(d).unwrap())
                .collect();
            map
        };
    }
    KEY.iter().position(|&x| x == disc).map(|d| d as u32)
}

fn parse_input(input: &str) -> Result<Vec<(Vec<&str>, Vec<&str>)>> {
    input
        .lines()
        .map(|line| {
            let (patterns, digits) = line.split_once(" | ").context("malformed input line")?;
            Ok((
                patterns.split_whitespace().collect(),
                digits.split_whitespace().collect(),
            ))
        })
        .collect()
}
