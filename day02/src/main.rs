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

    solve_part1(&lines);
    solve_part2(&lines);
}

fn solve_part1(lines: &Vec<String>) {
    print!("Part 1: ");
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

    print!("\n");
}

fn solve_part2(lines: &Vec<String>) {
    print!("Part 2: ");
    let mut last_button: (i32, i32) = (0, 2);
    for line in lines.iter() {
        for c in line.chars() {
            match c {
                'U' => {
                    last_button.1 = (last_button.1 - 1).max(last_button.0.abs_diff(2_i32) as i32)
                }
                'D' => {
                    last_button.1 =
                        (last_button.1 + 1).min(4 - last_button.0.abs_diff(2_i32) as i32)
                }
                'L' => {
                    last_button.0 = (last_button.0 - 1).max(last_button.1.abs_diff(2_i32) as i32)
                }
                'R' => {
                    last_button.0 =
                        (last_button.0 + 1).min(4 - last_button.1.abs_diff(2_i32) as i32)
                }
                _ => panic!("Unexpected direction"),
            }
        }

        print!(
            "{}",
            match last_button {
                (2, 0) => 1.to_string(),
                (x, 1) => (x + 1).to_string(),
                (x, 2) => (x + 5).to_string(),
                (1, 3) => "A".to_string(),
                (2, 3) => "B".to_string(),
                (3, 3) => "C".to_string(),
                (2, 4) => "D".to_string(),
                _ => panic!("Unexpected location"),
            }
        );
    }

    print!("\n");
}
