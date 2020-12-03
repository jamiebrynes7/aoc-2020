use anyhow::{Context, Result};
use aoc_2020::input_lines;

fn main() -> Result<()> {
    let input = input_lines(3)?
        .map(|line| line.context("Failed to read line"))
        .collect::<Result<Vec<String>>>()?;

    part1(&input);
    part2(&input);
    Ok(())
}

fn part1(input: &[String]) {
    println!("Hit {} trees", count(3, 1, input));
}

fn part2(input: &[String]) {
    let total = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .fold(1, |total, slope| total * count(slope.0, slope.1, input));

    println!("Combined the totals: {}", total);
}

fn count(x_increment: usize, y_increment: usize, input: &[String]) -> u64 {
    let mut x = 0;
    let mut y = 0;
    let mut count = 0;

    while y + y_increment < input.len() {
        y += y_increment;
        x += x_increment;

        if input[y].chars().cycle().nth(x) == Some('#') {
            count += 1;
        }
    }

    count
}
