#![feature(io_read_to_string)]

use anyhow::{Context, Error, Result};
use std::io::{read_to_string, stdin};

fn main() -> Result<()> {
    let values = parse_input(&read_to_string(&mut stdin())?)?;

    let len = values[0].len();
    let most_common: Vec<_> = (0..len).map(|i| most_common_bit(&values, i)).collect();
    let gamma = bits_to_int(most_common.iter().copied());
    let epsilon = bits_to_int(most_common.iter().map(|b| b ^ 1));
    println!("{}", gamma * epsilon);

    let oxygen_generator_rating = bits_to_int(filter(values.clone(), 0)?);
    let co2_scrubber_rating = bits_to_int(filter(values, 1)?);
    println!("{}", oxygen_generator_rating * co2_scrubber_rating);

    Ok(())
}

/// Return the most common bit and the given index. In case of a tie, return 1.
fn most_common_bit(values: &[Vec<u8>], index: usize) -> u8 {
    let ones: usize = values.iter().map(|v| v[index] as usize).sum();
    (ones >= (values.len() + 1) / 2) as u8
}

fn bits_to_int<I: IntoIterator<Item = u8>>(v: I) -> u32 {
    v.into_iter().fold(0, |i, b| i << 1 | b as u32)
}

fn filter(mut values: Vec<Vec<u8>>, invert: u8) -> Result<Vec<u8>> {
    for i in 0..values[0].len() {
        let bit = most_common_bit(&values, i) ^ invert;
        values.retain(|v| v[i] == bit);
        if values.len() <= 1 {
            break;
        }
    }
    values.pop().context("filtering resulted in empty set")
}

fn parse_input(input: &str) -> Result<Vec<Vec<u8>>> {
    let values = input.lines().map(parse_line).collect::<Result<Vec<_>>>()?;
    if values.is_empty() {
        return Err(Error::msg("empty input"));
    }
    if values[0].len() > 32 {
        return Err(Error::msg("input numbers can't have more than 32 bits"));
    }
    if values.iter().any(|v| v.len() != values[0].len()) {
        return Err(Error::msg(
            "input numbers must all have the same bit length",
        ));
    }
    Ok(values)
}

fn parse_line(line: &str) -> Result<Vec<u8>> {
    line.chars()
        .map(|c| match c {
            '0' => Ok(0),
            '1' => Ok(1),
            _ => Err(Error::msg("invalid input line")),
        })
        .collect()
}
