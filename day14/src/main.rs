use std::collections::{HashSet, VecDeque};

use clap::Parser;
use indicatif::ProgressBar;
use itertools::Itertools;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    salt: String,
    #[arg(long)]
    debug: bool,
    #[arg(long)]
    part2: bool,
}

fn main() {
    let args = Args::parse();

    let progress = ProgressBar::new_spinner();

    let mut i = 0;
    let mut seen_triples: VecDeque<(i32, char)> = VecDeque::new();
    let mut key_indexes = HashSet::new();
    'hashing: loop {
        progress.tick();
        progress.set_message(format!("{}", i));
        let mut digest = md5::compute(format!("{}{}", args.salt, i.to_string()));
        let mut hashed = format!("{:x}", digest);

        if args.part2 {
            for _ in 0..2016 {
                digest = md5::compute(hashed);
                hashed = format!("{:x}", digest);
            }
        }

        let (triple, quintuples) = get_repeats(&hashed);

        loop {
            if !seen_triples.is_empty() && seen_triples.front().unwrap().0 < i - 1000 {
                seen_triples.pop_front();
            } else {
                break;
            }
        }

        for quintuple in quintuples {
            for trip in seen_triples.iter().filter(|trip| trip.1 == quintuple) {
                if args.debug {
                    println!("Found key: {}", trip.0);
                }
                key_indexes.insert(trip.0);
                if key_indexes.len() >= 64
                    && *key_indexes.iter().sorted().nth(63).unwrap() < i - 1000
                {
                    println!("Part 1: {}", key_indexes.iter().sorted().nth(63).unwrap());
                    break 'hashing;
                }
            }
        }

        if let Some(triple) = triple {
            //println!("{}: {}", i, hashed);
            seen_triples.push_back((i, triple));
        }

        i += 1;
    }
}

fn get_repeats(digest: &str) -> (Option<char>, Vec<char>) {
    let mut last_char = 'Z';
    let mut last_char_count = 0;
    let mut triple = None;
    let mut quintuples = Vec::new();
    for c in digest.chars() {
        if c == last_char {
            last_char_count += 1;
            if last_char_count == 3 && triple.is_none() {
                triple = Some(c);
            }
            if last_char_count == 5 {
                quintuples.push(last_char);
            }
        } else {
            last_char = c;
            last_char_count = 1;
        }
    }

    quintuples.sort();
    quintuples.dedup();

    (triple, quintuples)
}
