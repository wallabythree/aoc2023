use crate::Solution;

pub const SOLUTION: Solution<usize, usize> = Solution { part1, part2 };

fn hash(plaintext: &str) -> Result<u8, &str> {
    let mut hash = 0u8;

    for c in plaintext.chars() {
        if !c.is_ascii() {
            return Err("Plaintext contains non-ASCII characters");
        }

        hash = hash.wrapping_add(c as u8);
        hash = hash.wrapping_mul(17);
    }

    Ok(hash)
}

struct Lens {
    label: String,
    focal_length: usize,
}

impl PartialEq for Lens {
    fn eq(&self, other: &Self) -> bool {
        self.label.eq(&other.label)
    }
}

impl Eq for Lens {}

struct BoxMap {
    boxes: [Vec<Lens>; 256],
}

impl BoxMap {
    fn new() -> Self {
        Self { boxes: [(); 256].map(|_| Vec::new())  }
    }

    fn remove(&mut self, key: &str) {
        let hash = hash(key).unwrap();
        self.boxes[hash as usize].retain(|e| e.label != key);
    }

    fn insert(&mut self, lens: Lens) {
        let hash = hash(&lens.label).unwrap();

        let existing = self
            .boxes[hash as usize]
            .iter()
            .position(|l| l == &lens);


        if let Some(index) = existing {
            self.boxes[hash as usize][index] = lens;
        }

        else {
            self.boxes[hash as usize].push(lens);
        }
    }

    fn focusing_power(&self) -> usize {
        self
            .boxes
            .iter()
            .enumerate()
            .map(|(box_slot, lenses)| lenses
                 .iter()
                 .enumerate()
                 .map(|(lens_slot, lens)| {
                     (box_slot + 1) * (lens_slot + 1) * lens.focal_length
                 })
                 .sum::<usize>()
            )
            .sum()
    }
}

struct LavaMaker {
    box_map: BoxMap,
}

impl LavaMaker {
    fn new() -> Self {
        Self { box_map: BoxMap::new() }
    }

    fn execute(&mut self, instruction: &str) {
        let op = instruction.chars().find(|c| *c == '=' || *c == '-').unwrap();
        let (label, focal_length_str) = instruction.split_once(op).unwrap();
    
        match op {
            '=' => {
                let focal_length = focal_length_str.parse::<usize>().unwrap();
                self
                    .box_map
                    .insert(Lens { label: label.to_string(), focal_length })
            },
            '-' => self.box_map.remove(label),
            _ => panic!("Invalid operation")
        }
    }

    fn focusing_power(&self) -> usize {
        self.box_map.focusing_power()
    }
}

fn part1(input: &str) -> usize {
    input
        .trim()
        .split(',')
        .map(|instruction| hash(instruction).unwrap() as usize)
        .sum()
}

fn part2(input: &str) -> usize {
    let mut lava_maker = LavaMaker::new();

    for instr in input.trim().split(',') {
        lava_maker.execute(instr);
    }

    lava_maker.focusing_power()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const TEST_INPUT: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 1320);
    }
    
    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 145);
    }
}

