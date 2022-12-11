use std::io::{stdin, Read};

use itertools::Itertools;

#[derive(Debug)]
struct Monkey {
    items: Vec<i32>,
    operation: Vec<String>,  // operand1, operation, operand2
    test: i32,
    if_true: u8,
    if_false: u8,
    inspections: usize,
}

fn inspect(item: i32, operation: &Vec<String>) -> i64 {
    assert!(operation[0] == "old", "operation[0] must be \"old\"");
    let op1 = item as i64;
    let op2 = if operation[2] == "old" { item as i64 } else { operation[2].parse::<i64>().unwrap() };
    // println!("{} {} {}", item, operation[1], op2);
    let inspect_result = match operation[1].as_str() {
        "+" => op1 + op2,
        "-" => op1 - op2,
        "*" => op1 * op2,
        &_ => todo!(),
    };
    inspect_result
}

fn parse_monkeys(input: &str) -> Vec<Monkey> {
    input
        .split("\n\n")
        .map(|ms| {
            let mls = ms.lines().collect_vec();
            Monkey {
                items: mls[1].trim_start().trim_start_matches("Starting items: ").split(", ").map(|item| item.parse::<i32>().unwrap()).collect_vec(),
                operation: mls[2].trim_start().trim_start_matches("Operation: new = ").split(" ").map(|op| op.to_string()).collect_vec(),
                test: mls[3].trim_start().trim_start_matches("Test: divisible by ").parse::<i32>().unwrap(),
                if_true: mls[4].trim_start().trim_start_matches("If true: throw to monkey ").parse::<u8>().unwrap(),
                if_false: mls[5].trim_start().trim_start_matches("If false: throw to monkey ").parse::<u8>().unwrap(),
                inspections: 0,
            }
        })
        .collect_vec()
}

fn part1(input: &str) -> usize {
    let mut monkeys = parse_monkeys(input);

    for _round in 0..20 {
        for i in 0..monkeys.len() {
            let movements = {
                let monkey = &mut monkeys[i];
                let movements = monkey.items.iter().map(|item| {
                    let item_new = (inspect(*item, &monkey.operation) / 3) as i32;
                    let destination = if item_new % monkey.test == 0 { monkey.if_true } else { monkey.if_false };
                    (item_new, destination)
                }).collect_vec();
                monkey.inspections += monkey.items.len();
                monkey.items.clear();
                movements
            };
            movements.iter().for_each(|(item, m)| (&mut monkeys[*m as usize]).items.push(*item));
        }

        // println!("Round {}", _round + 1);
        // for (i, monkey) in monkeys.iter().enumerate() {
        //     println!("  Monkey {}: {:?}", i, monkey.items);
        // }
    }
    let monkey_business = monkeys.iter().map(|m| m.inspections).sorted_unstable().rev().take(2).reduce(|a, b| a * b).unwrap();
    monkey_business
}

fn part2(input: &str) -> usize {
    let mut monkeys = parse_monkeys(input);
    let modulo = monkeys.iter().map(|m| m.test).reduce(|a, b| a * b).unwrap() as i64;

    for _round in 0..10000 {
        for i in 0..monkeys.len() {
            let movements = {
                let monkey = &mut monkeys[i];
                let movements = monkey.items.iter().map(|item| {
                    let item_new = (inspect(*item, &monkey.operation) % modulo) as i32;
                    let destination = if item_new % monkey.test == 0 { monkey.if_true } else { monkey.if_false };
                    (item_new, destination)
                }).collect_vec();
                monkey.inspections += monkey.items.len();
                monkey.items.clear();
                movements
            };
            movements.iter().for_each(|(item, m)| (&mut monkeys[*m as usize]).items.push(*item));
        }
    }
    let monkey_business = monkeys.iter().map(|m| m.inspections).sorted_unstable().rev().take(2).reduce(|a, b| a * b).unwrap();
    monkey_business
}

pub fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}