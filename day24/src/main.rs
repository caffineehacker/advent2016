use clap::Parser;
use itertools::Itertools;
use std::{
    collections::{HashMap, VecDeque},
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    data_file: String,
    #[arg(long)]
    debug: bool,
}

fn main() {
    let args = Args::parse();

    let file = File::open(&args.data_file).expect("Failed to open file");
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader
        .lines()
        .map(|line| line.expect("Failed to read line"))
        .collect();

    // True means open space, false means a wall
    let map = lines
        .iter()
        .map(|line| line.chars().map(|c| c != '#').collect_vec())
        .collect_vec();
    let required_points = lines
        .iter()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c != '#' && *c != '.')
                .map(|(x, c)| (x, y, c.to_string().parse::<u8>().unwrap()))
                .collect_vec()
        })
        .flatten()
        .sorted_by_key(|point| point.2)
        .collect_vec();

    // We have to start at 0, but otherwise can go any order.
    // The fastest way to do this will be to eliminate the maze and have a graph of the points and weighted edges.
    let mut edges = HashMap::new();

    for i in 0..required_points.len() {
        for j in (i + 1)..required_points.len() {
            // Now lets calculate edges
            let distance = distance_between(required_points[i], required_points[j], &map);
            edges.insert((i, j), distance);
            edges.insert((j, i), distance);

            if args.debug {
                println!("{} <-> {} = {}", i, j, distance);
            }
        }
    }

    // Now find the shortest distance
    let shortest_path_length = (1..(required_points.len()))
        .permutations(required_points.len() - 1)
        .map(|permutation| {
            let mut last_node = 0;
            let mut cost = 0;
            for i in 0..permutation.len() {
                cost += edges.get(&(last_node, permutation[i])).unwrap();
                last_node = permutation[i];
            }

            if args.debug {
                print!("0");
                permutation.iter().for_each(|n| print!(" -> {}", n));
                println!();
            }

            cost
        })
        .min()
        .unwrap();

    println!("Part 1: {}", shortest_path_length);
}

fn distance_between(
    point_a: (usize, usize, u8),
    point_b: (usize, usize, u8),
    map: &Vec<Vec<bool>>,
) -> usize {
    // Simple BFS - we could / should optimize to pick the most likely candidate position, but the map isn't that big
    let mut states = VecDeque::new();
    states.push_back((point_a.0, point_a.1, 0));
    let mut seen_states = Vec::new();

    while !states.is_empty() {
        let state = states.pop_front().unwrap();
        if seen_states.contains(&(state.0, state.1)) {
            continue;
        }

        seen_states.push((state.0, state.1));

        if state.0 == point_b.0 && state.1 == point_b.1 {
            return state.2;
        }

        if state.1 > 0 && map[state.1 - 1][state.0] {
            states.push_back((state.0, state.1 - 1, state.2 + 1));
        }
        if state.1 < map.len() - 1 && map[state.1 + 1][state.0] {
            states.push_back((state.0, state.1 + 1, state.2 + 1));
        }
        if state.0 > 0 && map[state.1][state.0 - 1] {
            states.push_back((state.0 - 1, state.1, state.2 + 1));
        }
        if state.0 < map[0].len() - 1 && map[state.1][state.0 + 1] {
            states.push_back((state.0 + 1, state.1, state.2 + 1));
        }
    }

    0
}
