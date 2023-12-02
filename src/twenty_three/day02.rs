use crate::Solution;

pub const SOLVER: Solution = Solution { part1, part2 };

const MAX_RED: usize = 12;
const MAX_GREEN: usize = 13;
const MAX_BLUE: usize = 14;

struct Game {
    id: usize,
    min_red: usize,
    min_green: usize,
    min_blue: usize,
}

impl Game {
    fn from(description: &str) -> Self {
        let (id_str, rounds_str) = description.split_once(':').unwrap();

        let id = id_str[5..].parse::<usize>().unwrap();
        let rounds = rounds_str.split(';');

        let (mut min_red, mut min_green, mut min_blue) = (0, 0, 0);

        for round in rounds {
            let color_counts = round.split(',');

            for color_count in color_counts {
                let (count_str, color) = color_count[1..]
                    .split_once(' ')
                    .unwrap();
                
                let count = count_str.parse::<usize>().unwrap();

                match color {
                    "red" => min_red = min_red.max(count),
                    "green" => min_green = min_green.max(count),
                    "blue" => min_blue = min_blue.max(count),
                    _ => panic!(),
                };
            }
        }

        Self { id, min_red, min_green, min_blue }
    }

    fn possible(
        &self,
        max_red: usize,
        max_green: usize,
        max_blue: usize
    ) -> bool {
        self.min_red <= max_red
            && self.min_green <= max_green
            && self.min_blue <= max_blue
    }

    fn power(&self) -> usize {
        self.min_red * self.min_green * self.min_blue
    }
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .map(Game::from)
        .filter(|game| game.possible(MAX_RED, MAX_GREEN, MAX_BLUE))
        .map(|game| game.id)
        .sum()
}

fn part2(input: &str) -> usize {
    input
        .lines()
        .map(|l| Game::from(l).power())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::SOLVER;

    const TEST_INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn test_part1() {
        assert_eq!(SOLVER.part1(TEST_INPUT), 8);
    }
    
    #[test]
    fn test_part2() {
        assert_eq!(SOLVER.part2(TEST_INPUT), 2286);
    }
}

