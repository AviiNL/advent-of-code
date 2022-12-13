use std::cmp::Ordering;

use lang::ast::{ArrayLiteral, Expression, NumericLiteral};

use crate::lang::parser::Parser;

mod lang;

// fn flatten(array: &ArrayLiteral) -> ArrayLiteral {
//     let mut elements = Vec::new();

//     for element in &array.elements {
//         match element {
//             Expression::Array(array) => {
//                 let flattened = flatten(array);
//                 elements.extend(flattened.elements);
//             }
//             _ => elements.push(element.clone()),
//         }
//     }

//     ArrayLiteral::new(elements)
// }

fn compare(a: &ArrayLiteral, b: &ArrayLiteral) -> Ordering {
    let most = if a.len() < b.len() { b.len() } else { a.len() };

    let mut a = a.clone();
    let mut b = b.clone();

    for _ in 0..most {
        let a_1 = &a.elements.pop_front();
        let b_1 = &b.elements.pop_front();

        if let Some(a_1) = a_1 {
            if let Some(b_1) = b_1 {
                // if a_1 is a number, but b_1 is an array, turn a_1 into an array and run compare again
                if let Expression::Number(a_1) = a_1 {
                    if let Expression::Array(b_1) = b_1 {
                        let a_1 = ArrayLiteral::new(vec![Expression::Number(a_1.clone())]);
                        let cmp = compare(&a_1, b_1);
                        if cmp != Ordering::Equal {
                            return cmp;
                        }
                    }
                }

                // if b_1 is a number but a_1 is an array, turn b_1 into an array and run compare again
                if let Expression::Number(b_1) = b_1 {
                    if let Expression::Array(a_1) = a_1 {
                        let b_1 = ArrayLiteral::new(vec![Expression::Number(b_1.clone())]);
                        let cmp = compare(a_1, &b_1);
                        if cmp != Ordering::Equal {
                            return cmp;
                        }
                    }
                }

                // if both are numbers, compare them
                if let Expression::Number(a_1) = a_1 {
                    if let Expression::Number(b_1) = b_1 {
                        if a_1.value < b_1.value {
                            return Ordering::Less;
                        }
                        if a_1.value > b_1.value {
                            return Ordering::Greater;
                        }
                    }
                }

                // if both are arrays, compare them
                if let Expression::Array(a_1) = a_1 {
                    if let Expression::Array(b_1) = b_1 {
                        let cmp = compare(a_1, b_1);
                        if cmp != Ordering::Equal {
                            return cmp;
                        }
                    }
                }
            } else {
                // right side ran out of elements, so b_1 is smaller
                return Ordering::Greater;
            }
        } else {
            // left side ran out of elements, so b_1 is greater
            return Ordering::Less;
        }
    }
    Ordering::Equal
}

pub fn process_part1<'a>(input: &'a str) -> String {
    let ast = Parser::produce_ast(input).unwrap();

    let mut index = 0;

    let mut corrent_indecies = vec![];

    for pair in &ast.body {
        match pair {
            Expression::Pair(p) => {
                index += 1;
                if let Expression::Array(a) = p[0].clone() {
                    if let Expression::Array(b) = p[1].clone() {
                        let _ = match compare(&a, &b) {
                            Ordering::Less => {
                                corrent_indecies.push(index);
                                "less than"
                            }
                            Ordering::Greater => "greater than",
                            Ordering::Equal => "equal to",
                        };
                    }
                }
            }
            _ => panic!("Invalid input"),
        }
    }

    // sum the correct_indices for the answer
    corrent_indecies.iter().sum::<i32>().to_string()
}

pub fn process_part2(input: &str) -> String {
    let ast = Parser::produce_ast(input).unwrap();

    let mut all_packets = vec![];

    // This is a bit convoluted ¯\_(ツ)_/¯
    all_packets.push(ArrayLiteral::new(vec![Expression::Array(
        ArrayLiteral::new(vec![Expression::Number(NumericLiteral { value: 2 })]),
    )]));

    all_packets.push(ArrayLiteral::new(vec![Expression::Array(
        ArrayLiteral::new(vec![Expression::Number(NumericLiteral { value: 6 })]),
    )]));

    for pair in &ast.body {
        match pair {
            Expression::Pair(p) => {
                // index += 1;
                if let Expression::Array(a) = p[0].clone() {
                    if let Expression::Array(b) = p[1].clone() {
                        all_packets.push(a.clone());
                        all_packets.push(b.clone());
                    }
                }
            }
            _ => panic!("Invalid input"),
        }
    }

    all_packets.sort_by(compare);

    let mut divider_indecies = vec![];

    for (i, p) in all_packets.iter().enumerate() {
        let index = i + 1;

        if "[[2]]" == format!("{}", p) || "[[6]]" == format!("{}", p) {
            divider_indecies.push(index);
        }
    }

    divider_indecies.iter().product::<usize>().to_string()
}

#[cfg(test)]
mod tests {

    use crate::*;

    const INPUT: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    pub fn test_part_1() {
        assert_eq!(process_part1(INPUT), "13");
    }

    #[test]
    pub fn test_part_2() {
        assert_eq!(process_part2(INPUT), "140");
    }
}
