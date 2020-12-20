use std::{fmt::Display, iter::FromIterator};

use anyhow::Result;
use aoc_2020::input_lines;
use lazy_static::lazy_static;

fn main() -> Result<()> {
    let mut input = input_lines(17)?
        .map(|line| line.unwrap())
        .collect::<PocketDimension>();

    let mut p1_input = input.clone();

    for _ in 0..6 {
        p1_input.cycle(false);
    }

    println!("Part 1 active: {}", p1_input.count_active());

    let mut p2_input = input.clone();

    for _ in 0..6 {
        p2_input.cycle(true);
    }

    println!("Part 2 active: {}", p2_input.count_active());

    Ok(())
}

#[derive(Clone)]
struct PocketDimension {
    bounds: (usize, usize, usize, usize), // x, y, z, w
    data: Vec<Vec<Vec<Vec<bool>>>>,       // w, z, y, x
}

lazy_static! {
    static ref ADJ3: Vec<(isize, isize, isize, isize)> = generate_adj3();
    static ref ADJ4: Vec<(isize, isize, isize, isize)> = generate_adj4();
}

impl PocketDimension {
    pub fn count_active(&self) -> usize {
        self.data
            .iter()
            .map(|slice_3d| {
                slice_3d
                    .iter()
                    .map(|slice_2d| {
                        slice_2d
                            .iter()
                            .map(|row| row.iter().filter(|cube| **cube).count())
                            .sum::<usize>()
                    })
                    .sum::<usize>()
            })
            .sum()
    }

    pub fn cycle(&mut self, fourth_dimension: bool) {
        self.expand(fourth_dimension);

        let adj = if fourth_dimension { &*ADJ4 } else { &*ADJ3 };

        let mut new_data = self.data.clone();

        for w in 0..self.bounds.3 {
            for z in 0..self.bounds.2 {
                for y in 0..self.bounds.1 {
                    for x in 0..self.bounds.0 {
                        let neighbors = self.count_neighbors(x, y, z, w, adj);
                        new_data[w][z][y][x] = match self.data[w][z][y][x] {
                            true => {
                                if neighbors == 2 || neighbors == 3 {
                                    true
                                } else {
                                    false
                                }
                            }
                            false => {
                                if neighbors == 3 {
                                    true
                                } else {
                                    false
                                }
                            }
                        }
                    }
                }
            }
        }

        self.data = new_data;
    }

    fn count_neighbors(
        &self,
        x: usize,
        y: usize,
        z: usize,
        w: usize,
        adj: &Vec<(isize, isize, isize, isize)>,
    ) -> u32 {
        let mut count = 0;
        for adj in adj {
            let x = adj.0 + x as isize;
            let y = adj.1 + y as isize;
            let z = adj.2 + z as isize;
            let w = adj.3 + w as isize;

            if x < 0 || x >= self.bounds.0 as isize {
                continue;
            }

            if y < 0 || y >= self.bounds.1 as isize {
                continue;
            }

            if z < 0 || z >= self.bounds.2 as isize {
                continue;
            }

            if w < 0 || w >= self.bounds.3 as isize {
                continue;
            }

            if self.data[w as usize][z as usize][y as usize][x as usize] {
                count += 1;
            }
        }

        count
    }

    fn expand(&mut self, fourth_dimension: bool) {
        // Increase the bounds by one one either side.
        if fourth_dimension {
            self.bounds = (
                self.bounds.0 + 2,
                self.bounds.1 + 2,
                self.bounds.2 + 2,
                self.bounds.3 + 2,
            );
        } else {
            self.bounds = (
                self.bounds.0 + 2,
                self.bounds.1 + 2,
                self.bounds.2 + 2,
                self.bounds.3,
            );
        }

        // Generate the sequences for empty values.
        let row = std::iter::repeat(false)
            .take(self.bounds.0)
            .collect::<Vec<bool>>();
        let slice_2d = std::iter::repeat(row.clone())
            .take(self.bounds.1)
            .collect::<Vec<Vec<bool>>>();

        // Expand the innermost elements.
        for slice_3d in &mut self.data {
            for slice_2d in slice_3d.iter_mut() {
                for r in slice_2d.iter_mut() {
                    r.insert(0, false);
                    r.push(false);
                }

                // Make sure to push an extra row.
                slice_2d.push(row.clone());
                slice_2d.insert(0, row.clone())
            }

            slice_3d.push(slice_2d.clone());
            slice_3d.insert(0, slice_2d.clone());
        }

        if fourth_dimension {
            let slice_3d = std::iter::repeat(slice_2d.clone())
                .take(self.bounds.2)
                .collect::<Vec<Vec<Vec<bool>>>>();

            self.data.push(slice_3d.clone());
            self.data.insert(0, slice_3d);
        }
    }
}

impl FromIterator<String> for PocketDimension {
    fn from_iter<T: IntoIterator<Item = String>>(iter: T) -> Self {
        let data = iter
            .into_iter()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        '#' => true,
                        '.' => false,
                        _ => panic!("Unknown character in input: '{}'", c),
                    })
                    .collect::<Vec<bool>>()
            })
            .collect::<Vec<Vec<bool>>>();

        PocketDimension {
            bounds: (data[0].len(), data.len(), 1, 1),
            data: vec![vec![data]],
        }
    }
}

// impl Display for PocketDimension {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         for slice in &self.data {
//             for row in slice {
//                 for item in row {
//                     let c = match item {
//                         true => "#",
//                         false => ".",
//                     };

//                     f.write_str(c)?;
//                 }

//                 f.write_str("\n")?;
//             }

//             f.write_str("\n")?;
//         }

//         Ok(())
//     }
// }

fn generate_adj3() -> Vec<(isize, isize, isize, isize)> {
    let mut adj = Vec::new();

    for x in -1..=1 {
        for y in -1..=1 {
            for z in -1..=1 {
                if x == 0 && y == 0 && z == 0 {
                    continue;
                }

                adj.push((x, y, z, 0))
            }
        }
    }

    adj
}

fn generate_adj4() -> Vec<(isize, isize, isize, isize)> {
    let mut adj = Vec::new();

    for x in -1..=1 {
        for y in -1..=1 {
            for z in -1..=1 {
                for w in -1..=1 {
                    if x == 0 && y == 0 && z == 0 && w == 0 {
                        continue;
                    }

                    adj.push((x, y, z, w))
                }
            }
        }
    }

    adj
}
