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

    let compressed: Vec<char> = lines.first().unwrap().chars().collect();

    let part1_length = get_decompressed_size(&compressed, false);
    println!("Part 1: {}", part1_length);

    let part2_length = get_decompressed_size(&compressed, true);
    println!("Part 2: {}", part2_length);
}

fn get_decompressed_size(compressed: &Vec<char>, use_version_2: bool) -> usize {
    let mut length = 0;
    let mut index = 0;
    while index < compressed.len() {
        if compressed[index] != '(' {
            length += 1;
            index += 1;
            continue;
        }

        let encoding: String = compressed
            .iter()
            .skip(index + 1)
            .take_while(|c| **c != ')')
            .collect();

        let (size, count) = encoding.split_once("x").unwrap();
        let count: usize = count.parse().unwrap();
        let size: usize = size.parse().unwrap();

        if use_version_2 {
            length += count
                * get_decompressed_size(
                    &compressed[(index + encoding.len() + 2)..(index + encoding.len() + 2 + size)]
                        .to_vec(),
                    use_version_2,
                );
        } else {
            length += count * size;
        }
        index += encoding.len() + 2 /* for end paren */ + size;
    }

    length
}
