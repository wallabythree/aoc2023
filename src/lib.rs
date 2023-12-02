mod twenty_three;

pub struct Solution {
    part1: fn(&str) -> usize,
    part2: fn(&str) -> usize,
}

impl Solution {
    pub fn part1(&self, input: &str) -> usize {
        (self.part1)(input)
    }

    pub fn part2(&self, input: &str) -> usize {
        (self.part2)(input)
    }
}

pub fn get_solution(year: usize, day: usize) -> &'static Solution {
    match year {
        2023 => twenty_three::get_solution(day),
        _ => panic!(),
    }
}

