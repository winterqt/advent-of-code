use aoc_shared::{tests, Day};

pub struct Day09;

#[cfg(not(test))]
const PREAMBLE_LEN: usize = 25;
#[cfg(test)]
const PREAMBLE_LEN: usize = 5;

impl Day for Day09 {
    const NUMBER: u8 = 9;
    type Input = Vec<u64>;
    type Answer = u64;

    fn parse_input(&self, input: &str) -> Vec<u64> {
        let lines = input.lines();
        let mut input = Vec::new();

        for line in lines {
            input.push(line.parse().unwrap());
        }

        input
    }

    #[allow(clippy::unseparated_literal_suffix)]
    fn part_one(&self, input: Vec<u64>) -> u64 {
        for window in input.windows(PREAMBLE_LEN + 1) {
            let nums = &window[..PREAMBLE_LEN];
            let num = window[window.len() - 1];

            let mut the = false;

            for x in nums {
                for y in nums {
                    if x + y == num {
                        the = true;
                        break;
                    }
                }
            }

            if !the {
                return num;
            }
        }

        0
    }

    fn part_two(&self, input: Vec<u64>) -> u64 {
        let p1 = self.part_one(input.clone());

        let mut i = 0;

        while i < input.len() {
            let mut j = input.len();

            while j > i {
                // quack
                let slice = &input[i..j];

                if slice.iter().sum::<u64>() == p1 {
                    let mut slice = slice.to_vec();
                    slice.sort_unstable();

                    return slice[0] + slice[slice.len() - 1];
                }

                j -= 1;
            }

            i += 1;
        }

        0
    }
}

tests!(Day09, "09", 127, 62);
