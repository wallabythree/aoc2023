use crate::Solution;

pub const SOLUTION: Solution<usize, usize> = Solution { part1, part2 };

#[derive(Clone, Copy, PartialEq, Eq)]
enum Rock {
    Round,
    Cube,
}

impl TryFrom<char> for Rock {
    type Error = ();

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'O' => Ok(Rock::Round),
            '#' => Ok(Rock::Cube),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

use Direction::{*};

#[derive(Clone, Copy)]
struct Position {
    x: isize,
    y: isize,
}

impl Position {
    fn adjacent(&self, dir: Direction) -> Self {
        match dir {
            North => Self { x: self.x, y: self.y - 1 },
            East => Self { x: self.x + 1, y: self.y },
            South => Self { x: self.x, y: self.y + 1 },
            West => Self { x: self.x - 1, y: self.y },
        }
    }
}

struct Platform {
    rows: Vec<Vec<Option<Rock>>>,
}

impl Platform {
    fn width(&self) -> usize {
        if self.rows.is_empty() {
            return 0;
        }

        self.rows[0].len()
    }

    fn height(&self) -> usize {
        self.rows.len()
    }

    fn validate_pos(&self, pos: Position) -> bool {
        pos.x >= 0
           && pos.x < self.width() as isize
           && pos.y >= 0
           && pos.y < self.height() as isize 
    }

    fn get(&self, pos: Position) -> Option<Rock> {
        if !self.validate_pos(pos) {
            panic!("Position outside of boundaries");
        }

        self.rows[pos.y as usize][pos.x as usize]
    }

    fn set(&mut self, pos: Position, val: Option<Rock>) {
        if !self.validate_pos(pos) {
            panic!("Position outside of boundaries");
        }

        self.rows[pos.y as usize][pos.x as usize] = val;
    }

    fn roll_rock(&mut self, mut rock: Position, dir: Direction) {
        if self.get(rock).is_none() || self.get(rock).unwrap() == Rock::Cube {
            return;
        }

        let mut adjacent = rock.adjacent(dir);

        while self.validate_pos(adjacent) && self.get(adjacent).is_none(){
            self.set(rock.adjacent(dir), self.get(rock));
            self.set(rock, None);

            rock = rock.adjacent(dir);
            adjacent = rock.adjacent(dir);
        }
    }

    fn tilt(&mut self, dir: Direction) {
        let outer: Box<dyn Iterator<Item = usize>> = match dir {
            South => Box::new((0..self.height()).rev()),
            _ => Box::new(0..self.height()),
        };

        for y in outer {
            let inner: Box<dyn Iterator<Item = usize>> = match dir {
                East => Box::new((0..self.width()).rev()),
                _ => Box::new(0..self.width()),
            };

            for x in inner {
                let pos = Position { x: x as isize, y: y as isize };
                self.roll_rock(pos, dir);
            }
        }
    }

    fn rock_load(&self, pos: Position, dir: Direction) -> usize {
        if !self.validate_pos(pos) {
            panic!("Position outside of boundaries");
        }

        if self.get(pos).is_none() || self.get(pos).unwrap() != Rock::Round {
            return 0;
        }

        match dir {
            North => self.height() - pos.y as usize,
            East => pos.x as usize + 1,
            South => pos.y as usize + 1,
            West => self.width() - pos.x as usize,
        }
    }

    fn load(&self, dir: Direction) -> usize {
        let mut load = 0;
        for y in 0..self.height() {
            for x in 0..self.width() {
                let pos = Position { x: x as isize, y: y as isize };
                load += self.rock_load(pos, dir);
            }
        }

        load
    }
}

impl TryFrom<&str> for Platform {
    type Error = ();

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let rows = input
            .lines()
            .map(|l| l
                 .chars()
                 .map(|c| match c {
                     '.' => None,
                     _ => Some(Rock::try_from(c).ok()?),
                 })
                 .collect()
            )
            .collect();

        Ok(Self { rows })
    }
}

fn part1(input: &str) -> usize {
    let mut platform = Platform::try_from(input).unwrap();

    platform.tilt(North);

    platform.load(North)
}

fn part2(input: &str) -> usize {
    let mut platform = Platform::try_from(input).unwrap();

    for _ in 0..1000 {
        platform.tilt(North);
        platform.tilt(West);
        platform.tilt(South);
        platform.tilt(East);
    }

    platform.load(North)
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const TEST_INPUT: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 136);
    }
    
    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 64);
    }
}

