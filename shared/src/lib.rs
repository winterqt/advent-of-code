#[macro_export]
macro_rules! tests {
    ($s:ident, $day:literal, $one:literal, $two:literal) => {
        #[cfg(test)]
        mod tests {
            use super::$s as this;
            use $crate::Day;

            #[test]
            fn part_one() {
                assert_eq!(
                    this.part_one(this.parse_input(
                        include_str!(concat!("../test_data/day", $day, ".txt")).trim()
                    )),
                    $one
                );
            }

            #[test]
            fn part_two() {
                assert_eq!(
                    this.part_two(this.parse_input(
                        include_str!(concat!("../test_data/day", $day, ".txt")).trim()
                    )),
                    $two
                );
            }
        }
    };
}

pub trait Day {
    const NUMBER: u8;
    type Input;
    type Answer;

    fn parse_input(&self, input: &str) -> Self::Input;
    fn part_one(&self, input: Self::Input) -> Self::Answer;
    fn part_two(&self, input: Self::Input) -> Self::Answer;
}

#[macro_export]
macro_rules! days {
($d:ident, $input:ident, $part:ident, $($day:path),*) => {
        use $crate::Day;

        match $d {
            $(<$day>::NUMBER => {
                let input = $day.parse_input($input.trim());
                let answer = match $part {
                    1 => $day.part_one(input),
                    2 => $day.part_two(input),
                    _ => panic!()
                };

                println!("answer: {}", answer);
            })*
            _ => unimplemented!()
        }
    };
}

#[macro_export]
macro_rules! main {
    ($($day:path),* ,) => {
        use aoc_shared::days;
        use std::{
            env::args,
            io::{stdin, Read},
        };

        fn main() {
            let day: u8 = args().nth(1).unwrap().parse().unwrap();
            let part: u8 = args()
                .nth(2)
                .unwrap_or_else(|| "1".to_string())
                .parse()
                .unwrap();

            let mut input = String::new();
            stdin().read_to_string(&mut input).unwrap();

            days!(day, input, part, $($day),*);
        }
    };
}
