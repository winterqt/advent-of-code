use aoc_shared::{tests, Day};

pub struct Day02;

impl Day for Day02 {
    const NUMBER: u8 = 2;
    type Input = Vec<Instruction>;
    type Answer = u32;

    fn parse_input(&self, input: &str) -> Vec<Instruction> {
        let lines = input.lines();
        let mut input = Vec::new();

        for line in lines {
            let parts: Vec<&str> = line.split(' ').collect();
            let argument = parts[1].parse().unwrap();

            let instruction = match parts[0] {
                "forward" => Instruction::Forward(argument),
                "down" => Instruction::Down(argument),
                "up" => Instruction::Up(argument),
                _ => unimplemented!(),
            };

            input.push(instruction);
        }

        input
    }

    fn part_one(&self, input: Vec<Instruction>) -> u32 {
        let mut x = 0;
        let mut y = 0;

        for instruction in input {
            match instruction {
                Instruction::Forward(n) => {
                    x += n;
                }
                Instruction::Down(n) => {
                    y += n;
                }
                Instruction::Up(n) => {
                    y -= n;
                }
            }
        }

        x * y
    }

    fn part_two(&self, input: Vec<Instruction>) -> u32 {
        let mut x = 0;
        let mut y = 0;
        let mut aim = 0;

        for instruction in input {
            match instruction {
                Instruction::Forward(n) => {
                    x += n;
                    y += aim * n;
                }
                Instruction::Down(n) => {
                    aim += n;
                }
                Instruction::Up(n) => {
                    aim -= n;
                }
            }
        }

        x * y
    }
}

pub enum Instruction {
    Forward(u32),
    Down(u32),
    Up(u32),
}

tests!(Day02, "02", 150, 900);
