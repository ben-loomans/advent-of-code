use std::{collections::{BTreeSet, BTreeMap}, fs::File, io::BufReader};

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

    /* fn part_one(&self) {
        let mut lines = BufReader::new(&self.input).lines();

        let seed_re = Regex::new(r"seeds: ((?:\d+ )+\d+)").unwrap();

        let seeds = lines.next()
            .unwrap()
            .unwrap();

        let mut seeds: Vec<_> = seed_re.captures(&seeds)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .split(' ')
            .map(|num| {
                num.parse::<usize>().unwrap()
            })
            .collect();

        println!("seeds: {:?}\n", seeds);

        lines.next();
        let mut done = false;

        while !done {
            let mut gm = GardenMap::new();

            loop {
                let line = match lines.next() {
                    Some(line) => line,
                    None => {done = true; break}
                }.unwrap();

                if line.contains(':') {println!("{}", line); continue}
                else if &line == "" {break}
                else {
                    let nums: Vec<usize> = line.split(' ').map(|num| {
                        num.parse().unwrap()
                    })
                    .collect();

                    if let (Some(&input), Some(&output), Some(&length)) = (nums.get(0), nums.get(1), nums.get(2)) {
                        gm.insert(input, output, length);
                    }
                }
            }

            println!("{:?}\n", gm);

            seeds.iter_mut().for_each(|seed| {
                *seed = gm.map(*seed);
            });

            println!("seeds: {:?}\n", seeds);
        }

        seeds.sort();
        println!("{}", seeds.get(0).unwrap());
    }
 */
    
    fn part_one(&self) {}
    fn part_two(&self) {
        let buf = BufReader::new(&self.input);
        let a = Interval::new(1, 10);
        let b = Interval::new(5, 15);

        let mut seeds = IntervalSet::default();
        seeds.insert(a);
        seeds.insert(b);
        //seeds.insert(a);
        
        println!("{:?}", seeds);
    }
}

/*
#[derive(Debug)]
struct GardenMap {
    // input range and offset
    map: BTreeMap<Chunk<usize>, isize>
}

impl GardenMap {
    fn new() -> Self {
        Self {
            map: BTreeMap::new()
        }
    }

    fn insert(&mut self, dest: usize, src: usize, length: usize) {
        let range = Chunk{start: src, end: src + length - 1};
        let offset: isize = dest as isize - src as isize;

        self.map.insert(range, offset);
    }

    fn map(&self, key: usize) -> usize {
        let chunk: Chunk<usize> = Chunk{start: key, end: key};
        match self.map.get(&chunk) {
            Some(offset) => (key as isize + *offset) as usize,
            None => key
        }
    }
}*/

#[derive(Debug)]
struct Interval {
    start: usize, // exclusive
    end: usize, // exclusive
}

impl Interval {
    fn new(start: usize, end: usize) -> Self {
        Self {
            start,
            end
        }
    }

    fn merge(self, other: Self) -> Self {
        Self {
            start: self.start.min(other.start),
            end: self.end.max(other.end)
        }
    }
}

#[derive(Default, Debug)]
struct IntervalSet {
    set: BTreeSet<Interval>
}

impl IntervalSet {
    fn insert(&mut self, interval: Interval) {
        let mut interval = interval;

        match self.set.take(&interval) {
            Some(previous) => interval = interval.merge(previous),
            None => ()
        }

        self.set.insert(interval);
    }
}

struct IntervalMap {
    map: BTreeMap<Interval, usize>
}

impl PartialOrd for Interval {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Interval {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.start >= other.end {std::cmp::Ordering::Greater}
        else if other.start >= self.end {std::cmp::Ordering::Less}
        else {std::cmp::Ordering::Equal} // Overlapping
    }

    /*fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.start.cmp(&other.start)
    }*/
}

impl PartialEq for Interval {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == std::cmp::Ordering::Equal
    }
}

impl Eq for Interval {}