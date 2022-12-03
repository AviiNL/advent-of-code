trait ValueOf {
    fn get_value(&self) -> usize;
}

impl ValueOf for char {
    fn get_value(&self) -> usize {
        // charactesr a-z are 1-26
        // characters A-Z are 27-52

        let mut value = *self as i32;

        if value > 64 && value < 91 {
            value -= 38;
        } else if value > 96 && value < 123 {
            value -= 96;
        } else {
            value = 0;
        }

        value as usize
    }
}

pub fn process_part1(input: &str) -> String {
    let a: usize = input
        .lines()
        .map(|l| {
            // split the line exactly in half
            let (a, b) = l.split_at(l.len() / 2);
            // get all characters from a that also exist in b
            let mut c = a.chars().filter(|c| b.contains(*c)).collect::<Vec<char>>();
            c.dedup();
            c.iter().map(|c| c.get_value()).sum::<usize>()
        })
        .sum();

    a.to_string()
}

pub fn find_badge(input: &[&str]) -> usize {
    // get the character that exists in all lines
    let mut c = input[0].chars().collect::<Vec<char>>();
    for line in input {
        c = c
            .iter()
            .filter(|&c| line.contains(*c))
            .map(|c| *c)
            .collect::<Vec<char>>();
    }
    c.dedup();
    c.iter().map(|c| c.get_value()).sum::<usize>()
}

pub fn process_part2(input: &str) -> String {
    input
        .lines()
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|c| find_badge(c))
        .sum::<usize>()
        .to_string()
}

#[cfg(test)]
mod tests {

    const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    pub fn test_part_1() {
        assert_eq!(super::process_part1(INPUT), "157");
    }

    #[test]
    pub fn test_part_2() {
        assert_eq!(super::process_part2(INPUT), "70");
    }
}
