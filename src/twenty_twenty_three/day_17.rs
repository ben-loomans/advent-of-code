use std::{fs::File, io::{BufReader, BufRead}, collections::{HashMap, BTreeMap, HashSet, VecDeque, BTreeSet}};
use advent_of_code::solved::Solved;

pub struct Solution {
    input: File,
}

impl Solution {
    fn parse(&self) -> Grid {
        let buf = BufReader::new(&self.input);
        let mut grid = Grid::new();

        let mut loc: Location = (0, 0);

        for (y, row) in buf.lines().enumerate() {

            for (x, col) in row.unwrap().chars().enumerate() {
                loc = (x, y);
                let heat_loss: HeatLoss = col as usize - '0' as usize;

                grid.grid.insert(loc, heat_loss);
            }
        }

        grid.target = loc;
        grid
    }
}

impl Solved for Solution {
    fn new(path: &str) -> Self {
        Self {
            input: File::open(path).expect("Couldn't open file")
        }
    }

    fn part_one(&self) {
        let mut grid = self.parse();
        
        println!("{:?}", grid.target);
    }

    fn part_two(&self) {
        
    }
}

struct Grid {
    grid: HashMap<Location, HeatLoss>,
    queue: MultiMap<HeatLoss, (Location, Journey)>,
    target: Location
}

type Location = (usize, usize);
type HeatLoss = usize;

impl Grid {
    fn new() -> Self {
        Self {
            grid: HashMap::new(),
            queue: MultiMap::new(),
            target: (0,0)
        }
    }

    fn run(&mut self, start: Location) {
        self.queue.insert(0, ((0, 0), Journey(Direction::Right, 0)))
        while let Some((cost, (location, direction))) = self.queue.pop_first() {
            for neighbour in self.get_neighbours((location, direction)) {
                
            }
        }

    }

    fn get_neighbours(&self, node: (Location, Journey)) {
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Journey(Direction, usize);


