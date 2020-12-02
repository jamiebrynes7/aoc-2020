use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::{Context, Result};

fn main() -> Result<()> {
    let file = File::open("./src/input/day01.txt")?;
    let reader = BufReader::new(file);
    let input = reader
        .lines()
        .into_iter()
        .map(|line| {
            line.context("Failed to read line")?
                .parse::<i64>()
                .context("Failed to parse number")
        })
        .collect::<Result<Vec<i64>>>()?;

    part1(&input);
    part2(&input);

    Ok(())
}

fn part1(input: &[i64]) {
    'outer: for x in input {
        for y in input {
            if x + y == 2020 {
                println!("Found {} * {} = {}", x, y, x * y);
                break 'outer;
            }
        }
    }
}

fn part2(input: &[i64]) {
    'outer: for x in input {
        for y in input {
            for z in input {
                if x + y + z == 2020 {
                    println!("Found {} * {} * {} = {}", x, y, z, x * y * z);
                    break 'outer;
                }
            }
        }
    }
}
