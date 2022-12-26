use clap::Parser;
use std::fs;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    data_file: String,
}

fn main() {
    let args = Args::parse();

    let destination = fs::read_to_string(args.data_file)
        .unwrap()
        .trim()
        .split(", ")
        .fold(((0_i32, 1_i32), (0, 0)), |acc, direction| {
            let facing = acc.0;
            let facing = match &direction[0..1] {
                "R" => (facing.1, -facing.0),
                "L" => (-facing.1, facing.0),
                _ => panic!("Unexpected direction"),
            };
            let movement: i32 = direction[1..].parse().unwrap();

            let new_location = (
                acc.1 .0 + (movement * facing.0),
                acc.1 .1 + (movement * facing.1),
            );

            (facing, new_location)
        })
        .1;

    println!("Part 1: {}", destination.0.abs() + destination.1.abs());
}
