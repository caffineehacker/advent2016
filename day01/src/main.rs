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
        .fold(
            ((0_i32, 1_i32), (0, 0), (Vec::new(), None)),
            |acc, direction| {
                let facing = acc.0;
                let facing = match &direction[0..1] {
                    "R" => (facing.1, -facing.0),
                    "L" => (-facing.1, facing.0),
                    _ => panic!("Unexpected direction"),
                };
                let movement: i32 = direction[1..].parse().unwrap();

                let mut new_location = acc.1;

                let mut visited = acc.2 .0.clone();
                let mut revisited = acc.2 .1;
                for _ in 0..movement {
                    new_location.0 += facing.0;
                    new_location.1 += facing.1;
                    if revisited.is_none() && visited.contains(&new_location) {
                        revisited = Some(new_location);
                        println!(
                            "Part 2: {}, {} = {}",
                            new_location.0,
                            new_location.1,
                            new_location.0.abs() + new_location.1.abs()
                        );
                    }
                    visited.push(new_location);
                }

                (facing, new_location, (visited, revisited))
            },
        )
        .1;

    println!("Part 1: {}", destination.0.abs() + destination.1.abs());
}
