use std::{fs::File, io::{BufReader, BufRead}, collections::HashMap};
use regex::*;
use advent_of_code::solved::Solved;

pub struct Solution {
    input: File,
}

#[derive(Debug)]
pub enum Command {
    LSHIFT(Value, u32),
    RSHIFT(Value, u32),
    AND(Value, Value),
    OR(Value, Value),
    NOT(Value),
    Assign(Value),
    None
}


#[derive(Debug)]
pub enum Value {
    Number(u32),
    Address(String)
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        let out = value.parse::<u32>();
        match out {
            Ok(num) => Value::Number(num),
            Err(_) => Value::Address(value.to_string()),
        }
    }
}

struct Wires {
    wires: HashMap<String, Command>,
    signals: HashMap<String, u32>,
}

impl Wires {
    fn new() -> Self {
        Self {
            wires: HashMap::new(),
            signals: HashMap::new(),
        }
    }

    fn solve(&mut self, key: &str) -> u32 {
        let value: u32 = match self.signals.get(key) {
            Some(num) => num.clone(),
            None => match self.wires.get(key).clone() {
                Some(command) => {
                    match command {
                        Command::LSHIFT(arg1, offset) => {
                            match arg1 {
                                Value::Number(num) => {
                                    (num << offset).clone()
                                },
                                Value::Address(addr) => {
                                    (self.solve(addr) << offset).clone()
                                },
                            }
                        },
                        Command::RSHIFT(arg1, offset) => {
                            match arg1 {
                                Value::Number(num) => {
                                    num >> offset
                                },
                                Value::Address(addr) => {
                                    self.solve(addr) << offset
                                },
                            }
                        },
                        Command::AND(arg1, arg2) => {
                            match (arg1, arg2) {
                                (Value::Number(num1), Value::Number(num2)) => {
                                    num1 & num2
                                },
                                (Value::Number(num1), Value::Address(addr2)) => {
                                    num1 & self.solve(addr2)
                                },
                                (Value::Address(addr1), Value::Number(num2)) => {
                                    self.solve(addr1) & num2
                                },
                                (Value::Address(addr1), Value::Address(addr2)) => {
                                    self.solve(addr1) & self.solve(addr2)
                                },
                            }
                        },
                        Command::OR(arg1, arg2) => {
                            match (arg1, arg2) {
                                (Value::Number(num1), Value::Number(num2)) => {
                                    num1 | num2
                                },
                                (Value::Number(num1), Value::Address(addr2)) => {
                                    num1 | self.solve(addr2)
                                },
                                (Value::Address(addr1), Value::Number(num2)) => {
                                    self.solve(addr1) | num2
                                },
                                (Value::Address(addr1), Value::Address(addr2)) => {
                                    self.solve(addr1) | self.solve(addr2)
                                },
                            }
                        },
                        Command::NOT(arg1) => {
                            match arg1 {
                                Value::Number(num) => {
                                    !num
                                },
                                Value::Address(addr) => {
                                    !self.solve(addr)
                                },
                            }
                        },
                        Command::Assign(arg1) => match arg1 {
                            Value::Number(num) => {
                                *num
                            },
                            Value::Address(addr) => {
                                self.solve(addr)
                            },
                        },
                        Command::None => panic!(),
                    }
                },
                None => {println!("couldn't find this key"); panic!()},
            }
        };
        self.signals.insert(key.to_string(), value);
        value
    }
}

impl Solved for Solution {
    fn new(path: &str) -> Self {
        Self {
            input: File::open(path).expect("Couldn't open file"),
        }
    }

    fn part_one(&self) {
        let re = Regex::new(r"(?:([a-z0-9]+) )?(?:((?:L|R)SHIFT|AND|OR|NOT) )?([a-z0-9]+) -> ([a-z]+)").unwrap();

        let mut wires = Wires::new();

        let buf = BufReader::new(&self.input);
        buf.lines().into_iter().for_each(|line| {
            let line = line.unwrap();
            let caps = re.captures(&line).unwrap();

            let mut command: Command;

            if let Some(opcode) = caps.get(2) {
                match opcode.as_str() {
                    "LSHIFT" => {
                        let arg1 = Value::from(caps.get(1).unwrap().as_str());
                        let arg2 = caps.get(3).unwrap().as_str().parse::<u32>().unwrap();
                        command = Command::LSHIFT(arg1, arg2);
                    },
                    "RSHIFT" => {
                        let arg1 = Value::from(caps.get(1).unwrap().as_str());
                        let arg2 = caps.get(3).unwrap().as_str().parse::<u32>().unwrap();
                        command = Command::RSHIFT(arg1, arg2);
                    },
                    "AND" => {
                        let arg1 = Value::from(caps.get(1).unwrap().as_str());
                        let arg2 = Value::from(caps.get(3).unwrap().as_str());
                        command = Command::AND(arg1, arg2);
                    },
                    "OR" => {
                        let arg1 = Value::from(caps.get(1).unwrap().as_str());
                        let arg2 = Value::from(caps.get(3).unwrap().as_str());
                        command = Command::OR(arg1, arg2);
                    },
                    "NOT" => {
                        let arg1 = Value::from(caps.get(3).unwrap().as_str());
                        command = Command::NOT(arg1);
                    },
                    _ => {
                        command = Command::None;
                    }
                }
            } else {
                let arg1 = Value::from(caps.get(3).unwrap().as_str());
                command = Command::Assign(arg1);
            }

            wires.wires.insert(caps.get(4).unwrap().as_str().to_string(), command);
        });
        println!("{:?}", wires.solve("a"));
    }

    fn part_two(&self) {
        
    }
}