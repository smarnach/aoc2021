#![feature(io_read_to_string)]

use anyhow::{Context, Error, Result};
use std::{
    cmp::min,
    io::{read_to_string, stdin},
    str::FromStr,
};

fn main() -> Result<()> {
    let mut dumbos: Dumbos = read_to_string(&mut stdin())?.parse()?;

    let mut count = 0;
    for step in 1.. {
        count += dumbos.step();
        if step == 100 {
            println!("Part 1: {}", count);
        }
        if dumbos.sync {
            println!("Part 2: {}", step);
        }
        if step >= 100 && dumbos.sync {
            break;
        }
    }

    Ok(())
}

struct Dumbos {
    levels: Vec<u8>,
    width: usize,
    height: usize,
    sync: bool,
}

impl Dumbos {
    fn inc(&mut self, x: usize, y: usize) {
        let i = y * self.width + x;
        self.levels[i] += 1;
        if self.levels[i] == 10 {
            for y0 in y.saturating_sub(1)..min(y + 2, self.height) {
                for x0 in x.saturating_sub(1)..min(x + 2, self.width) {
                    self.inc(x0, y0);
                }
            }
        }
    }

    fn step(&mut self) -> usize {
        for y in 0..self.height {
            for x in 0..self.width {
                self.inc(x, y);
            }
        }
        let flashes = self.levels
            .iter_mut()
            .filter(|e| **e > 9)
            .map(|e| *e = 0)
            .count();
        if flashes == self.levels.len() {
            self.sync = true;
        }
        flashes
    }
}

impl FromStr for Dumbos {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut width = None;
        let mut levels = Vec::with_capacity(s.len());
        for line in s.lines() {
            if *width.get_or_insert(line.len()) != line.len() {
                return Err(Error::msg("all lines must have the same length"));
            }
            if !line.bytes().all(|c| c.is_ascii_digit()) {
                return Err(Error::msg("non-digit character in input line"));
            }
            levels.extend(line.bytes().map(|c| c - b'0'));
        }
        let width = width.context("empty input")?;
        let height = levels.len() / width;
        Ok(Self {
            levels,
            width,
            height,
            sync: false,
        })
    }
}
