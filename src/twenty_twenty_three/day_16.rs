use std::{fs::File, io::{BufReader, BufRead}, collections::{HashMap, HashSet}, ops::AddAssign, fmt::Display};
use advent_of_code::solved::Solved;

pub struct Solution {
    input: File,
}

impl Solution {
    fn parse(&self) -> (usize, usize, BeamRoom) {
        let buf = BufReader::new(&self.input);
        let mut beam_room = BeamRoom::default();

        let mut width = 0;
        let mut height = 0;

        for (row, line) in buf.lines().enumerate() {
            if row > height {height = row}
            for (col, tile) in line.unwrap().chars().enumerate() {
                if col > width {width = col}
                let tile = Tile::from(tile);
                beam_room.tiles.insert(Location(row as isize, col as isize), tile);
            }
        }

        (width, height, beam_room)
    }
}

impl Solved for Solution {
    fn new(path: &str) -> Self {
        Self {
            input: File::open(path).expect("Couldn't open file")
        }
    }

    fn part_one(&self) {
        let (_, _, mut beam_room) = self.parse();

        let start_beam = Beam::new(Location(0, 0), Direction::Right);
        let energized = get_energized(start_beam, &mut beam_room);

        println!("total energized squares = {}", energized);
    }

    fn part_two(&self) {
        let (width, height, mut beam_room) = self.parse();
        let mut max_energized = 0;

        (0..height).into_iter().for_each(|row| {
            let start_beam = Beam::new(Location(row as isize, 0), Direction::Right);
            let energized = get_energized(start_beam, &mut beam_room);
            //print!("{energized}, ");
            max_energized = max_energized.max(energized);
        });

        (0..width).into_iter().for_each(|col: usize| {
            let start_beam = Beam::new(Location(0, col as isize), Direction::Down);
            let energized = get_energized(start_beam, &mut beam_room);
            //print!("{energized}, ");
            max_energized = max_energized.max(energized);
        });

        (0..height).into_iter().for_each(|row| {
            let start_beam = Beam::new(Location(row as isize, width as isize), Direction::Left);
            let energized = get_energized(start_beam, &mut beam_room);
            //print!("{energized}, ");
            max_energized = max_energized.max(energized);
        });

        (0..width).into_iter().for_each(|col| {
            let start_beam = Beam::new(Location(height as isize, col as isize), Direction::Up);
            let energized = get_energized(start_beam, &mut beam_room);
            //print!("{energized}, ");
            max_energized = max_energized.max(energized);
        });
        
        println!("max energized squares = {}", max_energized);
    }
}

fn get_energized(start: Beam, mut beam_room: &mut BeamRoom) -> usize {
    let mut beam_list = BeamList::new(start);

    while beam_list.beams.len() > 0 {
        beam_list.propagate(&mut beam_room);
    }

    let mut energized = HashSet::new();

    beam_room.visited.drain().for_each(|beam| {
        energized.insert(beam.location);
    });

    energized.len()
}

#[derive(Default)]
struct BeamRoom {
    tiles: HashMap<Location, Tile>,
    visited: HashSet<Beam>,
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Beam {
    location: Location,
    direction: Direction,
}

impl Display for Beam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}): {:?}", self.location.0, self.location.1, self.direction)
    }
}

#[derive(Hash, Eq, PartialEq, Clone, Copy)]
struct Location(isize, isize);

impl AddAssign for Location {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

impl Beam {
    fn new(location: Location, direction: Direction) -> Self {
        Self {
            location,
            direction
        }
    }

    fn propagate(&self, tile: &Tile) -> Vec<Self> {
        let mut new_beams = Vec::new();
        let mut loc = self.location;

        match tile {
            Tile::LMirror => { //   /
                let new_dir = match self.direction {
                    Direction::Up => Direction::Right,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Down,
                    Direction::Right => Direction::Up,
                };

                let new_beam = self.move_beam(&new_dir);
                new_beams.push(new_beam);
            },
            Tile::RMirror => { //   \
                let new_dir = match self.direction {
                    Direction::Up => Direction::Left,
                    Direction::Down => Direction::Right,
                    Direction::Left => Direction::Up,
                    Direction::Right => Direction::Down,
                };

                let new_beam = self.move_beam(&new_dir);
                new_beams.push(new_beam);
            },
            Tile::HSplitter => { //  -
                match self.direction {
                    Direction::Left | Direction::Right => {
                        let new_beam = self.move_beam(&self.direction);
                        new_beams.push(new_beam);
                    }, 
                    Direction::Up | Direction::Down => {
                        let left_beam = self.move_beam(&Direction::Left);
                        let right_beam = self.move_beam(&Direction::Right);
                        new_beams.push(left_beam);
                        new_beams.push(right_beam);
                    }
                };
            },
            Tile::VSplitter => { //     |
                match self.direction {
                    Direction::Up | Direction::Down => {
                        let new_beam = self.move_beam(&self.direction);
                        new_beams.push(new_beam);
                    }, 
                    Direction::Left | Direction::Right => {
                        let up_beam = self.move_beam(&Direction::Up);
                        let down_beam = self.move_beam(&Direction::Down);
                        new_beams.push(up_beam);
                        new_beams.push(down_beam);
                    }
                };
            },
            Tile::Empty => {
                let new_beam = self.move_beam(&self.direction);
                new_beams.push(new_beam);
            },
        }

        new_beams
    }

    fn move_beam(&self, direction: &Direction) -> Self {
        let mut location = self.location;

        match direction {
            Direction::Up => {location += Location(-1, 0);},
            Direction::Down => {location += Location(1, 0);},
            Direction::Left => {location += Location(0, -1);},
            Direction::Right => {location += Location(0, 1);},
        }

        Self {
            location,
            direction: direction.clone(),
        }
    }
}

struct BeamList {
    beams: Vec<Beam>
}

impl BeamList {
    fn new(start: Beam) -> Self {
        let mut beams = Vec::new();
        beams.push(start);

        Self {
            beams
        }
    }

    fn propagate(&mut self, beam_room: &mut BeamRoom) {
        let new_beams: Vec<_> = self.beams.iter()
            .filter_map(|beam| {
            beam_room.tiles.get(&beam.location).and_then(|tile| {
                if beam_room.visited.contains(beam) {
                    None
                } else {
                    beam_room.visited.insert(*beam);
                    Some(beam.propagate(tile))
                }
            })
        }).flatten().collect();

        self.beams = new_beams;
    }

    fn print(&self) {
        for beam in &self.beams {
            println!("{}", beam);
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

enum Tile {
    LMirror, //     /
    RMirror, //     \
    HSplitter, //   -
    VSplitter, //   |
    Empty //        .
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Empty,
            '/' => Self::LMirror,
            '\\' => Self::RMirror,
            '-' => Self::HSplitter,
            '|' => Self::VSplitter,
            _ => panic!("Invalid input char")
        }   
    }
}