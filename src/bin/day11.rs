use std::iter::FromIterator;

use anyhow::Result;
use aoc_2020::input_lines;

fn main() -> Result<()> {
    let input = input_lines(11)?
        .map(|line| line.unwrap())
        .collect::<SeatingArrangement>();

    println!(
        "Part 1 occupied seats: {}",
        SeatingArrangement::run::<Part1>(input.clone())
    );
    println!(
        "Part 2 occuped seats: {}",
        SeatingArrangement::run::<Part2>(input.clone())
    );

    Ok(())
}

struct Part1;

impl SeatCalculateStrategy for Part1 {
    const OCCUPIED_THRESHOLD: u32 = 4;

    fn count_occupied_seats(arr: &SeatingArrangement, x: usize, y: usize) -> u32 {
        ADJACENCY_DIFFS
            .iter()
            .map(|(x_diff, y_diff)| (x as isize + x_diff, y as isize + y_diff))
            .filter(|(x, y)| arr.is_in_bounds(*x, *y))
            .filter(|(x, y)| arr.data[*y as usize][*x as usize] == ElementState::Occupied)
            .count() as u32
    }
}

struct Part2;

impl SeatCalculateStrategy for Part2 {
    const OCCUPIED_THRESHOLD: u32 = 5;

    fn count_occupied_seats(arr: &SeatingArrangement, x: usize, y: usize) -> u32 {
        let mut count = 0;

        for (x_diff, y_diff) in ADJACENCY_DIFFS.iter() {
            let mut next_x = x as isize + x_diff;
            let mut next_y = y as isize + y_diff;

            while arr.is_in_bounds(next_x, next_y) {
                match arr.data[next_y as usize][next_x as usize] {
                    ElementState::Floor => {}
                    ElementState::Free => break,
                    ElementState::Occupied => {
                        count += 1;
                        break;
                    }
                }

                next_x += x_diff;
                next_y += y_diff;
            }
        }

        count
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum ElementState {
    Floor,
    Free,
    Occupied,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct SeatingArrangement {
    pub data: Vec<Vec<ElementState>>,
}

static ADJACENCY_DIFFS: [(isize, isize); 8] = [
    (-1, 0),
    (1, 0),
    (0, 1),
    (0, -1),
    (-1, 1),
    (1, 1),
    (1, -1),
    (-1, -1),
];

impl SeatingArrangement {
    pub fn run<T: SeatCalculateStrategy>(mut input: SeatingArrangement) -> usize {
        loop {
            let next = input.step::<T>();

            if next == input {
                break;
            }

            input = next;
        }

        input.num_occupied()
    }

    fn num_occupied(&self) -> usize {
        self.data
            .iter()
            .flat_map(|data| data.iter())
            .filter(|element| **element == ElementState::Occupied)
            .count()
    }

    fn step<T: SeatCalculateStrategy>(&self) -> SeatingArrangement {
        SeatingArrangement {
            data: self
                .data
                .iter()
                .enumerate()
                .map(|(y, row)| {
                    row.iter()
                        .enumerate()
                        .map(|(x, element)| match *element {
                            ElementState::Floor => ElementState::Floor,
                            ElementState::Free => {
                                if T::count_occupied_seats(&self, x, y) == 0 {
                                    ElementState::Occupied
                                } else {
                                    ElementState::Free
                                }
                            }
                            ElementState::Occupied => {
                                if T::count_occupied_seats(&self, x, y) >= T::OCCUPIED_THRESHOLD {
                                    ElementState::Free
                                } else {
                                    ElementState::Occupied
                                }
                            }
                        })
                        .collect()
                })
                .collect(),
        }
    }

    pub fn is_in_bounds(&self, x: isize, y: isize) -> bool {
        x >= 0 && y >= 0 && x < self.data[0].len() as isize && y < self.data.len() as isize
    }
}

impl FromIterator<String> for SeatingArrangement {
    fn from_iter<T: IntoIterator<Item = String>>(iter: T) -> Self {
        SeatingArrangement {
            data: iter
                .into_iter()
                .map(|line| {
                    line.chars()
                        .map(|c| match c {
                            'L' => ElementState::Free,
                            '.' => ElementState::Floor,
                            '#' => ElementState::Occupied,
                            _ => panic!("Unknown character: {}", c),
                        })
                        .collect()
                })
                .collect(),
        }
    }
}

trait SeatCalculateStrategy {
    const OCCUPIED_THRESHOLD: u32;

    fn count_occupied_seats(arr: &SeatingArrangement, x: usize, y: usize) -> u32;
}
