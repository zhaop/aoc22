use std::io::stdin;

pub fn main() {
    let (mut cycle, mut reg): (i32, i32) = (1, 1);

    let mut reg_vals: Vec<i32> = vec![0];
    let mut str_vals: Vec<i32> = vec![0];

    for ln in stdin().lines().map(|ln| ln.unwrap()) {
        if ln == "noop" {
            reg_vals.push(reg); str_vals.push(reg*cycle);
            cycle += 1;
        } else {
            let x = ln.split_whitespace().skip(1).next().unwrap().parse::<i32>().unwrap();
            reg_vals.push(reg); str_vals.push(reg*cycle);
            cycle += 1;
            reg_vals.push(reg); str_vals.push(reg*cycle);
            cycle += 1;
            reg += x;
        }
    }
    let sum = vec![20, 60, 100, 140, 180, 220].iter().map(|c| str_vals[*c as usize]).sum::<i32>();
    println!("{}", sum);

    for i in 1..241 {
        let col = (i - 1) % 40;  // 0 <= col <= 39
        let reg_val = reg_vals[i];
        if (col as i32 - reg_val).abs() <= 1 {
            print!("##");
        } else {
            print!("  ");
        }
        if col == 39 {
            println!("");
        }
    }
}
