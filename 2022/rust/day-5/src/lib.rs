use std::{
    collections::VecDeque,
    fmt::{Display, Formatter},
};

#[derive(Debug)]
pub struct Crate(pub char);

#[derive(Debug)]
pub struct Warehouse {
    pub stacks: Vec<VecDeque<Crate>>,
}

impl Display for Warehouse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut output = String::new();

        for (index, stack) in self.stacks.iter().enumerate() {
            // Print stack as ABCDE
            let stack: String = stack.iter().map(|c| c.0).collect();
            output += format!("{}: {}\n", index + 1, stack).as_str();
        }
        write!(f, "{}", output)
    }
}

impl Warehouse {
    pub fn from_input(input: &str) -> Self {
        let mut stacks: Vec<VecDeque<Crate>> = Vec::new();

        for line in input.lines() {
            let mut characters = line.chars().peekable();
            if line.trim().len() == 0 {
                break;
            }
            // in a loop, grab 3 characters at a time
            let mut column = 0;
            while let Some(c) = characters.next() {
                if c == '[' {
                    let c = characters.next().unwrap();
                    let c = Crate(c);

                    // create the stack if it doesn't exist yet
                    if stacks.len() <= column {
                        stacks.push(vec![].into());
                    }

                    stacks[column].push_front(c);

                    // skip the closing bracket
                    characters.next();
                    column += 1;
                } else if c == ' ' {
                    if stacks.len() <= column {
                        stacks.push(vec![].into());
                    }

                    // eat 2 more characters
                    characters.next();
                    characters.next();
                    column += 1;
                } else {
                    panic!("Unexpected character: {}", c);
                }
                // eat the space
                characters.next();
            }
        }

        Warehouse { stacks }
    }

    pub fn process_input_9000(&mut self, input: &str) {
        let mut start = false;
        for line in input.lines() {
            if line.trim().len() == 0 {
                start = true;
                continue;
            }

            if start == false {
                continue;
            }

            // move 1 from 2 to 1
            let mut words = line.trim().split_whitespace();
            let command = words.next().unwrap();
            let amount = words.next().unwrap().parse::<usize>().unwrap();
            // skip the word from
            words.next();
            let from = words.next().unwrap().parse::<usize>().unwrap();
            // skip the word to
            words.next();
            let to = words.next().unwrap().parse::<usize>().unwrap();

            match command {
                "move" => self.move_crates_9000(amount, from, to),
                _ => panic!("Unexpected command: {}", command),
            }
        }
    }

    pub fn move_crates_9000(&mut self, amount: usize, from: usize, to: usize) {
        for _ in (0..amount).rev() {
            let crate_ = self.stacks[from - 1].pop_back().unwrap();
            self.stacks[to - 1].push_back(crate_);
        }
    }

    pub fn process_input_9001(&mut self, input: &str) {
        let mut start = false;
        for line in input.lines() {
            if line.trim().len() == 0 {
                start = true;
                continue;
            }

            if start == false {
                continue;
            }

            // move 1 from 2 to 1
            let mut words = line.trim().split_whitespace();
            let command = words.next().unwrap();
            let amount = words.next().unwrap().parse::<usize>().unwrap();
            // skip the word from
            words.next();
            let from = words.next().unwrap().parse::<usize>().unwrap();
            // skip the word to
            words.next();
            let to = words.next().unwrap().parse::<usize>().unwrap();

            match command {
                "move" => self.move_crates_9001(amount, from, to),
                _ => panic!("Unexpected command: {}", command),
            }
        }
    }

    pub fn move_crates_9001(&mut self, amount: usize, from: usize, to: usize) {
        // grab amount crates from the top of from and place it on top of to
        let mut crates = VecDeque::new();
        for _ in (0..amount).rev() {
            let crate_ = self.stacks[from - 1].pop_back().unwrap();
            crates.push_front(crate_);
        }

        self.stacks[to - 1].append(&mut crates);
    }

    pub fn top_row(&self) -> String {
        let mut output = String::new();

        for stack in self.stacks.iter() {
            if let Some(crate_) = stack.back() {
                output += format!("{}", crate_.0).as_str();
            }
        }

        output
    }
}

pub fn process_part1(input: &str) -> String {
    let mut warehouse = Warehouse::from_input(&input);

    warehouse.process_input_9000(input);

    println!("{}", warehouse);

    warehouse.top_row()
}

pub fn process_part2(input: &str) -> String {
    let mut warehouse = Warehouse::from_input(&input);

    warehouse.process_input_9001(input);

    println!("{}", warehouse);

    warehouse.top_row()
}

#[cfg(test)]
mod tests {

    const INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    pub fn test_part_1() {
        assert_eq!(super::process_part1(INPUT), "CMZ");
    }

    #[test]
    pub fn test_part_2() {
        assert_eq!(super::process_part2(INPUT), "MCD");
    }
}
