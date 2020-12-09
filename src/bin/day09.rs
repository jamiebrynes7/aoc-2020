use anyhow::Result;
use aoc_2020::input_lines;

fn main() -> Result<()> {
    let input = input_lines(9)?
        .map(|line| line.unwrap().parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let target = part1(&input).unwrap();
    part2(&input, target);

    Ok(())
}

fn part1(input: &[usize]) -> Option<usize> {
    let mut possible_numbers = input.iter().cloned().take(25).collect::<Vec<usize>>();

    fn is_sum_of(possible: &[usize], target: usize) -> bool {
        for x in possible.iter() {
            for y in possible.iter() {
                if x == y {
                    continue;
                }

                if *x + *y == target {
                    return true;
                }
            }
        }

        return false;
    }

    for number in input.iter().skip(25) {
        if is_sum_of(&possible_numbers, *number) {
            possible_numbers.remove(0);
            possible_numbers.push(*number);
        } else {
            println!("First number without the sum property: {}", number);
            return Some(*number);
        }
    }

    None
}

fn part2(input: &[usize], target: usize) {
    let mut cursor = 0;
    let mut size = 2;

    loop {
        let mut sum = input.iter().skip(cursor).take(size).sum::<usize>();

        while sum < target {
            sum += input[cursor + size];
            size += 1;
        }

        if sum == target {
            break;
        } else {
            cursor += 1;
            size = 2;
        }
    }

    let range = input
        .iter()
        .skip(cursor)
        .take(size)
        .cloned()
        .collect::<Vec<usize>>();

    println!(
        "Encryption weakness is: {}",
        range.iter().min().unwrap() + range.iter().max().unwrap()
    );
}
