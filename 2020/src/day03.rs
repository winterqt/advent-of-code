use aoc_shared::{tests, Day};

pub struct Day03;

fn tree_count(map: &[Vec<char>], slope_x: usize, slope_y: usize) -> u64 {
    let mut x = 0;
    let mut y = 0;
    let mut trees = 0;

    while y < map.len() && x < map[0].len() {
        if map[y][x] == '#' {
            trees += 1;
        }

        x += slope_x;
        y += slope_y;
    }

    trees
}

impl Day for Day03 {
    const NUMBER: u8 = 3;
    type Input = Vec<Vec<char>>;
    type Answer = u64;

    fn parse_input(&self, input: &str) -> Vec<Vec<char>> {
        let lines = input.lines();
        let mut input = Vec::new();

        for line in lines {
            let mut row = Vec::new();

            for c in line.chars() {
                row.push(c);
            }

            // lol
            // lmao
            input.push(row.repeat(100));
        }

        input
    }

    fn part_one(&self, input: Vec<Vec<char>>) -> u64 {
        tree_count(&input, 3, 1)
    }

    fn part_two(&self, input: Vec<Vec<char>>) -> u64 {
        let mut answer = 1;

        for (slope_x, slope_y) in [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)] {
            answer *= tree_count(&input, slope_x, slope_y);
        }

        answer
    }
}

tests!(Day03, "03", 7, 336);
