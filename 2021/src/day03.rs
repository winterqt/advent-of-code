use aoc_shared::{tests, Day};

pub struct Day03;

fn binary_to_u32(binary: &[bool]) -> u32 {
    let mut s = String::new();

    for b in binary {
        if *b {
            s.push('1');
        } else {
            s.push('0');
        }
    }

    u32::from_str_radix(&s, 2).unwrap()
}

impl Day for Day03 {
    const NUMBER: u8 = 3;
    type Input = Vec<Vec<bool>>;
    type Answer = u32;

    fn parse_input(&self, input: &str) -> Vec<Vec<bool>> {
        let lines = input.lines();
        let mut input = Vec::new();

        for line in lines {
            let mut n = Vec::new();

            for bit in line.chars() {
                n.push(bit == '1');
            }

            input.push(n);
        }

        input
    }

    fn part_one(&self, input: Vec<Vec<bool>>) -> u32 {
        let mut gamma = String::new();
        let mut epsilon = String::new();

        for i in 0..input[0].len() {
            let mut ones = 0;

            for x in &input {
                if x[i] {
                    ones += 1;
                }
            }

            if ones > input.len() - ones {
                gamma.push('1');
                epsilon.push('0');
            } else {
                gamma.push('0');
                epsilon.push('1');
            }
        }

        let gamma = u32::from_str_radix(&gamma, 2).unwrap();
        let epsilon = u32::from_str_radix(&epsilon, 2).unwrap();

        gamma * epsilon
    }

    fn part_two(&self, input: Vec<Vec<bool>>) -> u32 {
        let mut oxygen = input.clone();
        let mut i = 0;

        while oxygen.len() != 1 && i < input[0].len() {
            let ones = oxygen.iter().filter(|v| v[i]).count();
            let zeros = oxygen.len() - ones;

            let mcv = if ones == zeros { true } else { ones > zeros };

            oxygen = oxygen.iter().filter(|v| v[i] == mcv).cloned().collect();

            i += 1;
        }

        let oxygen = binary_to_u32(&oxygen[0]);

        let mut co2 = input.clone();
        i = 0;

        while co2.len() != 1 && i < input[0].len() {
            let ones = co2.iter().filter(|v| v[i]).count();
            let zeros = co2.len() - ones;

            let mcv = if ones == zeros { false } else { ones < zeros };

            co2 = co2.iter().filter(|v| v[i] == mcv).cloned().collect();

            i += 1;
        }

        let co2 = binary_to_u32(&co2[0]);

        oxygen * co2
    }
}

tests!(Day03, "03", 198, 230);
