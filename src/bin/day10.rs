use anyhow::{Error, Result};
use std::io::{stdin, BufRead};

fn main() -> Result<()> {
    let mut syntax = 0;
    let mut complete = vec![];
    for line in stdin().lock().lines() {
        match syntax_score(&line?)? {
            (0, stack) => complete.push(complete_score(stack)?),
            (score, _) => syntax += score,
        }
    }
    println!("{}", syntax);
    let len = complete.len();
    println!("{}", complete.select_nth_unstable(len / 2).1);

    Ok(())
}

fn syntax_score(line: &str) -> Result<(u64, Vec<char>)> {
    let mut stack = vec![];
    for c in line.chars() {
        match c {
            '(' => stack.push(')'),
            '[' => stack.push(']'),
            '{' => stack.push('}'),
            '<' => stack.push('>'),
            _ => {
                if stack.pop() != Some(c) {
                    let score = match c {
                        ')' => 3,
                        ']' => 57,
                        '}' => 1197,
                        '>' => 25137,
                        _ => return Err(Error::msg("invalid character in input line")),
                    };
                    return Ok((score, stack));
                }
            }
        }
    }
    Ok((0, stack))
}

fn complete_score(stack: Vec<char>) -> Result<u64> {
    stack.into_iter().try_rfold(0, |score, c| {
        Ok(5 * score
            + match c {
                ')' => 1,
                ']' => 2,
                '}' => 3,
                '>' => 4,
                _ => return Err(Error::msg("invalid character in input line")),
            })
    })
}
