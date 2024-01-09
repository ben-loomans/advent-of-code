use std::{fs::File, io::{BufReader, BufRead}};
use advent_of_code::solved::Solved;
use regex::Regex;

pub struct Solution {
    input: File,
}

struct Draw {
    colors: [u32; 3],
}

impl Draw {
    fn new() -> Self {
        Self {
            colors: [0; 3],
        }
    }

    fn red(&mut self, num: u32) {
        self.colors[0] = num;
    }

    fn green(&mut self, num: u32) {
        self.colors[1] = num;
    }

    fn blue(&mut self, num: u32) {
        self.colors[2] = num;
    }

    fn possible(&self, colors: [u32; 3]) -> bool {
        (self.colors[0] <= colors[0]) &
        (self.colors[1] <= colors[1]) &
        (self.colors[2] <= colors[2])
    }
}

impl Solved for Solution {
    fn new(path: &str) -> Self {
        Self {
            input: File::open(path).expect("Couldn't open file")
        }
    }

    fn part_one(&self) {
        let re1 = Regex::new(r"Game \d+:((?: \d+ (?:red|green|blue)[,;]?)+)").unwrap();
        let re2 = Regex::new(r"(\d+) (green|red|blue)").unwrap();

        let buf = BufReader::new(&self.input);
        
        let output = buf.lines().into_iter().enumerate().fold(0, |acc, (id, line)| {
            let line = line.unwrap();
            let game = re1.captures(&line).unwrap().get(1).unwrap().as_str().to_string();

            if game.split(';').into_iter().fold(true, |acc, draw| {
                let mut d = Draw::new();

                draw.split(',').into_iter().for_each(|colour| {
                    let draw = re2.captures(colour).unwrap();
                    let num = draw.get(1).unwrap().as_str().parse::<u32>().unwrap();
                    match draw.get(2).unwrap().as_str() {
                        "red" => d.red(num),
                        "green" => d.green(num),
                        "blue" => d.blue(num),
                        _ => ()
                    }
                });

                acc & d.possible([12, 13, 14])
            }) {acc + id + 1} else {acc}
        });

        println!("output = {}", output);
    }

    fn part_two(&self) {
        let re1 = Regex::new(r"Game \d+:((?: \d+ (?:red|green|blue)[,;]?)+)").unwrap();
        let re2 = Regex::new(r"(\d+) (green|red|blue)").unwrap();

        let buf = BufReader::new(&self.input);
        
        let output = buf.lines().into_iter().fold(0, |acc,  line| {
            let line = line.unwrap();
            let game = re1.captures(&line).unwrap().get(1).unwrap().as_str().to_string();

            let mut c = [0; 3];

            game.split(';').into_iter().for_each(|draw| {
                draw.split(',').into_iter().for_each(|colour| {
                    let draw = re2.captures(colour).unwrap();
                    let num = draw.get(1).unwrap().as_str().parse::<u32>().unwrap();
                    match draw.get(2).unwrap().as_str() {
                        "red" => if c[0] < num {c[0] = num},
                        "green" => if c[1] < num {c[1] = num},
                        "blue" => if c[2] < num {c[2] = num},
                        _ => ()
                    }
                });
            });
            
            acc + c.into_iter().fold(1, |acc, count| {
                acc * count
            })
        });

        println!("output = {}", output);
    }
}