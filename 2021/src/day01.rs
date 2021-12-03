use aoc_shared::{tests, Day};

pub struct Day01;

impl Day for Day01 {
    const NUMBER: u8 = 1;
    type Input = Vec<u16>;
    type Answer = u16;

    fn parse_input(&self, input: &str) -> Vec<u16> {
        let lines = input.lines();
        let mut input = Vec::new();

        for line in lines {
            input.push(line.parse().unwrap());
        }

        input
    }

    fn part_one(&self, input: Vec<u16>) -> u16 {
        let mut answer = 0;

        for window in input.windows(2) {
            if window[1] > window[0] {
                answer += 1;
            }
        }

        answer
    }

    fn part_two(&self, input: Vec<u16>) -> u16 {
        let mut answer = 0;
        let mut last = 0;

        for window in input.windows(3) {
            let sum = window.iter().sum();

            if last != 0 && sum > last {
                answer += 1;
            }

            last = sum;
        }

        answer
    }
}

tests!(Day01, "01", 7, 5);
