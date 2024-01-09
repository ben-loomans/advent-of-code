use std::{fs::File, io::{BufReader, BufRead}, collections::HashMap};
use advent_of_code::solved::Solved;
use regex::Regex;

pub struct Solution {
    input: File,
}

impl Solved for Solution {
    fn new(path: &str) -> Self {
        Self {
            input: File::open(path).expect("Couldn't open file")
        }
    }

    fn part_one(&self) {
        let buf = BufReader::new(&self.input);

        let mut lines = buf.lines();

        let re = Regex::new(r"(\w+) = \((\w+), (\w+)\)").unwrap();

        let right_left = lines.next().unwrap().unwrap();
        lines.next();

        let mut map = HashMap::new();

        lines.for_each(|line| {
            let line = line.unwrap();

            let caps = re.captures(&line).unwrap();
            let key = caps.get(1).unwrap().as_str().to_string();
            let value = 
                (caps.get(2).unwrap().as_str().to_string(), 
                 caps.get(3).unwrap().as_str().to_string());
            map.insert(key, value);
        });

        let mut curr = "AAA";
        let mut dirs = right_left.chars().cycle();
        let mut count = 0;

        while curr != "ZZZ" {
            let dir = dirs.next().unwrap();
            let next = map.get(curr).unwrap();

            curr = match dir {
                'L' => &next.0,
                'R' => &next.1,
                _ => panic!()
            };

            count += 1;
        }

        println!("count = {count}");
    }

    fn part_two(&self) {
        let buf = BufReader::new(&self.input);

        let mut lines = buf.lines();

        let re = Regex::new(r"(\w+) = \((\w+), (\w+)\)").unwrap();

        let right_left = lines.next().unwrap().unwrap();
        lines.next();

        println!("dirs length = {}", right_left.len());

        let mut map = HashMap::new();

        lines.for_each(|line| {
            let line = line.unwrap();

            let caps = re.captures(&line).unwrap();
            let key = caps.get(1).unwrap().as_str().to_string();
            let value = 
                (caps.get(2).unwrap().as_str().to_string(), 
                 caps.get(3).unwrap().as_str().to_string());
            map.insert(key, value);
        });

        let mut curr: Vec<_> = map.keys().into_iter().filter(|item| {
            item.ends_with(|end| end == 'Z')
        }).collect();

        let mut dirs = right_left.chars().cycle();
        let mut count = 0;
        let mut cycles = Vec::new();

        while curr.len() > 0 {
            count += 1;
            let dir = dirs.next().unwrap();
            
            curr = curr.into_iter().filter_map(|key| {
                let value = map.get(key).unwrap();
                
                let next = match dir {
                    'L' => &value.0,
                    'R' => &value.1,
                    _ => panic!()
                };

                if next.ends_with(|end| end == 'Z') {
                    cycles.push(count);
                    None
                } else {Some(next)}
            }).collect();
        }
        println!("{:?}", cycles);

        // this answer is a bit dodgy and relies on two facts:
        // firstly, that each cycle is divisible by the length of the right-left directions provided,
        // and secondly that each cycle is otherwise prime
        // if not, I would need to find the least common multiple properly

        // also, if each cycle WASN'T divisible by the number of left-right directions,
        // this problem would likely be unsolvable
        let total: usize = cycles.iter().map(|num| {
            num / right_left.len()
        }).product();

        println!("total = {}", total * right_left.len());
    }
}