use aoc_shared::{tests, Day};
use std::collections::{HashMap, HashSet};

pub struct Day06;

impl Day for Day06 {
    const NUMBER: u8 = 6;
    type Input = Vec<Vec<String>>;
    type Answer = usize;

    fn parse_input(&self, input: &str) -> Vec<Vec<String>> {
        let groups = input.split("\n\n");
        let mut input = Vec::new();

        for group in groups {
            input.push(group.split('\n').map(String::from).collect());
        }

        input
    }

    fn part_one(&self, input: Vec<Vec<String>>) -> usize {
        let mut answer = 0;

        for group in input {
            let mut answers = HashSet::new();

            for person in group {
                for answer in person.chars() {
                    answers.insert(answer);
                }
            }

            answer += answers.len();
        }

        answer
    }

    fn part_two(&self, input: Vec<Vec<String>>) -> usize {
        let mut answer = 0;

        for group in input {
            let mut answers: HashMap<char, usize> = HashMap::new();

            for person in &group {
                for answer in person.chars() {
                    *answers.entry(answer).or_default() += 1;
                }
            }

            answer += answers.values().filter(|&&v| v == group.len()).count();
        }

        answer
    }
}

tests!(Day06, "06", 11, 6);
