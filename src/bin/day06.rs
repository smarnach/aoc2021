#![feature(bool_to_option, io_read_to_string)]

use anyhow::{Context, Result};
use std::io::{read_to_string, stdin};

fn main() -> Result<()> {
    let fish = parse_input(&read_to_string(&mut stdin())?)?;

    let mut school = School::new(&fish);
    school.step(80);
    println!("{}", school.count());
    school.step(256 - 80);
    println!("{}", school.count());

    Ok(())
}

#[derive(Debug)]
struct School {
    counts: [u64; 9],
}

impl School {
    fn new(fish: &[u8]) -> Self {
        let mut counts = [0; 9];
        for &f in fish {
            counts[f as usize] += 1;
        }
        Self { counts }
    }

    fn step(&mut self, count: u32) {
        for _ in 0..count {
            self.counts.rotate_left(1);
            self.counts[6] += self.counts[8];
        }
    }

    fn count(&self) -> u64 {
        self.counts.iter().sum()
    }
}

fn parse_input(input: &str) -> Result<Vec<u8>> {
    let parse_int = |s: &str| -> Result<u8> {
        let i = s.parse()?;
        (i < 9)
            .then_some(i)
            .context("invalid internal lanternfish timer")
    };
    input.trim().split(',').map(parse_int).collect()
}
