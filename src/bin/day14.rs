use anyhow::Result;
use aoc_2020::input_lines;
use lazy_static::lazy_static;
use regex::Regex;
use std::{collections::HashMap, str::FromStr};

fn main() -> Result<()> {
    let instructions = input_lines(14)?
        .map(|line| Instruction::from_str(&line.unwrap()))
        .collect::<Result<Vec<Instruction>>>()?;

    part1(&instructions);
    part2(&instructions);

    Ok(())
}

fn part1(instructions: &[Instruction]) {
    let mut mem = HashMap::new();
    let mut mask = Bitmask::default();

    for inst in instructions {
        match inst {
            Instruction::SetMask(new_mask) => {
                mask = Bitmask::from_str(&new_mask).unwrap();
            }
            Instruction::Memset(addr, value) => {
                mem.insert(addr, mask.apply(*value));
            }
        }
    }

    let sum = mem.iter().map(|(_, value)| *value).sum::<u64>();

    println!("Sum: {}", sum);
}

fn part2(instructions: &[Instruction]) {
    let mut mem = HashMap::new();
    let mut mask = Bitmask::default();

    for inst in instructions {
        match inst {
            Instruction::SetMask(new_mask) => {
                mask = Bitmask::from_str(&new_mask).unwrap();
            }
            Instruction::Memset(addr, value) => {
                for addr_permutation in mask.permute(*addr) {
                    mem.insert(addr_permutation, value);
                }
            }
        }
    }

    let sum = mem.iter().map(|(_, value)| *value).sum::<u64>();

    println!("Sum: {}", sum);
}

enum Instruction {
    SetMask(String),
    Memset(u64, u64),
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("mask") {
            return Ok(Instruction::SetMask(
                s.split(" ").nth(2).unwrap().to_owned(),
            ));
        }

        lazy_static! {
            static ref MEM_REGEX: Regex = Regex::new(r"mem\[([0-9]*)\] = ([0-9]*)").unwrap();
        }

        let matches = MEM_REGEX.captures(s).unwrap();
        let addr = matches.get(1).unwrap().as_str().parse::<u64>().unwrap();
        let value = matches.get(2).unwrap().as_str().parse::<u64>().unwrap();

        Ok(Instruction::Memset(addr, value))
    }
}

#[derive(Debug)]
struct Bitmask {
    ones: u64,
    zeroes: u64,
    floating: Vec<usize>, // Indices
}

impl Bitmask {
    pub fn apply(&self, value: u64) -> u64 {
        (value & self.zeroes) | self.ones
    }

    pub fn permute(&self, value: u64) -> Vec<u64> {
        let base = value | self.ones;

        fn generate_permutations(data: &mut Vec<u64>, remaining: &[usize], value: u64) {
            if remaining.len() == 0 {
                data.push(value);
                return;
            }

            let index = remaining[0];
            generate_permutations(data, &remaining[1..], value | (1 << index));
            generate_permutations(data, &remaining[1..], value & (u64::MAX - (1 << index)))
        }

        let mut values = Vec::new();
        generate_permutations(&mut values, self.floating.as_slice(), base);
        values
    }
}

impl Default for Bitmask {
    fn default() -> Self {
        Bitmask {
            ones: 0,
            zeroes: u64::MAX,
            floating: Vec::new(),
        }
    }
}

impl FromStr for Bitmask {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut ones: u64 = 0;
        let mut zeroes = u64::MAX;
        let mut floating = Vec::new();

        for (index, c) in s.chars().rev().enumerate() {
            match c {
                '1' => ones |= 1 << index,
                '0' => zeroes &= u64::MAX - (1 << index),
                'X' => floating.push(index),
                _ => panic!("Unexpected char '{}' at position '{}'", c, index),
            }
        }

        Ok(Bitmask {
            ones,
            zeroes,
            floating,
        })
    }
}
