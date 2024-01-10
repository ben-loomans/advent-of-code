use std::{fs::File, io::{BufReader, BufRead}};
use advent_of_code::solved::Solved;

pub struct Solution {
    input: File,
}

type Vertex = (isize, isize);

impl Solution {
    fn parse1(&self) -> Vec<Instruction> {
        let buf = BufReader::new(&self.input);
        let mut instructions = Vec::new();

        for line in buf.lines() {
            let line = line.unwrap();
            let mut substring = line.split(' ');
            let direction = Direction::try_from(substring.next().unwrap()).unwrap();
            let steps: usize = substring.next().unwrap().parse().unwrap();
            instructions.push((direction, steps));
        }

        instructions
    }

    fn parse2(&self) -> Vec<Instruction> {
        let buf = BufReader::new(&self.input);
        let mut instructions = Vec::new();

        for line in buf.lines() {
            let line = line.unwrap();
            let mut substrings: Vec<_> = line.split(' ').collect();
            let hex = substrings[2];

            instructions.push(process_hex(hex));
        }

        instructions
    }
}

fn process_hex(hex: &str) -> Instruction {
    let (steps, dir) = hex.strip_prefix("(#")
        .unwrap()
        .strip_suffix(')')
        .unwrap()
        .split_at(5);

    let dir = Direction::try_from(dir).unwrap();
    let steps = usize::from_str_radix(steps, 16).unwrap();

    (dir, steps)
}

type Instruction = (Direction, usize);

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
            "0" => Ok(Self::Right),
            "1" => Ok(Self::Down),
            "2" => Ok(Self::Left),
            "3" => Ok(Self::Up),
            _ => Err(())
        }
    }
}

fn step_vertex(vertex: &mut Vertex, instruction: Instruction) {
    let (dir, steps) = instruction;

    match dir {
        Direction::Up => vertex.0 -= steps as isize,
        Direction::Down => vertex.0 += steps as isize,
        Direction::Left => vertex.1 -= steps as isize,
        Direction::Right => vertex.1 += steps as isize,
    }
}

fn get_area(instructions: Vec<Instruction>) -> isize {
    let mut perimeter: isize = 0;
    let mut vertex = (0, 0);
    let mut vertices = Vec::new();

    vertices.push(vertex);
    for instruction in instructions {
        perimeter += instruction.1 as isize;
        step_vertex(&mut vertex, instruction);
        vertices.push(vertex.clone());
    }

    let inner_area: isize = vertices.windows(2).map(|window| {
        window[0].1 * window[1].0 - window[1].1 * window[0].0
    }).sum();

    let area = (inner_area + perimeter + 2) / 2;

    area
}

impl Solved for Solution {
    fn new(path: &str) -> Self {
        Self {
            input: File::open(path).expect("Couldn't open file")
        }
    }

    fn part_one(&self) {
        let instructions = self.parse1();
        let area = get_area(instructions);
        println!("area = {:?}", area);
    }

    fn part_two(&self) {
        let instructions = self.parse2();
        let area = get_area(instructions);
        println!("area = {:?}", area);
    }
}