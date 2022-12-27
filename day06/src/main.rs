use clap::Parser;
use itertools::Itertools;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    data_file: String,
}

fn main() {
    let args = Args::parse();

    let file = File::open(&args.data_file).expect("Failed to open file");
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader
        .lines()
        .map(|line| line.expect("Failed to read line"))
        .collect();

    let mut part1 = "".to_string();
    let mut part2 = "".to_string();
    for i in 0..lines[0].len() {
        let counts: HashMap<char, usize> =
            lines.iter().counts_by(|line| line.chars().nth(i).unwrap());
        part1 += &counts.iter().max_by_key(|(_, c)| *c).unwrap().0.to_string();
        part2 += &counts.iter().min_by_key(|(_, c)| *c).unwrap().0.to_string();
    }

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
