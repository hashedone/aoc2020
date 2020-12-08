use anyhow::{anyhow, Error, Result};
use std::str::FromStr;

#[derive(Debug, Clone)]
enum Op {
    Nop(isize),
    Acc(i32),
    Jmp(isize),
}

impl FromStr for Op {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.trim().split(' ');
        let token = s.next().ok_or_else(|| anyhow!("No op token".to_owned()))?;
        match token {
            "nop" => Ok(Self::Nop(
                s.next()
                    .ok_or_else(|| anyhow!("Invalid jmp offset".to_owned()))?
                    .parse()?,
            )),
            "acc" => Ok(Self::Acc(
                s.next()
                    .ok_or_else(|| anyhow!("Invalid acc argument".to_owned()))?
                    .parse()?,
            )),
            "jmp" => Ok(Self::Jmp(
                s.next()
                    .ok_or_else(|| anyhow!("Invalid jmp offset".to_owned()))?
                    .parse()?,
            )),
            _ => Err(anyhow!("Invalid operation".to_owned())),
        }
    }
}

struct Machine {
    pc: usize,
    accumulator: i32,
    program: Vec<Op>,
}

impl Machine {
    fn new(program: impl IntoIterator<Item = Op>) -> Self {
        Self {
            pc: 0,
            accumulator: 0,
            program: program.into_iter().collect(),
        }
    }

    fn step(&mut self) -> bool {
        let op = if let Some(op) = self.program.get(self.pc) {
            op
        } else {
            return false;
        };

        match op {
            Op::Nop(_) => {
                println!("[{}] Nop", self.pc);
                self.pc += 1;
            }
            Op::Acc(arg) => {
                println!(
                    "[{}] Acc {}, acc = {} + {1} = {}",
                    self.pc,
                    arg,
                    self.accumulator,
                    self.accumulator + arg
                );
                self.accumulator += arg;
                self.pc += 1;
            }
            Op::Jmp(offset) => {
                println!(
                    "[{}] Jmp {}, pc = {0} + {1} = {}",
                    self.pc,
                    offset,
                    ((self.pc as isize) + offset) as usize
                );
                self.pc = ((self.pc as isize) + offset) as usize;
            }
        }

        self.pc < self.program.len()
    }

    fn reset(&mut self) {
        self.pc = 0;
        self.accumulator = 0;
    }
}

fn part1(program: Vec<Op>) -> i32 {
    let mut machine = Machine::new(program);
    let mut visited = vec![false; machine.program.len()];

    while !visited[machine.pc] {
        visited[machine.pc] = true;
        machine.step();
    }

    machine.accumulator
}

fn part2(program: Vec<Op>) -> Result<i32> {
    let mut machine = Machine::new(program);
    let mut visited = vec![false; machine.program.len()];

    let swap = |machine: &mut Machine, n: usize| {
        let new = match &machine.program[n] {
            Op::Nop(arg) => Op::Jmp(*arg),
            Op::Jmp(arg) => Op::Nop(*arg),
            _ => return false,
        };

        machine.program[n] = new;
        true
    };

    for n in 0..machine.program.len() {
        if !swap(&mut machine, n) {
            continue;
        }

        while !visited[machine.pc] {
            visited[machine.pc] = true;
            if !machine.step() {
                return Ok(machine.accumulator);
            }
        }

        swap(&mut machine, n);
        for v in visited.iter_mut() {
            *v = false;
        }
        machine.reset()
    }

    Err(anyhow!("Loops, loops everywhere!".to_owned()))
}

fn main() -> Result<()> {
    let program: Vec<Op> = common::std_input_vec()?;
    println!("Part1: {}", part1(program.clone()));
    println!("Part2: {}", part2(program)?);

    Ok(())
}
