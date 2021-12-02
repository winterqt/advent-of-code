use aoc_shared::{tests, Day};
use std::collections::HashMap;

pub struct Day07;

impl Day for Day07 {
    const NUMBER: u8 = 7;
    type Input = HashMap<String, Vec<(String, u8)>>;
    type Answer = usize;

    fn parse_input(&self, input: &str) -> HashMap<String, Vec<(String, u8)>> {
        enum State {
            Name,
            Contents,
        }

        let lines = input.lines();
        let mut input = HashMap::new();

        for line in lines {
            if line.ends_with(" contain no other bags.") {
                input.insert(
                    line.strip_suffix(" bags contain no other bags.")
                        .unwrap()
                        .to_string(),
                    Vec::new(),
                );

                continue;
            }

            let words: Vec<&str> = line.split(' ').collect();

            let mut state = State::Name;
            let mut i = 0;
            let mut name = String::new();
            let mut contents = Vec::new();

            while i < words.len() {
                let word = words[i];

                match state {
                    State::Name => {
                        if word == "bags" {
                            name.pop();
                            state = State::Contents;
                            // skip to number
                            i += 2;
                            continue;
                        }

                        name.push_str(word);
                        name.push(' ');
                        i += 1;
                    }
                    State::Contents => {
                        let count: u8 = word.parse().unwrap();
                        i += 1;

                        let mut name = String::new();

                        while i < words.len() {
                            if words[i].ends_with(',') | words[i].ends_with('.') {
                                i += 1;
                                break;
                            }

                            name.push_str(words[i]);
                            name.push(' ');

                            i += 1;
                        }

                        name.pop();

                        contents.push((name, count));

                        if i >= words.len() {
                            break;
                        }
                    }
                }
            }

            input.insert(name, contents);
        }

        input
    }

    fn part_one(&self, input: HashMap<String, Vec<(String, u8)>>) -> usize {
        fn holds_shiny_gold(rules: &HashMap<String, Vec<(String, u8)>>, typ: &str) -> bool {
            if let Some(contents) = rules.get(typ) {
                if contents
                    .iter()
                    .map(|(typ, _)| typ)
                    .any(|typ| typ == "shiny gold")
                {
                    return true;
                }

                for (bag, _) in contents {
                    if holds_shiny_gold(rules, bag) {
                        return true;
                    }
                }
            }

            false
        }

        let mut answer = 0;

        for bag in input.keys() {
            if holds_shiny_gold(&input, bag) {
                answer += 1;
            }
        }

        answer
    }

    fn part_two(&self, input: HashMap<String, Vec<(String, u8)>>) -> usize {
        fn count(rules: &HashMap<String, Vec<(String, u8)>>, typ: &str) -> usize {
            let mut answer = 0;

            if let Some(contents) = rules.get(typ) {
                for (typ, c) in contents {
                    let c = *c as usize;

                    answer += c;
                    answer += c * count(rules, typ);
                }
            }

            answer
        }

        count(&input, "shiny gold")
    }
}

tests!(Day07, "07", 4, 32);
