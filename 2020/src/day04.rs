use aoc_shared::{tests, Day};
use std::collections::HashMap;

pub struct Day04;

impl Day for Day04 {
    const NUMBER: u8 = 4;
    type Input = Vec<Passport>;
    type Answer = usize;

    fn parse_input(&self, input: &str) -> Vec<Passport> {
        let passports = input.split("\n\n");
        let mut input = Vec::new();

        for passport in passports {
            let parts: Vec<Vec<&str>> = passport
                .split('\n')
                .map(|p| p.split(' ').collect::<Vec<&str>>())
                .collect();
            let mut map = HashMap::new();

            for part in parts {
                for field in part {
                    let kv: Vec<&str> = field.split(':').collect();
                    assert_eq!(kv.len(), 2);
                    map.insert(kv[0], kv[1]);
                }
            }

            input.push(Passport {
                byr: map.get("byr").map(|&s| String::from(s)),
                iyr: map.get("iyr").map(|&s| String::from(s)),
                eyr: map.get("eyr").map(|&s| String::from(s)),
                hgt: map.get("hgt").map(|&s| String::from(s)),
                hcl: map.get("hcl").map(|&s| String::from(s)),
                ecl: map.get("ecl").map(|&s| String::from(s)),
                pid: map.get("pid").map(|&s| String::from(s)),
            });
        }

        input
    }

    fn part_one(&self, input: Vec<Passport>) -> usize {
        input.iter().map(Passport::valid_one).filter(|&v| v).count()
    }

    fn part_two(&self, input: Vec<Passport>) -> usize {
        input.iter().map(Passport::valid_two).filter(|&v| v).count()
    }
}

pub struct Passport {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
}

impl Passport {
    fn valid_one(&self) -> bool {
        macro_rules! check {
            ($($f:ident),*) => {
                $(if self.$f.is_none() {
                    return false;
                })*
            };
        }

        check!(byr, iyr, eyr, hgt, hcl, ecl, pid);

        true
    }

    #[allow(clippy::similar_names)]
    fn valid_two(&self) -> bool {
        if !self.valid_one() {
            return false;
        }

        let byr = self.byr.as_ref().unwrap();

        if byr.len() != 4 {
            return false;
        } else if let Ok(byr) = byr.parse::<u16>() {
            if !(1920..=2002).contains(&byr) {
                return false;
            }
        } else {
            return false;
        }

        let iyr = self.iyr.as_ref().unwrap();

        if iyr.len() != 4 {
            return false;
        } else if let Ok(iyr) = iyr.parse::<u16>() {
            if !(2010..=2020).contains(&iyr) {
                return false;
            }
        } else {
            return false;
        }

        let eyr = self.eyr.as_ref().unwrap();

        if eyr.len() != 4 {
            return false;
        } else if let Ok(eyr) = eyr.parse::<u16>() {
            if !(2020..=2030).contains(&eyr) {
                return false;
            }
        } else {
            return false;
        }

        let hgt = self.hgt.as_ref().unwrap();

        if !hgt.ends_with("cm") && !hgt.ends_with("in") {
            return false;
        } else if let Ok(hgt_num) = hgt[..hgt.len() - 2].parse::<u16>() {
            let bounds = if hgt.ends_with("cm") {
                150..=193
            } else {
                59..=76
            };

            if !bounds.contains(&hgt_num) {
                return false;
            }
        } else {
            return false;
        }

        let hcl = self.hcl.as_ref().unwrap();

        if !hcl.starts_with('#') {
            return false;
        } else if hcl.len() - 1 == 6 {
            if hcl[1..].chars().filter(|c| c.is_alphanumeric()).count() != 6 {
                return false;
            }
        } else {
            return false;
        }

        let ecl = self.ecl.as_ref().unwrap();

        if !["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&ecl.as_str()) {
            return false;
        }

        let pid = self.pid.as_ref().unwrap();

        if pid.len() == 9 {
            if pid.chars().filter(|c| c.is_numeric()).count() != 9 {
                return false;
            }
        } else {
            return false;
        }

        true
    }
}

tests!(Day04, "04", 2, 2);
