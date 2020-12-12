use std::str::FromStr;

use anyhow::Result;
use aoc_2020::input_lines;

fn main() -> Result<()> {
    let input = input_lines(12)?
        .map(|line| Action::from_str(&line.unwrap()))
        .collect::<Result<Vec<Action>>>()?;

    part1(&input);
    part2(&input);

    Ok(())
}

fn part1(inputs: &[Action]) {
    let mut ship = Ship::default();

    for action in inputs {
        ship.apply_p1(action);
    }

    println!("Final Manhattan distance: {}", ship.manhattan_distance());
}

fn part2(inputs: &[Action]) {
    let mut ship = Ship::default();
    let mut waypoint = Position::new(10, 1);

    for action in inputs {
        ship.apply_p2(&mut waypoint, action);
    }

    println!("Final Manhattan distance: {}", ship.manhattan_distance());
}

struct Ship {
    position: Position,
    heading: Direction,
}

impl Default for Ship {
    fn default() -> Self {
        Ship {
            position: Position::new(0, 0),
            heading: Direction::East,
        }
    }
}

impl Ship {
    pub fn apply_p1(&mut self, action: &Action) {
        match action {
            Action::Move(dir, magnitude) => self.position.translate(*dir, *magnitude),
            Action::Turn(dir, magnitude) => {
                if magnitude % 90 != 0 {
                    panic!("Found a turn that wasn't 90 degress!");
                }

                for _ in 0..(magnitude / 90) {
                    self.heading = match dir {
                        TurnDirection::Left => self.heading.rotate_ccw(),
                        TurnDirection::Right => self.heading.rotate_cw(),
                    }
                }
            }
            Action::Forward(magnitude) => self.position.translate(self.heading, *magnitude),
        }
    }

    pub fn apply_p2(&mut self, waypoint: &mut Position, action: &Action) {
        match action {
            Action::Move(dir, magnitude) => waypoint.translate(*dir, *magnitude),
            Action::Turn(dir, magnitude) => {
                if magnitude % 90 != 0 {
                    panic!("Found a turn that wasn't 90 degress!");
                }

                for _ in 0..(magnitude / 90) {
                    waypoint.rotate(*dir)
                }
            }
            Action::Forward(magnitude) => {
                self.position.x += magnitude * waypoint.x;
                self.position.y += magnitude * waypoint.y
            }
        }
    }

    pub fn manhattan_distance(&self) -> i32 {
        self.position.manhattan_distance()
    }
}

enum Action {
    Move(Direction, i32),
    Turn(TurnDirection, i32),
    Forward(i32),
}

impl FromStr for Action {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let action = s.chars().nth(0).unwrap();
        let magnitude = s[1..].parse()?;

        Ok(match action {
            'N' => Action::Move(Direction::North, magnitude),
            'S' => Action::Move(Direction::South, magnitude),
            'E' => Action::Move(Direction::East, magnitude),
            'W' => Action::Move(Direction::West, magnitude),
            'L' => Action::Turn(TurnDirection::Left, magnitude),
            'R' => Action::Turn(TurnDirection::Right, magnitude),
            'F' => Action::Forward(magnitude),
            _ => panic!("Unknown action: {}", action),
        })
    }
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    pub fn rotate_cw(self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::South => Direction::West,
            Direction::East => Direction::South,
            Direction::West => Direction::North,
        }
    }

    pub fn rotate_ccw(self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
            Direction::West => Direction::South,
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum TurnDirection {
    Left,
    Right,
}

struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Position { x, y }
    }

    pub fn translate(&mut self, direction: Direction, magnitude: i32) {
        match direction {
            Direction::North => self.y += magnitude,
            Direction::South => self.y -= magnitude,
            Direction::East => self.x += magnitude,
            Direction::West => self.x -= magnitude,
        }
    }

    pub fn rotate(&mut self, direction: TurnDirection) {
        match direction {
            TurnDirection::Left => {
                // Transform (x,y) to (-y, x)
                let tmp = self.y;
                self.y = self.x;
                self.x = -tmp;
            }
            TurnDirection::Right => {
                // Transform (x,y) to (y, -x)
                let tmp = self.x;
                self.x = self.y;
                self.y = -tmp;
            }
        }
    }

    pub fn manhattan_distance(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}
