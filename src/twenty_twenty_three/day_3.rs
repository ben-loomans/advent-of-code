use std::{fs::File, io::{BufReader, BufRead}, collections::{BTreeSet, BTreeMap}};
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
        let mut numbers: Vec<(String, (i32, i32))> = Vec::new();
        let mut symbols: BTreeSet<(i32, i32)> = BTreeSet::new();

        buf.lines().into_iter().enumerate().for_each(|(row, line)| {
            let line = line.unwrap();
            let mut number = String::new();

            let mut col: i32 = 0;
            let mut chars = line.chars();

            loop {
                let c = chars.next();

                match c {
                    Some('0'..='9') => {
                        number.push(c.unwrap());
                    }
                    _ => {
                        if number.len() > 0 {
                            numbers.push((number, (row as i32, col)));
                        }
                        
                        number = String::new();
                        match c {
                            Some(x) => {
                                if x != '.' {
                                    symbols.insert((row as i32, col + 1));
                                }
                            }
                            None => {break;}
                        }
                    }
                }
                col += 1;
            }
        });
        
        let sum = numbers.into_iter().fold(0, |acc, (num, (row, col))| {
            let mut has_adjacent = false;

            for x in row - 1 ..= row + 1 {
                for y in col - num.len() as i32 ..= col + 1 {
                    if let Some(_) = symbols.get(&(x, y)) {
                        has_adjacent = true;
                        break;
                    }
                }
            }

            let num = num.parse::<u32>().unwrap();
            println!("{num}: {has_adjacent}");

            if has_adjacent {
                acc + num
            } else {acc}

            
        });

        println!("{sum}");
        
    }

    fn part_two(&self) {
        let buf = BufReader::new(&self.input);
        let mut numbers: Vec<(String, (i32, i32))> = Vec::new();
        let mut symbols: BTreeMap<(i32, i32), char> = BTreeMap::new();

        buf.lines().into_iter().enumerate().for_each(|(row, line)| {
            let line = line.unwrap();
            let mut number = String::new();

            let mut col: i32 = 0;
            let mut chars = line.chars();

            loop {
                let c = chars.next();

                match c {
                    Some('0'..='9') => {
                        number.push(c.unwrap());
                    }
                    _ => {
                        if number.len() > 0 {
                            numbers.push((number, (row as i32, col)));
                        }
                        
                        number = String::new();
                        match c {
                            Some(x) => {
                                if x != '.' {
                                    symbols.insert((row as i32, col + 1), x);
                                }
                            }
                            None => {break;}
                        }
                    }
                }
                col += 1;
            }
        });

        let mut collection = BTreeMap::new();
        
        numbers.into_iter().for_each(|(num, (row, col))| {
  
            for x in row - 1 ..= row + 1 {
                for y in col - num.len() as i32 ..= col + 1 {
                    if let Some('*') = symbols.get(&(x, y)) {
                        let num = num.parse::<u32>().unwrap();
                        match collection.get_mut(&(x, y)) {
                            None => {
                                collection.insert((x, y), vec![num]);
                            }
                            Some(vec) => {
                                vec.push(num);
                            }
                        }

                    }
                }
            }
        });

        let sum = collection.into_iter().fold(0, |acc: u32, (_, vec)| {
            if vec.len() == 2 {
                acc + vec.get(0).unwrap() * vec.get(1).unwrap()
            } else {acc}
        });

        println!("{sum}");
    }
}