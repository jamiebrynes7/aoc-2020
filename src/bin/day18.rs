use std::collections::HashMap;

use anyhow::{Context, Result};
use aoc_2020::input_lines;
use lazy_static::lazy_static;
use regex::Regex;

fn main() -> Result<()> {
    println!(
        "Sum of results, simple: {}",
        input_lines(18)?
            .map(|line| eval(line.unwrap(), eval_expr_p1))
            .collect::<Result<Vec<u64>>>()?
            .iter()
            .sum::<u64>()
    );

    println!(
        "Sum of results, advanced: {}",
        input_lines(18)?
            .map(|line| eval(line.unwrap(), eval_expr_p2))
            .collect::<Result<Vec<u64>>>()?
            .iter()
            .sum::<u64>()
    );

    Ok(())
}

fn eval<F>(mut expr: String, eval_expr: F) -> Result<u64>
where
    F: Fn(&str) -> Result<u64>,
{
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\([^\(\)]*\)").unwrap();
    }

    while RE.is_match(&expr) {
        let mut matches = HashMap::new();
        for caps in RE.captures_iter(&expr) {
            let expr = &caps[0];
            let result = eval_expr(&expr[1..expr.len() - 1])?;
            matches.insert(expr.to_owned(), result);
        }

        for (sub_expr, value) in matches {
            expr = expr.replace(&sub_expr, &format!("{}", value));
        }
    }

    eval_expr(&expr)
}

/// Evaluate an expression which does not contain any parenthesis.
fn eval_expr_p1(expr: &str) -> Result<u64> {
    if expr.contains(")") || expr.contains("(") {
        return Err(anyhow::anyhow!("expression contained parenthesis"));
    }

    let mut iter = expr.split(" ");

    // Get the initial value.
    let mut value = iter
        .nth(0)
        .unwrap()
        .parse::<u64>()
        .context("Failed to parse character as number")?;

    let mut operator: Option<Operator> = None;

    while let Some(c) = iter.next() {
        if let Some(operator) = operator.take() {
            let operand = c
                .parse::<u64>()
                .context("Failed to parse string as number")?;

            match operator {
                Operator::Plus => value += operand,
                Operator::Times => value *= operand,
            }

            continue;
        }

        operator = Some(match c {
            "*" => Operator::Times,
            "+" => Operator::Plus,
            _ => panic!("Unknown operator: {}", c),
        });

        continue;
    }

    Ok(value)
}

/// Evaluate an expression which does not contain any parenthesis.
fn eval_expr_p2(expr: &str) -> Result<u64> {
    // Need to do addition first, then multiplication.
    if expr.contains(")") || expr.contains("(") {
        return Err(anyhow::anyhow!("expression contained parenthesis"));
    }

    let mut parts = expr
        .split(" ")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    // Find the indices of addition signs.
    let addition_indices = parts
        .iter()
        .enumerate()
        .filter_map(|(index, pat)| if *pat == "+" { Some(index) } else { None })
        .collect::<Vec<usize>>();

    let mut offset = 0;

    // For each addition sign, compute the addition and replace the expression with the result.
    // Must take care with indexing as we are mutating in place.
    for indice in addition_indices {
        let value = parts[indice - 1 - offset].parse::<u64>()?
            + parts[indice + 1 - offset].parse::<u64>()?;
        parts[indice - offset] = format!("{}", value);
        parts.remove(indice + 1 - offset);
        parts.remove(indice - 1 - offset);

        offset += 2;
    }

    // Now find all the remaining numbers and multiply them together.
    Ok(parts
        .iter()
        .filter_map(|pat| {
            if pat != "*" {
                Some(pat.parse::<u64>().context("Failed to parse number"))
            } else {
                None
            }
        })
        .collect::<Result<Vec<u64>>>()?
        .iter()
        .product::<u64>())
}

enum Operator {
    Plus,
    Times,
}
