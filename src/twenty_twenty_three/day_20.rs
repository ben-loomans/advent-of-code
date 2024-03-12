use std::{collections::{HashMap, VecDeque}, fs::File, io::{BufRead, BufReader}};
use advent_of_code::solved::Solved;

pub struct Solution {
    input: File,
    modules: HashMap<String, Box<dyn Module>>,
    stack: VecDeque<Pulse>,
}

impl Solution {
    fn process_button(&mut self) {
        while let Some(pulse) = self.stack.pop_front() {
            let receiver = &pulse.dest;
            let module = self.modules.get_mut(receiver).unwrap();
            let output = module.as_mut().process_pulse(pulse);
            for pulse in output {
                self.stack.push_back(pulse);
            }
        }
    }
}

impl Solved for Solution {
    fn new(path: &str) -> Self {
        Self {
            input: File::open(path).expect("Couldn't open file"),
            modules: HashMap::new(),
            stack: VecDeque::new(),
        }
    }

    fn part_one(&mut self) {
        
    }

    fn part_two(&mut self) {
        
    }
}

#[derive(Clone, Debug)]
struct Pulse {
    src: String,
    dest: String,
    is_high: bool,
}

trait Module {
    fn process_pulse(&mut self, pulse: Pulse) -> Vec<Pulse>;
}

struct FlipFlop {
    name: String,
    is_on: bool,
    outputs: Vec<String>,
}

impl FlipFlop {
    fn send_pulse(&self) -> Vec<Pulse> {
        self.outputs.iter().map(|dest| {
            Pulse {
                src: self.name.clone(),
                dest: dest.clone(),
                is_high: self.is_on,
            }
        }).collect()
    }
}

impl Module for FlipFlop {
    fn process_pulse(&mut self, pulse: Pulse) -> Vec<Pulse> {
        let mut output = Vec::new();

        if pulse.is_high {
            self.is_on = !self.is_on;
            output = self.send_pulse();
        }

        output
    }
}

struct Conjunction {
    inputs: HashMap<String, bool>,
    high_count: u32,
    outputs: Vec<String>,
}

impl Conjunction {
    fn update_memory(&mut self, pulse: &Pulse) {
        let old_input = self.inputs.get_mut(&pulse.src).unwrap();

        match (&old_input, pulse.is_high) {
            (true, false) => {
                *old_input = false;
                self.high_count -= 1;
            },
            (false, true) => {
                *old_input = true;
                self.high_count += 1;
            },
            _ => {},
        }
    }

    fn send_pulse(&self, pulse: &Pulse) {
        if self.
    }
}

impl Module for Conjunction {
    fn process_pulse(&mut self, pulse: Pulse) -> Vec<Pulse> {
        self.update_memory(&pulse);


    }
}