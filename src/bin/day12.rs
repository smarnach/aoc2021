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
        let mut path = vec!["start"];
        let mut stack = vec![(self.get_iter("start"), twice)];
        while !stack.is_empty() {
            let &mut (ref mut iter, twice) = stack.last_mut().unwrap();
            match iter.next().copied() {
                Some(n) => {
                    if n == "end" {
                        count += 1;
                    } else if is_big_cave(n) || !path.contains(&n) {
                        path.push(n);
                        stack.push((self.get_iter(n), twice));
                    } else if twice && n != "start" {
                        path.push(n);
                        stack.push((self.get_iter(n), false));
                    }
                }
                None => {
                    path.pop();
                    stack.pop();
                }
            }
        }
        count
    }

    fn get_iter(&self, node: &'a str) -> std::slice::Iter<'_, &'a str> {
        self.0.get(node).unwrap().iter()
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
