#![feature(io_read_to_string)]

use anyhow::{Context, Error, Result};
use std::{
    io::{read_to_string, stdin},
    str::FromStr,
};

fn main() -> Result<()> {
    let mut cave: Cave = read_to_string(&mut stdin())?.parse()?;

    cave.flow_down();
    println!("{}", cave.risk_level());
    println!("{}", cave.largest_basins());

    Ok(())
}

struct Cave {
    heights: Vec<u8>,
    smoke: Vec<u32>,
    width: usize,
}

impl Cave {
    fn flow_down(&mut self) {
        for k in (1..9).rev() {
            for (i, &h) in self.heights.iter().enumerate() {
                if h == k {
                    let down = self.neighbours(i).find(|&j| self.heights[j] < k);
                    if let Some(j) = down {
                        self.smoke[j] += self.smoke[i];
                        self.smoke[i] = 0;
                    };
                }
            }
        }
    }

    fn neighbours(&self, i: usize) -> impl Iterator<Item = usize> + '_ {
        (0..4).filter_map(move |j| match j {
            0 => i.checked_sub(self.width),
            1 if i % self.width > 0 => Some(i - 1),
            2 if (i + 1) % self.width > 0 => Some(i + 1),
            3 if i + self.width < self.heights.len() => Some(i + self.width),
            _ => None,
        })
    }

    fn risk_level(&self) -> u32 {
        self.heights
            .iter()
            .zip(&self.smoke)
            .filter_map(|(&h, &s)| (s > 0).then(|| h as u32 + 1))
            .sum()
    }

    fn largest_basins(&self) -> u32 {
        let mut max = [0; 3];
        let mut min = &mut max[0];
        for &s in &self.smoke {
            if s > *min {
                *min = s;
                min = max.iter_mut().min().unwrap();
            }
        }
        max.iter().product()
    }
}

impl FromStr for Cave {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut width = None;
        let mut heights = Vec::with_capacity(s.len());
        for line in s.lines() {
            if *width.get_or_insert(line.len()) != line.len() {
                return Err(Error::msg("all lines must have the same length"));
            }
            if !line.bytes().all(|c| c.is_ascii_digit()) {
                return Err(Error::msg("non-digit character in input line"));
            }
            heights.extend(line.bytes().map(|c| c - b'0'));
        }
        let smoke = heights.iter().map(|&h| (h != 9) as _).collect();
        let width = width.context("empty input")?;
        Ok(Self {
            heights,
            smoke,
            width,
        })
    }
}
