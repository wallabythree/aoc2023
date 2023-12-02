use crate::Solution;

pub const SOLVER: Solution = Solution { part1, part2 };

pub fn part1(input: &str) -> usize {
    input.len()
}

pub fn part2(input: &str) -> usize {
    input.len()
}

#[cfg(test)]
mod tests {
    use super::SOLVER;

    const TEST_INPUT: &str = "";

    #[test]
    fn test_part1() {
        assert_eq!(SOLVER.part1(TEST_INPUT), 0);
    }
    
    #[test]
    fn test_part2() {
        assert_eq!(SOLVER.part2(TEST_INPUT), TEST_INPUT.len());
    }
}

