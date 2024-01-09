use std::{fs::File, io::{BufReader, BufRead}};
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
        let mut sum = 0;

        let re = Regex::new(r"^[a-z]*([0-9])").unwrap();
        let buf = BufReader::new(&self.input);
        buf.lines().into_iter().for_each(|line| {
            let line: String = line.unwrap();
            let rev: String = line.chars().rev().collect();

            let caps1 = re.captures(&line).unwrap();
            let caps2 = re.captures(&rev).unwrap();

            let a = caps1.get(1).unwrap().as_str().parse::<u32>().unwrap();
            let b = caps2.get(1).unwrap().as_str().parse::<u32>().unwrap();

            sum += 10*a + b;
        });

        println!("sum = {}", sum);
    }

    fn part_two(&self) {
        let mut sum = 0;

        let re1 = Regex::new(r"^[a-z]*?(one|two|three|four|five|six|seven|eight|nine|[0-9])").unwrap();
        let re2 = Regex::new(r"^[a-z]*?(eno|owt|eerht|ruof|evif|xis|neves|thgie|enin|[0-9])").unwrap();
        let buf = BufReader::new(&self.input);

        buf.lines().into_iter().for_each(|line| {
            let line: String = line.unwrap();
            let rev: String = line.chars().rev().collect();

            let caps1 = re1.captures(&line).unwrap();
            let caps2 = re2.captures(&rev).unwrap();

            let a = match caps1.get(1).unwrap().as_str() {
                "one" => 1,
                "two" => 2,
                "three" => 3,
                "four" => 4,
                "five" => 5,
                "six" => 6,
                "seven" => 7,
                "eight" => 8,
                "nine" => 9,
                num => {
                    println!("{}", num);
                    num.parse::<u32>().unwrap()
                }
            };

            let b = match caps2.get(1).unwrap().as_str() {
                "eno" => 1,
                "owt" => 2,
                "eerht" => 3,
                "ruof" => 4,
                "evif" => 5,
                "xis" => 6,
                "neves" => 7,
                "thgie" => 8,
                "enin" => 9,
                num => {
                    println!("{}", num);
                    num.parse::<u32>().unwrap()
                }
            };

            sum += 10*a + b;
        });

        println!("sum = {}", sum);
    }
}