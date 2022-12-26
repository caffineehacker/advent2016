use clap::Parser;
use std::{
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

    let valid_count = lines
        .iter()
        .map(|line| {
            line.split_whitespace()
                .map(|v| v.parse().unwrap())
                .collect::<Vec<i32>>()
        })
        .filter(|sides| {
            let mut sides = sides.clone();
            sides.sort();
            sides[0] + sides[1] > sides[2]
        })
        .count();

    println!("Part 1: {}", valid_count);
}
