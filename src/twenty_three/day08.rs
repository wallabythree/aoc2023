use crate::Solution;
use std::clone::Clone;
use std::collections::HashMap;
use num::integer::lcm;

pub const SOLUTION: Solution<usize, usize> = Solution { part1, part2 };

pub const START: &str = "AAA";
pub const END: &str = "ZZZ";

struct Graph {
    nodes: HashMap<String, (String, String)>,
}

impl Graph {
    fn left(&self, node: &str) -> Option<&str> {
        Some(&self.nodes.get(node)?.0)
    }

    fn right(&self, node: &str) -> Option<&str> {
        Some(&self.nodes.get(node)?.1)
    }

    fn filter_nodes<F>(&self, predicate: F) -> Vec<&str>
        where F: Fn(&str) -> bool {

        self
            .nodes
            .keys()
            .filter(|label| predicate(label))
            .map(|label| label.as_str())
            .collect()
    }
}

struct DesertMap {
    instructions: Vec<u8>,
    graph: Graph,
}

impl DesertMap {
    fn moves<F>(
        &self,
        src: &str,
        dst_pred: F
    ) -> usize where F: Fn(&str) -> bool {
        
        let mut current = src;
        let mut moves = 0;

        while !dst_pred(current) {
            current = match self.instructions[moves % self.instructions.len()] {
                b'L' => self.graph.left(current).unwrap(),
                b'R' => self.graph.right(current).unwrap(),
                _ => panic!(),
            };

            moves += 1;
        }

        moves
    }

    fn moves_from_predicated<F, G>(
        &self,
        src_pred: F,
        dst_pred: G,
    ) -> Vec<usize>
    where F: Fn(&str) -> bool + Clone, G: Fn(&str) -> bool + Clone {
        let sources: Vec<_> = self.graph.filter_nodes(src_pred.clone());
            
        sources
            .iter()
            .map(|src| self.moves(src, dst_pred.clone()))
            .collect::<Vec<_>>()
    }
}

impl From<&str> for DesertMap {
    fn from(input: &str) -> Self {
        let (instructions, graph_desc) = input
            .split_once("\n\n")
            .map(|(l, r)| (l.as_bytes().to_vec(), r))
            .unwrap();

        let nodes = graph_desc
            .lines()
            .map(|l| (l[0..3].to_owned(), (l[7..10].to_owned(), l[12..15].to_owned())))
            .collect::<HashMap<_, _>>();

        let graph = Graph { nodes };

        Self { instructions, graph }
    }
}

fn part1(input: &str) -> usize {
    let desert_map = DesertMap::from(input);
    
    desert_map.moves(START, |node| node == END)
}

fn part2(input: &str) -> usize {
    let desert_map = DesertMap::from(input);

    desert_map
        .moves_from_predicated(
            |node: &str| node.ends_with('A'),
            |node: &str| node.ends_with('Z')
        )
        .iter()
        .fold(1, |moves, acc| lcm(moves, *acc))
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const TEST_INPUT_1: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

    const TEST_INPUT_2: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    const TEST_INPUT_3: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT_1), 2);
        assert_eq!(part1(TEST_INPUT_2), 6);
    }
    
    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT_3), 6);
    }
}

