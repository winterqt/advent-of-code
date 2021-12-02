#![allow(clippy::unreadable_literal)]

use aoc_shared::{tests, Day};

pub struct Day01;

impl Day for Day01 {
    const NUMBER: u8 = 1;
    type Input = Vec<u32>;
    type Answer = u32;

    fn parse_input(&self, input: &str) -> Vec<u32> {
        let lines = input.lines();
        let mut input = Vec::new();

        for i in lines {
            input.push(i.trim().parse().unwrap());
        }

        input
    }

    fn part_one(&self, input: Vec<u32>) -> u32 {
        for x in &input {
            for y in &input {
                if x + y == 2020 {
                    return x * y;
                }
            }
        }

        0
    }

    fn part_two(&self, input: Vec<u32>) -> u32 {
        for x in &input {
            for y in &input {
                for z in &input {
                    if x + y + z == 2020 {
                        return x * y * z;
                    }
                }
            }
        }

        0
    }
}

tests!(Day01, "01", 514579, 241861950);
