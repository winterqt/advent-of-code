use aoc_shared::{tests, Day};
use std::fmt::Display;

pub struct Day04;

impl Day for Day04 {
    const NUMBER: u8 = 4;
    type Input = (Vec<u8>, Vec<Board>);
    type Answer = u16;

    fn parse_input(&self, input: &str) -> (Vec<u8>, Vec<Board>) {
        let mut numbers = Vec::new();
        let mut boards = Vec::new();

        for num in input.lines().next().unwrap().split(',') {
            numbers.push(num.parse().unwrap());
        }

        for board in input.split("\n\n").skip(1) {
            let rows = board.split('\n');
            let mut board = Vec::new();

            for row in rows {
                let nums = row.split_ascii_whitespace();
                let mut row = Vec::new();

                for num in nums {
                    row.push(num.parse().unwrap());
                }

                board.push(row);
            }

            boards.push(Board::new(board));
        }

        (numbers, boards)
    }

    fn part_one(&self, (numbers, mut boards): (Vec<u8>, Vec<Board>)) -> u16 {
        for num in numbers {
            for board in &mut boards {
                board.mark(num);

                if board.bingo() {
                    return board.score(num);
                }
            }
        }

        0
    }

    fn part_two(&self, (numbers, mut boards): (Vec<u8>, Vec<Board>)) -> u16 {
        for num in numbers {
            if boards.len() == 1 {
                boards[0].mark(num);
                return boards[0].score(num);
            }

            for board in &mut boards {
                board.mark(num);
            }

            boards.retain(|board| !board.bingo());
        }

        0
    }
}

pub struct Board(Vec<Vec<(u8, bool)>>);

impl Board {
    fn new(numbers: Vec<Vec<u8>>) -> Board {
        Board(
            numbers
                .into_iter()
                .map(|row| row.into_iter().map(|n| (n, false)).collect())
                .collect(),
        )
    }

    fn mark(&mut self, num: u8) {
        for row in &mut self.0 {
            for (n, marked) in row {
                if *n == num {
                    *marked = true;
                    return;
                }
            }
        }
    }

    fn bingo(&self) -> bool {
        for row in &self.0 {
            if row.iter().all(|(_, marked)| *marked) {
                return true;
            }
        }

        for i in 0..self.0[0].len() {
            if self.0.iter().all(|row| row[i].1) {
                return true;
            }
        }

        false
    }

    fn score(&self, winning: u8) -> u16 {
        self.0
            .iter()
            .map(|row| {
                row.iter()
                    .filter(|(_, matched)| !*matched)
                    .map(|(num, _)| num)
                    .sum::<u8>() as u16
            })
            .sum::<u16>()
            * winning as u16
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.0 {
            for (num, marked) in row {
                if *marked {
                    write!(f, "({:>2}) ", num)?;
                } else {
                    write!(f, "{:>4} ", num)?;
                }
            }

            writeln!(f)?;
        }

        Ok(())
    }
}

tests!(Day04, "04", 4512, 1924);
