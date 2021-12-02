use aoc_shared::{tests, Day};
use std::collections::HashMap;

pub struct Day10;

impl Day for Day10 {
    const NUMBER: u8 = 10;
    type Input = Vec<u16>;
    type Answer = u16;

    fn parse_input(&self, input: &str) -> Vec<u16> {
        let lines = input.lines();
        let mut input = Vec::new();

        for line in lines {
            input.push(line.parse().unwrap());
        }

        input.sort_unstable();

        input.insert(0, 0);
        input.push(input[input.len() - 1] + 3);

        input
    }

    fn part_one(&self, input: Vec<u16>) -> u16 {
        let mut differences: HashMap<u16, u16> = HashMap::new();

        for x in input.windows(2) {
            *differences.entry(x[1] - x[0]).or_default() += 1;
        }

        differences.get(&3).unwrap() * differences.get(&1).unwrap()
    }

    fn part_two(&self, input: Vec<u16>) -> u16 {
        todo!();
    }
}

tests!(Day10, "10", 220, 0);
