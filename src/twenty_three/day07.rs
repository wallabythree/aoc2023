use crate::Solution;
use std::cmp::Ordering;

pub const SOLUTION: Solution<usize, usize> = Solution { part1, part2 };

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Clone)]
struct Hand {
    cards: [u8; 5],
    bid: usize
}

impl Hand {
    fn hand_type(&self) -> HandType {
        let mut counts = [0u8; 15];

        for card in self.cards {
            counts[card as usize] += 1;
        }

        let jokers = counts[1];
        let remaining_counts = &mut counts[2..];

        remaining_counts.sort();

        match counts[counts.len() - 1] + jokers {
            5 => HandType::FiveOfAKind,
            4 => HandType::FourOfAKind,
            3 => match counts[counts.len() - 2] {
                2 => HandType::FullHouse,
                _ => HandType::ThreeOfAKind,
            },
            2 => match counts[counts.len() - 2] {
                2 => HandType::TwoPair,
                _ => HandType::OnePair
            },
            _ => HandType::HighCard,
        }
    }
}


impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards.eq(&other.cards)
    }
}

impl Eq for Hand {}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.hand_type().cmp(&other.hand_type()) {
            Ordering::Equal => {
                let pairs = self
                    .cards
                    .iter()
                    .zip(other.cards.iter());

                for (l, r) in pairs {
                    if l != r {
                        return l.cmp(r);
                    }
                }

                Ordering::Equal
            },
            ordering => ordering,
        }
    }
}

impl TryFrom<&str> for Hand {
    type Error = ();

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let (cards_str, bid_str) = input.split_once(' ').ok_or(())?;

        let cards = cards_str
            .as_bytes()
            .iter()
            .map(|b| match b {
                b'1'..=b'9' => *b - b'0',
                b'T' => 10,
                b'J' => 11,
                b'Q' => 12,
                b'K' => 13,
                b'A' => 14,
                _ => panic!(),
            })
            .collect::<Vec<_>>()
            .try_into()
            .or(Err(()))?;

        let bid = bid_str.parse().or(Err(()))?;

        Ok(Self { cards, bid })
    }
}

struct Game {
    hands: Vec<Hand>,
}

impl Game {
    fn winnings(&self) -> usize {
        let mut ranking = self.hands.to_vec();
        ranking.sort();

        ranking
            .iter()
            .enumerate()
            .map(|(i, hand)| hand.bid * (i + 1))
            .sum()
    }
}

impl TryFrom<&str> for Game {
    type Error = ();

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let hands = input
            .lines()
            .map(|line| Hand::try_from(line).unwrap())
            .collect();

        Ok(Self { hands })
    }
}

fn part1(input: &str) -> usize {
    Game::try_from(input)
        .unwrap()
        .winnings()
}

fn part2(input: &str) -> usize {
    let input = input.replace('J', "1");

    Game::try_from(input.as_str())
        .unwrap()
        .winnings()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const TEST_INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 6440);
    }
    
    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 5905);
    }
}

