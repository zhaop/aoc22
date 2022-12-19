use std::{io::stdin, collections::HashSet, cmp::{Ordering, self}};

use itertools::Itertools;

fn parse_coords(s: &str) -> (i32, i32) {
    let split: (&str, &str) = s.split(", ").take(2).collect_tuple().unwrap();
    (split.0.strip_prefix("x=").unwrap().parse().unwrap(), split.1.strip_prefix("y=").unwrap().parse().unwrap())
}

fn compute_exclusion_ranges(sensors: &Vec<((i32, i32), (i32, i32))>, query_y: i32) -> Vec<(i32, i32)> {
    let mut exclusion_ranges_it =
    sensors
    .iter()
    .filter_map(|&((sx, sy), (bx, by))| {
        let dist = (sx - bx).abs() + (sy - by).abs();
        let ydist = (sy - query_y).abs();
        let half_range = dist - ydist;
        if half_range > 0 { Some((sx - half_range, sx + half_range + 1)) } else { None }
    })
    .sorted_by(|a, b|
        match a.0.cmp(&b.0) {
            Ordering::Equal => a.1.cmp(&b.1),
            other => other,
        }
    );
    let mut exclusion_ranges: Vec<(i32, i32)> = vec![exclusion_ranges_it.next().unwrap()];
    for (r0, r1) in exclusion_ranges_it {
        let (l0, l1) = exclusion_ranges.last().unwrap();
        if r0 <= *l1 {
            let (_, l1) = exclusion_ranges.last_mut().unwrap();
            *l1 = cmp::max(r1, *l1);
        } else {
            exclusion_ranges.push((*l0, *l1));
        }
    }
    exclusion_ranges
}

pub fn main() {
    let sensors: Vec<((i32, i32), (i32, i32))> =
        stdin()
        .lines()
        .map(|ln|
            ln.unwrap().strip_prefix("Sensor at ").unwrap().split(": closest beacon is at ").take(2).map(parse_coords).collect_tuple().unwrap()
        )
        .collect_vec();
    let query_y = if sensors.len() > 15 { 2000000 } else { 10 };
    let exclusion_ranges = compute_exclusion_ranges(&sensors, query_y);
    let query_row_exclusion_beacons: HashSet::<i32> = HashSet::from_iter(
        sensors
        .iter()
        .filter_map(|&(_, (bx, by))|
            if by == query_y && exclusion_ranges.iter().any(|&(r0, r1)| r0 <= bx && bx < r1 ) {
                Some(bx)
            } else {
                None
            }
        )
    );
    let exclusion_size =
        exclusion_ranges.iter().map(|&(r0, r1)| r1 - r0).sum::<i32>()
        - (query_row_exclusion_beacons.len() as i32);
    println!("{}", exclusion_size);

    let lim = query_y * 2;
    let location = (0..lim).find_map(|i| {
        let exclusion_ranges = compute_exclusion_ranges(&sensors, i);
        let maybe_j = exclusion_ranges.iter().find_map(|&(r0, r1)|
            // Return first
            if r0 > 0 {
                Some(r0 - 1)
            } else if r1 < lim - 1 {
                Some(r1)
            } else {
                None
            }
        );
        match maybe_j {
            Some(j) => Some((i, j)),
            None => None,
        }
    });

    match location {
        Some((i, j)) => {
            let tuning_frequency: i64 = (i as i64) + (j as i64) * 4000000i64;
            println!("{}", tuning_frequency)
        },
        None => println!("Not found"),
    };
}
