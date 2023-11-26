use std::fs;

pub fn get_input() -> Vec<usize> {
    let input = fs::read_to_string("input/day01.txt").unwrap();
    input
        .lines()
        .into_iter()
        .map(|line| line.parse().unwrap())
        .collect()
}

pub fn solve_1(input: Vec<usize>, wanted: usize) -> Option<usize> {
    let mut skip = 1;
    for n in input.iter() {
        for n2 in input.iter().skip(skip) {
            if n + n2 == wanted {
                return Some(n * n2);
            }
        }
        skip += 1;
    }
    None
}

fn solve_2(input: Vec<usize>, wanted: usize) -> Option<usize> {
    for n in input.iter() {
        for n2 in input.iter() {
            for n3 in input.iter() {
                if n == n2 || n == n3 || n2 == n3 {
                    continue;
                }
                if n + n2 <= wanted {
                    if n + n2 + n3 == wanted {
                        return Some(n * n2 * n3);
                    }
                }
            }
        }
    }
    None
}

pub fn answer_1() {
    let input = get_input();
    println!("day01 part1: {:?}", solve_1(input, 2020));
}

pub fn answer_2() {
    let input = get_input();
    println!("day01 part2: {:?}", solve_2(input, 2020));
}

#[test]
fn test1() {
    let input = vec![1721, 979, 366, 299, 675, 1456];
    let result = solve_1(input, 2020);
    assert_eq!(Some(514579), result);
}

#[test]
fn test2() {
    let input = vec![1721, 979, 366, 299, 675, 1456];
    let result = solve_2(input, 2020);
    assert_eq!(Some(241861950), result);
}
