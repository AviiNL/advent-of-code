pub fn process_part1(input: &str) -> String {
    // make groups of numbers split by empty lines
    let groups = input.split("\n\n");

    // iterate over groups
    let mut result = 0;
    for group in groups {
        // split group by new line and parse to u32
        let answer = group
            .lines()
            .map(|line| line.trim())
            .filter(|line| line.len() > 0)
            .map(|line| line.parse::<u32>().unwrap())
            // sum all numbers in group
            .sum::<u32>();

        if answer > result {
            result = answer;
        }
    }

    result.to_string()
}

pub fn process_part2(input: &str) -> String {
    // make groups of numbers split by empty lines
    let groups = input.split("\n\n");

    // iterate over groups
    let mut results = Vec::new();
    for group in groups {
        // split group by new line and parse to u32
        let answer = group
            .lines()
            // trim all lines
            .map(|line| line.trim())
            .filter(|line| line.len() > 0)
            .map(|line| line.parse::<u32>().unwrap())
            // sum all numbers in group
            .sum::<u32>();

        results.push(answer);
    }

    // sort results
    results.sort();

    // take the last 3 values and sum them
    results
        .iter()
        .rev()
        .take(3)
        .fold(0, |acc, x| acc + x)
        .to_string()
}

#[cfg(test)]
mod tests {
    #[test]
    pub fn test_part_1() {
        let input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

        assert_eq!(super::process_part1(input), "24000");
    }

    #[test]
    pub fn test_part_2() {
        let input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

        assert_eq!(super::process_part2(input), "45000");
    }
}
