use std::io::stdin;
use itertools::Itertools;

fn distinct_n(n: usize) {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    match input
        .trim_end()
        .chars()
        .collect::<Vec<_>>()
        .windows(n)
        .enumerate()
        .find(
            |(_, w)|
            (*w).iter().unique().collect::<String>().len() == n
        )
    {
        Some((i, _)) => println!("{}", i + n),
        None => println!("Not found"),
    }
}

pub fn main1() {
    distinct_n(4);
}

pub fn main2() {
    distinct_n(14);
}
