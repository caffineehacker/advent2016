use clap::Parser;
use itertools::Itertools;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    input: String,
    #[arg(long)]
    rows: usize,
    #[arg(long)]
    debug: bool,
}

fn main() {
    let args = Args::parse();

    // True if a tile is a trap
    let mut traps = args.input.chars().map(|c| c == '^').collect_vec();

    let mut safe_count = traps.iter().filter(|t| !**t).count();
    for _ in 0..(args.rows - 1) {
        traps = next_row(&traps);
        safe_count += traps.iter().filter(|t| !**t).count();

        if args.debug {
            println!(
                "{}",
                traps
                    .iter()
                    .map(|t| if *t { "^" } else { "." })
                    .collect::<String>()
            );
        }
    }

    println!("Part 1: {}", safe_count);
}

fn next_row(traps: &Vec<bool>) -> Vec<bool> {
    let mut result = vec![false; traps.len()];
    for i in 0..traps.len() {
        let left_is_trap = i != 0 && traps[i - 1];
        let right_is_trap = i < traps.len() - 1 && traps[i + 1];
        if left_is_trap ^ right_is_trap {
            result[i] = true;
        }
    }

    result
}
