#![feature(io_read_to_string, iter_partition_in_place)]

use anyhow::{Context, Error, Result};
use std::{
    io::{read_to_string, stdin},
    str::FromStr,
};

const SIZE: usize = 1000;

fn main() -> Result<()> {
    let mut lines = parse_input(&read_to_string(&mut stdin())?)?;

    let straight = lines
        .iter_mut()
        .partition_in_place(|line| line.x0 == line.x1 || line.y0 == line.y1);
    let mut counts = vec![0; SIZE * SIZE];
    count_points(&mut counts, &lines[..straight]);
    println!("{}", counts.iter().filter(|&&c| c > 1).count());
    count_points(&mut counts, &lines[straight..]);
    println!("{}", counts.iter().filter(|&&c| c > 1).count());

    Ok(())
}

fn count_points(counts: &mut [u32], lines: &[Line]) {
    for line in lines {
        for (x, y) in line.points() {
            counts[SIZE * y as usize + x as usize] += 1;
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
        let parse_int = |i: &str| -> Result<i32> {
            let i = i.parse()?;
            if 0 <= i && i < SIZE as i32 {
                Ok(i)
            } else {
                Err(Error::msg("coordinate out of range 0..1000"))
            }
        };
        let parse_point = |p: &str| -> Result<(i32, i32)> {
            let (x, y) = p.split_once(",").context("invalid input line format")?;
            Ok((parse_int(x)?, parse_int(y)?))
        };
        let (p0, p1) = s.split_once(" -> ").context("invalid input line format")?;
        let (x0, y0) = parse_point(p0)?;
        let (x1, y1) = parse_point(p1)?;
        Ok(Self { x0, y0, x1, y1 })
    }
}

fn parse_input(input: &str) -> Result<Vec<Line>> {
    input.lines().map(str::parse).collect()
}
