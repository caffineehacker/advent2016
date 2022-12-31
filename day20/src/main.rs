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

    let ranges = lines
        .iter()
        .map(|line| {
            let components = line.split_once('-').unwrap();
            (
                components.0.parse::<u32>().unwrap(),
                components.1.parse::<u32>().unwrap(),
            )
        })
        .sorted()
        .fold(Vec::new(), |mut acc: Vec<(u32, u32)>, (start, end)| {
            if let Some(range) = acc.iter_mut().find(|r| r.0 <= start && r.1 >= start) {
                range.1 = range.1.max(end);
            } else {
                acc.push((start, end));
            }

            acc
        });

    if args.debug {
        println!("Ranges: {}", ranges.len());
        ranges.iter().for_each(|r| println!("{} - {}", r.0, r.1));
    }

    let first_allowed = if ranges.len() == 1 {
        if ranges[0].0 > 0 {
            0
        } else {
            ranges[0].1 + 1
        }
    } else {
        ranges[0].1 + 1
    };

    println!("Part 1: {}", first_allowed);

    let mut total_allowed = ranges[0].0;
    for i in 1..ranges.len() {
        total_allowed += ranges[i].0 - 1 - ranges[i - 1].1;
    }
    total_allowed += u32::MAX - ranges.last().unwrap().1;

    println!("Part 2: {}", total_allowed);
}
