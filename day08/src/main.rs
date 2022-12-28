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

    let mut board: Vec<Vec<bool>> = vec![vec![false; 6]; 50];
    for line in lines {
        let components: Vec<&str> = line.split_whitespace().collect();
        match components[0] {
            "rect" => {
                let (w, h) = components[1].split_once("x").unwrap();
                for y in 0..h.parse().unwrap() {
                    for x in 0..w.parse().unwrap() {
                        board[x][y] = true;
                    }
                }
            }
            "rotate" => {
                let amount: usize = components[4].parse().unwrap();
                let starting_index: usize =
                    components[2].split_once("=").unwrap().1.parse().unwrap();
                match components[1] {
                    "row" => {
                        let old_row: Vec<bool> = board.iter().map(|c| c[starting_index]).collect();
                        for x in 0..50 {
                            board[(x + amount) % 50][starting_index] = old_row[x];
                        }
                    }
                    "column" => {
                        let old_column = board[starting_index].clone();
                        for y in 0..6 {
                            board[starting_index][(y + amount) % 6] = old_column[y];
                        }
                    }
                    _ => panic!("Unexpected rotation"),
                }
            }
            _ => panic!("Unexpected command"),
        }
    }

    let pixels_lit: usize = board
        .iter()
        .map(|col| col.iter().filter(|v| **v).count())
        .sum();
    println!("Part 1: {}", pixels_lit);

    for y in 0..6 {
        for x in 0..50 {
            if board[x][y] {
                print!("#");
            } else {
                print!(" ");
            }
        }
        print!("\n");
    }
}
