#![feature(array_windows, io_read_to_string)]

use anyhow::{Context, Error, Result};
use std::{fmt, io};

fn main() -> Result<()> {
    let input = io::read_to_string(&mut io::stdin())?;
    let (mut paper, axes) = parse_input(&input)?;

    paper.fold(&axes[0]);
    println!("{}", paper.count());
    for a in &axes[1..] {
        paper.fold(a);
    }
    paper.count();
    println!("{}", paper);

    Ok(())
}

struct Paper {
    dots: Vec<[i32; 2]>,
}

impl Paper {
    fn fold(&mut self, axis: &Axis) {
        let i = axis.x_axis as usize;
        for yx in self.dots.iter_mut() {
            if yx[i] > axis.pos {
                yx[i] = 2 * axis.pos - yx[i];
            }
        }
    }

    fn count(&mut self) -> usize {
        self.dots.sort_unstable();
        self.dots.dedup();
        self.dots.len()
    }
}

impl fmt::Display for Paper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for &[[y0, x0], [y1, x1]] in self.dots.array_windows() {
            if y0 == y1 {
                write!(f, "{:1$}", "█", (x1 - x0) as usize)?;
            } else {
                write!(f, "{:\n<1$}", "█", (y1 - y0 + 1) as usize)?;
                write!(f, "{:1$}", "", x1 as usize)?;
            }
        }
        writeln!(f, "█")
    }
}

struct Axis {
    x_axis: bool,
    pos: i32,
}

fn parse_input(input: &str) -> Result<(Paper, Vec<Axis>)> {
    let (dots, axes) = input.split_once("\n\n").context("invalid input format")?;
    let dots = dots
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(",").context("invalid dot line")?;
            Ok([y.parse()?, x.parse()?])
        })
        .collect::<Result<_>>()?;
    let axes = axes
        .lines()
        .map(|line| {
            let line = line
                .strip_prefix("fold along ")
                .context("invalid fold line")?;
            let (x_axis, pos) = line.split_once("=").context("invalid fold line")?;
            let x_axis = x_axis == "x";
            let pos = pos.parse()?;
            Ok(Axis { x_axis, pos })
        })
        .collect::<Result<Vec<_>>>()?;
    if axes.is_empty() {
        return Err(Error::msg("input must contain at least one fold axis"));
    }
    Ok((Paper { dots }, axes))
}
