use clap::Parser;
use itertools::Itertools;
use std::{cmp::Reverse, collections::BinaryHeap};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    input: u32,
    #[arg(long)]
    debug: bool,
}

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
struct State {
    steps: u32,
    position: (u32, u32),
    target: (u32, u32),
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self.steps
            + self.position.0.abs_diff(self.target.0)
            + self.position.1.abs_diff(self.target.1))
        .partial_cmp(
            &(other.steps
                + other.position.0.abs_diff(other.target.0)
                + other.position.1.abs_diff(other.target.1)),
        ) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }

        match self.steps.partial_cmp(&other.steps) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.position.partial_cmp(&other.position)
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn main() {
    let args = Args::parse();

    let mut states: BinaryHeap<Reverse<State>> = BinaryHeap::new();
    states.push(Reverse(State {
        position: (1, 1),
        steps: 0,
        target: (31, 39),
    }));

    let mut positions_seen = Vec::new();

    while !states.is_empty() {
        let state = states.pop().unwrap().0;

        if positions_seen.contains(&state.position) {
            continue;
        }

        positions_seen.push(state.position);

        if state.position == state.target {
            println!("Part 1: {}", state.steps);
        }

        let mut state = state.clone();
        state.steps += 1;

        if state.position.0 > 0 && !is_wall((state.position.0 - 1, state.position.1), args.input) {
            let mut state = state.clone();
            state.position.0 -= 1;
            states.push(Reverse(state));
        }

        if state.position.1 > 0 && !is_wall((state.position.0, state.position.1 - 1), args.input) {
            let mut state = state.clone();
            state.position.1 -= 1;
            states.push(Reverse(state));
        }

        if !is_wall((state.position.0, state.position.1 + 1), args.input) {
            let mut state = state.clone();
            state.position.1 += 1;
            states.push(Reverse(state));
        }

        if !is_wall((state.position.0 + 1, state.position.1), args.input) {
            let mut state = state.clone();
            state.position.0 += 1;
            states.push(Reverse(state));
        }
    }
}

fn is_wall(position: (u32, u32), input: u32) -> bool {
    let test = position.0.pow(2)
        + position.0 * 3
        + position.0 * position.1 * 2
        + position.1
        + position.1.pow(2)
        + input;
    test.count_ones() % 2 == 1
}
