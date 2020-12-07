use std::{collections::HashMap, collections::HashSet, str::FromStr};

use anyhow::{Context, Result};
use aoc_2020::input_lines;
use lazy_static::lazy_static;
use regex::Regex;

fn main() -> Result<()> {
    let input = input_lines(7)?
        .map(|line| {
            let line = line.context("Failed to read line")?;
            Relationship::from_str(&line)
        })
        .collect::<Result<Vec<Relationship>>>()?;

    part1(&input);
    part2(&input);
    Ok(())
}

fn part1(input: &[Relationship]) {
    let mut graph = HashMap::new();

    for value in input {
        for (_, parent) in &value.contains {
            let entry = graph.entry(parent.clone()).or_insert(HashSet::new());
            entry.insert(value.ident.clone());
        }
    }

    let mut visited = HashSet::new();
    let mut queue = Vec::new();

    queue.extend(graph.get("shiny gold".into()).unwrap().clone());

    while queue.len() > 0 {
        let item = queue.pop().unwrap();
        if visited.contains(&item) {
            continue;
        }

        visited.insert(item.clone());

        if graph.contains_key(&item) {
            queue.extend(graph.get(&item).unwrap().clone());
        }
    }

    println!("Found {} possible parents for 'shiny gold'", visited.len());
}

fn part2(input: &[Relationship]) {
    let graph = input
        .iter()
        .map(|rela| (rela.ident.clone(), rela.clone()))
        .collect::<HashMap<String, Relationship>>();

    fn child_bag_count(
        graph: &HashMap<String, Relationship>,
        ident: &String,
        parent_count: usize,
    ) -> usize {
        match graph.get(ident) {
            Some(relationship) => relationship
                .contains
                .iter()
                .fold(0, |total, (count, child)| {
                    total
                        + parent_count * count
                        + child_bag_count(graph, child, parent_count * count)
                }),
            None => 0,
        }
    }

    println!(
        "A shiny gold bag contains {} bags",
        child_bag_count(&graph, &"shiny gold".into(), 1)
    );
}

#[derive(Clone)]
struct Relationship {
    pub ident: String,
    pub contains: Vec<(usize, String)>,
}

impl FromStr for Relationship {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(" bags contain ");
        let ident = parts.nth(0).unwrap();
        let contains_parts = parts.nth(0).unwrap();

        lazy_static! {
            static ref RE: Regex = Regex::new(r"([0-9]{1,}) ([a-zA-Z\s]*) bags?[,|.]").unwrap();
        }

        let contains = RE
            .captures_iter(contains_parts)
            .map(|cap| {
                let count = &cap[1].parse::<usize>().context("Failed to parse")?;
                let ident = &cap[2];
                Ok((*count, ident.into()))
            })
            .collect::<Result<Vec<(usize, String)>>>()?;

        Ok(Relationship {
            ident: ident.into(),
            contains,
        })
    }
}
