use clap::Parser;
use itertools::Itertools;
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
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

#[derive(Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
struct Floor {
    microchips: Vec<String>,
    generators: Vec<String>,
}

#[derive(Clone, Eq)]
struct State {
    steps: u32,
    floors: Vec<Floor>,
    elevator_floor: usize,
}

impl State {
    fn steps_to_finish(&self) -> u32 {
        let mut steps = self.steps;
        let mut count = 0;
        for f in 0..(self.floors.len() - 1) {
            let floor_count =
                count + self.floors[f].generators.len() + self.floors[f].microchips.len();
            if floor_count > 0 {
                // All items will take two trips except the last 2 which will take 1
                steps += (2 * (floor_count as i32 - 2).max(0)) as u32 + 1;
            }

            count = floor_count;
        }

        steps
    }
}

impl std::hash::Hash for State {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.floors.hash(state);
        self.elevator_floor.hash(state);
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.floors == other.floors && self.elevator_floor == other.elevator_floor
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // We're returning the minimum steps to get to a final state
        match self.steps_to_finish().partial_cmp(&other.steps_to_finish()) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.steps.partial_cmp(&other.steps) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.floors.partial_cmp(&other.floors) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.elevator_floor.partial_cmp(&other.elevator_floor)
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn main() {
    let args = Args::parse();

    let file = File::open(&args.data_file).expect("Failed to open file");
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader
        .lines()
        .map(|line| line.expect("Failed to read line"))
        .collect();

    let floors = lines.iter().map(to_floor).collect_vec();

    let progress_bar = indicatif::ProgressBar::new_spinner();

    let mut states: BinaryHeap<Reverse<State>> = BinaryHeap::new();
    states.push(Reverse(State {
        steps: 0,
        floors: floors.clone(),
        elevator_floor: 0,
    }));
    let mut seen_states = HashSet::new();

    'search: while !states.is_empty() {
        let state = states.pop().unwrap().0;

        if !seen_states.insert(state.clone()) {
            continue;
        }

        if args.debug {
            println!("Step: {}, States: {}", state.steps, states.len());
        } else {
            progress_bar.set_message(format!("Step: {}, States: {}", state.steps, states.len()));
        }

        // Is the current state valid
        for floor in state.floors.iter() {
            if floor.generators.len() > 0
                && floor
                    .microchips
                    .iter()
                    .any(|m| !floor.generators.contains(m))
            {
                // This is an invalid state
                continue 'search;
            }
        }

        if !state
            .floors
            .iter()
            .take(3)
            .any(|f| f.microchips.len() > 0 || f.generators.len() > 0)
        {
            if args.debug {
                println!("Part 1: {}", state.steps);
            } else {
                progress_bar.finish_with_message(format!("Final steps: {}", state.steps));
            }
            return;
        }

        // The elevator can move up or down and must carry at least one item
        // If a microchip and a generator are in the same space and the microchip does
        // not have a matching generator then the state is invalid.

        // Moving up
        for g in 0..state.floors[state.elevator_floor].generators.len() {
            // For each generator, we can take the generator, the generator and another generator, or the generator and a matching chip.
            let mut floors = state.floors.clone();
            let generator = floors[state.elevator_floor].generators.remove(g);

            // Just the generator up
            if state.elevator_floor < state.floors.len() - 1 {
                let mut floors = floors.clone();
                floors[state.elevator_floor + 1]
                    .generators
                    .push(generator.clone());
                floors[state.elevator_floor + 1].generators.sort();

                states.push(Reverse(State {
                    steps: state.steps + 1,
                    floors,
                    elevator_floor: state.elevator_floor + 1,
                }));
            }

            // Just the generator down
            if state.elevator_floor > 0 {
                let mut floors = floors.clone();
                floors[state.elevator_floor - 1]
                    .generators
                    .push(generator.clone());
                floors[state.elevator_floor - 1].generators.sort();

                states.push(Reverse(State {
                    steps: state.steps + 1,
                    floors,
                    elevator_floor: state.elevator_floor - 1,
                }));
            }

            if floors[state.elevator_floor].microchips.contains(&generator) {
                let mut floors = floors.clone();
                floors[state.elevator_floor].microchips = floors[state.elevator_floor]
                    .microchips
                    .iter()
                    .cloned()
                    .filter(|m| *m != generator)
                    .collect_vec()
                    .into();

                // Generator and microchip up
                if state.elevator_floor < state.floors.len() - 1 {
                    let mut floors = floors.clone();
                    floors[state.elevator_floor + 1]
                        .generators
                        .push(generator.clone());
                    floors[state.elevator_floor + 1].generators.sort();
                    floors[state.elevator_floor + 1]
                        .microchips
                        .push(generator.clone());
                    floors[state.elevator_floor + 1].microchips.sort();

                    states.push(Reverse(State {
                        steps: state.steps + 1,
                        floors,
                        elevator_floor: state.elevator_floor + 1,
                    }));
                }

                // Generator and microchip down
                if state.elevator_floor > 0 {
                    floors[state.elevator_floor - 1]
                        .generators
                        .push(generator.clone());
                    floors[state.elevator_floor - 1].generators.sort();
                    floors[state.elevator_floor - 1]
                        .microchips
                        .push(generator.clone());
                    floors[state.elevator_floor - 1].microchips.sort();

                    states.push(Reverse(State {
                        steps: state.steps + 1,
                        floors,
                        elevator_floor: state.elevator_floor - 1,
                    }));
                }
            }

            // Since we already removed the item at index g, we can start at g again
            for second_g in g..floors[state.elevator_floor].generators.len() {
                let mut floors = floors.clone();
                let second_generator = floors[state.elevator_floor].generators.remove(second_g);

                // Both generators up
                if state.elevator_floor < state.floors.len() - 1 {
                    let mut floors = floors.clone();
                    floors[state.elevator_floor + 1]
                        .generators
                        .push(generator.clone());
                    floors[state.elevator_floor + 1]
                        .generators
                        .push(second_generator.clone());
                    floors[state.elevator_floor + 1].generators.sort();

                    states.push(Reverse(State {
                        steps: state.steps + 1,
                        floors,
                        elevator_floor: state.elevator_floor + 1,
                    }));
                }

                // Both generators down
                if state.elevator_floor > 0 {
                    let mut floors = floors.clone();
                    floors[state.elevator_floor - 1]
                        .generators
                        .push(generator.clone());
                    floors[state.elevator_floor - 1]
                        .generators
                        .push(second_generator.clone());
                    floors[state.elevator_floor - 1].generators.sort();

                    states.push(Reverse(State {
                        steps: state.steps + 1,
                        floors,
                        elevator_floor: state.elevator_floor - 1,
                    }));
                }
            }
        }

        // Just moving 1 or two microchips
        for m in 0..state.floors[state.elevator_floor].microchips.len() {
            let mut floors = state.floors.clone();
            let microchip = floors[state.elevator_floor].microchips.remove(m);

            // Just the microchip up
            if state.elevator_floor < state.floors.len() - 1 {
                let mut floors = floors.clone();
                floors[state.elevator_floor + 1]
                    .microchips
                    .push(microchip.clone());
                floors[state.elevator_floor + 1].microchips.sort();

                states.push(Reverse(State {
                    steps: state.steps + 1,
                    floors,
                    elevator_floor: state.elevator_floor + 1,
                }));
            }

            // Just the microchip down
            if state.elevator_floor > 0 {
                let mut floors = floors.clone();
                floors[state.elevator_floor - 1]
                    .microchips
                    .push(microchip.clone());
                floors[state.elevator_floor - 1].microchips.sort();

                states.push(Reverse(State {
                    steps: state.steps + 1,
                    floors,
                    elevator_floor: state.elevator_floor - 1,
                }));
            }

            // Since we already removed the item at index m, we can start at g again
            for second_m in m..floors[state.elevator_floor].microchips.len() {
                let mut floors = floors.clone();
                let second_microchip = floors[state.elevator_floor].microchips.remove(second_m);

                // Both microchips up
                if state.elevator_floor < state.floors.len() - 1 {
                    let mut floors = floors.clone();
                    floors[state.elevator_floor + 1]
                        .microchips
                        .push(microchip.clone());
                    floors[state.elevator_floor + 1]
                        .microchips
                        .push(second_microchip.clone());
                    floors[state.elevator_floor + 1].microchips.sort();

                    states.push(Reverse(State {
                        steps: state.steps + 1,
                        floors,
                        elevator_floor: state.elevator_floor + 1,
                    }));
                }

                // Both microchips down
                if state.elevator_floor > 0 {
                    let mut floors = floors.clone();
                    floors[state.elevator_floor - 1]
                        .microchips
                        .push(microchip.clone());
                    floors[state.elevator_floor - 1]
                        .microchips
                        .push(second_microchip.clone());
                    floors[state.elevator_floor - 1].microchips.sort();

                    states.push(Reverse(State {
                        steps: state.steps + 1,
                        floors,
                        elevator_floor: state.elevator_floor - 1,
                    }));
                }
            }
        }
    }
}

fn to_floor(line: &String) -> Floor {
    let mut generators = Vec::new();
    let mut microchips = Vec::new();

    let components = line.split_whitespace().collect_vec();
    let mut index = 5;
    while index < components.len() - 1 {
        // This means there was an extra word, "and" for this entry
        if components[index] == "a" {
            index += 1;
        }
        match components[index + 1]
            .trim_end_matches(",")
            .trim_end_matches(".")
        {
            "generator" => generators.push(components[index].to_string()),
            "microchip" => microchips.push(
                components[index]
                    .trim_end_matches("-compatible")
                    .to_string(),
            ),
            _ => panic!("Unexpected string"),
        }

        index += 3;
    }

    Floor {
        microchips,
        generators,
    }
}
