use std::fmt::{Display, Formatter};

pub struct Path {
    pub path: Vec<Point>,
    pub cost: i32,
}

impl Path {
    pub fn new(path: Vec<Point>, cost: i32) -> Self {
        Self { path, cost }
    }

    pub fn draw(&self, map: &Heightmap) -> String {
        let str = map.to_string();
        let mut lines = str.lines().map(|s| s.to_string()).collect::<Vec<String>>();

        let mut previous_point = self.path[0];

        for point in &self.path {
            let mut chars = lines[point.y as usize].chars().collect::<Vec<char>>();

            if point == &previous_point {
                chars[point.x as usize] = '#';
            } else if point.x == previous_point.x {
                if point.y > previous_point.y {
                    chars[point.x as usize] = 'v';
                } else {
                    chars[point.x as usize] = '^';
                }
            } else if point.y == previous_point.y {
                if point.x > previous_point.x {
                    chars[point.x as usize] = '>';
                } else {
                    chars[point.x as usize] = '<';
                }
            } else {
                chars[point.x as usize] = 'X';
            }

            lines[point.y as usize] = chars.iter().collect::<String>();

            previous_point = *point;
        }
        lines.join("\n")
    }
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

pub struct Heightmap {
    map: Vec<Vec<u8>>,
}

impl Heightmap {
    pub fn from_input(input: &str) -> Self {
        let heightmap = input
            .lines()
            .map(|line| line.replace("S", "a").replace("E", "z"))
            .map(|line| line.chars().map(|c| c as u8 - 97 + 1).collect::<Vec<u8>>())
            .collect::<Vec<Vec<u8>>>();

        Self { map: heightmap }
    }

    pub fn get_cost(&self, current: &Point, point: &Point) -> i32 {
        let c = self.map[current.y as usize][current.x as usize] as i32;
        let p = self.map[point.y as usize][point.x as usize] as i32;

        if c > p {
            c - p
        } else if c == p {
            1
        } else {
            1 + (p - c)
        }
    }

    pub fn too_steep(&self, current: &Point, point: &Point) -> bool {
        // we can climb one up, or we can go down
        let c = self.map[current.y as usize][current.x as usize] as i32;
        let p = self.map[point.y as usize][point.x as usize] as i32;

        !(c > p || c == p || c + 1 == p)
    }

    pub fn too_deep(&self, current: &Point, point: &Point) -> bool {
        // we can climb one up, or we can go down
        let c = self.map[current.y as usize][current.x as usize] as i32;
        let p = self.map[point.y as usize][point.x as usize] as i32;

        !(c < p || c == p || c == p + 1)
    }
}

impl Display for Heightmap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for line in &self.map {
            for c in line {
                // convert back into a char
                let c = (c + 97 - 1) as char;
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl std::fmt::Debug for Heightmap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for line in &self.map {
            for c in line {
                // if c is less than 10, preceed with a space
                if c < &10 {
                    write!(f, " ")?;
                }
                write!(f, "{} ", c)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

pub fn find_char(input: &str, char: char) -> Point {
    let point = input
        .lines()
        .enumerate()
        .find(|(_, line)| line.contains(char))
        .map(|(i, line)| (line.find(char).unwrap(), i))
        .unwrap();

    Point {
        x: point.0 as i32,
        y: point.1 as i32,
    }
}

pub fn process_part1<'a>(input: &'a str) -> String {
    let start = find_char(input, 'S');
    let end = find_char(input, 'E');

    println!("Start: {:?}", start);
    println!("End: {:?}", end);

    let heightmap = Heightmap::from_input(input);

    // use the pathfinding crate to find the shortest path between start and end on hightmap
    let path = pathfinding::directed::astar::astar(
        &start,
        |current| {
            let neighbours = vec![
                // Up
                Point {
                    x: current.x,
                    y: current.y - 1,
                },
                // Down
                Point {
                    x: current.x,
                    y: current.y + 1,
                },
                // Left
                Point {
                    x: current.x - 1,
                    y: current.y,
                },
                // Right
                Point {
                    x: current.x + 1,
                    y: current.y,
                },
            ];

            let neighbours = neighbours
                .into_iter()
                // stay in bounds
                .filter(|point| {
                    point.x >= 0
                        && point.x < heightmap.map[0].len() as i32
                        && point.y >= 0
                        && point.y < heightmap.map.len() as i32
                })
                // filter out the neighbours that are too steep
                .filter(|point| !heightmap.too_steep(current, point))
                .collect::<Vec<Point>>();

            // return the neighbours with their cost
            neighbours
                .into_iter()
                .map(|point| (point, heightmap.get_cost(current, &point)))
                .collect::<Vec<(Point, i32)>>()
        },
        |p| {
            // calculate the distance between the current point and the end point
            let dx = (p.x - end.x).abs();
            let dy = (p.y - end.y).abs();
            dx + dy
        },
        |p| p == &end,
    );

    println!("{}", heightmap);

    if let Some(path) = path {
        let path = Path::new(path.0, path.1);

        println!("{}", path.draw(&heightmap));
        return (path.path.len() - 1).to_string();
    }

    panic!("No Path found!");
}

pub fn process_part2(input: &str) -> String {
    let start = find_char(input, 'E');

    println!("Start: {:?}", start);

    let heightmap = Heightmap::from_input(input);

    // use the pathfinding crate to find the shortest path between start and end on hightmap
    let path = pathfinding::directed::dijkstra::dijkstra(
        &start,
        |current| {
            let neighbours = vec![
                // Up
                Point {
                    x: current.x,
                    y: current.y - 1,
                },
                // Down
                Point {
                    x: current.x,
                    y: current.y + 1,
                },
                // Left
                Point {
                    x: current.x - 1,
                    y: current.y,
                },
                // Right
                Point {
                    x: current.x + 1,
                    y: current.y,
                },
            ];

            let neighbours = neighbours
                .into_iter()
                // stay in bounds
                .filter(|point| {
                    point.x >= 0
                        && point.x < heightmap.map[0].len() as i32
                        && point.y >= 0
                        && point.y < heightmap.map.len() as i32
                })
                // filter out the neighbours that are too steep
                .filter(|point| !heightmap.too_deep(current, point))
                .collect::<Vec<Point>>();

            // return the neighbours with their cost
            neighbours
                .into_iter()
                .map(|point| (point, heightmap.get_cost(current, &point)))
                .collect::<Vec<(Point, i32)>>()
        },
        |p| heightmap.map[p.y as usize][p.x as usize] == 1,
    );

    println!("{}", heightmap);

    if let Some(path) = path {
        let path = Path::new(path.0, path.1);

        println!("{}", path.draw(&heightmap));
        return (path.path.len() - 1).to_string();
    }

    panic!("No Path found!");
}

#[cfg(test)]
mod tests {

    use crate::*;

    const INPUT: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    pub fn test_part_1() {
        assert_eq!(process_part1(INPUT), "31");
    }

    #[test]
    pub fn test_part_2() {
        assert_eq!(process_part2(INPUT), "29");
    }
}
