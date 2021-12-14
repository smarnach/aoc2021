#![feature(array_windows, io_read_to_string)]

use anyhow::{Context, Error, Result};
use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    io,
    str::FromStr,
};

fn main() -> Result<()> {
    let mut polymer: Polymer = io::read_to_string(&mut io::stdin())?.parse()?;

    polymer.step(10);
    println!("{}", polymer.score());
    polymer.step(30);
    println!("{}", polymer.score());

    Ok(())
}

struct Polymer {
    first: u8,
    rules: HashMap<[u8; 2], u8>,
    pairs: HashMap<[u8; 2], usize>,
}

impl Polymer {
    fn step(&mut self, steps: usize) {
        for _ in 0..steps {
            self.pairs = std::mem::take(&mut self.pairs)
                .into_iter()
                .flat_map(|(pair, count)| {
                    let insert = self.rules[&pair];
                    [([pair[0], insert], count), ([insert, pair[1]], count)]
                })
                .count_by_key();
        }
    }

    fn score(&self) -> usize {
        let counts = self
            .pairs
            .iter()
            .map(|(&[_, y], &count)| (y, count))
            .chain([(self.first, 1)])
            .count_by_key();
        counts.values().max().unwrap() - counts.values().min().unwrap()
    }
}

trait Counter<K> {
    fn count_by_key(self) -> HashMap<K, usize>;
}

impl<I, K> Counter<K> for I
where
    I: IntoIterator<Item = (K, usize)>,
    K: Eq + Hash,
{
    fn count_by_key(self) -> HashMap<K, usize> {
        let mut map = HashMap::new();
        for (k, count) in self {
            *map.entry(k).or_default() += count;
        }
        map
    }
}

fn validate_monomers(polymer: &str) -> Result<()> {
    if !polymer.bytes().all(|c| c.is_ascii_uppercase()) {
        return Err(Error::msg("monomers must be capital letters"));
    }
    Ok(())
}

fn parse_rule(line: &str) -> Result<([u8; 2], u8)> {
    let (pair, rule) = line
        .split_once(" -> ")
        .context("invalid pair insertion rule format")?;
    if (pair.len(), rule.len()) != (2, 1) {
        return Err(Error::msg("invalid pair insertion rule format"));
    }
    validate_monomers(pair)?;
    validate_monomers(rule)?;
    let (pair, rule) = (pair.as_bytes(), rule.as_bytes());
    Ok(([pair[0], pair[1]], rule[0]))
}

fn validate_rules(start: &str, rules: &HashMap<[u8; 2], u8>) -> Result<()> {
    let mut monomers = HashSet::<_>::from_iter(start.as_bytes().iter().copied());
    monomers.extend(rules.values().copied());
    for &x in &monomers {
        for &y in &monomers {
            if !rules.contains_key(&[x, y]) {
                return Err(Error::msg("incomplete pair insertion rules"));
            }
        }
    }
    Ok(())
}

impl FromStr for Polymer {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, rules) = s.split_once("\n\n").context("invalid input format")?;
        if start.is_empty() {
            return Err(Error::msg("empty start sequence"));
        }
        validate_monomers(start)?;
        let rules = rules.lines().map(parse_rule).collect::<Result<_>>()?;
        validate_rules(start, &rules)?;
        let pairs = start
            .as_bytes()
            .array_windows()
            .map(|&pair| (pair, 1))
            .count_by_key();
        Ok(Self {
            first: start.as_bytes()[0],
            rules,
            pairs,
        })
    }
}
