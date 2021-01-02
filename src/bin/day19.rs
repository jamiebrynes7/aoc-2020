use std::{collections::HashMap, io, str::FromStr};

use anyhow::{Context, Result};
use aoc_2020::input_lines;
use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;

fn main() -> Result<()> {
    let lines = input_lines(19)?.collect::<Result<Vec<String>, io::Error>>()?;
    let mut ruleset = RuleSet::new(lines.iter().take_while(|line| !line.is_empty()))?;

    let messages = lines
        .iter()
        .skip_while(|line| !line.is_empty())
        .skip(1)
        .cloned()
        .collect::<Vec<String>>();

    part1(&ruleset, &messages);
    part2(&mut ruleset, &messages);

    Ok(())
}

fn part1(ruleset: &RuleSet, messages: &[String]) {
    let count = messages.iter().filter(|msg| ruleset.check(msg)).count();

    println!("{} messages match", count);
}

fn part2(ruleset: &mut RuleSet, messages: &[String]) {
    ruleset.replace(
        8,
        Expression::Or(
            Box::new(Expression::Series(vec![42])),
            Box::new(Expression::Series(vec![42, 8])),
        ),
    );

    ruleset.replace(
        11,
        Expression::Or(
            Box::new(Expression::Series(vec![42, 31])),
            Box::new(Expression::Series(vec![42, 11, 31])),
        ),
    );

    let count = messages.iter().filter(|msg| ruleset.check(msg)).count();

    println!("{} messages match", count);
}

struct RuleSet {
    pub rules: HashMap<usize, Expression>,
}

impl RuleSet {
    pub fn new<'a, T: Iterator<Item = &'a String>>(lines: T) -> Result<RuleSet> {
        Ok(RuleSet {
            rules: lines
                .map(|line| {
                    let mut parts = line.split(": ");
                    let index = parts
                        .nth(0)
                        .context("No index for the rule")?
                        .parse::<usize>()?;

                    Ok((
                        index,
                        Expression::from_str(parts.nth(0).context("No content for the rule")?)?,
                    ))
                })
                .collect::<Result<HashMap<usize, Expression>>>()?,
        })
    }

    pub fn check(&self, candidate: &str) -> bool {
        match self.rules[&0].eval(&self, candidate) {
            Some(remainder) => remainder.is_empty(),
            None => false,
        }
    }

    pub fn replace(&mut self, index: usize, exp: Expression) {
        self.rules.insert(index, exp);
    }
}

#[derive(Debug)]
enum Expression {
    Series(Vec<usize>),
    Or(Box<Expression>, Box<Expression>),
    Literal(char),
}

impl Expression {
    pub fn eval<'a>(&self, ruleset: &RuleSet, candidate: &'a str) -> Option<&'a str> {
        if candidate.is_empty() {
            return Some(candidate);
        }

        let result = match self {
            Expression::Series(indicies) => indicies
                .iter()
                .fold_while(Some(candidate), |remainder, index| {
                    match ruleset.rules[index].eval(ruleset, remainder.unwrap()) {
                        Some(r) => Continue(Some(r)),
                        None => Done(None),
                    }
                })
                .into_inner(),
            Expression::Or(first, second) => first
                .eval(ruleset, candidate)
                .or_else(|| second.eval(ruleset, candidate)),
            Expression::Literal(c) => {
                if candidate.starts_with(*c) {
                    Some(&candidate[1..])
                } else {
                    None
                }
            }
        };

        return result;
    }
}

impl FromStr for Expression {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("\"") {
            return Ok(Expression::Literal(s.chars().nth(1).context("")?));
        }

        if s.contains("|") {
            let mut parts = s.split(" | ");
            return Ok(Expression::Or(
                Box::new(Expression::from_str(parts.nth(0).context("")?)?),
                Box::new(Expression::from_str(parts.nth(0).context("")?)?),
            ));
        }

        let indices = s
            .split(" ")
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        Ok(Expression::Series(indices))
    }
}
