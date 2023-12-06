// solution template

use crate::Solution;

pub const SOLUTION: Solution<usize, usize> = Solution { part1, part2 };

fn solve_quadratic(a: f64, b: f64, c: f64) -> Option<(f64, f64)> {
    let discriminant = b.powi(2) - 4.0 * a * c;

    if discriminant < 0.0 {
        return None;
    }

    let x = (
        (-b + discriminant.sqrt()) / 2.0 * a,
        (-b - discriminant.sqrt()) / 2.0 * a,
    );

    Some(x)
}

#[derive(Debug)]
struct Race {
    time: isize,
    record: isize,
}

impl Race {
    fn winning_times(&self) -> (usize, usize) {
        let times = solve_quadratic(
            -1.0,
            self.time as f64,
            -self.record as f64
        ).unwrap();

        (times.0.floor() as usize + 1, times.1.ceil() as usize)
    }

    fn winning_ways(&self) -> usize {
        let winning_interval = self.winning_times();

        winning_interval .1 - winning_interval.0
    }
}

fn part1(input: &str) -> usize {
    let pairs = input
        .lines()
        .map(|l| l
             .split_whitespace()
             .skip(1)
             .map(|n| n.parse::<isize>().unwrap())
             .collect::<Vec<_>>()
        )
        .collect::<Vec<_>>();

    pairs[0]
        .iter()
        .zip(&pairs[1])
        .map(|r| Race { time: *r.0, record: *r.1 }.winning_ways())
        .product()
}

fn part2(input: &str) -> usize {
    let race_data: Vec<isize> = input
        .replace(' ', "")
        .lines()
        .map(|l| l
             .split_once(':')
             .unwrap()
             .1
             .parse::<isize>()
             .unwrap()
        )
        .collect();

    let race = Race { time: race_data[0], record: race_data[1] };

    race.winning_ways()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const TEST_INPUT: &str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 288);
    }
    
    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 71503);
    }
}

