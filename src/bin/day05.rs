use std::{ops::Range, str::FromStr};

use aoc_2020::input_lines;

use anyhow::{Context, Result};

fn main() -> Result<()> {
    let input = input_lines(5)?
        .map(|line| {
            let line = line.context("Failed to read line")?;
            SeatId::from_str(&line)
        })
        .collect::<Result<Vec<SeatId>>>()?;

    part1(&input);
    part2(&input);

    Ok(())
}

fn part1(input: &[SeatId]) {
    let max = input.iter().map(|id| id.seat_id()).max();
    println!("Maximum seat ID: {}", max.unwrap());
}

fn part2(input: &[SeatId]) {
    let mut ids = input.iter().map(|id| id.seat_id()).collect::<Vec<i32>>();
    ids.sort();

    let mut expected_id = 0;

    for id in ids {
        if expected_id != id {
            while expected_id != id {
                println!("Missing ID: {}", expected_id);
                expected_id += 1;
            }
        }

        expected_id += 1;
    }
}

struct SeatId {
    row_id: i32,
    column_id: i32,
}

impl SeatId {
    pub fn seat_id(&self) -> i32 {
        self.row_id * 8 + self.column_id
    }
}

impl FromStr for SeatId {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars = s.chars();

        let row_sequence = chars.clone().take(7);
        let column_sequence = chars.skip(7).take(3);

        let row_id = collapse_range(0, 127, 'B', 'F', row_sequence);
        let column_id = collapse_range(0, 7, 'R', 'L', column_sequence);

        Ok(SeatId { row_id, column_id })
    }
}

fn collapse_range<T: Iterator<Item = char>>(
    min: i32,
    max: i32,
    sel_upper: char,
    sel_lower: char,
    sequence: T,
) -> i32 {
    sequence
        .fold(
            Range {
                start: min,
                end: max,
            },
            |range, operator| match operator {
                c if c == sel_lower => Range {
                    start: range.start,
                    end: (range.start + range.end) / 2,
                },
                c if c == sel_upper => Range {
                    start: round_up(range.start, range.end),
                    end: range.end,
                },
                _ => panic!("Unknown operator: {}", operator),
            },
        )
        .start
}

fn round_up(first: i32, second: i32) -> i32 {
    (((first as f64 + second as f64) / 2.0) + 0.5) as i32
}
