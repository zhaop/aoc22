use std::io::{self, BufRead};

pub fn main1() {
    let score: i32 = io::stdin().lock().lines().map(|x| match x.unwrap().as_str() {
        "A X" => 4, // 3draw 1R
        "A Y" => 8, // 6win  2P
        "A Z" => 3, // 0loss 3S
        "B X" => 1, // 0loss 1R
        "B Y" => 5, // 3draw 2P
        "B Z" => 9, // 6win  3S
        "C X" => 7, // 6win  1R
        "C Y" => 2, // 0loss 2P
        "C Z" => 6, // 3draw 3S
        &_ => todo!(),
    }).sum();
    println!("{}", score);
}

pub fn main2() {
    let score: i32 = io::stdin().lock().lines().map(|x| match x.unwrap().as_str() {
        "A X" => 3, // 0lose 3S
        "A Y" => 4, // 3draw 1R
        "A Z" => 8, // 6win  2P
        "B X" => 1, // 0lose 1R
        "B Y" => 5, // 3draw 2P
        "B Z" => 9, // 6win  3S
        "C X" => 2, // 0lose 2P
        "C Y" => 6, // 3draw 3S
        "C Z" => 7, // 6win  1R
        &_ => todo!(),
    }).sum();
    println!("{}", score);
}
