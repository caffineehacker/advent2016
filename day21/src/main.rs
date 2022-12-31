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
    password: String,
    #[arg(long)]
    debug: bool,
}

fn main() {
    let args = Args::parse();

    let file = File::open(&args.data_file).expect("Failed to open file");
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader
        .lines()
        .map(|line| line.expect("Failed to read line"))
        .collect();

    let mut password = args.password.chars().collect_vec();
    for instruction in lines.iter() {
        let components = instruction.split_whitespace().collect_vec();
        match components[0] {
            "swap" => match components[1] {
                "position" => {
                    let first = components[2].parse::<usize>().unwrap();
                    let second = components[5].parse::<usize>().unwrap();
                    let temp = password[first];
                    password[first] = password[second];
                    password[second] = temp;
                }
                "letter" => {
                    let first_letter = components[2].chars().last().unwrap();
                    let second_letter = components[5].chars().last().unwrap();
                    password.iter_mut().for_each(|c| {
                        if *c == first_letter {
                            *c = second_letter;
                        } else if *c == second_letter {
                            *c = first_letter;
                        }
                    })
                }
                _ => panic!("Unexpected swap"),
            },
            "rotate" => {
                let mut amount;
                if components[1] == "based" {
                    let test_char = components[6].chars().last().unwrap();
                    amount = 1 + password.iter().position(|c| *c == test_char).unwrap();
                    if amount >= 5 {
                        amount += 1;
                    }
                } else {
                    amount = components[2].parse::<usize>().unwrap();
                }
                if components[1] == "right" || components[1] == "based" {
                    // It's easiest to always rotate left
                    amount %= password.len();
                    amount = password.len() - amount;
                }
                amount %= password.len();

                let len = password.len();
                password = password
                    .into_iter()
                    .cycle()
                    .skip(amount)
                    .take(len)
                    .collect_vec();
            }
            "reverse" => {
                let start_index = components[2].parse::<usize>().unwrap();
                let end_index = components[4].parse::<usize>().unwrap();
                let mut new_password = password.iter().take(start_index).cloned().collect_vec();
                new_password.append(
                    &mut password
                        .iter()
                        .skip(start_index)
                        .take(end_index + 1 - start_index)
                        .rev()
                        .cloned()
                        .collect_vec(),
                );
                new_password.append(&mut password.into_iter().skip(end_index + 1).collect_vec());
                password = new_password;
            }
            "move" => {
                let removed = password.remove(components[2].parse().unwrap());
                password.insert(components[5].parse().unwrap(), removed);
            }
            _ => panic!("Unexpected instruction"),
        }

        if args.debug {
            println!("{}", instruction);
            println!("{}", password.iter().collect::<String>());
        }
    }

    println!("Part 1: {}", password.iter().collect::<String>());
}
