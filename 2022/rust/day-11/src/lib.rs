use std::{
    collections::{BTreeMap, VecDeque},
    fmt::Formatter,
};

use nom::IResult;

pub struct Monkey<'a> {
    pub items: VecDeque<i64>,
    pub inspected_items: i64,
    inspect: Box<dyn Fn(i64) -> i64 + 'a>,
    pub diviser: i64,
    test: Box<dyn Fn(i64) -> i64 + 'a>,
}

impl<'a> Monkey<'a> {
    pub fn inspect(&mut self, prime: i64) {
        let item = self.items.pop_front();
        if let Some(item) = item {
            self.inspected_items += 1;
            let new_item = (self.inspect)(item);
            self.items.push_front(new_item % prime);
        }
    }

    pub fn bored(&mut self) {
        let item = self.items.pop_front();

        if let Some(item) = item {
            let new_item = (item as f64 / 3.0).floor() as i64;
            self.items.push_front(new_item);
        }
    }

    pub fn throw_to(&mut self) -> Option<(i64, i64)> {
        let item = self.items.pop_front();

        if let Some(item) = item {
            let new_id = (self.test)(item);
            return Some((item.clone(), new_id));
        }

        None
    }
}

pub struct MonkeyPit<'a> {
    pub monkeys: BTreeMap<i64, Monkey<'a>>,
}

impl<'a> std::fmt::Debug for Monkey<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Monkey")
            .field("items", &self.items)
            .finish()
    }
}

pub fn parse_id(input: &str) -> IResult<&str, i64> {
    let (input, _) = nom::bytes::complete::tag("Monkey ")(input)?;
    let (input, id) = nom::character::complete::i64(input)?;
    let (input, _) = nom::character::complete::char(':')(input)?;
    let (input, _) = nom::character::complete::newline(input)?;
    Ok((input, id))
}

pub fn parse_items(input: &str) -> IResult<&str, VecDeque<i64>> {
    let (input, _) = nom::character::complete::multispace0(input)?;
    let (input, _) = nom::bytes::complete::tag("Starting items: ")(input)?;
    let (input, items) = nom::multi::separated_list1(
        nom::bytes::complete::tag(", "),
        nom::character::complete::i64,
    )(input)?;
    let items = items.into_iter().collect();
    let (input, _) = nom::character::complete::newline(input)?;

    Ok((input, items))
}

fn operate<'a>(i: i64, left: &'a str, operation: &'a str, right: &'a str) -> i64 {
    let left = if left == "old" {
        i
    } else {
        left.parse().unwrap()
    };
    let right = if right == "old" {
        i
    } else {
        (*right.clone()).parse().unwrap()
    };

    match operation {
        "+" => left + right,
        "*" => left * right,
        _ => panic!("Unknown operator: {}", operation),
    }
}

pub fn parse_operation<'a>(input: &'a str) -> IResult<&str, Box<dyn Fn(i64) -> i64 + 'a>> {
    let (input, _) = nom::character::complete::multispace0(input)?;
    let (input, _) = nom::bytes::complete::tag("Operation: new = ")(input)?;
    let (input, op_as_str) = nom::bytes::complete::is_not("\n")(input)?;

    let operation: Vec<&str> = op_as_str.split_whitespace().collect();

    let operation = match operation.as_slice() {
        [left, operator, right] => Box::new(|i| operate(i, left, operator, right)),
        _ => panic!("Unknown operation: {:?}", operation),
    };

    Ok((input, operation))
}

pub fn parse_test(input: &str) -> IResult<&str, (i64, Box<dyn Fn(i64) -> i64>)> {
    let (input, _) = nom::character::complete::multispace0(input)?;
    let (input, _) = nom::bytes::complete::tag("Test: divisible by ")(input)?;
    let (input, test) = nom::character::complete::i64(input)?;
    let (input, _) = nom::bytes::complete::tag("\n")(input)?;

    // parse out monkey id if true
    let (input, _) = nom::character::complete::multispace0(input)?;
    let (input, _) = nom::bytes::complete::tag("If true: throw to monkey ")(input)?;
    let (input, true_id) = nom::character::complete::i64(input)?;

    // parse out if false
    let (input, _) = nom::character::complete::multispace0(input)?;
    let (input, _) = nom::bytes::complete::tag("If false: throw to monkey ")(input)?;
    let (input, false_id) = nom::character::complete::i64(input)?;

    Ok((
        input,
        (
            test.clone(),
            Box::new(move |i| if i % test == 0 { true_id } else { false_id }),
        ),
    ))
}

pub fn parse_monkey<'a>(input: &'a str) -> IResult<&'a str, (i64, Monkey)> {
    let (input, id) = parse_id(input)?;
    let (input, items) = parse_items(input)?;
    let (input, inspect) = parse_operation(input)?;
    let (input, test) = parse_test(input)?;

    // remove any optional newlines
    let (input, _) = nom::character::complete::multispace0(input)?;

    let diviser = test.0;
    let test = test.1;

    Ok((
        input,
        (
            id,
            Monkey {
                items,
                inspected_items: 0,
                inspect,
                diviser,
                test,
            },
        ),
    ))
}

pub fn process_part1<'a>(input: &'a str) -> String {
    let monkeys = match nom::multi::many1(parse_monkey)(input) {
        Ok((_, monkeys)) => monkeys,
        Err(e) => panic!("Error parsing input: {:?}", e),
    };

    let mut monkeys: BTreeMap<i64, Monkey> = monkeys.into_iter().collect();

    const MAX_ROUNDS: usize = 20;
    let mut round: usize = 0;
    let mut monkey_turn_id: i64 = 0;

    let prime = monkeys.iter().map(|(_, monkey)| monkey.diviser).product();

    while round < MAX_ROUNDS {
        let mut receivers: BTreeMap<i64, Vec<i64>> = BTreeMap::new();

        let monkey = monkeys.get_mut(&monkey_turn_id).unwrap();
        while monkey.items.len() != 0 {
            monkey.inspect(prime);
            monkey.bored();
            if let Some((item, monkey_id)) = monkey.throw_to() {
                receivers.entry(monkey_id).or_insert(vec![]).push(item);
            }
        }

        for (monkey_id, items) in receivers {
            let monkey = monkeys.get_mut(&monkey_id).unwrap();
            for item in items {
                monkey.items.push_back(item);
            }
        }

        monkey_turn_id += 1;
        if monkey_turn_id == monkeys.len() as i64 {
            monkey_turn_id = 0;
            round += 1;
        }
    }

    // grab the 2 monkeys that inspected the most items
    let mut monkeys: Vec<_> = monkeys.into_iter().collect();
    monkeys.sort_by(|(_, a), (_, b)| b.inspected_items.cmp(&a.inspected_items));

    let sum: i64 = monkeys
        .iter()
        .take(2)
        .map(|(_, monkey)| monkey.inspected_items)
        .product();

    sum.to_string()
}

pub fn process_part2(input: &str) -> String {
    let monkeys = match nom::multi::many1(parse_monkey)(input) {
        Ok((_, monkeys)) => monkeys,
        Err(e) => panic!("Error parsing input: {:?}", e),
    };

    let mut monkeys: BTreeMap<i64, Monkey> = monkeys.into_iter().collect();

    let prime = monkeys.iter().map(|(_, monkey)| monkey.diviser).product();

    const MAX_ROUNDS: usize = 10000;
    let mut round: usize = 0;
    let mut monkey_turn_id: i64 = 0;

    while round < MAX_ROUNDS {
        let mut receivers: BTreeMap<i64, Vec<i64>> = BTreeMap::new();

        let monkey = monkeys.get_mut(&monkey_turn_id).unwrap();
        while monkey.items.len() != 0 {
            monkey.inspect(prime);
            // monkey.bored();
            if let Some((item, monkey_id)) = monkey.throw_to() {
                receivers.entry(monkey_id).or_insert(vec![]).push(item);
            }
        }

        for (monkey_id, items) in receivers {
            let monkey = monkeys.get_mut(&monkey_id).unwrap();
            for item in items {
                monkey.items.push_back(item);
            }
        }

        monkey_turn_id += 1;
        if monkey_turn_id == monkeys.len() as i64 {
            monkey_turn_id = 0;
            round += 1;
        }
    }

    // grab the 2 monkeys that inspected the most items
    let mut monkeys: Vec<_> = monkeys.into_iter().collect();
    monkeys.sort_by(|(_, a), (_, b)| b.inspected_items.cmp(&a.inspected_items));

    let sum: i64 = monkeys
        .iter()
        .take(2)
        .map(|(_, monkey)| monkey.inspected_items)
        .product();

    sum.to_string()
}

#[cfg(test)]
mod tests {

    use crate::*;

    const INPUT: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    pub fn test_part_1() {
        assert_eq!(process_part1(INPUT), "10605");
    }

    #[test]
    pub fn test_part_2() {
        assert_eq!(process_part2(INPUT), "2713310158");
    }
}
