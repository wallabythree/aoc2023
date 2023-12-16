use crate::Solution;
use std::collections::{ HashMap, HashSet, VecDeque };

pub const SOLUTION: Solution<usize, usize> = Solution { part1, part2 };

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

use Direction::{*};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: isize,
    y: isize,
}

impl Position {
    fn neighbour(&self, dir: Direction) -> Self {
        let (x, y) = match dir {
            North => (self.x, self.y - 1),
            East => (self.x + 1, self.y),
            South => (self.x, self.y + 1),
            West => (self.x - 1, self.y),
        };

        Self { x, y }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Photon {
    pos: Position,
    dir: Direction,
}

impl Photon {
    fn travel(&mut self) {
        self.pos = self.pos.neighbour(self.dir);
    }
}

#[derive(Debug)]
enum Tile {
    Empty,
    AcuteMirror,
    ObtuseMirror,
    VerticalSplitter,
    HorizontalSplitter,
}

impl Tile {
    fn collide(&self, mut photon: Photon) -> Vec<Photon> {
        let mut next_photons = vec!();

        match self {
            Tile::Empty => {
                photon.travel();
                next_photons.push(photon);
            },
            Tile::AcuteMirror => {
                let next_dir = match photon.dir {
                    North => East,
                    East => North,
                    South => West,
                    West => South,
                };

                photon.dir = next_dir;
                photon.travel();
                next_photons.push(photon);
            },
            Tile::ObtuseMirror => {
                let next_dir = match photon.dir {
                    North => West,
                    East => South,
                    South => East,
                    West => North,
                };

                photon.dir = next_dir;
                photon.travel();
                next_photons.push(photon);
            },
            Tile::VerticalSplitter => {
                match photon.dir {
                    East | West => {
                        let mut north = photon.clone();
                        north.dir = North;
                        north.travel();

                        let mut south = photon.clone();
                        south.dir = South;
                        south.travel();

                        next_photons.push(north);
                        next_photons.push(south);
                    }
                    North | South => {
                        photon.travel();
                        next_photons.push(photon);
                    }
                }
            }
            Tile::HorizontalSplitter => {
                match photon.dir {
                    North | South => {
                        let mut west = photon.clone();
                        west.dir = West;
                        west.travel();

                        let mut east = photon.clone();
                        east.dir = East;
                        east.travel();

                        next_photons.push(west);
                        next_photons.push(east);
                    }
                    East | West => {
                        photon.travel();
                        next_photons.push(photon);
                    }
                }
            }
        }

        next_photons
    }
}

impl TryFrom<char> for Tile {
    type Error = ();

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '.' => Ok(Tile::Empty),
            '/' => Ok(Tile::AcuteMirror),
            '\\' => Ok(Tile::ObtuseMirror),
            '|' => Ok(Tile::VerticalSplitter),
            '-' => Ok(Tile::HorizontalSplitter),
            _ => Err(())
        }
    }
}

#[derive(Debug)]
struct Cave {
    rows: Vec<Vec<Tile>>,
}

impl Cave {
    fn width(&self) -> usize {
        if self.rows.len() == 0 {
            return 0;
        }

        self.rows[0].len()
    }

    fn height(&self) -> usize {
        self.rows.len()
    }

    fn validate_pos(&self, pos: &Position) -> bool {
        pos.x >= 0
            && pos.x < self.width() as isize
            && pos.y >= 0
            && pos.y < self.height() as isize
    }

    fn validate_photon(&self, photon: &Photon) -> bool {
        self.validate_pos(&photon.pos)
    }

    fn next_photons(&self, photon: Photon) -> Vec<Photon> {
        if !self.validate_photon(&photon) {
            panic!("Invalid photon");
        }

        self
            .rows[photon.pos.y as usize][photon.pos.x as usize]
            .collide(photon)
    }

    fn dfs(
        &self,
        photon: Photon,
        visited: &mut HashMap<Photon, usize>
    ) -> usize {
        if !self.validate_photon(&photon) {
            return 0;
        }

        if let Some(distance) = visited.get(&photon) {
            return *distance;
        }

        let next_photons = self.next_photons(photon);

        let mut distances = vec!();

        for next_photon in next_photons {
            let distance = self.dfs(next_photon, visited);
            distances.push(distance);
        }

        let distance = distances.iter().sum::<usize>() + 1;
        visited.insert(photon, distance);

        distance
    }

    fn bfs(&self, src: Photon, visited: &mut Vec<Photon>) -> usize {
        let mut energized = HashSet::new();

        let mut queue = VecDeque::from([src]);

        while let Some(photon) = queue.pop_front() {
            //println!("Photon: {:?}", photon);

            visited.push(photon);
            energized.insert(photon.pos);

            let next_photons = self.next_photons(photon);

            for next_photon in next_photons {
                if !self.validate_photon(&next_photon) || visited.contains(&next_photon) {
                    continue;
                }

                queue.push_back(next_photon);
            }
        }



        energized.len()
    }

    fn energized(&self) -> usize {
        let spark = Photon { pos: Position { x: 0, y: 0 }, dir: East };
        let mut visited = vec!();// HashMap::new();

        let energized = self.bfs(spark, &mut visited);

        for y in 0..self.height() {
            for x in 0..self.width() {
                if visited.iter().any(|key| key.pos == Position { x: x as isize, y: y as isize }) {
                    print!("#");
                }
                else {
                    print!(".");
                }
            }
            println!("");
        }

        energized
    }

    fn max_energized(&self) -> usize {
        let mut possibilities = vec!();

        for x in 0..self.width() {
            let north = Position { x: x as isize, y: 0 };
            let south = Position {
                x: x as isize,
                y: self.height() as isize - 1,
            };

            possibilities.push(Photon { pos: north, dir: South });
            possibilities.push(Photon { pos: south, dir: North });
        }

        for y in 0..self.height() {
            let west = Position { x: 0, y: y as isize };
            let east = Position {
                x: self.width() as isize - 1,
                y: y as isize,
            };

            possibilities.push(Photon { pos: west, dir: East });
            possibilities.push(Photon { pos: east, dir: West});
        }


        possibilities
            .iter()
            .map(|spark| self.bfs(*spark, &mut vec!()))
            .max()
            .unwrap()
    }
}

impl TryFrom<&str> for Cave {
    type Error = ();

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let rows: Result<Vec<_>, _> = input
            .lines()
            .map(|l| l.chars().map(Tile::try_from).collect())
            .collect();

        Ok(Self { rows: rows? })
    }
}

fn part1(input: &str) -> usize {
    let cave = Cave::try_from(input).unwrap();

    cave.energized()
}

fn part2(input: &str) -> usize {
    let cave = Cave::try_from(input).unwrap();

    cave.max_energized()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const TEST_INPUT: &str = ".|...\\....
|.-.\\.....
.....|-...
........|.
..........
.........\\
..../.\\\\..
.-.-/..|..
.|....-|.\\
..//.|....";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 46);
    }
    
    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 51);
    }
}

