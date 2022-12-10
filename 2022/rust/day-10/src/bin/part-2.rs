use day_10::process_part2;
use std::fs;

fn main() {
    let file = fs::read_to_string("./input.txt").expect("Unable to read file");

    println!("Part 2:\n{}", process_part2(&file));
}
