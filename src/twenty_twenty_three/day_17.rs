use std::{fs::File, io::{BufReader, BufRead}, collections::{BinaryHeap, BTreeMap}};
use advent_of_code::solved::Solved;

pub struct Solution {
    input: File,
}

impl<'a> Solution {
    fn parse(&self, metric: (usize, usize)) -> City {
        let buf = BufReader::new(&self.input);
        let mut city = City::new(metric);

        city.queue.push(Path {cost: 0, location: (0, 0), history: (Direction::Right, 0)});
        //city.queue.push(Path {cost: 0, location: (0, 0), history: (Direction::Down, 0)});

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
        let mut city = self.parse((0, 3));
        let output = city.find_path();
        println!("Shortest path = {output}");
    }

    fn part_two(&self) {
        let mut city = self.parse((4, 10));
        let output = city.find_path();
        println!("Shortest path = {output}");

        /* let visit = city.get_visit_array(&Path {
            cost: 0,
            location: (0, 0),
            history: (Direction::Down, 4),
        });

        println!("{:?}", visit); */
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
        let history = self.update_history(city, dir);

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

    fn valid_direction(&self, city: &City, dir: &Direction) -> bool {
        city.get_visit_array(&self)[dir.index()] > 0
    }

    fn update_history(&self, city: &City, dir: Direction) -> Option<(Direction, usize)> {
        let (prev_dir, steps) = &self.history;

        if self.valid_direction(&city, &dir) {
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
    metric: (usize, usize),
}

impl City {
    fn new(metric: (usize, usize)) -> Self {
        Self {
            blocks: BTreeMap::new(),
            visited: BTreeMap::new(),
            queue: BinaryHeap::new(),
            finish: (0,0),
            metric
        }
    }

    fn find_path(&mut self) -> usize {
        let mut found = false;
        let mut cost = 0;

        while !found {
            for path in self.get_next_paths() {
                if path.location == self.finish {
                    cost = path.cost;
                    found = true;
                }
                self.queue.push(path);
            }
        }
        
        cost
    }

    fn get_next_paths(&mut self) -> Vec<Path> {
        //println!("test");
        let curr_path = self.queue.pop().unwrap();
        let mut new_paths = Vec::new();

        if !self.visited(&curr_path) {
            new_paths.push(curr_path.step(&self, Direction::Up));
            new_paths.push(curr_path.step(&self, Direction::Down));
            new_paths.push(curr_path.step(&self, Direction::Left));
            new_paths.push(curr_path.step(&self, Direction::Right));
        }

        new_paths.into_iter().filter_map(|path| path).collect()
    }

    fn visited(&mut self, path: &Path) -> bool {
        let visit = self.get_visit_array(path);

        if let Some(prev_visit) = self.visited.get_mut(&path.location) {
            let mut visited = true;

            for i in 0..4 {
                visited &= visit[i] <= prev_visit[i];
                prev_visit[i] = prev_visit[i].max(visit[i]);
            }

            visited
        } else {
            self.visited.insert(path.location, visit);
            false
        }
    }

    fn get_visit_array(&self, path: &Path) -> [usize; 4] {
        let (min, max) = &self.metric;
        let (dir, dist) = &path.history;

        let mut visits = if dist < min {
            [0; 4]
        } else {
            [*max; 4]
        };

        visits[dir.reverse().index()] = 0;
        visits[dir.index()] = max - dist;

        visits
    }
}