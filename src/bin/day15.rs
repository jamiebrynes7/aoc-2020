use std::collections::HashMap;

use anyhow::Result;

static INPUT: [usize; 6] = [6, 3, 15, 13, 1, 0];

fn main() -> Result<()> {
    part1();
    part2();
    Ok(())
}

fn part1() {
    let mut game = MemoryGame::from_seed(&INPUT);
    game.run_until(2020);

    println!("Last number: {}", game.last_number);
}

fn part2() {
    let mut game = MemoryGame::from_seed(&INPUT);
    game.run_until(30000000);

    println!("Last number: {}", game.last_number);
}

struct MemoryGame {
    occurences: HashMap<usize, usize>,
    current_turn: usize,
    pub last_number: usize,
}

impl MemoryGame {
    pub fn from_seed(seed: &[usize]) -> MemoryGame {
        MemoryGame {
            // We want to insert everything except for the last element which is stored in the last_number element.
            occurences: seed[0..seed.len() - 1]
                .iter()
                .enumerate()
                .map(|(index, value)| (*value, index + 1))
                .collect(),
            current_turn: seed.len(),
            last_number: seed[seed.len() - 1],
        }
    }

    pub fn run_until(&mut self, turn: usize) {
        while self.current_turn < turn {
            self.step();
        }
    }

    fn step(&mut self) {
        let next_number = match self.occurences.get(&self.last_number) {
            Some(last_turn) => self.current_turn - last_turn,
            None => 0,
        };

        self.occurences.insert(self.last_number, self.current_turn);
        self.current_turn += 1;
        self.last_number = next_number;
    }
}
