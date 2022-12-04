pub fn process_part1(input: &str) -> String {
    input
        .lines()
        .map(|l| {
            let pairs = l.split(',');
            let mut ranges = pairs.map(|p| {
                let mut range = p.split('-');
                let start = range.next().unwrap().parse::<u32>().unwrap();
                let end = range.next().unwrap().parse::<u32>().unwrap();
                start..=end
            });

            let one = ranges.next().unwrap();
            let two = ranges.next().unwrap();

            // check if two completely contains one
            if one.start() >= two.start() && one.end() <= two.end() {
                return 1;
            }

            // check if one completely contains two
            if two.start() >= one.start() && two.end() <= one.end() {
                return 1;
            }

            0
        })
        .sum::<i32>()
        .to_string()
}

pub fn process_part2(input: &str) -> String {
    input
        .lines()
        .map(|l| {
            let pairs = l.split(',');
            let mut ranges = pairs.map(|p| {
                let mut range = p.split('-');
                let start = range.next().unwrap().parse::<u32>().unwrap();
                let end = range.next().unwrap().parse::<u32>().unwrap();
                start..=end
            });

            let one = ranges.next().unwrap();
            let two = ranges.next().unwrap();

            // check if one overlaps two
            if one.start() <= two.start() && one.end() >= two.start() {
                return 1;
            }

            // check if two overlaps one
            if two.start() <= one.start() && two.end() >= one.start() {
                return 1;
            }

            0
        })
        .sum::<i32>()
        .to_string()
}

#[cfg(test)]
mod tests {

    const INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    pub fn test_part_1() {
        assert_eq!(super::process_part1(INPUT), "2");
    }

    #[test]
    pub fn test_part_2() {
        assert_eq!(super::process_part2(INPUT), "4");
    }
}
