use anyhow::{Context, Result};
use aoc_2020::input_lines;
use lazy_static::lazy_static;
use regex::Regex;
use std::str::FromStr;

fn main() -> Result<()> {
    let input = input_lines(2)?
        .map(|line| {
            let line = line.context("Failed to read line")?;
            Policy::from_str(&line)
        })
        .collect::<Result<Vec<Policy>>>()?;

    part1(&input);
    part2(&input);

    Ok(())
}

fn part1(policies: &[Policy]) {
    let valid = policies
        .iter()
        .filter(|policy| Policy::is_valid_part_1(*policy))
        .count();

    println!("There are {} valid passwords with first policy!", valid);
}

fn part2(policies: &[Policy]) {
    let valid = policies
        .iter()
        .filter(|policy| Policy::is_valid_part_2(*policy))
        .count();

    println!("There are {} valid passwords with second policy!", valid);
}

struct Policy {
    min: usize,
    max: usize,
    target_char: char,
    password: String,
}

impl Policy {
    pub fn is_valid_part_1(&self) -> bool {
        let count = self.password.matches(self.target_char).count();
        count >= self.min && self.max >= count
    }

    pub fn is_valid_part_2(&self) -> bool {
        [
            self.password.chars().nth(self.min - 1),
            self.password.chars().nth(self.max - 1),
        ]
        .iter()
        .filter(|opt| **opt == Some(self.target_char))
        .count()
            == 1
    }
}

impl FromStr for Policy {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"([0-9]*)-([0-9]*) ([a-z]): (.*)").unwrap();
        }

        let caps = RE
            .captures(s)
            .with_context(|| format!("String did not match regex: '{}'", s))?;

        Ok(Policy {
            min: caps[1]
                .parse::<usize>()
                .with_context(|| format!("Invalid usize: {}", &caps[1]))?,
            max: caps[2]
                .parse::<usize>()
                .with_context(|| format!("Invalid usize: {}", &caps[2]))?,
            target_char: caps[3]
                .chars()
                .next()
                .context("Target char was 0 characters long?")?,
            password: caps[4].to_string(),
        })
    }
}
