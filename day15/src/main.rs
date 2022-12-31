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

fn main() {
    let args = Args::parse();

    let file = File::open(&args.data_file).expect("Failed to open file");
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader
        .lines()
        .map(|line| line.expect("Failed to read line"))
        .collect();

    let discs = lines
        .iter()
        .enumerate()
        .map(|(index, line)| to_disc(index, line))
        .collect_vec();

    let mut t = 0;
    loop {
        if !discs.iter().any(|disc| (disc.0 + t) % disc.1 != 0) {
            println!("Part 1: {}", t);
            break;
        }
        let furthest = discs
            .iter()
            .max_by_key(|disc| disc.1 - ((disc.0 + t) % disc.1))
            .unwrap();

        t += furthest.1 - ((furthest.0 + t) % furthest.1);
    }
}

fn to_disc(index: usize, line: &str) -> (usize, usize) {
    let components = line.split_whitespace().collect_vec();
    (
        index
            + 1
            + components[11]
                .trim_end_matches(".")
                .parse::<usize>()
                .unwrap(),
        components[3].parse().unwrap(),
    )
}
