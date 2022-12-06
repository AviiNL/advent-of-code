use std::collections::VecDeque;

pub fn process_part1(input: &str) -> String {
    let characters = input.chars();

    let mut marker: VecDeque<char> = VecDeque::with_capacity(4);
    let mut found = false;
    let mut index = Option::<usize>::None;

    let mut enumerator = characters.clone().enumerate().peekable();
    while let Some((_, c)) = enumerator.next() {
        // if marker is full, remove first character
        if marker.len() == 4 {
            marker.pop_front();
        }

        // add character to marker
        marker.push_back(c);

        if found {
            break;
        }

        if marker.len() >= 3 {
            // does marker contains duplicate characters?
            let mut dupe = false;
            for c in marker.iter() {
                let mut count = 0;
                for c2 in marker.iter() {
                    if c == c2 {
                        count += 1;
                    }
                }
                if count > 1 {
                    dupe = true;
                    break;
                }
            }

            if dupe {
                continue;
            }

            if let Some((idx, c)) = enumerator.peek() {
                if !marker.contains(c) {
                    index = Some(*idx);
                    found = true;
                }
            }
        }
    }

    // return marker as string
    index.unwrap().to_string()
}

pub fn process_part2(input: &str) -> String {
    let characters = input.chars();

    let mut marker: VecDeque<char> = VecDeque::with_capacity(14);
    let mut index = Option::<usize>::None;
    let mut enumerator = characters.clone().enumerate().peekable();
    'outer: while let Some((idx, c)) = enumerator.next() {
        // if marker is full, remove first character
        if marker.len() == 14 {
            marker.pop_front();
        }

        // add character to marker
        marker.push_back(c);

        // as soon as marker does not contain any duplicate characters with a length of 14, we have found the index
        if idx > 13 {
            // does marker contains duplicate characters?
            let mut dupe = false;
            for c in marker.iter() {
                let mut count = 0;
                for c2 in marker.iter() {
                    if c == c2 {
                        count += 1;
                    }
                }
                if count > 1 {
                    dupe = true;
                    break;
                }
            }

            if !dupe {
                index = Some(idx + 1);
                break 'outer;
            }
        }
    }

    // return marker as string
    index.unwrap().to_string()
}

#[cfg(test)]
mod tests {

    #[test]
    pub fn test_part_1() {
        assert_eq!(super::process_part1("bvwbjplbgvbhsrlpgdmjqwftvncz"), "5");
        assert_eq!(super::process_part1("nppdvjthqldpwncqszvftbrmjlhg"), "6");
        assert_eq!(
            super::process_part1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"),
            "10"
        );
        assert_eq!(
            super::process_part1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"),
            "11"
        );
    }

    #[test]
    pub fn test_part_2() {
        assert_eq!(super::process_part2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), "19");
        assert_eq!(super::process_part2("bvwbjplbgvbhsrlpgdmjqwftvncz"), "23");
        assert_eq!(super::process_part2("nppdvjthqldpwncqszvftbrmjlhg"), "23");
        assert_eq!(
            super::process_part2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"),
            "29"
        );
        assert_eq!(
            super::process_part2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"),
            "26"
        );
    }
}
