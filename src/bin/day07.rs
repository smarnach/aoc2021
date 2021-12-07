#![feature(bool_to_option, io_read_to_string)]

use anyhow::{Error, Result};
use std::{
    cmp::min,
    convert::identity,
    io::{read_to_string, stdin},
};

fn main() -> Result<()> {
    let mut a = parse_input(&read_to_string(&mut stdin())?)?;
    let len = a.len();

    let median = *a.select_nth_unstable(len / 2).1;
    println!("{}", fuel(&a, median, identity));
    let mean = a.iter().sum::<i32>() / len as i32;
    println!("{}", min(fuel(&a, mean, tri), fuel(&a, mean + 1, tri)));

    Ok(())
}

fn fuel(a: &[i32], x: i32, metric: impl Fn(i32) -> i32) -> i32 {
    a.iter().map(|y| (x - y).abs()).map(metric).sum()
}

fn tri(x: i32) -> i32 {
    x * (x + 1) / 2
}

fn parse_input(input: &str) -> Result<Vec<i32>> {
    let a = input
        .trim()
        .split(',')
        .map(str::parse)
        .collect::<Result<Vec<_>, _>>()?;
    if a.is_empty() {
        return Err(Error::msg("empty input"));
    }
    Ok(a)
}
