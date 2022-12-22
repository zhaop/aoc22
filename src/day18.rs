use std::{io::stdin, collections::HashSet};

use itertools::Itertools;

pub fn main() {
    let voxels: HashSet<(i32, i32, i32)> =
        stdin().lines()
        .map(|ln|
            ln.unwrap()
            .split(",")
            .map(|n| n.parse::<i32>().unwrap())
            .collect_tuple()
            .unwrap()
        )
        .collect();
    let sides = vec![
        (-1, 0, 0),
        (1, 0, 0),
        (0, -1, 0),
        (0, 1, 0),
        (0, 0, -1),
        (0, 0, 1),
    ];
    let surface_area =
        voxels.iter()
        .map(|&(i, j, k)|
            sides.iter()
            .filter(|&&(di, dj, dk)|
                !voxels.contains(&(i + di, j + dj, k + dk))
            )
            .count()
        ).sum::<usize>();
    println!("{:?}", surface_area);
}
