#![feature(io_read_to_string)]

use anyhow::{Context, Error, Result};
use itertools::iproduct;
use std::{
    cmp::Reverse,
    collections::BinaryHeap,
    io::{read_to_string, stdin},
    str::FromStr,
};

fn main() -> Result<()> {
    let mut cave: Cave = read_to_string(&mut stdin())?.parse()?;
    println!("{}", cave.least_risky_path());
    cave.tile();
    println!("{}", cave.least_risky_path());
    Ok(())
}

struct Cave {
    risk: Vec<u8>,
    width: usize,
}

impl Cave {
    fn least_risky_path(&self) -> u32 {
        let mut path_risk = vec![u32::MAX; self.risk.len()];
        path_risk[0] = 0;
        let mut queue = BinaryHeap::new();
        queue.push(Reverse((0, 0)));
        while let Some(Reverse((_, i))) = queue.pop() {
            for j in self.neighbours(i) {
                let new_risk = path_risk[i] + self.risk[j] as u32;
                if new_risk < path_risk[j] {
                    path_risk[j] = new_risk;
                    queue.push(Reverse((new_risk, j)));
                }
            }
        }
        *path_risk.last().unwrap()
    }

    fn tile(&mut self) {
        let height = self.risk.len() / self.width;
        self.risk = iproduct!(0..5, 0..height, 0..5, 0..self.width)
            .map(|(tile_y, y, tile_x, x)| {
                (self.risk[y * self.width + x] + tile_x + tile_y - 1) % 9 + 1
            })
            .collect();
        self.width *= 5;
    }

    fn neighbours(&self, i: usize) -> impl Iterator<Item = usize> + '_ {
        (0..4).filter_map(move |j| match j {
            0 => i.checked_sub(self.width),
            1 if i % self.width > 0 => Some(i - 1),
            2 if (i + 1) % self.width > 0 => Some(i + 1),
            3 if i + self.width < self.risk.len() => Some(i + self.width),
            _ => None,
        })
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
            if !line.bytes().all(|c| (b'1'..=b'9').contains(&c)) {
                return Err(Error::msg("non-digit character in input line"));
            }
            heights.extend(line.bytes().map(|c| c - b'0'));
        }
        let width = width.context("empty input")?;
        Ok(Self {
            risk: heights,
            width,
        })
    }
}
