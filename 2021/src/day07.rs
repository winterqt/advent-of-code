use aoc_shared::{tests, Day};
use itertools::Itertools;

pub struct Day07;

impl Day for Day07 {
    const NUMBER: u8 = 7;
    type Input = Vec<u16>;
    type Answer = usize;

    fn parse_input(&self, input: &str) -> Vec<u16> {
        input
            .split(',')
            .map(str::parse)
            .map(Result::unwrap)
            .collect()
    }

    fn part_one(&self, mut input: Vec<u16>) -> usize {
        input.sort_unstable();

        (0..=*input.last().unwrap())
            .into_iter()
            .map(|x| {
                input
                    .iter()
                    .map(|n| if x > *n { *n..x } else { x..*n }.len())
                    .sum::<usize>()
            })
            .sorted_by(|a, b| Ord::cmp(a, b))
            .next()
            .unwrap()
    }

    fn part_two(&self, mut input: Vec<u16>) -> usize {
        input.sort_unstable();

        (0..=*input.last().unwrap())
            .into_iter()
            .map(|x| {
                input
                    .iter()
                    .map(|n| {
                        if x > *n { *n..x } else { x..*n }
                            .into_iter()
                            .enumerate()
                            .map(|(i, _)| i + 1)
                            .sum::<usize>()
                    })
                    .sum::<usize>()
            })
            .sorted_by(|a, b| Ord::cmp(a, b))
            .next()
            .unwrap()
    }
}

tests!(Day07, "07", 37, 168);
