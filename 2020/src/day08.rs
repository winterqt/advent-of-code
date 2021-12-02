use aoc_shared::{tests, Day};
use std::collections::HashSet;

pub struct Day08;

impl Day for Day08 {
    const NUMBER: u8 = 8;
    type Input = Vec<Instruction>;
    type Answer = i32;

    fn parse_input(&self, input: &str) -> Vec<Instruction> {
        let lines = input.lines();
        let mut input = Vec::new();

        for line in lines {
            let parts: Vec<String> = line.split(' ').map(String::from).collect();
            assert_eq!(parts.len(), 2);

            let argument = parts[1].parse().unwrap();

            let instruction = match parts[0].as_str() {
                "acc" => Instruction::Acc(argument),
                "jmp" => Instruction::Jmp(argument),
                "nop" => Instruction::Nop(argument),
                _ => unimplemented!(),
            };

            input.push(instruction);
        }

        input
    }

    fn part_one(&self, input: Vec<Instruction>) -> i32 {
        let mut accumulator = 0;
        let mut ran = HashSet::new();
        let mut i: isize = 0;

        #[allow(clippy::cast_possible_wrap, clippy::cast_sign_loss)]
        while i < input.len() as isize {
            if ran.contains(&i) {
                return accumulator;
            }

            ran.insert(i);

            #[allow(clippy::match_on_vec_items)]
            match input[i as usize] {
                Instruction::Acc(val) => {
                    accumulator += val;
                    i += 1;
                }
                Instruction::Jmp(offset) => {
                    i += offset as isize;
                }
                Instruction::Nop(_) => {
                    i += 1;
                }
            }
        }

        0
    }

    #[allow(clippy::match_on_vec_items)]
    fn part_two(&self, input: Vec<Instruction>) -> i32 {
        fn run(instructions: &[Instruction]) -> Option<i32> {
            let mut accumulator = 0;
            let mut ran = HashSet::new();
            let mut i: isize = 0;

            #[allow(clippy::cast_possible_wrap, clippy::cast_sign_loss)]
            while i < instructions.len() as isize {
                if ran.contains(&i) {
                    return None;
                }

                ran.insert(i);

                match instructions[i as usize] {
                    Instruction::Acc(val) => {
                        accumulator += val;
                        i += 1;
                    }
                    Instruction::Jmp(offset) => {
                        i += offset as isize;
                    }
                    Instruction::Nop(_) => {
                        i += 1;
                    }
                }
            }

            Some(accumulator)
        }

        for i in input
            .iter()
            .enumerate()
            .filter(|&(_, i)| matches!(i, &Instruction::Nop(_)))
            .map(|(i, _)| i)
        {
            let mut instructions = input.clone();

            instructions[i] = match instructions[i] {
                Instruction::Nop(v) => Instruction::Jmp(v),
                _ => panic!(),
            };

            if let Some(accumulator) = run(&instructions) {
                return accumulator;
            }
        }

        for i in input
            .iter()
            .enumerate()
            .filter(|&(_, i)| matches!(i, &Instruction::Jmp(_)))
            .map(|(i, _)| i)
        {
            let mut instructions = input.clone();

            instructions[i] = match instructions[i] {
                Instruction::Jmp(v) => Instruction::Nop(v),
                _ => panic!(),
            };

            if let Some(accumulator) = run(&instructions) {
                return accumulator;
            }
        }

        0
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

tests!(Day08, "08", 5, 8);
