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
    #[arg(long)]
    debug: bool,
}

#[derive(Clone, Debug)]
enum Destination {
    Bot(u32),
    Output(u32),
}

#[derive(Clone, Debug)]
struct Bot {
    number: u32,
    values: (Option<u32>, Option<u32>),
    high_value_dest: Destination,
    low_value_dest: Destination,
}

fn main() {
    let args = Args::parse();

    let file = File::open(&args.data_file).expect("Failed to open file");
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader
        .lines()
        .map(|line| line.expect("Failed to read line"))
        .collect();

    let mut bots: Vec<Bot> = lines
        .iter()
        .filter(|line| line.starts_with("bot"))
        .map(to_bot)
        .collect();
    bots.sort_by_key(|b| b.number);

    lines
        .iter()
        .filter(|line| line.starts_with("value"))
        .for_each(|line| {
            let components = line.split_whitespace().collect_vec();
            let bot = bots
                .iter_mut()
                .find(|b| b.number == components[5].parse().unwrap())
                .unwrap();
            if bot.values.0.is_some() {
                if bot.values.1.is_some() {
                    panic!("Bot has too many values");
                }

                bot.values.1 = Some(components[1].parse().unwrap());
            } else {
                bot.values.0 = Some(components[1].parse().unwrap());
            }
        });

    let mut part2: u64 = 1;
    let mut part2_count = 0;
    'processing: loop {
        let mut i = 0;
        while i < bots.len() {
            let bot = bots.get(i).unwrap().clone();
            if bot.values.0.is_some() && bot.values.1.is_some() {
                if args.debug {
                    println!("Processing: {:?}", bot);
                }
                let low = bot.values.0.unwrap().min(bot.values.1.unwrap());
                let high = bot.values.0.unwrap().max(bot.values.1.unwrap());

                if low == 17 && high == 61 {
                    println!("Part 1: {}", bot.number);
                    if part2_count == 3 {
                        break 'processing;
                    }
                }

                match bot.low_value_dest {
                    Destination::Bot(dest) => {
                        let lb = bots.iter_mut().find(|b| b.number == dest).unwrap();
                        if lb.values.0.is_none() {
                            lb.values.0 = Some(low);
                        } else if lb.values.1.is_none() {
                            lb.values.1 = Some(low);
                        } else {
                            panic!("Too many values");
                        }
                    }
                    Destination::Output(bin) => {
                        if bin < 3 {
                            part2 *= low as u64;
                            part2_count += 1;

                            if part2_count == 3 {
                                break 'processing;
                            }
                        }
                    }
                }

                match bot.high_value_dest {
                    Destination::Bot(dest) => {
                        let hb = bots.iter_mut().find(|b| b.number == dest).unwrap();
                        if hb.values.0.is_none() {
                            hb.values.0 = Some(high);
                        } else if hb.values.1.is_none() {
                            hb.values.1 = Some(high);
                        } else {
                            panic!("Too many values");
                        }
                    }
                    Destination::Output(bin) => {
                        if bin < 3 {
                            part2 *= high as u64;
                            part2_count += 1;

                            if part2_count == 3 {
                                break 'processing;
                            }
                        }
                    }
                }

                bots.get_mut(i).unwrap().values = (None, None);
            }

            i += 1;
        }
    }

    println!("Part 2: {}", part2);
}

fn to_bot(line: &String) -> Bot {
    let components = line.split_whitespace().collect_vec();

    Bot {
        number: components[1].parse().unwrap(),
        values: (None, None),
        low_value_dest: match components[5] {
            "bot" => Destination::Bot(components[6].parse().unwrap()),
            "output" => Destination::Output(components[6].parse().unwrap()),
            _ => panic!("Unexpected destination"),
        },
        high_value_dest: match components[10] {
            "bot" => Destination::Bot(components[11].parse().unwrap()),
            "output" => Destination::Output(components[11].parse().unwrap()),
            _ => panic!("Unexpected destination"),
        },
    }
}
