use std::{fs::File, io::{BufReader, BufRead}};
use advent_of_code::solved::Solved;

pub struct Solution {
    input: File,
}

impl Solution {
    fn parse(&self) -> Vec<Pattern> {
        let buf = BufReader::new(&self.input);
        let mut patterns = Vec::new();
        let mut builder = PatternBuilder::default();

        let mut y = 0;
        for line in buf.lines() {
            let line = line.unwrap();

            if line == "" {
                builder.y = Some(y);
                y = 0;

                let pattern = builder.build();
                patterns.push(pattern.unwrap());
                builder = PatternBuilder::default();
            } else {
                if builder.x.is_none() {
                    builder.x = Some(line.len());
                }

                builder.pattern.push_str(&line);
                y += 1;
            }
        }
        builder.y = Some(y);
        let pattern = builder.build().unwrap();
        patterns.push(pattern);

        patterns
    }
}

impl Solved for Solution {
    fn new(path: &str) -> Self {
        Self {
            input: File::open(path).expect("Couldn't open file")
        }
    }

    fn part_one(&self) {
        let patterns = self.parse();
        let sum: usize = patterns.iter().map(|pat| {
            let mut num = 0;
            if let Some(y) = pat.vert_sym() {
                num += y;
            }
            if let Some(x) = pat.hor_sym() {
                num += 100 * x;
            }
            num
        }).sum();

        println!("Sum = {sum}");
    }

    fn part_two(&self) {
        let patterns = self.parse();
        let sum: usize = patterns.iter().map(|pat| {
            let mut num = 0;
            //println!("Looking for vertical symmetry...");
            if let Some(y) = pat.vert_sym_2() {
                num += y;
            }
            //println!("Looking for horizontal symmetry...");
            if let Some(x) = pat.hor_sym_2() {
                num += 100 * x;
            }

            println!("{num}");
            num
        }).sum();

        println!("Sum = {sum}");
    }
}

#[derive(Debug)]
struct Pattern {
    pattern: String,
    dimension: (usize, usize),
}

impl Pattern {
    fn get_row(&self, num: usize) -> String {
        if num > self.dimension.1 {
            "".to_string()
        } else {
            let offset = self.dimension.0 * num;
            self.pattern[offset..offset + self.dimension.0].to_string()
        }
    }

    fn get_col(&self, num: usize) -> String {
        if num > self.dimension.0 {
            "".to_string()
        } else {
            let mut s = String::new();
            for row in 0..self.dimension.1 {
                let offset = row * self.dimension.0 + num;
                s.push_str(&self.pattern[offset..=offset]);
            }

            s
        }
    }

    fn hor_sym(&self) -> Option<usize> {
        let mut i = 1;
        let mut j = 2;
        let mut sym_line = None;

        while i > 0 && j <= self.dimension.1 {
            let row1 = self.get_row(i - 1);
            let row2 = self.get_row(j - 1);

            match (sym_line, row1 == row2) {
                (None, false) => {
                    i = j;
                    j += 1;
                },
                (None, true) => {
                    sym_line = Some(i);
                    i -= 1;
                    j += 1;
                },
                (Some(row), false) => {
                    sym_line = None;
                    i = row + 1;
                    j = i + 1;
                },
                (Some(_), true) => {
                    i -= 1;
                    j += 1;
                }
            }
        }

        sym_line
    }

    fn vert_sym(&self) -> Option<usize> {
        let mut i = 1;
        let mut j = 2;
        let mut sym_line = None;

        while i > 0 && j <= self.dimension.0 {
            let col1 = self.get_col(i - 1);
            let col2 = self.get_col(j - 1);

            match (sym_line, col1 == col2) {
                (None, false) => {
                    i = j;
                    j += 1;
                },
                (None, true) => {
                    sym_line = Some(i);
                    i -= 1;
                    j += 1;
                },
                (Some(col), false) => {
                    sym_line = None;
                    i = col + 1;
                    j = i + 1;
                },
                (Some(_), true) => {
                    i -= 1;
                    j += 1;
                }
            }
        }

        sym_line
    }

    fn vert_sym_2(&self) -> Option<usize> {
        let old_line = self.vert_sym();

        let mut i = 1;
        let mut j = 2;
        let mut sym_line = None;
        let mut smudge = false;

        while i > 0 && j <= self.dimension.0 {
            let col1 = self.get_col(i - 1);
            let col2 = self.get_col(j - 1);

            match (sym_line, Self::compare(&col1, &col2, &mut smudge)) {
                (None, false) => {
                    i = j;
                    j += 1;
                },
                (None, true) => {
                    if old_line == Some(i) {
                        i = j;
                        j += 1;
                    } else {
                        sym_line = Some(i);
                        i -= 1;
                        j += 1;
                    }
                },
                (Some(col), false) => {
                    sym_line = None;
                    smudge = false;
                    i = col + 1;
                    j = i + 1;
                },
                (Some(_), true) => {
                    i -= 1;
                    j += 1;
                }
            }
        }

        sym_line
    }

    fn hor_sym_2(&self) -> Option<usize> {
        let old_line = self.hor_sym();

        let mut i = 1;
        let mut j = 2;
        let mut sym_line = None;
        let mut smudge = false;

        while i > 0 && j <= self.dimension.1 {
            let row1 = self.get_row(i - 1);
            let row2 = self.get_row(j - 1);

            match (sym_line, Self::compare(&row1, &row2, &mut smudge)) {
                (None, false) => {
                    i = j;
                    j += 1;
                },
                (None, true) => {
                    if old_line == Some(i) {
                        i = j;
                        j += 1;
                    } else {
                        sym_line = Some(i);
                        i -= 1;
                        j += 1;
                    }
                },
                (Some(row), false) => {
                    sym_line = None;
                    smudge = false;
                    i = row + 1;
                    j = i + 1;
                },
                (Some(_), true) => {
                    i -= 1;
                    j += 1;
                }
            }
        }

        sym_line
    }

    fn compare(line1: &str, line2: &str, smudge: &mut bool) -> bool {
        //println!("comparing with smudge={smudge}\n\t{line1},\n\t{line2}");

        let comp = line1.chars().zip(line2.chars()).all(|(c1, c2)|{
            if c1 == c2 {
                true
            } else {
                if !*smudge {
                    *smudge = true;
                    true
                } else {
                    *smudge = false;
                    false
                }
            }
        });

        //println!("\t{comp}");
        comp
    }
}

#[derive(Default)]
struct PatternBuilder {
    pattern: String,
    x: Option<usize>,
    y: Option<usize>
}

impl PatternBuilder {
    fn build(self) -> Option<Pattern> {
        match (self.x, self.y) {
            (Some(x), Some(y)) => {
                Some(
                    Pattern {
                        pattern: self.pattern,
                        dimension: (x, y)
                    }
                )
            },
            _ => None
        }
    }
}