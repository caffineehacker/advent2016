use clap::Parser;
use itertools::Itertools;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    vec,
};

#[derive(Clone, Copy, Debug)]
#[repr(usize)]
enum Register {
    A = 0,
    B,
    C,
    D,
}

impl From<&str> for Register {
    fn from(value: &str) -> Self {
        match value {
            "a" => Register::A,
            "b" => Register::B,
            "c" => Register::C,
            "d" => Register::D,
            _ => panic!("Unexpected register"),
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Operand {
    Value(i32),
    Register(Register),
}

impl From<&str> for Operand {
    fn from(value: &str) -> Self {
        if let Ok(value) = value.parse::<i32>() {
            Operand::Value(value)
        } else {
            Operand::Register(value.into())
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Instruction {
    Copy(Operand, Operand),
    Increment(Register),
    Decrement(Register),
    JumpNotZero(Operand, Operand),
    Toggle(Register),
}

impl From<&str> for Instruction {
    fn from(value: &str) -> Self {
        let components = value.split_whitespace().collect_vec();

        match components[0] {
            "cpy" => Instruction::Copy(components[1].into(), components[2].into()),
            "inc" => Instruction::Increment(components[1].into()),
            "dec" => Instruction::Decrement(components[1].into()),
            "jnz" => Instruction::JumpNotZero(components[1].into(), components[2].into()),
            "tgl" => Instruction::Toggle(components[1].into()),
            _ => panic!("Unexpected instruction"),
        }
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    data_file: String,
    #[arg(long)]
    debug: bool,
    #[arg(long)]
    start_a: Option<i32>,
}

fn main() {
    let args = Args::parse();

    let file = File::open(&args.data_file).expect("Failed to open file");
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader
        .lines()
        .map(|line| line.expect("Failed to read line"))
        .collect();

    let mut instructions = lines
        .iter()
        .map(|line| Instruction::from(line.as_str()))
        .collect_vec();

    let mut pc: i32 = 0;
    let mut registers = vec![0; 4];

    if args.start_a.is_some() {
        registers[0] = args.start_a.unwrap();
    }

    while (pc as usize) < instructions.len() {
        if args.debug {
            println!("{}: {:?}", pc, instructions[pc as usize]);
        }
        match &instructions[pc as usize] {
            Instruction::Copy(src, dst) => {
                // Copying to an immediate is invalid
                if let Operand::Register(reg) = dst {
                    registers[*reg as usize] = match src {
                        Operand::Value(val) => *val,
                        Operand::Register(src) => registers[*src as usize],
                    }
                }
            }
            Instruction::Increment(reg) => registers[*reg as usize] += 1,
            Instruction::Decrement(reg) => registers[*reg as usize] -= 1,
            Instruction::JumpNotZero(operand, dst) => {
                if match operand {
                    Operand::Value(val) => *val,
                    Operand::Register(reg) => registers[*reg as usize],
                } != 0
                {
                    pc += match dst {
                        Operand::Value(val) => *val,
                        Operand::Register(reg) => registers[*reg as usize],
                    };

                    // To offset the += 1 later in this function
                    pc -= 1;
                }
            }
            Instruction::Toggle(reg) => {
                let target_index = (registers[*reg as usize] + pc) as usize;
                // Do nothing if we're not pointing to an instruction in the program
                if target_index < instructions.len() {
                    instructions[target_index] = match instructions[target_index] {
                        Instruction::Copy(a, b) => Instruction::JumpNotZero(a, b),
                        Instruction::Increment(reg) => Instruction::Decrement(reg),
                        Instruction::Decrement(reg) => Instruction::Increment(reg),
                        Instruction::JumpNotZero(test, dst) => Instruction::Copy(test, dst),
                        Instruction::Toggle(reg) => Instruction::Increment(reg),
                    };
                }
            }
        }

        pc += 1;
    }

    print!("Registers: ");
    registers.iter().for_each(|reg| print!("{} ", reg));
    println!("");
}
