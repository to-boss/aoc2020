use std::fs;

fn input() -> String {
    fs::read_to_string("input/day05.txt").unwrap()
}

fn solve_1(input: &str) -> usize {
    let mut rows: Vec<usize> = (0..128).into_iter().collect();
    let mut cols: Vec<usize> = (0..8).into_iter().collect();
    let (first, second) = input.split_at(7);

    for c in first.chars() {
        let (left, right) = rows.split_at(rows.len() / 2);
        rows = match c {
            'F' => left.to_vec(),
            'B' => right.to_vec(),
            _ => panic!("Should never happen!"),
        };
    }

    for c in second.chars() {
        let (left, right) = cols.split_at(cols.len() / 2);
        cols = match c {
            'R' => right.to_vec(),
            'L' => left.to_vec(),
            _ => panic!("Should never happen!"),
        };
    }

    rows[0] * 8 + cols[0]
}

pub fn answer_1() {
    let input = input();
    let seats: Vec<_> = input.lines().map(|l| solve_1(l)).collect();
    let max_id = seats.into_iter().max().unwrap();

    println!("day05 part1: {}", max_id);
}

pub fn answer_2() {
    let input = input();
    let mut seats: Vec<_> = input.lines().map(|l| solve_1(l)).collect();
    seats.sort();

    for (i, val) in seats.iter().enumerate() {
        if let Some(&val2) = seats.get(i + 1) {
            if val + 1 != val2 {
                println!("day05 part2: {}", val + 1);
            }
        }
    }
}

#[test]
fn test1() {
    let input = "FBFBBFFRLR";
    assert_eq!(357, solve_1(input));
}
