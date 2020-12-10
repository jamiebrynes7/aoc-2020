use std::collections::HashMap;

use anyhow::Result;
use aoc_2020::input_lines;

fn main() -> Result<()> {
    let input = input_lines(10)?
        .map(|line| line.unwrap().parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    part1(input.clone());
    part2(input);

    Ok(())
}

fn part1(mut input: Vec<usize>) {
    // The starting point.
    input.push(0);
    input.sort();

    // There's always a difference of 3 between the last adapter and my device.
    let mut results = [0, 0, 0, 1];

    for pair in input.windows(2) {
        let diff = pair[1] - pair[0];
        results[diff] += 1;
    }

    println!("Result: {}", results[1] * results[3]);
}

fn part2(mut input: Vec<usize>) {
    fn calculate_permutations(
        current_value: usize,
        remaining: &[usize],
        mut cache: &mut HashMap<usize, u64>,
    ) -> u64 {
        // The base case is that we are at '0', there is only 1 way to get to '0'.
        if remaining.is_empty() {
            return 1;
        }

        // If we've already checked how many ways to get to current_value, return it in the cache.
        if let Some(val) = cache.get(&current_value) {
            return *val;
        }

        // Otherwise sum the various ways of getting to current_value by traversing all
        // possible directions recursively.
        let value = remaining
            .iter()
            .enumerate()
            .filter(|(_, value)| current_value - *value <= 3)
            .fold(0u64, |total, (index, value)| {
                total + calculate_permutations(*value, &remaining[index + 1..], &mut cache)
            });

        // Save into the cache!
        cache.insert(current_value, value);
        value
    }

    // Insert the starting point and sort the input in largest --> smallest
    input.push(0);
    input.sort();
    input.reverse();

    let mut cache: HashMap<usize, u64> = HashMap::new();
    println!(
        "Total permutations: {}",
        calculate_permutations(input[0] + 3, &input, &mut cache)
    );
}
