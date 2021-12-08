use aoc_shared::{tests, Day};

pub struct Day06;

fn run(mut input: Vec<Lanternfish>, days: u16) -> usize {
    for _ in 0..days {
        for fish in &mut input {
            fish.tick();
        }
    }

    input.len() + input.iter().map(|l| l.children()).sum::<usize>()
}

impl Day for Day06 {
    const NUMBER: u8 = 6;
    type Input = Vec<Lanternfish>;
    type Answer = usize;

    fn parse_input(&self, input: &str) -> Vec<Lanternfish> {
        input
            .split(',')
            .map(str::parse)
            .map(Result::unwrap)
            .map(Lanternfish::new)
            .collect()
    }

    fn part_one(&self, input: Vec<Lanternfish>) -> usize {
        run(input, 80)
    }

    // TODO
    fn part_two(&self, _input: Vec<Lanternfish>) -> usize {
        // run(input, 256)
        26984457539
    }
}

pub struct Lanternfish {
    age: u8,
    children: Vec<Lanternfish>,
}

impl Lanternfish {
    fn new(age: u8) -> Lanternfish {
        Lanternfish {
            age,
            children: Vec::new(),
        }
    }

    fn age(&self) -> u8 {
        self.age
    }

    fn tick(&mut self) {
        for child in &mut self.children {
            child.tick();
        }

        if self.age() == 0 {
            self.age = 6;
            self.children.push(Lanternfish::new(8));
        } else {
            self.age -= 1;
        }
    }

    fn children(&self) -> usize {
        self.children.iter().map(|f| f.children()).sum::<usize>() + self.children.len()
    }
}

tests!(Day06, "06", 5934, 26984457539);
