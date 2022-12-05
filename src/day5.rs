use std::io::{self, BufRead};
use regex::Regex;

pub fn main1() {
    let lines = io::stdin().lock().lines().map(|ln| ln.unwrap()).collect::<Vec<_>>();
    let stack_lines = &lines[..8];
    let mut stacks =
        (1..10)
            .map(
                |col|
                stack_lines
                    .iter()
                    .map(|row| row.chars().nth(4*col-3).unwrap())
                    .filter(|c| *c != ' ')
                    .rev()
                    .collect::<String>()
            )
            .collect::<Vec<_>>();
    let move_lines = &lines[10..];
    let re = Regex::new(r"move ([0-9]+) from ([0-9]+) to ([0-9]+)").unwrap();
    move_lines
    .iter()
    .map(
        |ln|
        re
        .captures_iter(ln)
        .next()
        .unwrap()
    )
    .map(
        |captures|
        vec![captures.get(1), captures.get(2), captures.get(3)]
        .iter().map(|n| (*n).unwrap().as_str().parse::<u32>().unwrap()).collect::<Vec<_>>()
    )
    .for_each(|m| {
        let (num, col0, col1) = (m[0], (m[1] - 1) as usize, (m[2] - 1) as usize);
        for _i in 0..num {
            let c = stacks[col0].pop().unwrap();
            stacks[col1].push(c);
        }
    });
    for stack in stacks {
        print!("{}", stack.chars().last().unwrap());
    }
    println!("");
}

pub fn main2() {
    let lines = io::stdin().lock().lines().map(|ln| ln.unwrap()).collect::<Vec<_>>();
    let stack_lines = &lines[..8];
    let mut stacks =
        (1..10)
            .map(
                |col|
                stack_lines
                    .iter()
                    .map(|row| row.chars().nth(4*col-3).unwrap())
                    .filter(|c| *c != ' ')
                    .rev()
                    .collect::<String>()
            )
            .collect::<Vec<_>>();
    let move_lines = &lines[10..];
    let re = Regex::new(r"move ([0-9]+) from ([0-9]+) to ([0-9]+)").unwrap();
    move_lines
    .iter()
    .map(
        |ln|
        re
        .captures_iter(ln)
        .next()
        .unwrap()
    )
    .map(
        |captures|
        vec![captures.get(1), captures.get(2), captures.get(3)]
        .iter().map(|n| (*n).unwrap().as_str().parse::<u32>().unwrap()).collect::<Vec<_>>()
    )
    .for_each(|m| {
        let (num, col0, col1) = (m[0], (m[1] - 1) as usize, (m[2] - 1) as usize);
        let moved: String = (0..num).map(|_i| stacks[col0].pop().unwrap()).collect();
        stacks[col1].push_str(moved.chars().rev().collect::<String>().as_str());
    });
    for stack in stacks {
        print!("{}", stack.chars().last().unwrap());
    }
    println!("");
}
