use clap::Parser;
use std::{
    cmp::Ordering,
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

    let good_sectors: u32 = lines
        .iter()
        .filter_map(|line| {
            if is_valid(line) {
                Some(get_sector_id(line))
            } else {
                None
            }
        })
        .sum();

    println!("Part 1: {}", good_sectors);
}

fn is_valid(line: &str) -> bool {
    let mut char_counts: HashMap<char, u32> = HashMap::new();

    for c in line
        .chars()
        .filter(|c| *c != '-')
        .take_while(|c| ('a'..='z').contains(c))
    {
        if let Some(entry) = char_counts.get_mut(&c) {
            *entry += 1;
        } else {
            char_counts.insert(c, 1);
        }
    }

    let mut counts: Vec<(char, u32)> = char_counts.into_iter().collect();
    counts.sort_by(|(c1, count1), (c2, count2)| {
        // Highest count first
        if count2.cmp(count1) != Ordering::Equal {
            return count2.cmp(count1);
        }

        // Lowest char first
        return c1.cmp(c2);
    });

    let checksum = line.split_once("[").unwrap().1.trim_end_matches("]");
    for i in 0..checksum.len() {
        if checksum.chars().nth(i).unwrap() != counts[i].0 {
            return false;
        }
    }

    true
}

fn get_sector_id(line: &str) -> u32 {
    line.split_once("[")
        .unwrap()
        .0
        .split("-")
        .last()
        .unwrap()
        .parse()
        .unwrap()
}
