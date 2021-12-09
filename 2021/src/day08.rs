use aoc_shared::{tests, Day};
use itertools::Itertools;
use std::{convert::Infallible, ops::Index, str::FromStr};

pub struct Day08;

impl Day for Day08 {
    const NUMBER: u8 = 8;
    type Input = Vec<Entry>;
    type Answer = usize;

    fn parse_input(&self, input: &str) -> Vec<Entry> {
        input.lines().map(str::parse).map(Result::unwrap).collect()
    }

    fn part_one(&self, input: Vec<Entry>) -> usize {
        input
            .into_iter()
            .map(|e| {
                e.outputs
                    .into_iter()
                    .filter(|o| [2, 4, 3, 7].contains(&o.on()))
                    .count()
            })
            .sum()
    }

    fn part_two(&self, input: Vec<Entry>) -> usize {
        input
            .into_iter()
            .map(|entry| {
                let perm = INITIAL
                    .into_iter()
                    .permutations(7)
                    .find(|perm| {
                        entry
                            .patterns
                            .iter()
                            .cloned()
                            .map(|display| display.apply_permutation(perm))
                            .all(|display| u8::try_from(display).is_ok())
                    })
                    .unwrap();

                entry
                    .outputs
                    .into_iter()
                    .map(move |display| display.apply_permutation(&perm))
                    .map(u8::try_from)
                    .map(Result::unwrap)
                    .rev()
                    .zip((0..).into_iter().map(|x| 10_usize.pow(x as u32)))
                    .map(|(x, n)| x as usize * n)
            })
            .flatten()
            .sum()
    }
}

pub struct Entry {
    patterns: Vec<SegmentDisplay>,
    outputs: Vec<SegmentDisplay>,
}

impl FromStr for Entry {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Entry, Infallible> {
        let (patterns, outputs) = s.split_once(" | ").unwrap();

        let patterns = patterns
            .split(' ')
            .map(|p| p.chars().collect())
            .collect_vec();
        let outputs = outputs
            .split(' ')
            .map(|p| p.chars().collect())
            .collect_vec();

        Ok(Entry { patterns, outputs })
    }
}

#[derive(Clone, Default)]
struct SegmentDisplay {
    a: bool,
    b: bool,
    c: bool,
    d: bool,
    e: bool,
    f: bool,
    g: bool,
}

impl SegmentDisplay {
    fn on(&self) -> usize {
        [self.a, self.b, self.c, self.d, self.e, self.f, self.g]
            .into_iter()
            .filter(|x| *x)
            .count()
    }

    fn apply_permutation(&self, perm: &[Segment]) -> SegmentDisplay {
        let mut new = SegmentDisplay::default();

        for (i, seg) in perm.iter().enumerate() {
            let r = match i {
                0 => &mut new.a,
                1 => &mut new.b,
                2 => &mut new.c,
                3 => &mut new.d,
                4 => &mut new.e,
                5 => &mut new.f,
                6 => &mut new.g,
                _ => unimplemented!(),
            };

            *r = self[*seg];
        }

        new
    }
}

impl From<u8> for SegmentDisplay {
    #[allow(clippy::many_single_char_names)]
    fn from(n: u8) -> SegmentDisplay {
        let (a, b, c, d, e, f, g) = match n {
            0 => (true, true, true, false, true, true, true),
            1 => (false, false, true, false, false, true, false),
            2 => (true, false, true, true, true, false, true),
            3 => (true, false, true, true, false, true, true),
            4 => (false, true, true, true, false, true, false),
            5 => (true, true, false, true, false, true, true),
            6 => (true, true, false, true, true, true, true),
            7 => (true, false, true, false, false, true, false),
            8 => (true, true, true, true, true, true, true),
            9 => (true, true, true, true, false, true, true),
            _ => unimplemented!(),
        };

        SegmentDisplay {
            a,
            b,
            c,
            d,
            e,
            f,
            g,
        }
    }
}

impl TryFrom<SegmentDisplay> for u8 {
    type Error = ();

    fn try_from(us: SegmentDisplay) -> Result<u8, ()> {
        Ok(match (us.a, us.b, us.c, us.d, us.e, us.f, us.g) {
            (true, true, true, false, true, true, true) => 0,
            (false, false, true, false, false, true, false) => 1,
            (true, false, true, true, true, false, true) => 2,
            (true, false, true, true, false, true, true) => 3,
            (false, true, true, true, false, true, false) => 4,
            (true, true, false, true, false, true, true) => 5,
            (true, true, false, true, true, true, true) => 6,
            (true, false, true, false, false, true, false) => 7,
            (true, true, true, true, true, true, true) => 8,
            (true, true, true, true, false, true, true) => 9,
            _ => return Err(()),
        })
    }
}

impl FromIterator<char> for SegmentDisplay {
    fn from_iter<T: IntoIterator<Item = char>>(chars: T) -> SegmentDisplay {
        let mut seg = SegmentDisplay::default();

        for c in chars {
            match c {
                'a' => seg.a = true,
                'b' => seg.b = true,
                'c' => seg.c = true,
                'd' => seg.d = true,
                'e' => seg.e = true,
                'f' => seg.f = true,
                'g' => seg.g = true,
                _ => unimplemented!(),
            }
        }

        seg
    }
}

impl Index<Segment> for SegmentDisplay {
    type Output = bool;

    fn index(&self, index: Segment) -> &Self::Output {
        match index {
            Segment::A => &self.a,
            Segment::B => &self.b,
            Segment::C => &self.c,
            Segment::D => &self.d,
            Segment::E => &self.e,
            Segment::F => &self.f,
            Segment::G => &self.g,
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Segment {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

const INITIAL: [Segment; 7] = [
    Segment::A,
    Segment::B,
    Segment::C,
    Segment::D,
    Segment::E,
    Segment::F,
    Segment::G,
];

tests!(Day08, "08", 26, 61229);
