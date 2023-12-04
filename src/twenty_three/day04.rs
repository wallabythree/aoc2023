use crate::Solution;

pub const SOLUTION: Solution<usize, usize> = Solution { part1, part2 };

struct Card {
    winning: Vec<usize>,
    owned: Vec<usize>,
}

impl Card {
    fn from(description: &str) -> Self {
        let (winning_str, owned_str) = description
            .split_once(':')
            .map(|(_, r)| r.split_once('|').unwrap())
            .unwrap();

        let winning = winning_str
            .split_whitespace()
            .map(|n| n.parse::<usize>().unwrap())
            .collect();

        let owned = owned_str
            .split_whitespace()
            .map(|n| n.parse::<usize>().unwrap())
            .collect();

        Self { winning, owned }
    }

    fn matches(&self) -> usize {
        self
            .owned
            .iter()
            .filter(|n| self.winning.contains(n))
            .count()
    }

    fn points(&self) -> usize {
        let matches = self.matches();

        if matches > 0 {
            2usize.pow(matches as u32 - 1)
        }
        else {
            0
        }
    }
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .map(Card::from)
        .map(|card| card.points())
        .sum()
}

fn part2(input: &str) -> usize {
    let cards: Vec<_> = input
        .lines()
        .map(Card::from)
        .collect();

    let mut card_count: Vec<usize> = cards
        .iter()
        .map(|_| 1)
        .collect();

    for (i, card) in cards.iter().enumerate() {
        let matches = card.matches();

        for m in 0..matches {
            card_count[i + 1 + m] += card_count[i];
        }
    }

    card_count.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const TEST_INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 13);
    }
    
    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 30);
    }
}

