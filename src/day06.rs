use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn input() -> String {
    fs::read_to_string("input/day06.txt").unwrap()
}

fn count_unique(input: &str) -> usize {
    let mut insertions = 0;
    let mut set = HashSet::new();
    for c in input.chars() {
        if set.insert(c) {
            insertions += 1;
        }
    }
    insertions
}

fn count_mutual(persons: &Vec<String>) -> usize {
    let mut map: HashMap<char, usize> = HashMap::new();
    for p in persons.iter() {
        for c in p.chars() {
            map.entry(c).and_modify(|n| *n += 1).or_insert(1);
        }
    }
    map.iter().fold(
        0,
        |acc, (_, &v)| if v == persons.len() { acc + 1 } else { acc },
    )
}

fn solve_2(input: &str) -> usize {
    let mut acc: Vec<String> = Vec::new();
    let mut groups: Vec<Vec<String>> = vec![];
    for line in input.lines() {
        if line.is_empty() {
            groups.push(acc.clone());
            acc.clear()
        } else {
            let persons: Vec<String> = line.split_whitespace().map(|s| s.to_owned()).collect();
            for person in persons {
                acc.push(person);
            }
        }
    }
    if acc.is_empty() == false {
        groups.push(acc);
    }

    groups.iter().fold(0, |acc, g| acc + count_mutual(g))
}

fn solve_1(input: &str) -> usize {
    let mut acc = String::new();
    let mut groups = vec![];
    for line in input.lines() {
        if line.is_empty() {
            groups.push(acc.clone());
            acc.clear();
        } else {
            let line = line.trim();
            acc.push_str(line);
        }
    }
    if acc.is_empty() == false {
        groups.push(acc);
    }

    groups.iter().fold(0, |acc, g| acc + count_unique(g))
}

pub fn answer_1() {
    let input = input();
    println!("day06 part1: {}", solve_1(&input));
}

pub fn answer_2() {
    let input = input();
    println!("day06 part2: {}", solve_2(&input));
}

#[test]
fn test1() {
    let input = "abc

    a
    b
    c

    ab
    ac

    a
    a
    a
    a

    b";
    assert_eq!(11, solve_1(input));
}

#[test]
fn test2() {
    let input = "abc

    a
    b
    c

    ab
    ac

    a
    a
    a
    a

    b";
    assert_eq!(6, solve_2(input));
}
