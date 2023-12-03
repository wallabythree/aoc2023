use crate::Solution;
use regex::Regex;

pub const SOLUTION: Solution<usize, usize> = Solution { part1, part2 };

struct Pos {
    x: i64,
    y: i64,
}

impl Pos {
    fn from_offset(offset: usize, line_len: usize) -> Self {
        Self {
            x: (offset % line_len) as i64,
            y: (offset.div_euclid(line_len)) as i64
        }
    }
}

struct Part<'a> {
    num_str: &'a str,
    pos: Pos
}

impl Part<'_> {
    fn num(&self) -> usize {
        self.num_str.parse::<usize>().unwrap()
    }

    fn adjacent(&self, pos: &Pos) -> bool {
        pos.x >= self.pos.x - 1
            && pos.x <= self.pos.x + self.num_str.len() as i64
            && pos.y >= self.pos.y - 1
            && pos.y <= self.pos.y + 1
    }
}

fn part1(input: &str) -> usize {
    let line_len = input.find('\n').unwrap();
    let stripped = input.replace('\n', "");

    let symbols: Vec<_> = stripped
        .char_indices()
        .filter(|(_, c)| *c != '.' && !c.is_ascii_alphanumeric()) //find symbols
        .map(|(i, _)| Pos::from_offset(i, line_len))
        .collect();

    let re = Regex::new(r"[0-9]+").unwrap(); // find possible parts

    re
        .find_iter(&stripped)
        .map(|m| {
            Part { 
                num_str: m.as_str(),
                pos: Pos::from_offset(m.start(), line_len)
            }
        })
        .filter(|part| {
            // select parts that are adjacent to a symbol
           symbols 
                .iter()
                .any(|sym| part.adjacent(sym))
        })
        .map(|part| part.num())
        .sum()
}

fn part2(input: &str) -> usize {
    let line_len = input.find('\n').unwrap();
    let stripped = input.replace('\n', "");

    let gears: Vec<_> = stripped
        .match_indices('*')  // find gears
        .map(|(i, _)| Pos::from_offset(i, line_len))
        .collect();

    let re = Regex::new(r"[0-9]+").unwrap();

    let parts: Vec<_> = re
        .find_iter(&stripped) // find possible parts
        .map(|m| {
            Part { 
                num_str: m.as_str(),
                pos: Pos::from_offset(m.start(), line_len)
            }
        })
        .collect();

    gears
        .iter()
        .map(|gear| {
            // find any parts attached to the gear
            let adjacent_parts: Vec<_> = parts
                .iter()
                .filter(|part| part.adjacent(gear))
                .collect();

            // verify that there are exactly two parts attached to the gear
            if adjacent_parts.len() == 2 {
                adjacent_parts.iter().map(|part| part.num()).product()
            }
            else {
                0
            }
        })
        .sum()
    }

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const TEST_INPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 4361);
    }
    
    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 467835);
    }
}

