use std::io::{self, BufRead};

pub fn main1() {
    let result: u32 = io::stdin()
        .lock()
        .lines()
        .map(|ln| ln.unwrap())
        .map(
            |ln|
            ln.split(",")
            .map(
                |elf|
                elf
                    .split("-")
                    .map(|n| n.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>()
            ).collect::<Vec<_>>()
        )
        .map(
            |pair|
            if  (pair[0][0] <= pair[1][0] && pair[0][1] >= pair[1][1])
             || (pair[1][0] <= pair[0][0] && pair[1][1] >= pair[0][1])
            { 1 } else { 0 }
        )
        .sum();

    println!("{}", result);
}

pub fn main2() {
    let result: u32 = io::stdin()
        .lock()
        .lines()
        .map(|ln| ln.unwrap())
        .map(
            |ln|
            ln.split(",")
            .map(
                |elf|
                elf
                    .split("-")
                    .map(|n| n.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>()
            ).collect::<Vec<_>>()
        )
        .map(
            |pair|
            if  (pair[0][1] < pair[1][0]) || (pair[0][0] > pair[1][1])
            { 0 } else { 1 }
        )
        .sum();

    println!("{}", result);
}