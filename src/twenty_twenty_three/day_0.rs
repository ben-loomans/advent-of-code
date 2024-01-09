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
        
    }

    fn part_two(&self) {
        
    }
}