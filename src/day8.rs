use std::cmp::{min, max};
use std::io::{stdin, BufRead};

use itertools::izip;
use take_until::TakeUntilExt;

fn flip_x(vals: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    vals
    .iter()
    .map(|row| row.iter().rev().cloned().collect::<Vec<_>>())
    .collect::<Vec<_>>()
}

fn transpose(vals: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    // vals is not empty; all rows have same length
    let w = vals[0].len();
    (0..w).map(|j|
        vals.iter().map(|row| row[j]).collect::<Vec<i32>>()
    )
    .collect::<Vec<_>>()
}

fn cumul_max_left(vals: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    vals
    .iter()
    .map(|row|
        row.iter().scan(-1, |x, &v| {
            let prev = *x;
            *x = max(*x, v);
            Some(prev)
        })
        .collect::<Vec<_>>()
    )
    .collect::<Vec<_>>()
}

fn viewing_distance_right(heights: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    heights
    .iter()
    .map(|row| {
        row
        .iter()
        .enumerate()
        .map(|(j, h)|
            row[j..].iter().skip(1).take_until(|h2| *h2 >= h).count() as i32
        )
        .collect::<Vec<_>>()
    })
    .collect::<Vec<_>>()
}

pub fn main() {
    let heights =
        stdin()
        .lock()
        .lines()
        .map(|ln|
            ln
            .unwrap()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as i32)
            .collect::<Vec<i32>>()
        )
        .collect::<Vec<_>>();

    let max_x0 = cumul_max_left(&heights);
    let max_x1 = flip_x(&cumul_max_left(&(flip_x(&heights))));
    let max_y0 = transpose(&cumul_max_left(&transpose(&heights)));
    let max_y1 = transpose(&flip_x(&cumul_max_left(&flip_x(&transpose(&heights)))));
    // let visibilities =
    //     izip!(&heights, &max_x0, &max_x1, &max_y0, &max_y1)
    //     .map(|(rh, rx0, rx1, ry0, ry1)|
    //         izip!(rh, rx0, rx1, ry0, ry1)
    //         .map(|(h, x0, x1, y0, y1)|
    //             if h > min(x0, min(x1, min(y0, y1))) { 1 } else { 0 }
    //         )
    //         .collect::<Vec<_>>()
    //     )
    //     .collect::<Vec<_>>();
    let num_visible =
        izip!(&heights, &max_x0, &max_x1, &max_y0, &max_y1)
        .map(|(rh, rx0, rx1, ry0, ry1)|
            izip!(rh, rx0, rx1, ry0, ry1)
            .map(|(h, x0, x1, y0, y1)|
            if h > min(x0, min(x1, min(y0, y1))) { 1 } else { 0 }
            )
            .sum::<i32>()
        )
        .sum::<i32>();

    // for row in heights.iter() { println!("{:?}", row); } println!("");

    // for row in max_x0.iter() { println!("{:?}", row); } println!("");
    // for row in max_x1.iter() { println!("{:?}", row); } println!("");
    // for row in max_y0.iter() { println!("{:?}", row); } println!("");
    // for row in max_y1.iter() { println!("{:?}", row); } println!("");
    // for row in visibilities.iter() { println!("{:?}", row); } println!("");
    println!("{}", num_visible);

    let dist_x0 = viewing_distance_right(&heights);
    let dist_x1 = flip_x(&viewing_distance_right(&(flip_x(&heights))));
    let dist_y0 = transpose(&viewing_distance_right(&transpose(&heights)));
    let dist_y1 = transpose(&flip_x(&viewing_distance_right(&flip_x(&transpose(&heights)))));

    // for row in dist_x0.iter() { println!("{:?}", row); } println!("");
    // for row in dist_x1.iter() { println!("{:?}", row); } println!("");
    // for row in dist_y0.iter() { println!("{:?}", row); } println!("");
    // for row in dist_y1.iter() { println!("{:?}", row); } println!("");

    // let scores =
    //     izip!(&dist_x0, &dist_x1, &dist_y0, &dist_y1)
    //     .map(|(rx0, rx1, ry0, ry1)|
    //         izip!(rx0, rx1, ry0, ry1)
    //         .map(|(x0, x1, y0, y1)| x0 * x1 * y0 * y1)
    //         .collect::<Vec<_>>()
    //     )
    //     .collect::<Vec<_>>();
    // for row in scores.iter() { println!("{:?}", row); } println!("");

    let max_score: i32 =
        izip!(&dist_x0, &dist_x1, &dist_y0, &dist_y1)
        .map(|(rx0, rx1, ry0, ry1)|
            izip!(rx0, rx1, ry0, ry1)
            .map(|(x0, x1, y0, y1)| x0 * x1 * y0 * y1)
            .max().unwrap()
        )
        .max().unwrap();
    println!("{}", max_score);
}
