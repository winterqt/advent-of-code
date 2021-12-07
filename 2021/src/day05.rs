use aoc_shared::{tests, Day};
use std::{collections::HashMap, fmt::Display};

pub struct Day05;

impl Day for Day05 {
    const NUMBER: u8 = 5;
    type Input = Vec<Line>;
    type Answer = usize;

    fn parse_input(&self, input: &str) -> Vec<Line> {
        let lines = input.lines();
        let mut input = Vec::new();

        for line in lines {
            input.push(Line::new(line));
        }

        input
    }

    fn part_one(&self, input: Vec<Line>) -> usize {
        let mut occurances: HashMap<(u16, u16), u8> = HashMap::new();

        for line in input
            .into_iter()
            .filter(|line| line.start_x() == line.end_x() || line.start_y() == line.end_y())
        {
            for point in line.points() {
                *occurances.entry(point).or_default() += 1;
            }
        }

        occurances.values().filter(|x| **x >= 2).count()
    }

    fn part_two(&self, input: Vec<Line>) -> usize {
        let mut occurances: HashMap<(u16, u16), u8> = HashMap::new();

        for line in input {
            for point in line.points() {
                *occurances.entry(point).or_default() += 1;
            }
        }

        occurances.values().filter(|x| **x >= 2).count()
    }
}

pub struct Line {
    start: (u16, u16),
    end: (u16, u16),
}

impl Line {
    fn new(line: &str) -> Line {
        let (start, end) = line.split_once(" -> ").unwrap();
        let (start_x, start_y) = start.split_once(',').unwrap();
        let (end_x, end_y) = end.split_once(',').unwrap();

        Line {
            start: (start_x.parse().unwrap(), start_y.parse().unwrap()),
            end: (end_x.parse().unwrap(), end_y.parse().unwrap()),
        }
    }

    fn start_x(&self) -> u16 {
        self.start.0
    }

    fn start_y(&self) -> u16 {
        self.start.1
    }

    fn end_x(&self) -> u16 {
        self.end.0
    }

    fn end_y(&self) -> u16 {
        self.end.1
    }

    fn points(&self) -> Vec<(u16, u16)> {
        let range_x = if self.start_x() < self.end_x() {
            (self.start_x()..=self.end_x())
                .into_iter()
                .collect::<Vec<u16>>()
        } else {
            (self.end_x()..=self.start_x())
                .into_iter()
                .rev()
                .collect::<Vec<u16>>()
        };

        let range_y = if self.start_y() < self.end_y() {
            (self.start_y()..=self.end_y())
                .into_iter()
                .collect::<Vec<u16>>()
        } else {
            (self.end_y()..=self.start_y())
                .into_iter()
                .rev()
                .collect::<Vec<u16>>()
        };

        if self.start_x() == self.end_x() {
            let range = if self.start_y() < self.end_y() {
                self.start_y()..=self.end_y()
            } else {
                self.end_y()..=self.start_y()
            };

            return range.into_iter().map(|y| (self.start_x(), y)).collect();
        } else if self.start_y() == self.end_y() {
            let range = if self.start_x() < self.end_x() {
                self.start_x()..=self.end_x()
            } else {
                self.end_x()..=self.start_x()
            };

            return range.into_iter().map(|x| (x, self.start_y())).collect();
        }

        range_x.into_iter().zip(range_y.into_iter()).collect()
    }
}

impl Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{},{} -> {},{}",
            self.start_x(),
            self.start_y(),
            self.end_x(),
            self.end_y()
        )
    }
}

tests!(Day05, "05", 5, 12);
