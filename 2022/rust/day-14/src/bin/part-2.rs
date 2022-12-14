use day_14::process_part2;
use std::fs;

fn main() {
    let file = fs::read_to_string("./input.txt").expect("Unable to read file");

    println!("Part 1: {}", process_part2(&file));
}
