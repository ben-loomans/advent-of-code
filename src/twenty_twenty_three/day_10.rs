use std::{fs::File, io::{BufReader, BufRead}, collections::BTreeMap};
use advent_of_code::solved::Solved;

pub struct Solution {
    input: File,
}

#[derive(Debug, Clone, Copy)]
enum Tile {
    Vertical,       // '|'
    Horizontal,     // '-'
    NorthEast,      // 'L'
    NorthWest,      // 'J'
    SouthEast,      // 'F'
    SouthWest,      // '7'
    Ground,         // '.'
    Nothing,        // ' ', useful for part 2
}

impl Tile {
    fn navigate(&self, dir: Direction) -> Direction {
        match self {
            Tile::Vertical => {
                match dir {
                    Direction::North => Direction::North,
                    Direction::South => Direction::South,
                    _ => Direction::Invalid,
                }
            },
            Tile::Horizontal => {
                match dir {
                    Direction::East => Direction::East,
                    Direction::West => Direction::West,
                    _ => Direction::Invalid,
                }
            },
            Tile::NorthEast => {
                match dir {
                    Direction::South => Direction::East,
                    Direction::West => Direction::North,
                    _ => Direction::Invalid,
                }
            },
            Tile::NorthWest => {
                match dir {
                    Direction::South => Direction::West,
                    Direction::East => Direction::North,
                    _ => Direction::Invalid,
                }
            },
            Tile::SouthEast => {
                match dir {
                    Direction::North => Direction::East,
                    Direction::West => Direction::South,
                    _ => Direction::Invalid,
                }
            },
            Tile::SouthWest => {
                match dir {
                    Direction::North => Direction::West,
                    Direction::East => Direction::South,
                    _ => Direction::Invalid,
                }
            },
            _ => Direction::Invalid
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
    Invalid
}

impl Direction {
    fn step(&self, coords: &(usize, usize)) -> (usize, usize) {
        let mut coords = *coords;

        match self {
            Direction::North => coords.0 -= 1,
            Direction::South => coords.0 += 1,
            Direction::East => coords.1 += 1,
            Direction::West => coords.1 -= 1,
            Direction::Invalid => panic!("Invalid direction"),
        }

        coords
    }
}

#[derive(Default)]
struct Grid {
    start: (usize, usize),
    tiles: BTreeMap<(usize, usize), Tile>
}

impl Grid {
    fn insert(&mut self, coords: (usize, usize), tile: char) {
        let tile = match tile {
            '|' => Tile::Vertical,
            '-' => Tile::Horizontal,
            'L' => Tile::NorthEast,
            'J' => Tile::NorthWest,
            'F' => Tile::SouthEast,
            '7' => Tile::SouthWest,
            '.' => Tile::Ground,
            'S' => {
                self.start = coords;
                Tile::Nothing
            },
             _  => Tile::Nothing
        };

        self.tiles.insert(coords, tile);
    }

    fn get_start_tile(&mut self) {
        let start = self.start;

        let mut connecting_dirs = Vec::new();

        for dir in [Direction::North, Direction::South, Direction::East, Direction::West] {
            if self.tile_connects(&start, dir) {
                connecting_dirs.push(dir);
            }
        }

        let start_tile = match (connecting_dirs.get(0), connecting_dirs.get(1)) {
            (Some(Direction::North), Some(Direction::South)) => Tile::Vertical,
            (Some(Direction::North), Some(Direction::East)) => Tile::NorthEast,
            (Some(Direction::North), Some(Direction::West)) => Tile::NorthWest,
            (Some(Direction::South), Some(Direction::East)) => Tile::SouthEast,
            (Some(Direction::South), Some(Direction::West)) => Tile::SouthWest,
            (Some(Direction::East), Some(Direction::West)) => Tile::Horizontal,
            _ => Tile::Nothing
        };

        self.tiles.insert(start, start_tile);
    }

    fn tile_connects(&self, coords: &(usize, usize), dir: Direction) -> bool {
        let stepped = dir.step(&coords); // take a step in the specified direction

        match self.tiles.get(&stepped) {
            Some(tile) => {
                match tile.navigate(dir) { // try to navigate through this tile from the specified direction
                    Direction::Invalid => false,
                    _ => true
                }
            },
            None => false, 
        }
    }

    fn solve(&mut self) -> Self {
        self.get_start_tile();

        let mut coords = self.start;
        let mut tile = self.tiles.get(&coords).unwrap();
        let mut dir = Direction::Invalid;

        let mut solved = Self::default();
        solved.start = self.start;

        for d in [Direction::North, Direction::South, Direction::East, Direction::West] {
            match tile.navigate(d) {
                Direction::Invalid => {continue},
                new_dir => {
                    solved.tiles.insert(coords, tile.clone());
                    coords = new_dir.step(&coords);
                    dir = new_dir;
                    break;
                }
            }
        }

        while coords != self.start {
            solved.tiles.insert(coords, tile.clone());
            tile = self.tiles.get(&coords).unwrap();
            dir = tile.navigate(dir);
            coords = dir.step(&coords);
        }

        solved
    }

    fn get_area(&mut self) -> usize {
        let solved = self.solve();
        let mut area = 0;
        let mut enclosed = false;

        for (coords, tile) in self.tiles.iter_mut() {
            match (tile, solved.tiles.get(coords)) {
                (tile, None) => {
                    match tile {
                        Tile::Ground => {},
                        _ => {
                            *tile = Tile::Ground;
                        }
                    }
                },
                _ => {}
            }
        }

        let mut prev_dir = Direction::Invalid; // Had to mess around for cases like L----7, which counts as only ONE crossing

        for tile in self.tiles.values() {
            match tile {
                Tile::Vertical => {enclosed ^= true;},
                Tile::NorthEast => {prev_dir = Direction::North},
                Tile::NorthWest => {
                    if prev_dir == Direction::South {
                        enclosed ^= true;
                    }
                    prev_dir = Direction::Invalid;
                },
                Tile::SouthEast => {prev_dir = Direction::South},
                Tile::SouthWest => {
                    if prev_dir == Direction::North {
                        enclosed ^= true;
                    }
                    prev_dir = Direction::Invalid;
                },
                Tile::Ground => {
                    if enclosed {
                        area += 1;
                    }
                },
                _ => {},
            }
        }

        area
    }
}

impl Solved for Solution {
    fn new(path: &str) -> Self {
        Self {
            input: File::open(path).expect("Couldn't open file")
        }
    }

    fn part_one(&self) {
        let mut grid = Grid::default();
        let buf = BufReader::new(&self.input);

        for (row, line) in buf.lines().enumerate() {
            let line = line.unwrap();
            for (col, tile) in line.chars().enumerate() {
                grid.insert((row, col), tile);
            }
        }

        let length = grid.solve().tiles.len();
        println!("len = {}", length);
    }

    fn part_two(&self) {
        let mut grid = Grid::default();
        let buf = BufReader::new(&self.input);

        for (row, line) in buf.lines().enumerate() {
            let line = line.unwrap();
            for (col, tile) in line.chars().enumerate() {
                grid.insert((row, col), tile);
            }
        }

        let area = grid.get_area();

        println!("area = {area}");
    }
}