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

    let progress = indicatif::ProgressBar::new(args.input.into());
    progress.set_style(
        indicatif::ProgressStyle::with_template(
            "[{elapsed_precise}] {wide_bar:.cyan/blue} {pos:>7}/{len:7} {msg} ({eta_precise})",
        )
        .unwrap(),
    );

    // Part 2
    let mut elves = (1..=args.input).collect_vec();
    let mut turn_index = 0;
    while elves.len() > 1 {
        let index = (turn_index + (elves.len() / 2)) % elves.len();
        elves.remove(index);

        if index > turn_index {
            turn_index = turn_index + 1;
        }

        turn_index %= elves.len();

        progress.set_message(format!("Elf: {}", elves[turn_index]));
        progress.inc(1);
    }

    progress.finish_and_clear();

    println!("Part 2: {}", elves[0]);
}
