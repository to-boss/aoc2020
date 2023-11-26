use std::fs;

fn check_possible(preamble: &[i64], num: i64) -> bool {
    for x in preamble {
        for y in preamble {
            if x == y {
                continue;
            }
            if x + y == num {
                return true;
            }
        }
    }
    false
}

fn solve_1(preamble_len: usize, input: &str) -> Option<i64> {
    let mut split_point = preamble_len;
    let mut start_idx: usize = 0;

    let nums: Vec<i64> = input.lines().map(|l| l.trim().parse().unwrap()).collect();

    while split_point < nums.len() {
        let (preamble, rest) = nums.split_at(split_point);

        let curr = rest[0];
        if check_possible(&preamble[start_idx..], curr) == false {
            return Some(curr);
        }

        start_idx += 1;
        split_point += 1;
    }
    None
}

pub fn answer_1() {
    let input = input();
    println!("day09 part1: {}", solve_1(25, &input).unwrap());
}

fn input() -> String {
    fs::read_to_string("input/day09.txt").unwrap()
}

#[test]
fn test1() {
    let input = "35
    20
    15
    25
    47
    40
    62
    55
    65
    95
    102
    117
    150
    182
    127
    219
    299
    277
    309
    576";
    assert_eq!(127, solve_1(5, input).unwrap());
}
