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
}

fn main() {
    let args = Args::parse();

    let file = File::open(&args.data_file).expect("Failed to open file");
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader
        .lines()
        .map(|line| line.expect("Failed to read line"))
        .collect();

    let tls_count = lines
        .iter()
        .filter(supports_tls)
        //.inspect(|addr| println!("{}", addr))
        .count();
    println!("Part 1: {}", tls_count);
}

fn supports_tls(addr: &&String) -> bool {
    let addr: Vec<char> = addr.chars().collect();
    let mut i = 0;
    let mut in_brackets = false;
    let mut found_pattern = false;
    while i < addr.len() - 3 {
        if addr[i] == '[' {
            in_brackets = true;
        } else if addr[i] == ']' {
            in_brackets = false;
        } else if addr[i] != addr[i + 1] && addr[i] == addr[i + 3] && addr[i + 1] == addr[i + 2] {
            if in_brackets {
                return false;
            }
            found_pattern = true;
        }

        i += 1;
    }

    found_pattern
}
