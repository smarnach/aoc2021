#![feature(io_read_to_string)]

use anyhow::{Context, Result};
use std::io::{read_to_string, stdin};

fn main() -> Result<()> {
    let values = parse_input(&read_to_string(&mut stdin())?)?;
    let len = 32 - values.iter().fold(0, |i, v| i | v).leading_zeros() as usize;

    let bits: Vec<_> = (0..len).map(|i| most_common_bit(&values, i)).collect();
    let gamma = bits_to_int(bits.iter().copied());
    let epsilon = bits_to_int(bits.iter().map(|b| b ^ 1));
    println!("{}", gamma * epsilon);

    let oxygen_generator_rating = filter(values.clone(), len, 0)?;
    let co2_scrubber_rating = filter(values, len, 1)?;
    println!("{}", oxygen_generator_rating * co2_scrubber_rating);

    Ok(())
}

/// Return the most common bit and the given index. In case of a tie, return 1.
fn most_common_bit(values: &[u32], index: usize) -> u8 {
    let ones: usize = values.iter().map(|v| (v >> index) as usize & 1).sum();
    (ones >= (values.len() + 1) / 2) as u8
}

fn bits_to_int<I: IntoIterator<Item = u8>>(bits: I) -> u32 {
    bits.into_iter().fold(0, |i, b| i << 1 | b as u32)
}

fn filter(mut values: Vec<u32>, len: usize, invert: u8) -> Result<u32> {
    for index in (0..len).rev() {
        let bit = most_common_bit(&values, index) ^ invert;
        values.retain(|v| (v >> index) as u8 & 1 == bit);
        if values.len() <= 1 {
            break;
        }
    }
    values.pop().context("filtering resulted in empty set")
}

fn parse_input(input: &str) -> Result<Vec<u32>> {
    input
        .lines()
        .map(|line| Ok(u32::from_str_radix(line, 2)?))
        .collect::<Result<Vec<_>>>()
}
