use clap::Parser;
use itertools::Itertools;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    input: u32,
    #[arg(long)]
    debug: bool,
}

fn main() {
    let args = Args::parse();

    let mut next_turn = 1;
    let mut remaining = args.input;
    let mut skip = 1;

    while remaining > 1 {
        // For each cycle, start with the lowest number and remove every other entry
        // This will effectively increase the skip count by 1
        // If the number of entries is even then the player that starts the round will end the round
        // If not, then we'll have an appendage that can be resolved by adding skip count to the next turn number

        skip *= 2;
        if remaining % 2 == 0 {
            remaining /= 2;
        } else {
            next_turn += skip;
            remaining /= 2;
        }

        if args.debug {
            println!(
                "Elves: {}, Next turn: {}, Skip: {}",
                remaining, next_turn, skip
            );
        }
    }

    println!("Part 1: {}", next_turn);
}
