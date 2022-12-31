use clap::Parser;
use itertools::Itertools;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    input: String,
    #[arg(long)]
    space: usize,
    #[arg(long)]
    debug: bool,
}

fn main() {
    let args = Args::parse();

    let mut data = args
        .input
        .chars()
        .map(|c| if c == '0' { false } else { true })
        .collect_vec();

    while data.len() < args.space {
        let mut to_append = data.iter().map(|b| !b).rev().collect_vec();

        data.push(false);
        data.append(&mut to_append);

        if args.debug {
            println!(
                "{}",
                data.iter()
                    .map(|b| if *b { "1" } else { "0" })
                    .collect::<String>()
            );
        }
    }

    data.truncate(args.space);

    // Now the checksum
    loop {
        if data.len() % 2 == 1 {
            println!(
                "Part 1: {}",
                data.iter()
                    .map(|b| if *b { "1" } else { "0" })
                    .collect::<String>()
            );

            break;
        }

        data = data
            .iter()
            .chunks(2)
            .into_iter()
            .map(|mut pairs| pairs.next() == pairs.next())
            .collect_vec();
    }
}
