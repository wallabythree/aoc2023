use std::collections::hash_map::HashMap;

/// Returns the sum of all calibration values. Simple forward and backward
/// scanning to find the first and last digit in each line respectively.
pub fn part1(input: &str) -> usize {
    let mut sum = 0;
    
    for line in input.lines() {
        for c in line.chars() {
            if let Some(digit) = c.to_digit(10) {
                sum += digit as usize * 10;
                break;
            }
        }

        for c in line.chars().rev() {
            if let Some(digit) = c.to_digit(10) {
                sum += digit as usize;
                break;
            }
        }
    }

    sum
}

#[derive(PartialEq)]
enum Direction {
    Forward,
    Backward
}

use Direction::*;

/// Find the first instance in the given direction of an element in a set of
/// needles in a haystack.
fn find<'a>(
    haystack: &'a [u8],
    needles: &'a [&'a [u8]],
    dir: Direction,
) -> Option<&'a [u8]> {
    let mut cursor = haystack;

    while !cursor.is_empty() {
        for needle in needles {
            if (dir == Forward && cursor.starts_with(needle)) ||
               (dir == Backward && cursor.ends_with(needle)) {
                return Some(needle);
            }
        }

        match dir {
            Forward => cursor = &cursor[1..],
            Backward => cursor = &cursor[..cursor.len() - 1],
        }
    }

    None
}

/// These values are what we consider to be valid representations of digits in
/// our target language.
const LEXEMES: [&[u8]; 20] = [
    b"0", b"1", b"2", b"3", b"4", b"5", b"6", b"7", b"8", b"9", b"zero", b"one",
    b"two", b"three", b"four", b"five", b"six", b"seven", b"eight", b"nine",
];

/// Returns the sum of all calibration values, where digits may be given as
/// simple numbers or spelled out as letters. Scans in forward and backward
/// direction to find the first and last digit in each line respectively.
pub fn part2(input: &str) -> usize {

    // assign a value to each forward lexeme equal to lexeme_position % 10
    let tokens: HashMap<_,_> = LEXEMES
        .iter()
        .enumerate()
        .map(|(i, e)| (e, i % 10))
        .collect();

    let mut sum = 0;

    for line in input.lines() {
        let first_lexeme = find(line.as_bytes(), &LEXEMES, Forward).unwrap();
        let last_lexeme = find(line.as_bytes(), &LEXEMES, Backward).unwrap();

        // retrieve the value associated with our lexemes
        let first = tokens.get(&first_lexeme).unwrap();
        let last = tokens.get(&last_lexeme).unwrap();

        // [T]: You do the math.
        // [J]: NO!
        sum += first * 10 + last;
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const TEST_INPUT_1: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    const TEST_INPUT_2: &str ="two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT_1), 142);
    }
    
    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT_2), 281);
    }
}

