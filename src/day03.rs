use std::fs;

struct World {
    x_pos: usize,
    y_pos: usize,
    map: Vec<Vec<Cell>>,
    width: usize,
    height: usize,
}

impl World {
    pub fn new(map: Vec<Vec<Cell>>) -> Self {
        let width = map.get(0).unwrap().len();
        let height = map.len();
        World {
            x_pos: 0,
            y_pos: 0,
            map,
            width,
            height,
        }
    }

    pub fn check_slope(&mut self, right: usize, down: usize) -> usize {
        let mut tree_count = 0;
        while self.reached_bottom() == false {
            self.move_right(right);
            self.move_down(down);
            if self.is_at_tree() {
                tree_count += 1;
            }
        }
        self.reset();
        tree_count
    }

    fn reset(&mut self) {
        self.x_pos = 0;
        self.y_pos = 0;
    }

    pub fn is_at_tree(&self) -> bool {
        match self.map.get(self.y_pos).unwrap().get(self.x_pos).unwrap() {
            Cell::Empty => false,
            Cell::Tree => true,
        }
    }

    fn reached_bottom(&self) -> bool {
        self.y_pos == self.height - 1
    }

    fn move_right(&mut self, amount: usize) {
        self.x_pos += amount;
        while self.x_pos > self.width - 1 {
            self.x_pos -= self.width;
        }
    }

    fn move_down(&mut self, amount: usize) {
        self.y_pos += amount;
    }

    pub fn _print(&self) {
        for (y, row) in self.map.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                let curr_pos = self.x_pos == x && self.y_pos == y;
                match (cell, curr_pos) {
                    (Cell::Empty, true) => print!("O"),
                    (Cell::Empty, false) => print!("."),
                    (Cell::Tree, true) => print!("X"),
                    (Cell::Tree, false) => print!("#"),
                }
            }
            println!("");
        }
        println!("")
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Cell {
    Empty,
    Tree,
}

impl Cell {
    fn from_char(c: char) -> anyhow::Result<Cell> {
        let res = match c {
            '.' => Cell::Empty,
            '#' => Cell::Tree,
            _ => panic!("Bad input map!"),
        };
        Ok(res)
    }
}

pub fn input() -> Vec<Vec<Cell>> {
    let input = fs::read_to_string("input/day03.txt").unwrap();
    input
        .lines()
        .into_iter()
        .map(|line| line.chars().map(|c| Cell::from_char(c).unwrap()).collect())
        .collect()
}

pub fn solve_1(input: Vec<Vec<Cell>>) -> usize {
    let mut world = World::new(input);
    world.check_slope(3, 1)
}

pub fn solve_2(input: Vec<Vec<Cell>>) -> usize {
    let mut world = World::new(input);
    let rights: Vec<usize> = vec![1, 3, 5, 7, 1];
    let downs: Vec<usize> = vec![1, 1, 1, 1, 2];
    let tuple_iter = rights.iter().zip(downs.iter());
    tuple_iter.fold(1, |acc, (&right, &down)| {
        acc * world.check_slope(right, down)
    })
}

pub fn answer_1() {
    let input = input();
    println!("day03 part1: {}", solve_1(input));
}

pub fn answer_2() {
    let input = input();
    println!("day03 part2: {}", solve_2(input));
}

#[test]
fn test1() {
    let input = vec![
        "..##.......",
        "#...#...#..",
        ".#....#..#.",
        "..#.#...#.#",
        ".#...##..#.",
        "..#.##.....",
        ".#.#.#....#",
        ".#........#",
        "#.##...#...",
        "#...##....#",
        ".#..#...#.#",
    ];

    let parsed_input: Vec<Vec<Cell>> = input
        .into_iter()
        .map(|line| line.chars().map(|c| Cell::from_char(c).unwrap()).collect())
        .collect();
    assert_eq!(7, solve_1(parsed_input));
}

#[test]
fn test2() {
    let input = vec![
        "..##.......",
        "#...#...#..",
        ".#....#..#.",
        "..#.#...#.#",
        ".#...##..#.",
        "..#.##.....",
        ".#.#.#....#",
        ".#........#",
        "#.##...#...",
        "#...##....#",
        ".#..#...#.#",
    ];

    let parsed_input: Vec<Vec<Cell>> = input
        .into_iter()
        .map(|line| line.chars().map(|c| Cell::from_char(c).unwrap()).collect())
        .collect();
    assert_eq!(336, solve_2(parsed_input));
}
