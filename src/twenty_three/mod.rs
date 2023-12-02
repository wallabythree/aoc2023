use crate::Solution;

mod day00;
mod day01;
mod day02;
mod day03;

const SOLUTIONS: [&Solution; 4] = [
    &day00::SOLVER,
    &day01::SOLVER,
    &day02::SOLVER,
    &day03::SOLVER,
];

pub fn get_solution(day: usize) -> &'static Solution {
    SOLUTIONS[day]
}

