use aoc_shared::{tests, Day};
use itertools::Itertools;
use std::ops::Range;

pub struct Day07;

impl Day for Day07 {
    const NUMBER: u8 = 7;
    type Input = Vec<u16>;
    type Answer = usize;

    fn parse_input(&self, input: &str) -> Vec<u16> {
        let mut input = input
            .split(',')
            .map(str::parse)
            .map(Result::unwrap)
            .collect_vec();

        input.sort_unstable();

        input
    }

    fn part_one(&self, input: Vec<u16>) -> usize {
        (0..=*input.last().unwrap())
            .into_iter()
            .map(|x| input.iter().map(|n| range(*n, x).len()).sum::<usize>())
            .sorted_by(|a, b| Ord::cmp(a, b))
            .next()
            .unwrap()
    }

    fn part_two(&self, input: Vec<u16>) -> usize {
        (0..=*input.last().unwrap())
            .into_iter()
            .map(|x| {
                input
                    .iter()
                    .map(|n| {
                        range(*n, x)
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

fn range(from: u16, to: u16) -> Range<u16> {
    if from > to {
        to..from
    } else {
        from..to
    }
}

tests!(Day07, "07", 37, 168);
