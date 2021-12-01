use anyhow::Result;
use std::io::{stdin, BufRead};

fn main() -> Result<()> {
    let depths = stdin()
        .lock()
        .lines()
        .map(|line| Ok(line?.parse()?))
        .collect::<Result<Vec<i32>>>()?;
    println!("{}", count_increases(&depths));
    println!("{}", count_increases(&smoothed(&depths)));
    Ok(())
}

fn count_increases(data: &[i32]) -> usize {
    data.windows(2).filter(|&x| x[0] < x[1]).count()
}

fn smoothed(data: &[i32]) -> Vec<i32> {
    data.windows(3).map(|x| x.iter().sum()).collect()
}
