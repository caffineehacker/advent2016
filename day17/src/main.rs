use std::{cmp::Reverse, collections::BinaryHeap};

use clap::Parser;
use itertools::Itertools;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    input: String,
    #[arg(long)]
    debug: bool,
}

#[derive(PartialEq, Eq)]
struct State {
    position: (i8, i8),
    code: String,
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self.position.0 as isize + self.position.1 as isize - self.code.len() as isize)
            .partial_cmp(
                &(other.position.0 as isize + other.position.1 as isize
                    - other.code.len() as isize),
            ) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.position.partial_cmp(&other.position) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.code.partial_cmp(&other.code)
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn main() {
    let args = Args::parse();

    do_part1(&args);
    do_part2(&args);
}

fn do_part1(args: &Args) {
    let mut states = BinaryHeap::new();
    states.push(State {
        position: (0, 0),
        code: args.input.clone(),
    });

    while !states.is_empty() {
        let state = states.pop().unwrap();

        if state.position == (3, 3) {
            println!("Part 1: {}", state.code.trim_start_matches(&args.input));
            break;
        }

        let hash = md5::compute(state.code.clone()).0;
        if state.position.1 > 0 && hash[0] >= 0xb0 {
            states.push(State {
                position: (state.position.0, state.position.1 - 1),
                code: state.code.clone() + "U",
            });
        }
        if state.position.1 < 3 && hash[0] & 0x0F >= 0x0b {
            states.push(State {
                position: (state.position.0, state.position.1 + 1),
                code: state.code.clone() + "D",
            });
        }

        if state.position.0 > 0 && hash[1] >= 0xb0 {
            states.push(State {
                position: (state.position.0 - 1, state.position.1),
                code: state.code.clone() + "L",
            });
        }
        if state.position.0 < 3 && hash[1] & 0x0F >= 0x0b {
            states.push(State {
                position: (state.position.0 + 1, state.position.1),
                code: state.code.clone() + "R",
            });
        }
    }
}

fn do_part2(args: &Args) {
    let mut states = BinaryHeap::new();
    states.push(State {
        position: (0, 0),
        code: args.input.clone(),
    });

    let mut longest_path = 0;
    while !states.is_empty() {
        let state = states.pop().unwrap();

        if state.position == (3, 3) {
            longest_path = longest_path.max(state.code.trim_start_matches(&args.input).len());
            continue;
        }

        let hash = md5::compute(state.code.clone()).0;
        if state.position.1 > 0 && hash[0] >= 0xb0 {
            states.push(State {
                position: (state.position.0, state.position.1 - 1),
                code: state.code.clone() + "U",
            });
        }
        if state.position.1 < 3 && hash[0] & 0x0F >= 0x0b {
            states.push(State {
                position: (state.position.0, state.position.1 + 1),
                code: state.code.clone() + "D",
            });
        }

        if state.position.0 > 0 && hash[1] >= 0xb0 {
            states.push(State {
                position: (state.position.0 - 1, state.position.1),
                code: state.code.clone() + "L",
            });
        }
        if state.position.0 < 3 && hash[1] & 0x0F >= 0x0b {
            states.push(State {
                position: (state.position.0 + 1, state.position.1),
                code: state.code.clone() + "R",
            });
        }
    }

    println!("Part 2: {}", longest_path);
}
