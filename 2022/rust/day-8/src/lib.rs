pub fn get_distance_up(matrix: &Vec<Vec<usize>>, x0: usize, y0: usize) -> usize {
    // check if we're on an edge looking up
    if y0 == 0 {
        return 0;
    }
    let x = x0;
    let mut y = y0;

    let value = matrix[y0][x0];
    let mut distance = 0;
    while y > 0 {
        y -= 1;
        distance += 1;
        if matrix[y][x] >= value {
            return distance;
        }
    }

    return distance;
}

pub fn get_distance_right(matrix: &Vec<Vec<usize>>, x0: usize, y0: usize) -> usize {
    // check if we're on an edge looking up
    if x0 == matrix[y0].len() - 1 {
        return 0;
    }
    let mut x = x0;
    let y = y0;

    let value = matrix[y0][x0];
    let mut distance = 0;
    while x < matrix[y].len() - 1 {
        x += 1;
        distance += 1;
        if matrix[y][x] >= value {
            return distance;
        }
    }

    return distance;
}

pub fn get_distance_down(matrix: &Vec<Vec<usize>>, x0: usize, y0: usize) -> usize {
    // check if we're on an edge looking up
    if y0 == matrix.len() - 1 {
        return 0;
    }
    let x = x0;
    let mut y = y0;

    let value = matrix[y0][x0];
    let mut distance = 0;
    while y < matrix.len() - 1 {
        y += 1;
        distance += 1;
        if matrix[y][x] >= value {
            return distance;
        }
    }

    return distance;
}

pub fn get_distance_left(matrix: &Vec<Vec<usize>>, x0: usize, y0: usize) -> usize {
    if x0 == 0 {
        return 0;
    }
    let mut x = x0;
    let y = y0;

    let value = matrix[y0][x0];
    let mut distance = 0;
    while x > 0 {
        x -= 1;
        distance += 1;
        if matrix[y][x] >= value {
            return distance;
        }
    }

    return distance;
}

pub fn is_visible(matrix: &Vec<Vec<usize>>, x0: usize, y0: usize) -> bool {
    let mut x = x0;
    let mut y = y0;

    let value = matrix[y][x];

    let mut visible_from_top = true;
    let mut visible_from_bottom = true;
    let mut visible_from_left = true;
    let mut visible_from_right = true;

    if x == 0 || x == matrix[y].len() - 1 || y == 0 || y == matrix.len() - 1 {
        return true;
    }

    while x > 0 && visible_from_left {
        x -= 1;
        if matrix[y][x] >= value {
            visible_from_left = false;
        }
    }

    x = x0;
    y = y0;
    while x < matrix[y].len() - 1 && visible_from_right {
        x += 1;
        if matrix[y][x] >= value {
            visible_from_right = false;
        }
    }

    x = x0;
    y = y0;
    while y > 0 && visible_from_top {
        y -= 1;
        if matrix[y][x] >= value {
            visible_from_top = false;
        }
    }

    x = x0;
    y = y0;
    while y < matrix.len() - 1 && visible_from_bottom {
        y += 1;
        if matrix[y][x] >= value {
            visible_from_bottom = false;
        }
    }

    visible_from_top || visible_from_bottom || visible_from_left || visible_from_right
}

pub fn get_distance(x0: usize, y0: usize, x1: usize, y1: usize) -> usize {
    ((x0 as isize - x1 as isize).abs() + (y0 as isize - y1 as isize).abs()) as usize
}

pub fn process_part1(input: &str) -> String {
    let matrix = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();

    let mut visible_count = 0;
    for y in 0..matrix.len() {
        for x in 0..matrix[y].len() {
            if is_visible(&matrix, x, y) {
                visible_count += 1;
            }
        }
    }

    visible_count.to_string()
}

pub fn process_part2(input: &str) -> String {
    let matrix = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();

    let mut score = 0;

    for y in 0..matrix.len() {
        for x in 0..matrix[y].len() {
            let up_distance = get_distance_up(&matrix, x, y);
            let down_distance = get_distance_down(&matrix, x, y);
            let left_distance = get_distance_left(&matrix, x, y);
            let right_distance = get_distance_right(&matrix, x, y);

            let s = up_distance * down_distance * left_distance * right_distance;
            if s > score {
                score = s;
            }
        }
    }

    score.to_string()
}

#[cfg(test)]
mod tests {

    use crate::*;

    const INPUT: &str = "30373
25512
65332
33549
35390";

    #[test]
    pub fn test_part_1() {
        assert_eq!(process_part1(INPUT), "21");
    }

    #[test]
    pub fn test_part_2() {
        assert_eq!(process_part2(INPUT), "8");
    }
}
