use aoc_shared::{tests, Day};

pub struct Day05;

impl Day for Day05 {
    const NUMBER: u8 = 5;
    type Input = Vec<u16>;
    type Answer = u16;

    fn parse_input(&self, input: &str) -> Vec<u16> {
        let lines: Vec<&str> = input.lines().collect();

        let rows: Vec<u8> = (0..=127).collect();
        let columns: Vec<u8> = (0..=7).collect();

        let mut ids = Vec::new();

        for seat in lines {
            let row = {
                let mut curr = &rows[..];

                for c in seat[..7].chars() {
                    curr = match c {
                        'F' => &curr[..curr.len() / 2],
                        'B' => &curr[curr.len() / 2..],
                        _ => panic!(),
                    };
                }

                assert_eq!(curr.len(), 1);

                curr[0]
            };

            let column = {
                let mut curr = &columns[..];

                for c in seat[7..].chars() {
                    curr = match c {
                        'L' => &curr[..curr.len() / 2],
                        'R' => &curr[curr.len() / 2..],
                        _ => panic!(),
                    };
                }

                assert_eq!(curr.len(), 1);

                curr[0]
            };

            ids.push((u16::from(row) * 8) + u16::from(column));
        }

        ids.sort_unstable();

        ids
    }

    fn part_one(&self, input: Vec<u16>) -> u16 {
        *input.last().unwrap()
    }

    fn part_two(&self, input: Vec<u16>) -> u16 {
        for x in input[1]..input[input.len() - 2] {
            if !input.contains(&x) && input.contains(&(x + 1)) && input.contains(&(x - 1)) {
                return x;
            }
        }

        0
    }
}

tests!(Day05, "05", 820, 0);
