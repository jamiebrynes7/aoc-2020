use std::{collections::HashSet, str::FromStr};

use anyhow::{Context, Result};
use aoc_2020::input_lines;

fn main() -> Result<()> {
    let input = input_lines(8)?
        .map(|line| {
            let line = line.context("Failed to read line")?;
            OpCode::from_str(&line)
        })
        .collect::<Result<Vec<OpCode>>>()?;

    part1(&input);
    part2(&input);

    Ok(())
}

fn part1(input: &[OpCode]) {
    println!("Final acc value: {}", run_machine(input).0);
}

fn part2(input: &[OpCode]) {
    let mut i = 0;
    loop {
        let mut cloned = input
            .iter()
            .map(|code| code.clone())
            .collect::<Vec<OpCode>>();

        while let OpCode::Acc(_) = cloned[i] {
            i += 1;
        }

        // Change the next jmp/nop execution.
        cloned[i] = match cloned[i].clone() {
            OpCode::Nop(val) => OpCode::Jmp(val),
            OpCode::Acc(val) => OpCode::Acc(val),
            OpCode::Jmp(val) => OpCode::Nop(val),
        };

        i += 1;

        let (acc_result, end_cursor) = run_machine(&cloned);

        if end_cursor == cloned.len() {
            println!(
                "Found permutation that finishes execution. Final acc value: {}",
                acc_result
            );
            break;
        }
    }
}

fn run_machine(input: &[OpCode]) -> (isize, usize) {
    let mut visited = HashSet::new();
    let mut cursor: usize = 0;
    let mut acc: isize = 0;

    while !visited.contains(&cursor) && cursor < input.len() {
        visited.insert(cursor);
        match input[cursor] {
            OpCode::Nop(_) => cursor += 1,
            OpCode::Acc(value) => {
                acc += value;
                cursor += 1;
            }
            OpCode::Jmp(value) => cursor = cursor.wrapping_add(value as usize),
        }
    }

    (acc, cursor)
}

#[derive(Clone, Debug)]
enum OpCode {
    Nop(isize),
    Acc(isize),
    Jmp(isize),
}

impl FromStr for OpCode {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(" ");
        let (op_code, value) = (
            parts.nth(0).unwrap(),
            parts
                .nth(0)
                .unwrap()
                .parse()
                .context("Failed to parse number")?,
        );

        let op_code = match op_code {
            "nop" => OpCode::Nop(value),
            "acc" => OpCode::Acc(value),
            "jmp" => OpCode::Jmp(value),
            _ => panic!("Unknown op code {}", op_code),
        };

        Ok(op_code)
    }
}
