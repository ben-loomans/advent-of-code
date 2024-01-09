use std::{fs::File, io::{BufReader, BufRead}};
use regex::Regex;
use advent_of_code::solved::Solved;

pub struct Solution {
    input: File,
}

pub struct Lights {
    lights: Box<[[u8; 1000]; 1000]>,
}

impl Lights {
    pub fn new() -> Self {
        Self {
            lights: Box::new([[0; 1000]; 1000])
        }
    }

    pub fn get_area(&mut self, x1: usize, y1: usize, x2: usize, y2: usize) -> impl Iterator<Item = &mut u8> {
        self.lights[y1..y2+1].iter_mut().map(move |s| &mut s[x1..x2+1]).flatten()
    }

    pub fn get_count(&self) -> u32 {
        let mut sum = 0;

        self.lights.iter().for_each(|row| {
            row.iter().for_each(|light| {
                sum += *light as u32;
            })
        });

        sum
    }
}



impl Solved for Solution {
    fn new(path: &str) -> Self {
        Self {
            input: File::open(path).expect("Couldn't open file"),
        }
    }

    fn part_one(&self) {
        let re = Regex::new(r"(toggle|turn (?:off|on)) (\d+),(\d+) through (\d+),(\d+)").unwrap();
        let mut lights = Lights::new();

        let buf = BufReader::new(&self.input);
        buf.lines()
            .into_iter()
            .for_each(|line| {
                let line = line.unwrap();
                let caps = re.captures(&line).unwrap();

                let x1 = caps[2].parse::<usize>().unwrap();
                let y1 = caps[3].parse::<usize>().unwrap();
                let x2 = caps[4].parse::<usize>().unwrap();
                let y2 = caps[5].parse::<usize>().unwrap();

                match &caps[1] {
                    "toggle" => {
                        lights.get_area(x1,y1,x2,y2).for_each(|light| {
                            *light = *light ^ 1;
                        });
                    },
                    "turn on" => {
                        lights.get_area(x1,y1,x2,y2).for_each(|light| {
                            *light = *light | 1;
                        });
                    },
                    "turn off" => {
                        lights.get_area(x1,y1,x2,y2).for_each(|light| {
                            *light = *light & 0;
                        });
                    },
                    _ => panic!("Dunno what's going on")
                }
            });

        println!("Count = {}", lights.get_count());
    }

    fn part_two(&self) {
        let re = Regex::new(r"(toggle|turn (?:off|on)) (\d+),(\d+) through (\d+),(\d+)").unwrap();
        let mut lights = Lights::new();

        let buf = BufReader::new(&self.input);
        buf.lines()
            .into_iter()
            .for_each(|line| {
                let line = line.unwrap();
                let caps = re.captures(&line).unwrap();

                let x1 = caps[2].parse::<usize>().unwrap();
                let y1 = caps[3].parse::<usize>().unwrap();
                let x2 = caps[4].parse::<usize>().unwrap();
                let y2 = caps[5].parse::<usize>().unwrap();

                match &caps[1] {
                    "toggle" => {
                        lights.get_area(x1,y1,x2,y2).for_each(|light| {
                            *light += 2;
                        });
                    },
                    "turn on" => {
                        lights.get_area(x1,y1,x2,y2).for_each(|light| {
                            *light += 1;
                        });
                    },
                    "turn off" => {
                        lights.get_area(x1,y1,x2,y2).for_each(|light| {
                            if *light > 0 {
                                *light -= 1;
                            }
                        });
                    },
                    _ => panic!("Dunno what's going on")
                }
            });

        println!("Count = {}", lights.get_count());
    }
}