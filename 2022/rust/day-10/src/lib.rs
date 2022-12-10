trait Ticker {
    fn tick(&mut self) -> bool;
}

pub struct Noop {
    pub counter: i32,
}

impl Noop {
    pub fn new() -> Noop {
        Noop { counter: 1 }
    }
}

impl Ticker for Noop {
    fn tick(&mut self) -> bool {
        self.counter -= 1;
        self.counter == 0
    }
}

pub struct AddX {
    pub counter: i32,
    pub x: i32,
}

impl AddX {
    pub fn new(x: i32) -> AddX {
        AddX { counter: 2, x }
    }

    pub fn run(&mut self, x: &mut i32) {
        *x += self.x;
    }
}

impl Ticker for AddX {
    fn tick(&mut self) -> bool {
        self.counter -= 1;
        self.counter == 0
    }
}

pub enum Command {
    Noop(Noop),
    AddX(AddX),
}

pub fn process_part1(input: &str) -> String {
    let mut command_list: Vec<Command> = input
        .lines()
        .map(|l| {
            let mut parts = l.split_whitespace();
            let command = parts.next().unwrap();
            match command {
                "noop" => Command::Noop(Noop::new()),
                "addx" => {
                    let value = parts.next().unwrap().parse::<i32>().unwrap();
                    Command::AddX(AddX::new(value))
                }
                _ => panic!("Unknown command"),
            }
        })
        .collect();

    let mut x = 1;
    let mut index = 0;

    let mut results = Vec::new();
    loop {
        results.push(x.clone());
        let command = &mut command_list[index];
        match command {
            Command::Noop(noop) => {
                if noop.tick() {
                    index += 1;
                }
            }
            Command::AddX(addx) => {
                if addx.tick() {
                    addx.run(&mut x);
                    index += 1;
                }
            }
        }

        if index >= command_list.len() {
            break;
        }
    }

    let twentieth = results[19] * 20;
    let sixtieth = results[59] * 60;
    let hundredth = results[99] * 100;
    let hundred_fortieth = results[139] * 140;
    let hundred_eightieth = results[179] * 180;
    let two_hundred_twentieth = results[219] * 220;

    // add all
    let sum = twentieth
        + sixtieth
        + hundredth
        + hundred_fortieth
        + hundred_eightieth
        + two_hundred_twentieth;

    sum.to_string()
}

pub fn process_part2(input: &str) -> String {
    let mut command_list: Vec<Command> = input
        .lines()
        .map(|l| {
            let mut parts = l.split_whitespace();
            let command = parts.next().unwrap();
            match command {
                "noop" => Command::Noop(Noop::new()),
                "addx" => {
                    let value = parts.next().unwrap().parse::<i32>().unwrap();
                    Command::AddX(AddX::new(value))
                }
                _ => panic!("Unknown command"),
            }
        })
        .collect();

    let mut x = 1;
    let mut index = 0;

    let mut results = Vec::new();
    loop {
        results.push(x.clone());
        let command = &mut command_list[index];
        match command {
            Command::Noop(noop) => {
                if noop.tick() {
                    index += 1;
                }
            }
            Command::AddX(addx) => {
                if addx.tick() {
                    addx.run(&mut x);
                    index += 1;
                }
            }
        }

        if index >= command_list.len() {
            break;
        }
    }

    // "borrowed" from https://github.com/scristobal/advent-of-code/blob/057e6da8cc90ed382c978f63f5512dc08c0a8abd/day-10/src/lib.rs
    let display = std::iter::zip(0..240, results)
        .map(|(pix, cmd)| (pix % 40, cmd))
        .map(|(pix, x)| {
            if x - 1 <= pix && pix <= x + 1 {
                '#'
            } else {
                '.'
            }
        })
        .collect::<Vec<char>>();

    // collect display in a string, 40 chars per line
    let mut display_string = String::new();
    for i in 0..240 {
        display_string.push(display[i]);
        if i % 40 == 39 {
            display_string.push('\n');
        }
    }

    display_string.trim().to_string()
}

#[cfg(test)]
mod tests {

    use crate::*;

    const INPUT: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    #[test]
    pub fn test_part_1() {
        assert_eq!(process_part1(INPUT), "13140");
    }

    #[test]
    pub fn test_part_2() {
        let result: &str = "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....";

        assert_eq!(process_part2(INPUT), result);
    }
}
