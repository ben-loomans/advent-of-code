use std::{fs, collections::BTreeMap};
use advent_of_code::solved::Solved;

pub struct Solution {
    input: String,
}

impl Solved for Solution {
    fn new(path: &str) -> Self {
        Self {
            input: path.to_string()
        }
    }

    fn part_one(&self) {
        let input = fs::read_to_string(&self.input).unwrap();
        let inputs = input.split(',');

        let output: usize = inputs.map(|input| hash(input)).sum();
        println!("{output}");
    }

    fn part_two(&self) {
        let input = fs::read_to_string(&self.input).unwrap();
        let inputs = input.split(',');

        let mut bin: BTreeMap<usize, Vec<(String, usize)>> = BTreeMap::new();

        inputs.map(|input| {
            let mut s = input.split(&['=', '-']);
            let code = s.next().unwrap();
            let value = match s.next().unwrap() {
                "" => None,
                x => x.parse::<usize>().ok()
            };

            (code, value)
        }).for_each(|(code, value)| {
            let hash = hash(code);

            match value {
                Some(lens) => {
                    match bin.get_mut(&hash) {
                        Some(slot) => {
                            slot.iter_mut()
                                .find(|(c, _)| c == code)
                                .and_then(|(_, l)| {
                                    Some(*l = lens)
                                }).or_else(|| {
                                    slot.push((code.to_string(), lens));
                                    Some(())
                                });
                        },
                        None => {
                            let mut slot = Vec::new();
                            slot.push((code.to_string(), lens));
                            bin.insert(hash, slot);
                        }
                    }
                },
                None => {
                    bin.get_mut(&hash)
                        .and_then(|slot| {
                            Some(slot.retain(|(c, _)| {
                                c != code
                            }))
                        });
                }
            }
        });

        let total: usize = bin.iter().map(|(slot_num, slot)| {
            (slot_num + 1) * slot.iter().enumerate().map(|(num, (_, lens))| {
                let out = (num + 1) * lens;
                out
            }).sum::<usize>()
        }).sum();

        println!("part 2 = {}", total);
        //println!("{:?}", bin);
    }
}

fn hash(input: &str) -> usize {
    let mut counter = 0;

    for c in input.chars() {
        counter += c as usize;
        counter *= 17;
        counter %= 256;
    }

    counter
}