use std::{fs::File, io::{BufReader, BufRead}, collections::{BinaryHeap, BTreeSet, BTreeMap}, ops::AddAssign};
use advent_of_code::solved::Solved;

pub struct Solution {
    input: File,
}

impl<'a> Solution {
    fn parse(&self) -> City {
        let buf = BufReader::new(&self.input);
        let mut city = City::new();

        city.queue.push(Path {cost: 0, location: (0, 0), history: (Direction::Right, 0)});

        for (row, line) in buf.lines().enumerate() {
            let line = line.unwrap();
            for (col, cost) in line.chars().enumerate() {
                let cost = cost.to_digit(10).unwrap() as usize;
                let loc = (row as isize, col as isize);

                city.blocks.insert(loc, cost);
                city.finish = city.finish.max(loc);
            }
        }

        city
    }
}

impl Solved for Solution {
    fn new(path: &str) -> Self {
        Self {
            input: File::open(path).expect("Couldn't open file")
        }
    }

    fn part_one(&self) {
        let mut city = self.parse();
        let output = city.find_path();
        println!("Shortest path = {output}");
    }

    fn part_two(&self) {
        
    }
}

type Location = (isize, isize);

type Cost = usize;

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl Direction {
    fn reverse(&self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }

    fn index(&self) -> usize {
        match self {
            Direction::Up => 0,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Right => 3,
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
struct Path {
    cost: Cost,
    location: Location,
    history: (Direction, usize)
}

impl Path {
    fn step(&self, city: &City, dir: Direction) -> Option<Self> {
        let location = self.update_location(&dir);
        let cost = city.blocks.get(&location).and_then(|cost| Some(*cost + self.cost));
        let history = self.update_history(dir);

        if let (Some(history), Some(cost)) = (history, cost) {
            Some(
                Path {
                    cost, 
                    location, 
                    history
                    }
                )
        } else {
            None
        }
    }

    fn valid_direction(&self, dir: &Direction) -> bool {
        let (prev_dir, steps) = &self.history;
        match (prev_dir, dir) {
            (s, o) if s == &o.reverse() => false,
            (s, o) if s == o && steps >= &3 => false,
            _ => true
        }
    }

    fn update_history(&self, dir: Direction) -> Option<(Direction, usize)> {
        let (prev_dir, steps) = &self.history;

        if self.valid_direction(&dir) {
            if *prev_dir == dir {
                Some((dir, *steps + 1))
            } else {
                Some((dir, 1))
            }
        } else {
            None
        }
    }

    fn update_location(&self, dir: &Direction) -> Location {
        let mut loc = self.location;

        match dir {
            Direction::Up => loc.0 -= 1,
            Direction::Down => loc.0 += 1,
            Direction::Left => loc.1 -= 1,
            Direction::Right => loc.1 += 1,
        }

        loc
    }
}

impl Ord for Path {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.cmp(&other.cost).reverse()
    }
}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

struct City {
    blocks: BTreeMap<Location, Cost>,
    visited: BTreeMap<Location, [usize; 4]>,
    queue: BinaryHeap<Path>,
    finish: Location,
}

impl City {
    fn new() -> Self {
        Self {
            blocks: BTreeMap::new(),
            visited: BTreeMap::new(),
            queue: BinaryHeap::new(),
            finish: (0,0)
        }
    }

    fn find_path(&mut self) -> usize {
        let mut found = false;
        let mut cost = 0;
        // println!("Finish = {:?}", self.finish);

        while !found {
            // println!("{}", self.queue.len());
            for path in self.get_next_paths() {
                if path.location == self.finish {
                    cost = path.cost;
                    found = true;
                }
                // print!("{:?}: {}, ", path.location, path.cost);
                self.queue.push(path);
            }
            // println!("");
        }
        
        cost
    }

    fn get_next_paths(&mut self) -> Vec<Path> {
        let curr_path = self.queue.pop().unwrap();
        let mut new_paths = Vec::new();

        if !self.visited(&curr_path) {
            new_paths.push(curr_path.step(&self, Direction::Up));
            new_paths.push(curr_path.step(&self, Direction::Down));
            new_paths.push(curr_path.step(&self, Direction::Left));
            new_paths.push(curr_path.step(&self, Direction::Right));
        } else {
            // println!("already visited {:?}", curr_path.location);
        }

        new_paths.into_iter().filter_map(|path| path).collect()
    }

    fn visited(&mut self, path: &Path) -> bool {
        let dir = &path.history.0;

        let mut visit = [0; 4];
        visit[dir.index()] = path.history.1;
        visit[dir.reverse().index()] = 3;

        if let Some(prev_visit) = self.visited.get_mut(&path.location) {
            let mut visited = true;

            for i in 0..4 {
                visited &= prev_visit[i] <= visit[i];
                prev_visit[i] = prev_visit[i].min(visit[i]);
            }

            visited
        } else {
            self.visited.insert(path.location, visit);
            false
        }
    }
}