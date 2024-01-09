use std::collections::HashMap;

use super::utils::{MultiMap, Direction};

struct Grid {
    grid: HashMap<Location, HeatLoss>,
    queue: MultiMap<HeatLoss, (Location, Journey)>,
    target: Location,
}

type Location = (isize, isize);
type HeatLoss = usize;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Journey(Direction, usize);

impl Grid {
    fn new() -> Self {
        Self {
            grid: HashMap::new(),
            queue: MultiMap::new(),
            target: (0,0)
        }
    }

    fn update(&mut self) -> Option<HeatLoss> {
        let (cost, (loc, journey)) = self.queue.pop_first().expect("queue empty");

        match journey {
            Journey(dir, steps) => {

            }
        }
        None
    }
}