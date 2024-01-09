use std::{fs::File, io::{BufReader, BufRead}, collections::BTreeSet};
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

        let sum = buf.lines().fold(0, |acc, line| {
            let line = line.unwrap();

            let mut numbers = line.split(':')
                .skip(1)
                .next()
                .unwrap()
                .split('|');

            let win_nums: BTreeSet<_> = numbers.next()
            .unwrap()
            .split(' ')
            .filter_map(|num| num.parse::<u32>().ok())
            .collect();

            let our_nums: BTreeSet<_> = numbers.next()
                .unwrap()
                .split(' ')
                .filter_map(|num| num.parse::<u32>().ok())
                .collect();

            let num_matches = our_nums.intersection(&win_nums).count();

            if num_matches > 0 {
                acc + (1 << (our_nums.intersection(&win_nums).count() - 1))
            } else {acc}
        });

        println!("{sum}");
    }

    fn part_two(&self) {
        let buf = BufReader::new(&self.input);

        let points: Vec<_> = buf.lines().map(|line| {
            let line = line.unwrap();

            let mut numbers = line.split(':')
                .skip(1)
                .next()
                .unwrap()
                .split('|');

            let win_nums: BTreeSet<_> = numbers.next()
            .unwrap()
            .split(' ')
            .filter_map(|num| num.parse::<u32>().ok())
            .collect();

            let our_nums: BTreeSet<_> = numbers.next()
                .unwrap()
                .split(' ')
                .filter_map(|num| num.parse::<u32>().ok())
                .collect();

            let num_matches = our_nums.intersection(&win_nums).count();
            num_matches
        }).collect();

        let mut times: Vec<_> = points.iter().map(|_| 1).collect();

        points.iter().enumerate().for_each(|(i, points)| {
            // Get the multiplier for this card
            let mult = times.get(i).unwrap().clone();

            // For all cards after this one until the number of points, add the multiplier
            for j in i + 1 ..= (i + points) {
                match times.get_mut(j) {
                    Some(num) => {*num += mult},
                    None => ()
                }
            }
        });

        let sum = times.iter().fold(0, |acc, num| acc + num);
        println!("sum = {sum}");
    }
}