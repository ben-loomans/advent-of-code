use std::{fs::File, io::{BufReader, BufRead}};
use advent_of_code::solved::Solved;

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

        let answer: isize = buf.lines().map(|line| {
            let line = line.unwrap();

            let mut nums: Vec<isize> = line.split(' ').filter_map(|num| {
                match num.parse() {
                    Ok(num) => Some(num),
                    Err(_) => None
                }
            }).collect();

            let mut final_diffs = Vec::new();
            
            loop {
                final_diffs.push(*nums.last().unwrap());

                let mut all_zero = true;

                nums = nums.windows(2).map(|pair| {
                    let diff = pair[1] - pair[0];

                    if diff != 0 {
                        all_zero = false;
                    }

                    diff
                }).collect();

                if all_zero {break}
            }

            let next_prediction: isize = final_diffs.into_iter().sum();
            println!("{next_prediction}");
            next_prediction
        }).sum();

        println!("answer = {answer}");
    }

    fn part_two(&self) {
        let buf = BufReader::new(&self.input);

        let answer: isize = buf.lines().map(|line| {
            let line = line.unwrap();

            let mut nums: Vec<isize> = line.split(' ').filter_map(|num| {
                match num.parse() {
                    Ok(num) => Some(num),
                    Err(_) => None
                }
            }).collect();

            let mut final_diffs = Vec::new();
            
            loop {
                let first = *nums.first().unwrap();
                //println!("first = {first}");
                final_diffs.push(first);

                let mut all_zero = true;

                nums = nums.windows(2).map(|pair| {
                    let diff = pair[1] - pair[0];

                    if diff != 0 {
                        all_zero = false;
                    }

                    diff
                }).collect();

                if all_zero {break}
            }

            let next_prediction: isize = final_diffs.into_iter().rev().reduce(|acc, item| {
                item - acc
            }).unwrap();
            println!("{next_prediction}");
            next_prediction
        }).sum();

        println!("answer = {answer}");
    }
}