use std::{collections::HashMap, fs::File, io::{BufRead, BufReader, Lines}};
use advent_of_code::solved::Solved;

pub struct Solution {
    input: File,
    workflows: Workflows,
    parts: Parts,
}

impl Solution {
    fn parse(&mut self) {
        let buf = BufReader::new(&self.input);
        let mut lines = buf.lines();

        (&mut lines).take_while(|line| line.as_ref().is_ok_and(|line| !line.is_empty()))
            .for_each(|line| {
                let line = line.unwrap();
                let (name, workflow) = line.split_once('{').unwrap();
                let workflow: Workflow = workflow.try_into().unwrap();

                self.workflows.workflows.insert(name.to_owned(), workflow);
            });

        (&mut lines).for_each(|line| {
            let line = line.unwrap();
            let part: Part = line.as_str().try_into().unwrap();

            self.parts.parts.push(part);
        })
    }
}

impl Solved for Solution {
    fn new(path: &str) -> Self {
        Self {
            input: File::open(path).expect("Couldn't open file"),
            workflows: Workflows::new(),
            parts: Parts::new(),
        }
    }

    fn part_one(&mut self) {
        self.parse();

        let sum: u64 = self.parts.parts.iter()
            .filter(|&part| self.workflows.evaluate("in", part) == Status::Accepted)
            .map(|part| part.xmas.iter().sum::<u64>())
            .sum();

        println!("Sum of accepted parts = {}", sum);
    }

    fn part_two(&mut self) {
        self.parse();

        //let workflow = Workflow::try_from("s>2770:qs,m<1801:hdj,A}").unwrap();
        //let eval = workflow.eval_range(PartRange::new());
        //println!("{:#?}", eval);

        let mut ranges = Vec::new();

        self.workflows.eval_range("in", &mut ranges, PartRange::new());

        let sum: u64 = ranges.iter().map(|range| range.comb()).sum();

        println!("Number of possibly accepted permutations = {}", sum);
    }
}

struct Parts {
    parts: Vec<Part>
}

impl Parts {
    fn new() -> Self {
        Self {
            parts: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Range {
    start: u64,
    end: u64,
}

impl Range {
    fn new(start: u64, end: u64) -> Option<Self> {
        if end < start {return None}

        Some(Self {
            start,
            end,
        })
    }

    fn split_off(self, cond: &Condition) -> (Option<Self>, Option<Self>) {
        let value = cond.value;
        match cond.comparison {
            Comparison::LessThan => {
                (Range::new(self.start, self.end.min(value - 1)), Range::new(self.start.max(value), self.end))
            },
            Comparison::GreaterThan => {
                (Range::new(self.start.max(value + 1), self.end), Range::new(self.start, self.end.min(value)))
            },
        }
    }

    fn overlaps(&self, other: &Self) -> bool {
        !((self.start > other.end) | (other.start > self.end))
    }
}

#[derive(Debug, Clone, Copy)]
struct PartRange {
    xmas: [Range; 4],
}

impl PartRange {
    fn new() -> Self {
        Self {
            xmas: [Range::new(1,4000).unwrap(); 4]
        }
    }

    fn split_off(self, cond: &Condition) -> (Option<Self>, Option<Self>) {
        let index = match cond.category {
            Xmas::X => 0,
            Xmas::M => 1,
            Xmas::A => 2,
            Xmas::S => 3,
        };

        let split = self.xmas[index].split_off(cond);

        let yes = split.0.map(|yes| {
            let mut pr = self.clone();
            pr.xmas[index] = yes;
            pr
        });

        let no = split.1.map(|no| {
            let mut pr = self.clone();
            pr.xmas[index] = no;
            pr
        });

        (yes, no)
    }

    fn comb(&self) -> u64 {
        self.xmas.iter().map(|range| range.end - range.start + 1).product()
    }

    fn is_disjoint(&self, other: &Self) -> bool {
        !self.xmas.iter().zip(other.xmas.iter()).all(|(s, o)| s.overlaps(o))
    }
}

#[derive(Debug)]
struct Part {
    xmas: [u64; 4],
}

impl Part {
    fn get_val(&self, category: Xmas) -> u64 {
        match category {
            Xmas::X => self.xmas[0],
            Xmas::M => self.xmas[1],
            Xmas::A => self.xmas[2],
            Xmas::S => self.xmas[3],
        }
    }
}

impl TryFrom<&str> for Part {
    type Error = SolutionError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let trimmed = value.trim_matches(['{', '}']);
        let split = trimmed.split(',');
        let xmas: Vec<_> = split.filter_map(|s| {
            s.split('=')
            .skip(1).next()
            .and_then(|s| {
                s.parse::<u64>()
                .ok()
            })
        }).collect();

        if xmas.len() == 4 {
            Ok(Part {
                xmas: xmas.try_into().unwrap()
            })
        } else {
            Err(SolutionError::PartError)
        }
    }
}

struct Workflows {
    workflows: HashMap<String, Workflow>
}

impl Workflows {
    fn new() -> Self {
        Self {
            workflows: HashMap::new(),
        }
    }

    fn evaluate(&self, key: &str, part: &Part) -> Status {
        let workflow = self.workflows.get(key).unwrap();

        match workflow.evaluate(part).unwrap() {
            Output::Workflow(new_key) => self.evaluate(&new_key, part),
            Output::Status(status) => status,
        }
    }

    fn eval_range(&self, key: &str, ranges: &mut Vec<PartRange>, part_range: PartRange) {
        let workflow = self.workflows.get(key).unwrap();

        workflow.eval_range(part_range).iter().for_each(|(range, output)| {
            match output {
                Output::Workflow(new_key) => {
                    self.eval_range(&new_key, ranges, *range)
                },
                Output::Status(status) => {
                    match status {
                        Status::Accepted => {
                            ranges.push(range.clone());
                        },
                        Status::Rejected => {},
                    }
                },
            }
        });
    }
}

struct Workflow {
    rules: Vec<Rule>
}

impl Workflow {
    fn evaluate(&self, part: &Part) -> Result<Output, SolutionError> {
        for rule in &self.rules {
            if let Some(output) = rule.evaluate(part) {
                return Ok(output);
            }
        }

        Err(SolutionError::RuleError)
    }

    fn eval_range(&self, part_range: PartRange) -> Vec<(PartRange, Output)> {
        let mut out = Vec::new();
        let mut part_range = part_range;

        for rule in &self.rules {
            if let Some(condition) = &rule.condition {
                let (yes, no) = part_range.split_off(condition);
                if let Some(yes) = yes {
                    out.push((yes, rule.output.clone()));
                }
                if let Some(no) = no {
                    part_range = no;
                } else {
                    break;
                }
            } else {
                out.push((part_range, rule.output.clone()));
                break;
            }
        }

        out
    }
}

impl TryFrom<&str> for Workflow {
    type Error = SolutionError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let split = value.trim_matches(['{', '}']).split(',');
        let rules = split.filter_map(|rule| {
            rule.try_into().ok()
        }).collect();

        Ok(Self {
            rules
        })
    }
}

struct Rule {
    condition: Option<Condition>,
    output: Output,
}

impl Rule {
    fn evaluate(&self, part: &Part) -> Option<Output> {
        if self.condition.as_ref().is_some_and(|cond| {
            !cond.evaluate(part)
        }) {
            None
        } else {
            Some(self.output.clone())
        }
    }
}

impl TryFrom<&str> for Rule {
    type Error = SolutionError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut split = value.split(':');

        let mut condition;
        let mut output;

        match (split.next(), split.next()) {
            (Some(cond), Some(out)) => {
                condition = Some(cond.try_into()?);
                output = out.into();
            },
            (Some(out), None) => {
                condition = None;
                output = out.into();
            },
            _ => return Err(SolutionError::RuleError)
        };

        Ok(Rule {
            condition,
            output,
        })
    }
}

#[derive(Debug)]
struct Condition {
    category: Xmas,
    comparison: Comparison,
    value: u64,
}

impl Condition {
    fn evaluate(&self, part: &Part) -> bool {
        let value = part.get_val(self.category.clone());
        match self.comparison {
            Comparison::LessThan => value < self.value,
            Comparison::GreaterThan => value > self.value,
        }
    }
}

impl TryFrom<&str> for Condition {
    type Error = SolutionError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let Some((cat, val)) = value.split_once(['<','>']) {
            Ok(Self {
                category: cat.try_into()?,
                comparison: if value.contains('>') {Comparison::GreaterThan} else {Comparison::LessThan},
                value: val.parse().unwrap(),
            })
        } else {
            Err(SolutionError::ParseError)
        }
    }
}

#[derive(Debug)]
enum Comparison {
    LessThan,
    GreaterThan
}

#[derive(Clone, Debug)]
enum Output {
    Workflow(String),
    Status(Status)
}

#[derive(Clone, PartialEq, Debug)]
enum Status {
    Accepted,
    Rejected,
}

impl From<&str> for Output {
    fn from(value: &str) -> Self {
        match value {
            "A" => Output::Status(Status::Accepted),
            "R" => Output::Status(Status::Rejected),
            _ => Output::Workflow(value.to_owned()),
        }
    }
}

#[derive(Debug)]
enum SolutionError {
    ParseError,
    RuleError,
    PartError,
}

#[derive(Clone, Debug)]
enum Xmas {
    X,
    M,
    A,
    S,
}

impl TryFrom<&str> for Xmas {
    type Error = SolutionError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "x" => Ok(Xmas::X),
            "m" => Ok(Xmas::M),
            "a" => Ok(Xmas::A),
            "s" => Ok(Xmas::S),
            _ => Err(SolutionError::ParseError)
        }
    }
}