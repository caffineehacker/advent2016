use clap::Parser;
use itertools::Itertools;
use std::{
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

struct Node {
    position: (i32, i32),
    name: String,
    size: u32,
    used: u32,
}

impl From<&str> for Node {
    fn from(value: &str) -> Self {
        let components = value.split_whitespace().collect_vec();

        let (x, y) = components[0]
            .split_once("-")
            .unwrap()
            .1
            .split_once("-")
            .unwrap();
        let position = (
            x.trim_start_matches("x").parse().unwrap(),
            y.trim_start_matches("y").parse().unwrap(),
        );

        Self {
            position,
            name: components[0].to_string(),
            size: components[1].trim_end_matches("T").parse().unwrap(),
            used: components[2].trim_end_matches("T").parse().unwrap(),
        }
    }
}

fn main() {
    let args = Args::parse();

    let file = File::open(&args.data_file).expect("Failed to open file");
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader
        .lines()
        .map(|line| line.expect("Failed to read line"))
        .collect();

    let nodes = lines
        .iter()
        .skip(2)
        .map(|line| Node::from(line.as_str()))
        .collect_vec();

    let mut valid_pairs = 0;
    for i in 0..nodes.len() {
        let node_a = &nodes[i];
        if node_a.used == 0 {
            continue;
        }

        for j in 0..nodes.len() {
            if i == j {
                continue;
            }

            let node_b = &nodes[j];

            if node_b.used + node_a.used <= node_b.size {
                valid_pairs += 1;
            }
        }
    }

    println!("Part 1: {}", valid_pairs);

    // Part 2, extract data from top-right
    let goal_position = nodes
        .iter()
        .map(|node| node.position)
        .filter(|position| position.1 == 0)
        .max_by_key(|position| position.1)
        .unwrap();

    // This is effectively a sliding block puzzle
    // This is honestly easier to answer just by printing the puzzle out
    for y in 0..nodes
        .iter()
        .max_by_key(|node| node.position.1)
        .unwrap()
        .position
        .1
    {
        for x in 0..goal_position.0 {
            let node = nodes.iter().find(|node| node.position == (x, y)).unwrap();
            if node.size > 100 {
                print!("X");
            } else if node.used == 0 {
                print!("_");
            } else {
                print!(".");
            }
        }
        println!();
    }
}
