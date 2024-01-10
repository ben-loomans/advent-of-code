use std::{fs::File, io::{BufReader, BufRead}};
use advent_of_code::solved::Solved;

pub struct Solution {
    input: File,
}

type Vertex = (isize, isize);

impl Solution {
    fn parse(&self) -> (Vec<Vertex>, isize) {
        let buf = BufReader::new(&self.input);

        let mut vertices = Vec::new();
        let mut vertex = (0, 0);
        vertices.push(vertex);

        let mut perimeter = 0;

        for line in buf.lines() {
            let line = line.unwrap();
            let mut substring = line.split(' ');
            let direction = Direction::try_from(substring.next().unwrap()).unwrap();
            let steps: usize = substring.next().unwrap().parse().unwrap();
            step_vertex(&mut vertex, direction, steps);
            vertices.push(vertex.clone());

            perimeter += steps;
        }

        (vertices, perimeter as isize)
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl TryFrom<&str> for Direction {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "U" => Ok(Self::Up),
            "D" => Ok(Self::Down),
            "L" => Ok(Self::Left),
            "R" => Ok(Self::Right),
            _ => Err(())
        }
    }
}

fn step_vertex(vertex: &mut Vertex, dir: Direction, steps: usize) {
    match dir {
        Direction::Up => vertex.0 -= steps as isize,
        Direction::Down => vertex.0 += steps as isize,
        Direction::Left => vertex.1 -= steps as isize,
        Direction::Right => vertex.1 += steps as isize,
    }
}

impl Solved for Solution {
    fn new(path: &str) -> Self {
        Self {
            input: File::open(path).expect("Couldn't open file")
        }
    }

    fn part_one(&self) {
        let (vertices, perimeter) = self.parse();

        let inner_area: isize = vertices.windows(2).map(|window| {
            window[0].1 * window[1].0 - window[1].1 * window[0].0
        }).sum();

        let area = (inner_area + perimeter + 2) / 2;

        println!("{:?}", area);
    }

    fn part_two(&self) {
        
    }
}