use std::str::FromStr;

use anyhow::{Context, Result};
use aoc_2020::input_lines;

fn main() -> Result<()> {
    let mut lines = input_lines(13)?;

    let earliest = lines.next().unwrap()?.parse::<usize>()?;
    let timetable = BusTimetable::from_str(&lines.next().unwrap()?)?;

    part1(earliest, timetable.clone());
    part2(timetable.clone());

    Ok(())
}

fn part1(earliest: usize, timetable: BusTimetable) {
    if timetable.ids.iter().any(|(_, id)| earliest % id == 0) {
        println!("There is a bus exactly at {}. Result: 0.", earliest);
        return;
    }

    if let Some((id, ttw)) = timetable
        .ids
        .iter()
        .map(|(_, id)| (*id, *id - (earliest % id)))
        .min_by_key(|(_, ttw)| *ttw)
    {
        println!(
            "First bus you can catch is: {} and would need to wait {} minutes. Result = {}",
            id,
            ttw,
            id * ttw
        );
    }
}

fn part2(timetable: BusTimetable) {
    // We want to find 'N' such that for all bus 'b' with index 'i' the following is true:
    //      (N + i) % b = 0
    // This then simplifies to:
    //      N % b = b - i since -i % b = b - i
    // We can then apply CRT (https://en.wikipedia.org/wiki/Chinese_remainder_theorem) to solve the system of equations to find N.
    let modulus: usize = timetable.ids.iter().map(|(_, id)| *id).product();

    fn inv_mod(a: usize, m: usize) -> usize {
        // First simplify...
        let r = a % m;

        // ...then brute force it.
        let mut candidate = 1;
        loop {
            if (r * candidate) % m == 1 {
                println!("Result: {}", candidate);
                return candidate;
            }

            candidate += 1;
        }
    }

    let x = timetable
        .ids
        .iter()
        .map(|(index, id)| {
            let ni = modulus / id;
            let xi = inv_mod(ni, *id);

            ni * xi * (id - index)
        })
        .sum::<usize>();

    println!("Result: {}", x % modulus)
}

#[derive(Debug, Clone)]
struct BusTimetable {
    pub ids: Vec<(usize, usize)>,
}

impl FromStr for BusTimetable {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(BusTimetable {
            ids: s
                .split(",")
                .enumerate()
                .filter(|(_, fragment)| *fragment != "x")
                .map(|(index, fragment)| {
                    (
                        index,
                        fragment
                            .parse::<usize>()
                            .context("Failed to parse number")
                            .unwrap(),
                    )
                })
                .collect::<Vec<(usize, usize)>>(),
        })
    }
}
