use std::io::{stdin, Read};

use itertools::Itertools;

fn top_of(grid: &Vec<i32>) -> usize {
    // Return index of last row containing rocks
    let edge_mask = 0b100000001;
    grid.iter().enumerate().rev().find_map(|(i, &row)| if row & !edge_mask != 0 { Some(i) } else { None }).unwrap()
}

fn moved_rock(rock: &Vec<i32>, jet: char) -> Vec<i32> {
    rock.iter().map(|row| match jet {
        '<' => row << 1,
        '>' => row >> 1,
        _ => todo!(),
    }).collect_vec()
}

fn collides(grid: &Vec<i32>, y: usize, rock: &Vec<i32>) -> bool {
    assert!(grid.len() >= y + rock.len());
    grid[y..(y+rock.len())].iter().zip(rock.iter()).any(|(grid_row, rock_row)| grid_row & rock_row != 0)
}

fn _print_grid(grid: &Vec<i32>) {
    for row in grid.iter().rev() {
        let mut mask = 0b100000000;
        while mask != 0 {
            if mask & 0b100000001 != 0 {
                print!("|");
            } else if mask & row != 0 {
                print!("#");
            } else {
                print!(".");
            }
            mask >>= 1;
        }
        println!();
    }
}

pub fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let rocks = vec![
        vec![0b00111100],
        vec![0b00010000,
             0b00111000,
             0b00010000],
        vec![0b00111000,
             0b00001000,
             0b00001000],
        vec![0b00100000,
             0b00100000,
             0b00100000,
             0b00100000],
        vec![0b00110000,
             0b00110000],
    ];
    let mut grid = vec![
        0b111111111,
    ];

    let mut jets = input.strip_suffix("\n").unwrap().chars().cycle();
    rocks.iter().cycle().take(2022).for_each(|rock_shape| {
        // Spawn rock & extend grid
        let mut rock = rock_shape.clone();

        // y is smallest index in grid where rock is present
        let mut y = top_of(&grid) + 4;

        for _ in grid.len()..(y + rock.len()) {
            // Extend grid upwards if necessary
            grid.push(0b100000001);
        }
        let mut moved;
        loop {
            let jet = jets.next().unwrap();
            moved = moved_rock(&rock, jet);
            if !collides(&grid, y, &moved) {
                // Blow sideways
                rock = moved;
            }
            if collides(&grid, y - 1, &rock) {
                // Settle rock
                for (i, row) in rock.iter().enumerate() {
                    grid[y + i] |= row;
                }
                // _print_grid(&grid);
                // println!();
                break;
            } else {
                y -= 1;
            }
        }
    });

    let rock_pile_height = top_of(&grid);
    println!("{}", rock_pile_height);
}
