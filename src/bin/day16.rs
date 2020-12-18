use std::{collections::HashMap, str::FromStr};

use anyhow::Result;
use aoc_2020::input_lines;

fn main() -> Result<()> {
    let (constraints, my_ticket, other_tickets) = parse()?;

    part1(&constraints, &other_tickets);
    part2(&constraints, &my_ticket, &other_tickets);

    Ok(())
}

fn part1(constraints: &[Constraint], other_tickets: &[Ticket]) {
    let error_rate = other_tickets
        .iter()
        .filter_map(|ticket| ticket.find_invalid_field(constraints))
        .sum::<u32>();

    println!("Error rate: {}", error_rate);
}

fn part2(constraints: &[Constraint], my_ticket: &Ticket, other_tickets: &[Ticket]) {
    let valid_tickets = other_tickets
        .iter()
        .filter(|ticket| ticket.find_invalid_field(constraints).is_none())
        .collect::<Vec<&Ticket>>();

    // Find possible matches.
    let mut possible_matches = constraints
        .iter()
        .map(|c| {
            (
                c.clone(),
                (0..20)
                    .into_iter()
                    .filter(|i| {
                        valid_tickets
                            .iter()
                            .all(|ticket| c.matches(ticket.number(*i)))
                    })
                    .collect(),
            )
        })
        .collect::<HashMap<Constraint, Vec<usize>>>();

    // Go through and find the element with only a single possibility.
    let mut matches: HashMap<Constraint, usize> = HashMap::new();

    while possible_matches.len() > 0 {
        let constraint = possible_matches
            .iter()
            .filter(|(_, indicies)| indicies.len() == 1)
            .map(|(c, _)| c.clone())
            .nth(0)
            .unwrap();

        let index = possible_matches.remove(&constraint).unwrap()[0];

        for (_, indices) in &mut possible_matches {
            indices.retain(|val| *val != index);
        }

        matches.insert(constraint, index);
    }

    let product = matches
        .iter()
        .filter(|(c, _)| c.name.starts_with("departure"))
        .map(|(_, index)| my_ticket.number(*index) as u64)
        .product::<u64>();

    println!("Product on my ticket: {}", product);
}

fn parse() -> Result<(Vec<Constraint>, Ticket, Vec<Ticket>)> {
    let mut lines = input_lines(16)?;

    let mut constraints = Vec::new();

    while let Some(line) = lines.next() {
        let line = line.unwrap();

        if line.is_empty() {
            break;
        }

        constraints.push(Constraint::from_str(&line)?);
    }

    let mut lines = lines.skip(1);
    let my_ticket = Ticket::from_str(&(lines.next().unwrap()?))?;
    let mut lines = lines.skip(2);

    let mut other_tickets = Vec::new();

    while let Some(line) = lines.next() {
        let line = line.unwrap();

        if line.is_empty() {
            break;
        }

        other_tickets.push(Ticket::from_str(&line)?);
    }

    Ok((constraints, my_ticket, other_tickets))
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Constraint {
    pub name: String,
    ranges: Vec<(u32, u32)>,
}

impl Constraint {
    pub fn matches(&self, number: u32) -> bool {
        self.ranges
            .iter()
            .any(|(lower, upper)| number >= *lower && number <= *upper)
    }
}

impl FromStr for Constraint {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(": ");
        let name = parts.nth(0).unwrap().to_owned();

        let ranges = parts
            .nth(0)
            .unwrap()
            .split(" or ")
            .map(|slice| {
                let mut numbers = slice.split("-");
                (
                    numbers.nth(0).unwrap().parse::<u32>().unwrap(),
                    numbers.nth(0).unwrap().parse::<u32>().unwrap(),
                )
            })
            .collect();

        Ok(Constraint { name, ranges })
    }
}

struct Ticket {
    numbers: Vec<u32>,
}

impl Ticket {
    pub fn find_invalid_field(&self, constraints: &[Constraint]) -> Option<u32> {
        self.numbers
            .iter()
            .filter(|num| constraints.iter().all(|c| !c.matches(**num)))
            .cloned()
            .nth(0)
    }

    pub fn number(&self, index: usize) -> u32 {
        self.numbers[index]
    }
}

impl FromStr for Ticket {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Ticket {
            numbers: s
                .split(",")
                .map(|number| number.parse::<u32>().unwrap())
                .collect(),
        })
    }
}
