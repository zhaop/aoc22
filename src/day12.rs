use std::{io::stdin, collections::HashSet};

use itertools::Itertools;

fn _print_grid(grid: &Vec<Vec<i32>>, cell_str: fn(i32) -> char) {
    for row in grid {
        for &cell in row {
            print!("{}", cell_str(cell));
        }
        println!();
    }
}

fn _height_str(val: i32) -> char {
    match val {
        0..=25 => (b'a' + val as u8) as char,
        _ => '?',
    }
}

fn _dist_str(val: i32) -> char {
    let v = val % 62;
    match v {
        _ if v < 0 => '-',
        0..=9 => (b'0' + v as u8) as char,
        10..=35 => (b'a' + (v - 10) as u8) as char,
        36..=61 => (b'A' + (v - 36) as u8) as char,
        _ => '?',
    }
}

pub fn main() {
    let mut heads: HashSet<(usize, usize)> = HashSet::new();
    let mut start: (usize, usize) = (usize::MAX, usize::MAX);

    let heights =
        stdin()
        .lines()
        .enumerate()
        .map(|(i, ln)|
            ln.unwrap().chars().enumerate().map(|(j, c)| match c {
                'S' => {start = (i, j); 0},
                'E' => {heads.insert((i, j)); 25},
                c => (c as i32) - ('a' as i32),
            }
        ).collect_vec()).collect_vec();

    let (h, w) = (heights.len(), heights[0].len());
    // println!("start: {:?}", start);
    // println!("end: {:?}", heads);
    // println!();

    let mut dists: Vec<Vec<i32>> = heights.iter().map(|row| vec![-1; row.len()]).collect_vec();
    let mut dist = 0;
    while dists[start.0][start.1] == -1 {
        let mut next_heads: HashSet<(usize, usize)> = HashSet::new();
        for &(i, j) in &heads {
            dists[i][j] = dist;
            let height = heights[i][j];
            if i > 0 && dists[i - 1][j] < 0 && height <= heights[i - 1][j] + 1 {
                next_heads.insert((i - 1, j));
            }
            if j > 0 && dists[i][j - 1] < 0 && height <= heights[i][j - 1] + 1 {
                next_heads.insert((i, j - 1));
            }
            if i < h - 1 && dists[i + 1][j] < 0 && height <= heights[i + 1][j] + 1 {
                next_heads.insert((i + 1, j));
            }
            if j < w - 1 && dists[i][j + 1] < 0 && height <= heights[i][j + 1] + 1 {
                next_heads.insert((i, j + 1));
            }
        }
        heads = next_heads;
        dist += 1;
    }

    // _print_grid(&heights, _height_str);
    // println!();
    // _print_grid(&dists, _dist_str);
    // println!();

    println!("{}", dists[start.0][start.1]);

    let min_hike_dist = heights.iter().zip(dists.iter())
        .map(|(hrow, drow)|
            hrow.iter().zip(drow.iter())
                .filter(|(&h, &d)| h == 0 && d >= 0)
                .map(|(_, &d)| d)
                .min()
        )
        .filter(|result| result.is_some())
        .map(|result| result.unwrap())
        .min().unwrap();
    println!("{}", min_hike_dist);
}
