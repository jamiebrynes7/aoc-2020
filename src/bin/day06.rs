use anyhow::{Context, Result};
use aoc_2020::input_lines;

fn main() -> Result<()> {
    let mut input = input_lines(6)?.peekable();
    let mut chunks = Vec::new();

    while input.peek().is_some() {
        let chunk = input
            .by_ref()
            .take_while(|line| line.is_ok() && !line.as_ref().unwrap().is_empty())
            .map(|line| line.context("Failed to read line"))
            .collect::<Result<Vec<String>>>()?;
        chunks.push(chunk);
    }

    part1(&chunks);
    part2(&chunks);

    Ok(())
}

fn part1(chunks: &[Vec<String>]) {
    fn generate_mask(input: &Vec<String>) -> u32 {
        let mut value = 0;

        for line in input {
            for c in line.chars() {
                value = value | (1 << c as u32 - 'a' as u32);
            }
        }

        value
    }

    let count: u32 = chunks
        .iter()
        .map(|value| generate_mask(value).count_ones())
        .sum();

    println!("Total sum: {}", count);
}

fn part2(chunks: &[Vec<String>]) {
    fn generate_mask(input: &Vec<String>) -> u32 {
        let mut value = u32::MAX;

        for line in input {
            let mut inner = 0;
            for c in line.chars() {
                inner = inner | (1 << c as u32 - 'a' as u32);
            }
            value = value & inner;
        }

        value
    }

    let count: u32 = chunks
        .iter()
        .map(|value| generate_mask(value).count_ones())
        .sum();

    println!("Total sum: {}", count);
}
