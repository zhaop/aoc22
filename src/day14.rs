use std::{cmp, io::stdin, collections::HashMap};

use itertools::{Itertools, MinMaxResult::MinMax};

fn drop_sand_until<F>(grid: &mut HashMap<(i32, i32), char>, source: (i32, i32), condition: F) -> usize
where F: Fn(&HashMap<(i32, i32), char>, (i32, i32)) -> bool {
    (0..).skip_while(|_| {
        let (mut i, mut j) = source;
        loop {
            // On each iteration, move sand by 1
            if condition(grid, (i, j)) {
                // Tripwire: sand is leaking
                return false;
            }

            if !grid.contains_key(&(i+1, j)) {
                (i, j) = (i + 1, j);
            } else if !grid.contains_key(&(i+1, j-1)) {
                (i, j) = (i + 1, j - 1);
            } else if !grid.contains_key(&(i+1, j+1)) {
                (i, j) = (i + 1, j + 1);
            } else {
                grid.insert((i, j), 'o');
                return true;
            }
        }
    }).next().unwrap()
}

fn get_bounds(grid: &HashMap<(i32, i32), char>) -> ((i32, i32), (i32, i32)) {
    let ibounds = match grid.keys().map(|(i, _)| i).minmax() {
        MinMax(&a, &b) => (a, b),
        _ => todo!(),
    };
    let jbounds = match grid.keys().map(|(_, j)| j).minmax() {
        MinMax(&a, &b) => (a, b),
        _ => todo!(),
    };
    (ibounds, jbounds)
}

fn _print_grid(grid: &HashMap<(i32, i32), char>) {
    let (ibounds, jbounds) = get_bounds(grid);
    for i in (ibounds.0)..(ibounds.1+1) {
        for j in (jbounds.0)..(jbounds.1+1) {
            match grid.get(&(i, j)) {
                Some(c) => print!("{}", c),
                None => print!("."),
            }
        }
        println!();
    }
}

pub fn main() {
    let rocks: Vec<Vec<(i32, i32)>> =
        stdin()
        .lines()
        .map(|ln|
            ln.unwrap().split(" -> ").map(|coords|
                coords.split(",").map(|coord| coord.parse::<i32>().unwrap()).collect_tuple().unwrap()
            )
            .collect_vec()
        )
        .collect_vec();
    let mut grid: HashMap<(i32, i32), char> = HashMap::new();
    for rock in &rocks {
        for line in rock.windows(2) {
            if line[0].0 == line[1].0 {
                // Vertical
                let x = line[0].0;
                for y in cmp::min(line[0].1, line[1].1)..(cmp::max(line[0].1, line[1].1) + 1) {
                    let (i, j) = (y, x);
                    grid.insert((i, j), '#');
                }
            } else if line[0].1 == line[1].1 {
                // Horizontal
                let y = line[0].1;
                for x in cmp::min(line[0].0, line[1].0)..(cmp::max(line[0].0, line[1].0) + 1) {
                    let (i, j) = (y, x);
                    grid.insert((i, j), '#');
                }
            } else {
                panic!("Diagonal line {:?}", line);
            }
        }
    }

    let source = (0, 500);
    let h = grid.len();
    let grains_dropped_until_leak = drop_sand_until(
        &mut grid,
        source,
        |_, (i, _)| i == h as i32 - 1,
    );

    // _print_grid(&grid);
    println!("{}", grains_dropped_until_leak);

    // Add floor, clear sand, resimulate
    let (ibounds, jbounds) = get_bounds(&grid);
    for j in (jbounds.0 - 500)..(jbounds.1 + 500) {
        let i = ibounds.1 + 2;
        grid.insert((i, j), '#');
    }
    let sand_locations = grid.iter().filter_map(|(&key, &value)| if value == 'o' { Some(key) } else { None }).collect_vec();
    sand_locations.iter().for_each(|loc| { grid.remove(&loc); });

    let grains_dropped_until_block = drop_sand_until(
        &mut grid,
        source,
        |grid, _| grid.contains_key(&source),
    );
    // _print_grid(&grid);
    println!("{}", grains_dropped_until_block);
}
