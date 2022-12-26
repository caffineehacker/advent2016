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

    let mut last_button = (1, 1);
    for line in lines.iter() {
        for c in line.chars() {
            match c {
                'U' => last_button.1 = (last_button.1 - 1).max(0),
                'D' => last_button.1 = (last_button.1 + 1).min(2),
                'L' => last_button.0 = (last_button.0 - 1).max(0),
                'R' => last_button.0 = (last_button.0 + 1).min(2),
                _ => panic!("Unexpected direction"),
            }
        }

        print!("{}", last_button.0 + 1 + 3 * last_button.1);
    }
}
