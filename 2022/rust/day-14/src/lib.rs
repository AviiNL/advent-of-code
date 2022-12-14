use std::io::Write;
use std::path::Path;
use std::{
    collections::HashMap,
    fmt::{Display, Formatter},
    fs::File,
};

#[derive(Debug)]
pub struct Tile(char);

#[derive(Debug)]
pub struct Map(HashMap<(i32, i32), Tile>);

pub struct Sand {
    x: i32,
    y: i32,
}

impl Sand {
    pub fn new(x: i32, y: i32) -> Self {
        Sand { x, y }
    }
}

pub struct SandEmitter {
    x: i32,
    y: i32,
}

pub struct Game {
    map: Map,
    sand: Vec<Sand>,
    sand_emitter: SandEmitter,
    floor: Option<i32>,
}

impl Game {
    pub fn load_map(input: &str) -> Self {
        let veins = input
            .lines()
            .map(|line| RockVein::from(line))
            .collect::<Vec<Vec<RockVein>>>();

        let map = Map::new(veins);
        let sand: Vec<Sand> = Vec::new();

        let sand_emitter = SandEmitter { x: 500, y: 0 };

        Game {
            map,
            sand,
            sand_emitter,
            floor: None,
        }
    }

    pub fn set_floor(&mut self, floor: i32) {
        self.floor = Some(floor);
    }

    pub fn is_occupied(&self, x: i32, y: i32) -> bool {
        self.map.is_collision(x, y)
            || self.sand.iter().any(|s| s.x == x && s.y == y)
            || self.floor.is_some() && self.floor.unwrap() == y
    }

    pub fn tick(&mut self) -> bool {
        // if there is no sand, spawn a sand particle from the emitter
        if self.sand.is_empty() {
            self.sand
                .push(Sand::new(self.sand_emitter.x, self.sand_emitter.y));
        }

        // for each sand particle
        for sand_index in self.sand.len() - 1..self.sand.len() {
            let mut new_sand_pos = (self.sand[sand_index].x, self.sand[sand_index].y);
            // if the sand particle is on the ground, do nothing
            if self.is_occupied(new_sand_pos.0, new_sand_pos.1 + 1) {
                // attempt to move diagonally left
                if !self.is_occupied(new_sand_pos.0 - 1, new_sand_pos.1 + 1) {
                    new_sand_pos.0 -= 1;
                } else {
                    // attempt to move diagonally right
                    if !self.is_occupied(new_sand_pos.0 + 1, new_sand_pos.1 + 1) {
                        new_sand_pos.0 += 1;
                    }
                }
            }

            // receck if the sand particle is on the ground
            if self.is_occupied(new_sand_pos.0, new_sand_pos.1 + 1) {
                // check if the emitter is not obscured
                if !self.is_occupied(self.sand_emitter.x, self.sand_emitter.y) {
                    self.sand
                        .push(Sand::new(self.sand_emitter.x, self.sand_emitter.y));
                } else {
                    println!("Unable to spawn sand particle, emitter is obscured");
                    return false;
                }
            } else {
                // if the sand particle is not on the ground, move down
                new_sand_pos.1 += 1;
                // update the sand particle with the new position
                self.sand[sand_index] = Sand::new(new_sand_pos.0, new_sand_pos.1);
            }
        }
        true
    }

    pub fn sand_in_abyss(&self) -> usize {
        self.sand
            .iter()
            .filter(|s| s.y > self.map.0.keys().map(|(_, y)| *y).max().unwrap())
            .count()
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // find the lowest x number
        let min_x_wall = self.map.0.keys().map(|(x, _)| x).min().unwrap();
        let min_x_sand = self.sand.iter().map(|s| s.x).min().unwrap();
        let min_x = if min_x_wall < &min_x_sand {
            min_x_wall - 1
        } else {
            &min_x_sand - 1
        };

        // find the highest x number
        let max_x_wall = self.map.0.keys().map(|(x, _)| x).max().unwrap();
        let max_x_sand = self.sand.iter().map(|s| s.x).max().unwrap();

        let max_x = if max_x_wall > &max_x_sand {
            max_x_wall + 1
        } else {
            &max_x_sand + 1
        };

        // find the highest y number
        let max_y = if self.floor.is_some() {
            self.floor.unwrap()
        } else {
            *self.map.0.keys().map(|(_, y)| y).max().unwrap()
        };

        // draw a grid
        for y in 0..=max_y {
            for x in min_x..=max_x {
                // check for sand
                if self.sand.iter().any(|s| s.x == x && s.y == y) {
                    write!(f, "o")?;
                    continue;
                }

                // check for floor
                if self.floor.is_some() && self.floor.unwrap() == y {
                    write!(f, "~")?;
                    continue;
                }

                if let Some(tile) = self.map.0.get(&(x, y)) {
                    write!(f, "{}", tile.0)?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl Map {
    pub fn new(veins: Vec<Vec<RockVein>>) -> Self {
        let mut m = HashMap::new();

        for vein in veins {
            for v in vein {
                let x_range = if v.x < v.x2 { v.x..=v.x2 } else { v.x2..=v.x };
                for x in x_range {
                    let y_range = if v.y < v.y2 { v.y..=v.y2 } else { v.y2..=v.y };
                    for y in y_range {
                        m.insert((x, y), Tile('#'));
                    }
                }
            }
        }

        Map(m)
    }

    pub fn is_collision(&self, x: i32, y: i32) -> bool {
        self.0.contains_key(&(x, y))
    }
}

#[derive(Debug)]
pub struct RockVein {
    pub x: i32,
    pub y: i32,
    pub x2: i32,
    pub y2: i32,
}

impl RockVein {
    pub fn from(input: &str) -> Vec<RockVein> {
        let parts = input.split(" -> ").collect::<Vec<&str>>();
        let mut parts = parts.windows(2);

        let mut veins = Vec::new();

        while let Some(pair) = parts.next() {
            let a = pair[0].split(',').collect::<Vec<&str>>();
            let b = pair[1].split(',').collect::<Vec<&str>>();

            let start_x = a[0].parse::<i32>().unwrap();
            let start_y = a[1].parse::<i32>().unwrap();

            let end_x = b[0].parse::<i32>().unwrap();
            let end_y = b[1].parse::<i32>().unwrap();

            veins.push(RockVein {
                x: start_x,
                y: start_y,
                x2: end_x,
                y2: end_y,
            });
        }

        veins
    }
}

impl Display for RockVein {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{},{} -> {},{}", self.x, self.y, self.x2, self.y2)
    }
}

pub fn process_part1(input: &str) -> String {
    let mut game = Game::load_map(input);

    while game.sand_in_abyss() == 0 {
        game.tick();
        // println!("Voided: {}", game.sand_in_abyss());
        // println!("{}", game);
    }

    // count sand particles
    let sand_count = game.sand.len() - game.sand_in_abyss();
    sand_count.to_string()
}

pub fn process_part2(input: &str) -> String {
    let mut game = Game::load_map(input);

    let max_y = game.map.0.keys().map(|(_, y)| *y).max().unwrap();

    game.set_floor(max_y + 2);

    let mut tick = 0;
    while game.tick() {
        // draw every frame to to a file in game/ if the file doesn't exist

        // Aborted this after like 5 hours or so, 260k files were generated, which added up to 3gb+ of data
        // After commenting out this code, it ran in like 10ish seconds on release....

        // if !Path::new(&format!("game/{}.txt", tick)).exists() {
        //     let mut file = File::create(format!("game/{}.txt", tick)).unwrap();
        //     write!(file, "{}", game).unwrap();
        // }

        // print tick every 1000 ticks, just so we know it's still running
        if tick % 10000 == 0 {
            println!("Tick: {}", tick);
        }

        tick += 1;
    }

    // count sand particles
    let sand_count = game.sand.len();
    sand_count.to_string()
}

#[cfg(test)]
mod tests {

    const INPUT: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    pub fn test_part_1() {
        assert_eq!(super::process_part1(INPUT), "24");
    }

    #[test]
    pub fn test_part_2() {
        assert_eq!(super::process_part2(INPUT), "93");
    }
}
