use std::{cmp, io::stdin};

use itertools::{Itertools, MinMaxResult::MinMax};

fn drop_sand_until<F>(grid: &mut Vec<Vec<char>>, source: (usize, usize), condition: F) -> usize
where F: Fn(&Vec<Vec<char>>, (usize, usize)) -> bool {
    (0..).skip_while(|_| {
        let (mut i, mut j) = source;
        loop {
            // On each iteration, move sand by 1
            if condition(grid, (i, j)) {
                // Tripwire: sand is leaking
                return false;
            }

            if grid[i+1][j] == '.' {
                (i, j) = (i + 1, j);
            } else if grid[i+1][j-1] == '.' {
                (i, j) = (i + 1, j - 1);
            } else if grid[i+1][j+1] == '.' {
                (i, j) = (i + 1, j + 1);
            } else {
                grid[i][j] = 'o';
                return true;
            }
        }
    }).next().unwrap()
}

fn print_grid(grid: &Vec<Vec<char>>) {
    for row in grid {
        for char in row {
            print!("{}", char);
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
    let xbounds = match rocks.iter().flatten().map(|(x, _)| x).minmax() {
        // Include x = 500 in bounds + 150 margin on each side
        MinMax(&a, &b) => (cmp::min(a, 500) - 150, cmp::max(b, 500) + 150 + 1),
        _ => todo!(),
    };
    let ybounds = match rocks.iter().flatten().map(|(_, y)| y).minmax() {
        // Include y = 0 in bounds + 1 margin on each side
        MinMax(&a, &b) => (cmp::min(a, 0) - 1, cmp::max(b, 0) + 2),
        _ => todo!(),
    };
    let mut grid = (0..(ybounds.1 - ybounds.0)).map(|_| vec!['.'; (xbounds.1 - xbounds.0) as usize]).collect_vec();
    for rock in &rocks {
        for line in rock.windows(2) {
            if line[0].0 == line[1].0 {
                // Vertical
                let x = line[0].0;
                for y in cmp::min(line[0].1, line[1].1)..(cmp::max(line[0].1, line[1].1) + 1) {
                    let (i, j) = (y - ybounds.0, x - xbounds.0);
                    grid[i as usize][j as usize] = '#';
                }
            } else if line[0].1 == line[1].1 {
                // Horizontal
                let y = line[0].1;
                for x in cmp::min(line[0].0, line[1].0)..(cmp::max(line[0].0, line[1].0) + 1) {
                    let (i, j) = (y - ybounds.0, x - xbounds.0);
                    grid[i as usize][j as usize] = '#';
                }
            } else {
                panic!("Diagonal line {:?}", line);
            }
        }
    }

    let source = ((0 - ybounds.0) as usize, (500 - xbounds.0) as usize);
    let h = grid.len();
    let grains_dropped_until_leak = drop_sand_until(
        &mut grid,
        source,
        |_, (i, _)| i == h - 1,
    );

    print_grid(&grid);
    println!("{}", grains_dropped_until_leak);

    // Add floor, clear sand, resimulate
    grid.push(vec!['#'; grid[0].len()]);
    for row in grid.iter_mut() {
        for char in row {
            if *char == 'o' {
                *char = '.';
            }
        }
    }
    let grains_dropped_until_block = drop_sand_until(
        &mut grid,
        source,
        |grid, _| grid[source.0][source.1] != '.',
    );
    print_grid(&grid);
    println!("{}", grains_dropped_until_block);
}
