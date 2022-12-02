use std::io::{self, BufRead};

pub fn main1() {
    let lines: Vec<String> = 
        io::stdin()
        .lock()
        .lines()
        .map(|x| x.unwrap())
        .collect();
    let max_cal: i32 = 
        lines
        .split(|ln| ln == "")
        .map(|elf|
            elf
            .iter()
            .map(|ln|
                ln.parse::<i32>().unwrap()
            ).sum()
        )
        .max()
        .unwrap();
    println!("{}", max_cal);
}

pub fn main2() {
    let lines: Vec<String> = 
        io::stdin()
        .lock()
        .lines()
        .map(|x| x.unwrap())
        .collect();
    let mut cals: Vec<i32> = 
        lines
        .split(|ln| ln == "")
        .map(|elf|
            elf
            .iter()
            .map(|ln|
                ln.parse::<i32>().unwrap()
            ).sum()
        )
        .collect();
    cals.sort_by(|a, b| b.cmp(a));
    println!("{}", &cals[..3].iter().sum::<i32>());
}
