use std::{collections::BTreeSet, str::FromStr};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Direction {
    Up(i32),
    Down(i32),
    Left(i32),
    Right(i32),
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let direction = chars.next().unwrap();
        let amount = chars.as_str().trim().parse::<i32>().unwrap();
        match direction {
            'U' => Ok(Direction::Up(amount)),
            'D' => Ok(Direction::Down(amount)),
            'L' => Ok(Direction::Left(amount)),
            'R' => Ok(Direction::Right(amount)),
            _ => Err(()),
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new() -> Position {
        Position { x: 0, y: 0 }
    }

    pub fn distance(&self, other: &Position) -> i32 {
        let x_diff = self.x - other.x;
        let y_diff = self.y - other.y;
        ((x_diff.pow(2) + y_diff.pow(2)) as f64).sqrt().floor() as i32
    }

    pub fn follow(&mut self, other: &Position) -> Vec<Position> {
        let mut result = vec![];
        while self.distance(other) > 1 {
            let mut new_x = self.x;
            let mut new_y = self.y;

            if other.x > self.x {
                new_x += 1;
            }
            if other.x < self.x {
                new_x -= 1;
            }

            if other.y > self.y {
                new_y += 1;
            }
            if other.y < self.y {
                new_y -= 1;
            }

            self.x = new_x;
            self.y = new_y;

            result.push(self.clone());
        }
        result
    }
}

#[derive(Debug)]
pub struct Head {
    pub position: Position,
}

impl Head {
    pub fn new() -> Head {
        Head {
            position: Position { x: 0, y: 0 },
        }
    }

    pub fn step(&mut self, direction: Direction) {
        match direction {
            Direction::Up(_) => self.position.y += 1,
            Direction::Down(_) => self.position.y -= 1,
            Direction::Left(_) => self.position.x -= 1,
            Direction::Right(_) => self.position.x += 1,
        }
    }
}

#[derive(Debug)]
pub struct Tail {
    pub position: Position,
    pub visited: Vec<Position>,
}

impl Tail {
    pub fn new() -> Tail {
        let mut visited = Vec::new();
        visited.push(Position { x: 0, y: 0 });
        Tail {
            position: Position { x: 0, y: 0 },
            visited,
        }
    }
}

pub fn process_part1(input: &str) -> String {
    let mut head = Head::new();
    let mut tail = Tail::new();

    let mut unique: BTreeSet<Position> = BTreeSet::new();
    unique.insert(Position::new());

    input.lines().for_each(|line| {
        let direction = line.parse::<Direction>().unwrap();
        let steps = match direction {
            Direction::Up(amount) => amount,
            Direction::Down(amount) => amount,
            Direction::Left(amount) => amount,
            Direction::Right(amount) => amount,
        };

        for _ in 0..steps {
            head.step(direction);
            let visited = tail.position.follow(&head.position);
            visited.iter().for_each(|pos| {
                unique.insert(pos.clone());
            });
        }
    });

    let amount = unique.len();

    amount.to_string()
}

pub fn process_part2(input: &str) -> String {
    let mut head = Head::new();
    let mut knot1 = Position::new();
    let mut knot2 = Position::new();
    let mut knot3 = Position::new();
    let mut knot4 = Position::new();
    let mut knot5 = Position::new();
    let mut knot6 = Position::new();
    let mut knot7 = Position::new();
    let mut knot8 = Position::new();
    let mut tail = Tail::new();

    let mut unique: BTreeSet<Position> = BTreeSet::new();
    unique.insert(Position::new());

    input.lines().for_each(|line| {
        let direction = line.parse::<Direction>().unwrap();
        let steps = match direction {
            Direction::Up(amount) => amount,
            Direction::Down(amount) => amount,
            Direction::Left(amount) => amount,
            Direction::Right(amount) => amount,
        };

        for _ in 0..steps {
            head.step(direction);
            knot1.follow(&head.position);
            knot2.follow(&knot1);
            knot3.follow(&knot2);
            knot4.follow(&knot3);
            knot5.follow(&knot4);
            knot6.follow(&knot5);
            knot7.follow(&knot6);
            knot8.follow(&knot7);
            let visited = tail.position.follow(&knot8);
            visited.iter().for_each(|pos| {
                unique.insert(pos.clone());
            });
        }
    });

    let amount = unique.len();

    amount.to_string()
}

#[cfg(test)]
mod tests {

    use crate::*;

    #[test]
    pub fn test_part_1() {
        let input: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

        assert_eq!(process_part1(input), "13");
    }

    #[test]
    pub fn test_part_2() {
        let input = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

        assert_eq!(process_part2(input), "36");
    }
}
