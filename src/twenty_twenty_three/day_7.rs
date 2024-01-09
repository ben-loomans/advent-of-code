use std::{fs::File, io::{BufReader, BufRead}, collections::HashMap};
use advent_of_code::solved::Solved;

pub struct Solution {
    input: File,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Class {
    Invalid,
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,    
}

#[derive(PartialEq, Eq, Debug)]
struct Hand {
    cards: String,
    wager: u32,
}

trait Rules {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering;
    fn classify(&self) -> Class;
}

#[derive(PartialEq, Eq, Debug)]
struct Game1 {
    class: Class,
    hand: Hand,
}

impl Ord for Game1 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        static ORDER: [char; 13] = ['2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A'];
        let mut order = self.class.cmp(&other.class);

        if order == std::cmp::Ordering::Equal {
            order = self.hand.cmp(&other.hand, &ORDER);
        }

        order
    }
}

impl PartialOrd for Game1 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(PartialEq, Eq, Debug)]
struct Game2 {
    class: Class,
    hand: Hand,
}

impl Ord for Game2 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        static ORDER: [char; 13] = ['J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A'];
        let mut order = self.class.cmp(&other.class);

        if order == std::cmp::Ordering::Equal {
            order = self.hand.cmp(&other.hand, &ORDER);
        }

        order
    }
}

impl PartialOrd for Game2 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Hand {
    fn new(input: &str) -> Option<Self> {
        let input: Vec<_> = input.split(' ').collect();

        if let (Some(&cards), Some(&wager)) = (input.get(0), input.get(1)) {
            if let Ok(wager) = wager.parse::<u32>() {
                Some(Self {
                    cards: cards.to_string(),
                    wager,
                })
            } else {
                None
            }
        } else {None}
    }

    fn cmp(&self, other: &Self, order: &[char; 13]) -> std::cmp::Ordering {
        for (a, b) in self.cards.chars().zip(other.cards.chars()) {
            if a == b {continue} else {
                let a = order.iter().position(|card| *card == a).unwrap();
                let b = order.iter().position(|card| *card == b).unwrap();

                return a.cmp(&b);
            }
            
        }

        std::cmp::Ordering::Equal
    }

    fn classify(&self, j_is_joker: bool) -> Class {
        let mut map = HashMap::new();

        for card in self.cards.chars() {
            map.entry(card.clone()).and_modify(|count| *count += 1).or_insert(1);
        }

        let mut num_jokers = 0;

        if j_is_joker {
            num_jokers = match map.get(&'J') {
                None => 0,
                Some(&num) => num
            };

            map.remove(&'J');
        }

        let mut counts: Vec<_> = map.values().collect();
        counts.sort();
        counts.reverse();

        match (counts.get(0), counts.get(1)) {
            (Some(&&5), _) => Class::FiveOfAKind,
            (Some(&&4), _) => {
                match num_jokers {
                    1 => Class::FiveOfAKind,
                    _ => Class::FourOfAKind,
                }
            }
            (Some(&&3), Some(&&2)) => Class::FullHouse,
            (Some(&&3), _) => {
                match num_jokers {
                    2 => Class::FiveOfAKind,
                    1 => Class::FourOfAKind,
                    _ => Class::ThreeOfAKind,
                }
            }
            (Some(&&2), Some(&&2)) => {
                match num_jokers {
                    1 => Class::FullHouse,
                    _ => Class::TwoPair,
                }
            }
            (Some(&&2), _) => {
                match num_jokers {
                    3 => Class::FiveOfAKind,
                    2 => Class::FourOfAKind,
                    1 => Class::ThreeOfAKind,
                    _ => Class::OnePair,
                }
            }
            (Some(&&1), _) => {
                match num_jokers {
                    4 => Class::FiveOfAKind,
                    3 => Class::FourOfAKind,
                    2 => Class::ThreeOfAKind,
                    1 => Class::OnePair,
                    _ => Class::HighCard,
                }
            },
            _ => {
                match num_jokers {
                    5 => Class::FiveOfAKind,
                    _ => Class::Invalid,
                }
            }
        }
    }
}

impl Solved for Solution {
    fn new(path: &str) -> Self {
        Self {
            input: File::open(path).expect("Couldn't open file")
        }
    }

    fn part_one(&self) {
        let buf = BufReader::new(&self.input);

        let mut games: Vec<_> = buf.lines().map(|line| {
            let line = line.unwrap();
            let hand = Hand::new(&line).unwrap();

            let class = hand.classify(false);

            Game1 {class, hand}
        }).collect();

        games.sort();
        
        let sum: u32 = games.iter().enumerate().map(|(rank, game)| {
            game.hand.wager * (rank + 1) as u32
        }).sum();

        println!("Sum = {sum}");
    }

    fn part_two(&self) {
        let buf = BufReader::new(&self.input);

        let mut games: Vec<_> = buf.lines().map(|line| {
            let line = line.unwrap();
            let hand = Hand::new(&line).unwrap();

            let class = hand.classify(true);

            Game2 {class, hand}
        }).collect();

        games.sort();
        
        let sum: u32 = games.iter().enumerate().map(|(rank, game)| {
            println!("{rank} = {:?}", game);
            game.hand.wager * (rank + 1) as u32
        }).sum();

        println!("Sum = {sum}");
    }
}