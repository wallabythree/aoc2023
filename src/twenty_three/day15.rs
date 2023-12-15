use crate::Solution;
use std::hash::{BuildHasher, Hash, Hasher};

pub const SOLUTION: Solution<usize, usize> = Solution { part1, part2 };

struct LavaMap<K: Hash + PartialEq, V, B: BuildHasher> {
    build_hasher: B,
    buckets: Vec<Vec<(K, V)>>,
}

impl<K: Hash + PartialEq, V, B: BuildHasher> LavaMap<K, V, B> {
    const DEFAULT_CAPACITY: usize = 256;

    fn new(build_hasher: B) -> Self {
        let buckets = (0..Self::DEFAULT_CAPACITY)
            .map(|_| vec!())
            .collect();

        Self { build_hasher, buckets }
    }

    fn calculate_bucket(&self, key: &K) -> usize {
        let mut hasher = self.build_hasher.build_hasher();
        key.hash(&mut hasher);

        (hasher.finish() % self.buckets.capacity() as u64) as usize
    }

    fn remove(&mut self, key: K) -> Option<V> {
        let bucket = self.calculate_bucket(&key);
        
        let position = self.buckets[bucket].iter().position(|(k, _)| *k == key);

        if let Some(index) = position {
            Some(self.buckets[bucket].remove(index).1)
        }
        else {
            None
        }
    }

    fn insert(&mut self, key: K, value: V) -> Option<V> {
        let bucket = self.calculate_bucket(&key);

        let position = self.buckets[bucket].iter().position(|(k, _)| *k == key);

        if let Some(index) = position {
            let replaced = std::mem::replace(
                &mut self.buckets[bucket][index], (key, value)
            );

            Some(replaced.1)
        }
        else {
            self.buckets[bucket].push((key, value));

            None
        }
    }
}

struct BuildLavaHasher;

impl BuildHasher for BuildLavaHasher {
    type Hasher = LavaHasher;

    fn build_hasher(&self) -> LavaHasher {
        LavaHasher { state: 0 }
    }
}

struct LavaHasher {
    state: u64,
}

impl Hasher for LavaHasher {
    fn write(&mut self, bytes: &[u8]) {
        for b in bytes {
            self.state = self.state.wrapping_add(*b as u64);
            self.state = self.state.wrapping_mul(17);
        }
    }

    fn finish(&self) -> u64 {
        self.state
    }
}

#[derive(PartialEq)]
struct LavaSliceKey<'a, K: Hash + PartialEq>(&'a [K]);

impl<K: Hash + PartialEq> Hash for LavaSliceKey<'_, K> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for e in self.0 {
            e.hash(state);
        }
    }
}

struct LavaMaker<'a> {
    lava_map: LavaMap<LavaSliceKey<'a, u8>, usize, BuildLavaHasher>,
}

impl<'a> LavaMaker<'a> {
    fn new() -> Self {
        Self { lava_map: LavaMap::new(BuildLavaHasher {}) }
    }

    fn execute(&mut self, instruction: &'a str) {
        let op = instruction.chars().find(|c| *c == '=' || *c == '-').unwrap();
        let (label, focal_length_str) = instruction.split_once(op).unwrap();
        let key = LavaSliceKey(label.as_bytes());
    
        match op {
            '=' => {
                let focal_length = focal_length_str.parse::<usize>().unwrap();
                self
                    .lava_map
                    .insert(key, focal_length);
            },
            '-' => { self.lava_map.remove(key); },
            _ => panic!("Invalid operation"),
        }
    }

    fn focusing_power(&self) -> usize {
        self
            .lava_map
            .buckets
            .iter()
            .enumerate()
            .map(|(box_slot, lenses)| lenses
                 .iter()
                 .enumerate()
                 .map(|(lens_slot, lens)| {
                     (box_slot + 1) * (lens_slot + 1) * lens.1
                 })
                 .sum::<usize>()
            )
            .sum()
    }
}

fn part1(input: &str) -> usize {
    let builder = BuildLavaHasher {};

    input
        .trim()
        .split(',')
        .map(|instruction| {
            let mut hasher = builder.build_hasher();
            let key = LavaSliceKey(instruction.as_bytes());
            key.hash(&mut hasher);
            hasher.finish() as usize % 256
        })
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

