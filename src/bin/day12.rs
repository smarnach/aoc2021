#![feature(io_read_to_string)]

use anyhow::{Context, Error, Result};
use rustc_hash::FxHashMap as HashMap;
use std::io::{read_to_string, stdin};

fn main() -> Result<()> {
    let input = read_to_string(&mut stdin())?;
    let graph = parse_input(&input)?;
    println!("{}", graph.count_paths(false));
    println!("{}", graph.count_paths(true));
    Ok(())
}

struct Graph<'a>(HashMap<&'a str, Vec<&'a str>>);

impl<'a> Graph<'a> {
    fn count_paths(&self, twice: bool) -> usize {
        let mut count = 0;
        let mut stack = vec![("start", None, twice)];
        while !stack.is_empty() {
            let &mut (node, ref mut iter, twice) = stack.last_mut().unwrap();
            let iter = iter.get_or_insert_with(|| self.0.get(node).unwrap().iter());
            match iter.next().copied() {
                Some(n) => {
                    if n == "end" {
                        count += 1;
                    } else if is_big_cave(n) || stack.iter().all(|&(m, _, _)| n != m) {
                        stack.push((n, None, twice));
                    } else if twice && n != "start" {
                        stack.push((n, None, false));
                    }
                }
                None => {
                    stack.pop();
                }
            }
        }
        count
    }
}

fn is_big_cave(n: &str) -> bool {
    n.as_bytes()[0].is_ascii_uppercase()
}

fn parse_node(n: &str) -> Result<&str> {
    let n = n.trim();
    if n.is_empty() {
        return Err(Error::msg("empty node name"));
    }
    if !n.chars().all(|c| c.is_ascii_lowercase()) && !n.chars().all(|c| c.is_ascii_uppercase()) {
        return Err(Error::msg(
            "node names must be all uppercase or all lowercase",
        ));
    }
    Ok(n)
}

fn parse_input(s: &str) -> Result<Graph, Error> {
    let mut graph = HashMap::<&str, Vec<&str>>::default();
    for line in s.lines() {
        let (m, n) = line.split_once('-').context("invalid in put line format")?;
        let m = parse_node(m)?;
        let n = parse_node(n)?;
        if is_big_cave(m) && is_big_cave(n) {
            return Err(Error::msg("two big caves cannot be adjacent"));
        }
        graph.entry(m).or_default().push(n);
        graph.entry(n).or_default().push(m);
    }
    if !graph.contains_key("start") || !graph.contains_key("end") {
        return Err(Error::msg("cave system must contain start and end caves"));
    }
    Ok(Graph(graph))
}
