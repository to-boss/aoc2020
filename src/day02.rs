use std::{fs, str::FromStr};

pub struct Policy {
    left_num: usize,
    right_num: usize,
    letter: char,
    password: String,
}

impl Policy {
    fn count_letter(&self) -> usize {
        self.password
            .chars()
            .fold(0, |acc, c| if c == self.letter { acc + 1 } else { acc })
    }

    fn is_valid1(&self) -> bool {
        let count = self.count_letter();
        count >= self.left_num && count <= self.right_num
    }

    fn is_valid2(&self) -> bool {
        let mut found = 0;
        let chars: Vec<char> = self.password.chars().collect();
        if let Some(char) = chars.get(self.left_num - 1) {
            if *char == self.letter {
                found += 1;
            }
        }
        if let Some(char) = chars.get(self.right_num - 1) {
            if *char == self.letter {
                found += 1;
            }
        }

        found == 1
    }
}

impl FromStr for Policy {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Policy> {
        let (min, rest) = s.split_once("-").unwrap();
        let min = min.parse()?;

        let (max, rest) = rest.split_once(" ").unwrap();
        let max = max.parse()?;

        let (letter, rest) = rest.split_once(":").unwrap();
        let letter = letter.trim_start().parse()?;

        let password = rest.trim().to_string();

        Ok(Policy {
            left_num: min,
            right_num: max,
            letter,
            password,
        })
    }
}

pub fn input() -> Vec<Policy> {
    let input = fs::read_to_string("input/day02.txt").unwrap();
    input
        .lines()
        .into_iter()
        .map(|line| line.parse().unwrap())
        .collect()
}

pub fn solve_1(input: Vec<Policy>) -> usize {
    input.iter().fold(
        0,
        |acc, policy| if policy.is_valid1() { acc + 1 } else { acc },
    )
}

pub fn solve_2(input: Vec<Policy>) -> usize {
    input.iter().fold(
        0,
        |acc, policy| if policy.is_valid2() { acc + 1 } else { acc },
    )
}

pub fn answer_1() {
    let input = input();
    println!("day02 part1: {:?}", solve_1(input));
}

pub fn answer_2() {
    let input = input();
    println!("day02 part2: {:?}", solve_2(input));
}

#[test]
fn test1() {
    let input = vec!["1-3 a: abcde", "1-3 b: cdefg", "2-9 c: ccccccccc"]
        .into_iter()
        .map(|line| line.parse().unwrap())
        .collect();
    let result = solve_1(input);
    assert_eq!(2, result);
}

#[test]
fn test2() {
    let input = vec!["1-3 a: abcde", "1-3 b: cdefg", "2-9 c: ccccccccc"]
        .into_iter()
        .map(|line| line.parse().unwrap())
        .collect();
    let result = solve_2(input);
    assert_eq!(1, result);
}
