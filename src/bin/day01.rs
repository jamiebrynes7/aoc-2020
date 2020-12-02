use anyhow::{Context, Result};
use aoc_2020::input_lines;

fn main() -> Result<()> {
    let input = input_lines(1)?
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
