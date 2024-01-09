use std::{fs::File, io::{BufReader, BufRead}, collections::HashMap};
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
        let mut cache = HashMap::new();

        let output: usize = buf.lines().into_iter().map(|line| {
            let line = line.unwrap();

            let line: Vec<_> = line.split(' ').collect();
            
            let mut input = ".".to_string();
            input.push_str(line[0]);

            let pattern = line[1];

            let pats: Vec<usize> = pattern.split(',')
                .map(|num| num.parse().unwrap()).collect();
            
            let sol = match_input(&input, &pats, &mut cache);
            //println!("solution = {sol}");
            sol
        }).sum();

        println!("cache size = {}", cache.len());
        println!("output = {output}");
    }

    fn part_two(&self) {
        let buf = BufReader::new(&self.input);
        let mut cache = HashMap::new();

        let output: usize = buf.lines().into_iter().map(|line| {
            let line = line.unwrap();

            let line: Vec<_> = line.split(' ').collect();
            
            let mut input = ".".to_string();

            (0..5).for_each(|_| {
                input.push_str(line[0]);
                input.push('?');
            });

            input.pop();

            let pattern = line[1];

            let pats: Vec<usize> = pattern.split(',')
                .map(|num| num.parse().unwrap()).collect();

            let mut pats_2: Vec<usize> = Vec::new();
            (0..5).for_each(|_| {
                pats.iter().for_each(|pat| {
                    pats_2.push(pat.clone())
                })
            });

            //println!("{:?}", pats_2);
            //println!("{}", input);
            
            let sol = match_input(&input, &pats_2, &mut cache);
            //println!("solution = {sol}");
            sol
        }).sum();

        println!("cache size = {}", cache.len());
        println!("output = {output}");
    }
}

fn match_input(input: &str, hashes: &[usize], cache: &mut HashMap<(String, Vec<usize>), usize>) -> usize {
    match cache.get(&(input.to_string(), hashes.to_vec())) {
        Some(output) => *output,
        None => {
            if hashes.len() == 0 {
                match match_char(input, '.', input.len()) {
                    false => {
                        //println!("'{input}' has hashes, invalid"); 
                        0
                    },
                    true => {
                        //println!("'{input}' has no hashes, +1"); 
                        1
                    },
                }  
            } else if hashes.iter().sum::<usize>() + hashes.len() > input.len() {
                0
            } else {
                let mut sum = 0;
        
                let mut count = 1;
                while match_char(input, '.', count) {
                    if match_char(&input[count..], '#', hashes[0]) {
                        //println!("{input}");
                        //println!("\tmatched '{}' with dots and '{}' with {} hashes", &input[0..count], &input[count..count + hashes[0]], hashes[0]);
                        sum += match_input(&input[count+hashes[0]..], &hashes[1..], cache);
                    } else {
                        //println!("couldn't match '{}' with {} hashes", &input[count..], hashes[0]);
                    }
                    count += 1;
                }
        
                //println!("couldn't match '{}' with {} dots", &input[0..count], count);
        
                cache.insert((input.to_string(), hashes.to_vec()), sum);
                sum
            }
        }
    }
}

fn match_char(input: &str, p: char, num: usize) -> bool {
    if input.len() < num {
        false
    } else {
        input.chars().take(num).all(|c| c == p || c == '?')
    }
}