// solution template

use crate::Solution;

pub const SOLUTION: Solution<isize, isize> = Solution { part1, part2 };

struct Sensor {
    readings: Vec<isize>,
}

impl From<&str> for Sensor {
    fn from(input: &str) -> Sensor {
        let readings: Vec<_> = input
            .split_whitespace()
            .map(|reading| reading.parse::<isize>().unwrap())
            .collect();

        Self { readings }
    }
}

impl Sensor {
    fn _extrapolate(readings: &[isize]) -> isize {
        if readings.iter().all(|r| *r == 0) {
            return 0;
        }

        let deltas = readings
            .windows(2)
            .map(|pair| pair[1] - pair[0])
            .collect::<Vec<_>>();

        readings[readings.len() - 1] + Self::_extrapolate(deltas.as_slice())
    }

    fn extrapolate(&self) -> isize {
        Self::_extrapolate(self.readings.as_slice())
    }

    fn _extrapolate_backwards(readings: &[isize]) -> isize {
        if readings.iter().all(|r| *r == 0) {
            return 0;
        }

        let deltas = readings
            .windows(2)
            .map(|pair| pair[1] - pair[0])
            .collect::<Vec<_>>();

        readings[0] - Self::_extrapolate_backwards(deltas.as_slice())
    }

    fn extrapolate_backwards(&self) -> isize {
        Self::_extrapolate_backwards(self.readings.as_slice())
    }
}

fn part1(input: &str) -> isize {
    input
        .lines()
        .map(|l| Sensor::from(l).extrapolate())
        .sum()
}

fn part2(input: &str) -> isize {
    input
        .lines()
        .map(|l| Sensor::from(l).extrapolate_backwards())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const TEST_INPUT: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 114);
    }
    
    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 2);
    }
}

