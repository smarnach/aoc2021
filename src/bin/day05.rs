#![feature(io_read_to_string, iter_partition_in_place)]

use anyhow::{Context, Error, Result};
use std::{
    collections::HashMap,
    io::{read_to_string, stdin},
    str::FromStr,
};

fn main() -> Result<()> {
    let mut lines = parse_input(&read_to_string(&mut stdin())?)?;

    let straight = lines
        .iter_mut()
        .partition_in_place(|line| line.x0 == line.x1 || line.y0 == line.y1);
    let mut counts = HashMap::new();
    count_points(&mut counts, &lines[..straight]);
    println!("{}", counts.values().filter(|&&c| c > 1).count());
    count_points(&mut counts, &lines[straight..]);
    println!("{}", counts.values().filter(|&&c| c > 1).count());

    Ok(())
}

fn count_points(counts: &mut HashMap<(i32, i32), u32>, lines: &[Line]) {
    for line in lines {
        for p in line.points() {
            *counts.entry(p).or_default() += 1;
        }
    }
}

#[derive(Debug)]
struct Line {
    x0: i32,
    y0: i32,
    x1: i32,
    y1: i32,
}

impl Line {
    fn points(&self) -> impl Iterator<Item = (i32, i32)> + '_ {
        let dx = self.x1 - self.x0;
        let dy = self.y1 - self.y0;
        let len = std::cmp::max(dx.abs(), dy.abs());
        (0..=len).map(move |t| (self.x0 + sign(dx) * t, self.y0 + sign(dy) * t))
    }
}

fn sign(x: i32) -> i32 {
    (x > 0) as i32 - (x < 0) as i32
}

impl FromStr for Line {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (p0, p1) = s.split_once(" -> ").context("invalid input line format")?;
        let parse_point = |p: &str| -> Result<(i32, i32)> {
            let (x, y) = p.split_once(",").context("invalid input line format")?;
            Ok((x.parse()?, y.parse()?))
        };
        let (x0, y0) = parse_point(p0)?;
        let (x1, y1) = parse_point(p1)?;
        Ok(Self { x0, y0, x1, y1 })
    }
}

fn parse_input(input: &str) -> Result<Vec<Line>> {
    input.lines().map(str::parse).collect()
}
