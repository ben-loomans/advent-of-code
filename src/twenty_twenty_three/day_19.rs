use std::{fs::File, io::{BufReader, BufRead, Lines}, collections::HashMap};
use advent_of_code::solved::Solved;

pub struct Solution {
    input: File,
}

impl Solution {
    fn parse(&self) -> (HashMap<String, String>, Vec<String>) {
        let buf = BufReader::new(&self.input);
        let mut lines: Lines<BufReader<&File>> = buf.lines();

        let workflows = parse_workflows(&mut lines);
        let parts = parse_parts(&mut lines);

        (workflows, parts)
    }
}

fn parse_workflows(lines: &mut Lines<BufReader<&File>>) -> HashMap<String, String> {
    lines.take_while(|line| !line.as_ref().unwrap().is_empty())
        .map(|line| {
            let line = line.unwrap();
            let mut split = line.split('{');

            (split.next().unwrap().to_owned(), split.next().unwrap().to_owned())
        }).collect()
}

fn parse_parts(lines: &mut Lines<BufReader<&File>>) -> Vec<String> {
    lines.filter_map(|line| {
        line.ok()
    }).collect()
}

impl Solved for Solution {
    fn new(path: &str) -> Self {
        Self {
            input: File::open(path).expect("Couldn't open file")
        }
    }

    fn part_one(&self) {
        let (workflows, parts) = self.parse();
        println!("{:?}", workflows);
        println!("{:?}", parts);
    }

    fn part_two(&self) {
        
    }
}