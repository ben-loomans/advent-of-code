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

    fn part_one(&self) {}
    fn part_two(&self) {
        let buf = BufReader::new(&self.input);
        let a = Interval::new(5, 10);
        let b = Interval::new(5, 15);
        let c = Interval::new(15, 20);
        let d = Interval::new(1, 20);

        let mut seeds = IntervalSet::new();
        seeds.insert(a);
        seeds.insert(b);
        seeds.insert(c);
        //seeds.insert(d);
        println!("{:?}", seeds);
        seeds.merge();
        println!("{:?}", seeds);

        println!("{:?}", d.map(&a, 5));
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Interval {
    start: usize, // inclusive
    end: usize, // exclusive
}

impl Interval {
    fn new(start: usize, end: usize) -> Self {
        Self {
            start,
            end
        }
    }

    fn union(&self, other: &Self) -> Option<Self> {
        if self.intersects(other) {
            let start = self.start.min(other.start);
            let end = self.end.max(other.end);
            Some(Self::new(start, end))
        } else {None}
    }

    fn intersects(&self, other: &Self) -> bool {
        match self.intersection(other) {
            Some(_) => true,
            None => false
        }
    }

    fn intersection(&self, other: &Self) -> Option<Self> {
        let start = self.start.max(other.start);
        let end = self.end.min(other.end);

        if start < end {
            Some(Self::new(start, end))
        } else {None}
    }

    fn map(&self, map: &Self, shift: usize) -> BTreeSet<Self> {
        let mut mapped_set = BTreeSet::new();
        let mut unmapped_set = BTreeSet::new();

        match self.intersection(map) {
            Some(mut interval) => {
                interval.start += shift;
                interval.end += shift;

                mapped_set.insert(interval);

                if self.start < map.start {
                    mapped_set.insert(Interval::new(self.start, map.start));
                }

                if self.end > map.end {
                    mapped_set.insert(Interval::new(map.end, self.end));
                }
            },
            None => {
                mapped_set.insert(self.clone());
            }
        }

        mapped_set
    }
}

#[derive(Default, Debug)]
struct IntervalSet {
    set: BTreeSet<Interval>
}

impl IntervalSet {
    fn new() -> Self {
        Self {
            set: BTreeSet::new()
        }
    }

    fn insert(&mut self, interval: Interval) {
        self.set.insert(interval);
    }

    fn merge(&mut self) {
        if self.set.len() > 1 {                                         // must have more than one entry
            let mut new_set = BTreeSet::new();     // create a new set 

            let mut intervals = self.set.iter();    // iterate over all intervals
            let mut merge = intervals.next().unwrap().clone();   

            intervals.for_each(|prev| {
                match merge.union(&prev) {
                    Some(merged) => {
                        merge = merged;
                    },
                    None => {
                        new_set.insert(merge);
                        merge = prev.clone();
                    },
                }
            });

            new_set.insert(merge);
            self.set = new_set;
        } 
    }
}

impl IntoIterator for IntervalSet {
    type Item = Interval;

    type IntoIter = std::collections::btree_set::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.set.into_iter()
    }
}

struct IntervalMap {
    map: BTreeMap<Interval, usize>
}

impl IntervalMap {
    fn new() -> Self {
        Self {
            map: BTreeMap::new()
        }
    }

    fn map(&self, set: IntervalSet) -> IntervalSet {
        let mut mapped_set = IntervalSet::new();

        for interval in set.into_iter() {
            for map in &self.map {
                mapped_set.insert(interval)
            }
        }

        mapped_set
    }
}