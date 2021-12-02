use aoc_shared::{tests, Day};

pub struct Day02;

impl Day for Day02 {
    const NUMBER: u8 = 2;
    type Input = Vec<Password>;
    type Answer = usize;

    fn parse_input(&self, input: &str) -> Vec<Password> {
        let lines = input.lines();
        let mut input = Vec::new();

        for line in lines {
            let parts: Vec<&str> = line.split(": ").collect();
            let rule_parts: Vec<&str> = parts[0].split_ascii_whitespace().collect();

            let (low, high) = {
                let bounds: Vec<&str> = rule_parts[0].split('-').collect();

                (bounds[0].parse().unwrap(), bounds[1].parse().unwrap())
            };

            let password = Password {
                low,
                high,
                letter: rule_parts[1].chars().next().unwrap(),
                password: parts[1].to_string(),
            };

            input.push(password);
        }

        input
    }

    fn part_one(&self, input: Vec<Password>) -> usize {
        input.iter().map(Password::valid_one).filter(|&v| v).count()
    }

    fn part_two(&self, input: Vec<Password>) -> usize {
        input.iter().map(Password::valid_two).filter(|&v| v).count()
    }
}

pub struct Password {
    low: usize,
    high: usize,
    letter: char,
    password: String,
}

impl Password {
    fn valid_one(&self) -> bool {
        let range = self.low..=self.high;

        range.contains(&self.password.chars().filter(|&i| i == self.letter).count())
    }

    fn valid_two(&self) -> bool {
        let chars: Vec<char> = self.password.chars().collect();

        (chars[self.low - 1] == self.letter) ^ (chars[self.high - 1] == self.letter)
    }
}

tests!(Day02, "02", 2, 1);
