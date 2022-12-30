use clap::Parser;
use itertools::Itertools;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    vec,
};

#[derive(Clone, Copy)]
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

#[derive(Clone, Copy)]
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

#[derive(Clone, Copy)]
enum Instruction {
    Copy(Operand, Register),
    Increment(Register),
    Decrement(Register),
    JumpNotZero(Operand, i32),
}

impl From<&str> for Instruction {
    fn from(value: &str) -> Self {
        let components = value.split_whitespace().collect_vec();

        match components[0] {
            "cpy" => Instruction::Copy(components[1].into(), components[2].into()),
            "inc" => Instruction::Increment(components[1].into()),
            "dec" => Instruction::Decrement(components[1].into()),
            "jnz" => Instruction::JumpNotZero(
                components[1].into(),
                // We subtract one here so we can always increment the program counter
                components[2]
                    .parse::<i32>()
                    .expect("Failed to parse jump target")
                    - 1,
            ),
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
}

fn main() {
    let args = Args::parse();

    let file = File::open(&args.data_file).expect("Failed to open file");
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader
        .lines()
        .map(|line| line.expect("Failed to read line"))
        .collect();

    let instructions = lines
        .iter()
        .map(|line| Instruction::from(line.as_str()))
        .collect_vec();

    let mut pc: i32 = 0;
    let mut registers = vec![0; 4];
    while (pc as usize) < instructions.len() {
        match &instructions[pc as usize] {
            Instruction::Copy(src, dst) => {
                registers[*dst as usize] = match src {
                    Operand::Value(val) => *val,
                    Operand::Register(src) => registers[*src as usize],
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
                    pc += *dst;
                }
            }
        }

        pc += 1;
    }

    print!("Registers: ");
    registers.iter().for_each(|reg| print!("{} ", reg));
    println!("");
}
