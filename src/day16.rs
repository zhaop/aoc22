extern crate regex;

use itertools::Itertools;
use regex::Regex;

use std::{io::stdin, collections::HashMap, cmp};

#[derive(Debug)]
struct Valve {
    name: String,
    flow: u32,
    tunnels: HashMap<String, u32>,
}

fn _print_digraph(valves: &HashMap<String, Valve>) {
    println!("digraph {{");
    for valve in valves.values() {
        println!("  {} [label=\"{} ({})\"]", valve.name, valve.name, valve.flow);
        for (name2, len) in &valve.tunnels {
            println!("  {} -> {} [label=\"{}\"]", valve.name, name2, len);
        }
    }
    println!("}}");
}

fn _print_graph(valves: &HashMap<String, Valve>) {
    println!("graph {{");
    for valve in valves.values() {
        println!("  {} [label=\"{} ({})\"]", valve.name, valve.name, valve.flow);
        for (name2, len) in &valve.tunnels {
            if valve.name < *name2 {
                println!("  {} -- {} [label=\"{}\"]", valve.name, name2, len);
            }
        }
    }
    println!("}}");
}

pub fn main() {
    let line_pattern = Regex::new(
        r"^Valve ([A-Z]{2}) has flow rate=([0-9]+); tunnels? leads? to valves? ([A-Z, ]+)$"
    ).unwrap();
    let mut valves: HashMap<String, Valve> = stdin().lines().map(|ln| {
        let matches =
            line_pattern
            .captures(ln.unwrap().as_str())
            .iter()
            .next()
            .unwrap()
            .iter()
            .skip(1)
            .map(|capture| String::from(capture.unwrap().as_str()))
            .collect_vec();
        (String::from(&matches[0]), Valve {
            name: String::from(&matches[0]),
            flow: matches[1].parse::<u32>().unwrap(),
            tunnels: matches[2].split(", ").map(|s| (s.into(), 1)).collect(),
        })
    }).collect();

    // Collapse empty valves
    loop {
        let next_empty = valves.iter().find_map(|(k, v)|
            if k != "AA" && v.flow == 0 { Some(k) } else { None }
        );
        if next_empty.is_none() {
            break;
        }
        let name = String::from(next_empty.unwrap());
        let valve = valves.remove(&name).unwrap();
        for (neighbor1, len1) in &valve.tunnels {
            valves.get_mut(neighbor1).unwrap().tunnels.remove(&name);
            for (neighbor2, len2) in &valve.tunnels {
                if neighbor1 == neighbor2 {
                    continue;
                }
                let newlen = len1 + len2;
                valves.get_mut(neighbor1).unwrap().tunnels.insert(neighbor2.clone(), newlen);
                valves.get_mut(neighbor2).unwrap().tunnels.insert(neighbor1.clone(), newlen);
            }
        }
    }

    // Get mapping from name to ID
    let valve_ids: HashMap<String, usize> =
        valves.keys()
        .sorted()
        .enumerate()
        .map(|(i, name)| (name.clone(), i))
        .collect();
    let valves_by_id: HashMap<usize, &Valve> =
        valve_ids.iter()
        .map(|(name, i)| (*i, valves.get(name).unwrap()))
        .collect();

    // Compute shortest paths matrix
    let mut dists: Vec<Vec<u32>> = Vec::from_iter(valves.iter().map(|_| vec![i32::MAX as u32; valves.len()]));
    for valve1 in valves.values() {
        let &id1 = valve_ids.get(&valve1.name).unwrap();
        dists[id1][id1] = 0;
        for (valve2_name, dist) in &valve1.tunnels {
            let &id2 = valve_ids.get(valve2_name).unwrap();
            dists[id1][id2] = *dist;
        }
    }
    for &k in valve_ids.values() {
        for &i in valve_ids.values() {
            for &j in valve_ids.values() {
                if dists[i][j] > dists[i][k] + dists[k][j] {
                    dists[i][j] = dists[i][k] + dists[k][j];
                }
            }
        }
    }

    // Backtracking search
    let mut solution: Vec<usize> = vec![*(valve_ids.get(&String::from("AA")).unwrap())];
    let (tmax, mut time, mut score, mut best_score): (u32, u32, u32, u32) = (30, 0, 0, 0);
    loop {
        if solution.is_empty() {
            break;  // Done
        }
        let &prev_id = solution.last().unwrap();
        let next_to_add = (0..valves.len()).find(|&id|
            // Not already in solution & would not run out of time
            (time + dists[prev_id][id] + 1) <= tmax && solution.iter().all(|&j| id != j)
        );
        if let Some(next_id) = next_to_add {
            solution.push(next_id);
            time += dists[prev_id][next_id] + 1;
            score += (tmax - time) * valves_by_id[&next_id].flow;
            best_score = cmp::max(best_score, score);
            // println!("+{:?}, time: {}, score: {}, best_score: {}", solution, time, score, best_score);
            continue;
        }
        loop {
            let popped_id = solution.pop().unwrap();
            if solution.is_empty() {
                break;  // Done
            }
            let &prev_id = solution.last().unwrap();
            score -= (tmax - time) * valves_by_id[&popped_id].flow;
            time -= dists[prev_id][popped_id] + 1;
            let next_to_try = ((popped_id + 1)..valves.len()).find(|&id|
                (time + dists[prev_id][id] + 1) <= tmax && solution.iter().all(|&j| id != j)
            );
            if let Some(next_id) = next_to_try {
                solution.push(next_id);
                time += dists[prev_id][next_id] + 1;
                score += (tmax - time) * valves_by_id[&next_id].flow;
                best_score = cmp::max(best_score, score);
                // println!(">{:?}, time: {}, score: {}, best_score: {}", solution, time, score, best_score);
                break;
            }
        }
    }
    println!("{}", best_score);

    // // Graph of all valve<>valve distances
    // let dists_graph =
    //     valve_ids.iter()
    //     .map(|(name, &id)| (valves.get(name).unwrap(), id))
    //     .map(|(valve, id)| (
    //         valve.name.clone(),
    //         Valve {
    //             name: valve.name.clone(),
    //             flow: valve.flow.clone(),
    //             tunnels:
    //                 dists[id].iter()
    //                 .enumerate()
    //                 .map(|(id2, &dist)| (valves_by_id.get(&id2).unwrap().name.clone(), dist))
    //                 .collect()
    //         }
    //     ))
    //     .collect();
    // _print_graph(&dists_graph);

    // println!("{:?}", valve_ids);
    // for row in dists {
    //     println!("{:?}", row);
    // }
}