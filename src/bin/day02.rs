use anyhow::{Context, Error, Result};
use std::{io::BufRead, str::FromStr};

fn main() -> Result<()> {
    let commands = std::io::stdin()
        .lock()
        .lines()
        .map(|line| line?.parse())
        .collect::<Result<Vec<Command>>>()?;
    println!("{}", State1::run(&commands));
    println!("{}", State2::run(&commands));
    Ok(())
}

enum Command {
    Forward(i32),
    Down(i32),
    Up(i32),
}

use Command::*;

impl FromStr for Command {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (action, amount) = s.split_once(' ').context("invalid input line")?;
        let amount = amount.parse()?;
        let command = match action {
            "forward" => Forward(amount),
            "down" => Down(amount),
            "up" => Up(amount),
            _ => return Err(Error::msg("invalid action")),
        };
        Ok(command)
    }
}

trait Runner: Default {
    fn step(&mut self, command: &Command);

    fn result(&self) -> i32;

    fn run(commands: &[Command]) -> i32 {
        let mut state = Self::default();
        for c in commands {
            state.step(c);
        }
        state.result()
    }
}

#[derive(Debug, Default)]
struct State1 {
    pos: i32,
    depth: i32,
}

impl Runner for State1 {
    fn step(&mut self, command: &Command) {
        match command {
            Forward(i) => self.pos += i,
            Down(i) => self.depth += i,
            Up(i) => self.depth -= i,
        }
    }

    fn result(&self) -> i32 {
        self.pos * self.depth
    }
}

#[derive(Debug, Default)]
struct State2 {
    pos: i32,
    aim: i32,
    depth: i32,
}

impl Runner for State2 {
    fn step(&mut self, command: &Command) {
        match command {
            Forward(i) => {
                self.pos += i;
                self.depth += self.aim * i;
            }
            Down(i) => self.aim += i,
            Up(i) => self.aim -= i,
        }
    }

    fn result(&self) -> i32 {
        self.pos * self.depth
    }
}
