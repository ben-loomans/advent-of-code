use std::{fs::File, io::{BufReader, BufRead}};
use advent_of_code::solved::Solved;

pub struct Solution {
    input: File,
}

impl Solution {
    fn parse(&self) -> Mirrors {
        let buf = BufReader::new(&self.input);
        let mut builder = MirrorBuilder::default();

        let mut y = 0;
        for line in buf.lines() {
            let line = line.unwrap();

            if builder.x.is_none() {
                builder.x = Some(line.len());
            }

            builder.mirrors.push_str(&line);
            y += 1;
        }

        builder.y = Some(y);
        builder.build().unwrap()
    }
}

impl Solved for Solution {
    fn new(path: &str) -> Self {
        Self {
            input: File::open(path).expect("Couldn't open file")
        }
    }

    fn part_one(&self) {
        let mirrors = self.parse();

        println!("{}", mirrors.get_row(0));
    }

    fn part_two(&self) {
        
    }
}

struct Mirrors {
    mirrors: String,
    dimension: (usize, usize),
}

impl Mirrors {
    fn get_row(&self, num: usize) -> String {
        if num > self.dimension.1 {
            "".to_string()
        } else {
            let offset = self.dimension.0 * num;
            self.mirrors[offset..offset + self.dimension.0].to_string()
        }
    }

    fn get_col(&self, num: usize) -> String {
        if num > self.dimension.0 {
            "".to_string()
        } else {
            let mut s = String::new();
            for row in 0..self.dimension.1 {
                let offset = row * self.dimension.0 + num;
                s.push_str(&self.mirrors[offset..=offset]);
            }

            s
        }
    }

    fn tilt_north(&mut self) {
        let mut strips = Vec::new();
        let mut mirrors = String::new();

        for col in 0..self.dimension.0 {
            let mut col = self.get_col(col);
            Self::tilt_strip(&mut col);
            strips.push(col);
        }

        for row in 0..self.dimension.1 {
            for strip in strips.iter_mut() {
                let c = strip.get(row..=row).unwrap();
                mirrors.push_str(c);
            }
        }
    }

    fn tilt_strip(strip: &mut String) {
        let s = String::with_capacity(strip.len());
        // move along strip, 
    }
}

#[derive(Default)]
struct MirrorBuilder {
    mirrors: String,
    x: Option<usize>,
    y: Option<usize>
}

impl MirrorBuilder {
    fn build(self) -> Option<Mirrors> {
        match (self.x, self.y) {
            (Some(x), Some(y)) => {
                Some(
                    Mirrors {
                        mirrors: self.mirrors,
                        dimension: (x, y)
                    }
                )
            },
            _ => None
        }
    }
}