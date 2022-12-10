use std::{io::stdin, collections::HashSet};

fn pull((hx, hy): (i32, i32), (tx, ty): (i32, i32)) -> (i32, i32) {
    let (dx, dy): (i32, i32) = (hx - tx, hy - ty);
    if std::cmp::max(dx.abs(), dy.abs()) > 1 {
        (tx + dx.signum(), ty + dy.signum())
    } else {
        (tx, ty)
    }
}

fn simulate(cmds: &Vec<(String, i32)>, tail_len: usize) -> usize {
    let mut xys: Vec<(i32, i32)> = vec![(0, 0)];
    xys.extend((0..tail_len).map(|_| (0, 0)));

    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    cmds.iter().for_each(|(dir, n)| {
        // println!("{} {}", dir, n);
        let (mx, my) = match dir.as_str() {
            "R" => (1, 0),
            "L" => (-1, 0),
            "U" => (0, 1),
            "D" => (0, -1),
            &_ => todo!(),
        };
        (0..*n).for_each(|_| {
            // println!("{} 1", dir);
            // Move head
            (xys[0].0, xys[0].1) = (xys[0].0 + mx, xys[0].1 + my);

            // Move the rest
            xys = xys.iter().scan(xys[0], |(sx, sy), &(x, y)| {
                let (nx, ny) = pull((*sx, *sy), (x, y));
                // println!("{} {} <- {} {} = {} {}", sx, sy, x, y, nx, ny);
                (*sx, *sy) = (nx, ny);
                Some((nx, ny))
            }).collect::<Vec<_>>();
            visited.insert(*xys.last().unwrap());
        });
    });

    return visited.len();
}

pub fn main() {
    let cmds = stdin().lines().map(|ln| {
        let ln_str = ln.unwrap();
        let split = ln_str.split(" ").collect::<Vec<_>>();
        (String::from(split[0]), split[1].parse::<i32>().unwrap())
    }).collect::<Vec<_>>();
    println!("{}", simulate(&cmds, 1));
    println!("{}", simulate(&cmds, 9));
}
