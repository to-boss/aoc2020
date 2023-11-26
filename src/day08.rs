use std::{collections::HashMap, fs, str::FromStr};

fn input() -> String {
    fs::read_to_string("input/day08.txt").unwrap()
}

#[derive(Debug, Clone, Copy)]
enum Op {
    Acc(i32),
    Jmp(i32),
    Nop,
}

#[derive(Debug, Clone)]
struct Console {
    instructions: Vec<Op>,
    acc: i32,
    ic: i32,
    visited: HashMap<usize, usize>,
}

impl Console {
    fn all_possible(&self) -> Vec<Console> {
        let mut consoles = Vec::new();

        let mut indeces: Vec<usize> = Vec::new();
        for (i, op) in self.instructions.iter().enumerate() {
            match op {
                Op::Jmp(_) | Op::Nop => indeces.push(i),
                Op::Acc(_) => (),
            };
        }

        for i in indeces {
            let mut cloned = self.clone();
            if let Some(op) = cloned.instructions.get_mut(i) {
                *op = match *op {
                    Op::Nop => Op::Jmp(0),
                    Op::Jmp(_) => Op::Nop,
                    _ => panic!("Should not happen"),
                };
            }
            consoles.push(cloned);
        }

        consoles
    }

    fn step(&mut self) -> (Option<i32>, Option<i32>) {
        if self.check_looped() {
            return (Some(self.acc), None);
        }

        if self.ic as usize == self.instructions.len() {
            return (None, Some(self.acc));
        }

        let curr_instruction = self.instructions.get(self.ic as usize);
        if let Some(instruction) = curr_instruction {
            match instruction {
                Op::Acc(val) => self.acc += val,
                Op::Jmp(val) => self.ic += val - 1,
                Op::Nop => (),
            }

            self.visited
                .entry(self.ic as usize)
                .and_modify(|n| *n += 1)
                .or_insert(1);

            self.ic += 1;
            //println!("{:?}: {}", instruction, self.ic);
        }

        (None, None)
    }

    fn check_looped(&self) -> bool {
        for (_, amount) in self.visited.iter() {
            if *amount > 1 {
                return true;
            }
        }
        false
    }
}

impl FromStr for Console {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        let mut instructions = Vec::new();

        for line in s.lines() {
            let line = line.trim();
            let (op, val) = line.split_once(" ").unwrap();
            let parsed_op = match op {
                "acc" => Op::Acc(val.parse()?),
                "jmp" => Op::Jmp(val.parse()?),
                "nop" => Op::Nop,
                _ => panic!("Should never happen."),
            };
            instructions.push(parsed_op);
        }

        Ok(Console {
            instructions,
            acc: 0,
            ic: 0,
            visited: HashMap::new(),
        })
    }
}

fn solve_1(input: &str) -> i32 {
    let mut console = Console::from_str(input).unwrap();
    loop {
        let res = console.step();
        if let (Some(val), None) = res {
            return val;
        }
    }
}

fn solve_2(input: &str) -> i32 {
    let console = Console::from_str(input).unwrap();
    let mut consoles = console.all_possible();

    for console in consoles.iter_mut() {
        loop {
            let res = console.step();
            match res {
                (None, Some(val)) => return val,
                (Some(_), None) => break,
                _ => (),
            }
        }
    }
    0
}

pub fn answer_1() {
    let input = input();
    println!("day08 part1: {}", solve_1(&input));
}

pub fn answer_2() {
    let input = input();
    println!("day08 part2: {}", solve_2(&input));
}

#[test]
fn test1() {
    let input = "nop +0
    acc +1
    jmp +4
    acc +3
    jmp -3
    acc -99
    acc +1
    jmp -4
    acc +6";
    assert_eq!(5, solve_1(input))
}

#[test]
fn test2() {
    let input = "nop +0
    acc +1
    jmp +4
    acc +3
    jmp -3
    acc -99
    acc +1
    jmp -4
    acc +6";
    assert_eq!(8, solve_2(input))
}
