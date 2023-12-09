use crate::Solver;

mod day00;
mod day01;
mod day02;
mod day03;
mod day04;
mod day06;
mod day08;
mod day09;

const SOLUTIONS: [&dyn Solver; 10] = [
    &day00::SOLUTION,
    &day01::SOLUTION,
    &day02::SOLUTION,
    &day03::SOLUTION,
    &day04::SOLUTION,
    &day00::SOLUTION, // TODO: implement domain flatmap solution
    &day06::SOLUTION,
    &day00::SOLUTION,
    &day08::SOLUTION,
    &day09::SOLUTION,
];

pub fn get_solution(day: usize) -> &'static dyn Solver {
    SOLUTIONS[day]
}

