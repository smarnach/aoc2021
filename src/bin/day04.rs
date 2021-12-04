#![feature(bool_to_option, io_read_to_string)]

use anyhow::{Context, Error, Result};
use std::{
    collections::HashSet,
    io::{read_to_string, stdin},
    str::FromStr,
};

fn main() -> Result<()> {
    let (calls, mut boards) = parse_input(&read_to_string(&mut stdin())?)?;
    let boards_len = boards.len();

    let mut winning_boards = HashSet::new();
    let mut first_score = None;
    let mut last_score = None;
    'outer: for call in calls {
        for (i, board) in boards.iter_mut().enumerate() {
            if board.mark(call) && !winning_boards.contains(&i) && board.has_won() {
                winning_boards.insert(i);
                first_score.get_or_insert_with(|| board.score(call));
                if winning_boards.len() == boards_len {
                    last_score = Some(board.score(call));
                    break 'outer;
                }
            }
        }
    }
    println!("{}", first_score.context("no board won")?);
    println!("{}", last_score.context("not all boards won")?);

    Ok(())
}

struct Board {
    numbers: Vec<u32>,
    marks: u32,
}

static ROWS_N_COLS: [u32; 10] = [
    0b11111,
    0b11111 << 5,
    0b11111 << 10,
    0b11111 << 15,
    0b11111 << 20,
    0b100001000010000100001,
    0b100001000010000100001 << 1,
    0b100001000010000100001 << 2,
    0b100001000010000100001 << 3,
    0b100001000010000100001 << 4,
];

impl Board {
    fn has_won(&self) -> bool {
        ROWS_N_COLS.iter().any(|&w| self.marks & w == w)
    }

    fn mark(&mut self, call: u32) -> bool {
        for (i, &number) in self.numbers.iter().enumerate() {
            if number == call {
                self.marks |= 1 << i;
                return true;
            }
        }
        false
    }

    fn score(&self, call: u32) -> u32 {
        self.numbers
            .iter()
            .enumerate()
            .filter_map(|(i, &number)| (self.marks & (1 << i) == 0).then_some(number))
            .sum::<u32>()
            * call
    }
}

impl FromStr for Board {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            numbers: s
                .split_whitespace()
                .map(|x| Ok(x.parse()?))
                .collect::<Result<_>>()?,
            marks: Default::default(),
        })
    }
}

fn parse_input(input: &str) -> Result<(Vec<u32>, Vec<Board>)> {
    let (first_line, rest) = input.split_once("\n\n").context("premature end of input")?;
    let calls = first_line
        .split(',')
        .map(|x| Ok(x.parse()?))
        .collect::<Result<_>>()?;
    let boards = rest
        .split("\n\n")
        .map(|b| b.parse())
        .collect::<Result<_>>()?;
    Ok((calls, boards))
}
