mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;

const SOLVE_ALL: bool = false;

fn main() {
    if SOLVE_ALL {
        day01::answer_1();
        day01::answer_2();
        day02::answer_1();
        day02::answer_2();
        day03::answer_1();
        day03::answer_2();
        day04::answer_1();
        day04::answer_2();
        day05::answer_1();
        day05::answer_2();
        day06::answer_1();
        day06::answer_2();
        day07::answer_1();
        day07::answer_2();
        day08::answer_1();
        day08::answer_2();
    }
    day09::answer_1();
    day09::answer_2();
}
