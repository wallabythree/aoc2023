use crate::Solver;

mod day00;
mod day01;
mod day02;
mod day03;
mod day04;

const SOLUTIONS: [&dyn Solver; 5] = [
    &day00::SOLUTION,
    &day01::SOLUTION,
    &day02::SOLUTION,
    &day03::SOLUTION,
    &day04::SOLUTION,
];

pub fn get_solution(day: usize) -> &'static dyn Solver {
    SOLUTIONS[day]
}

