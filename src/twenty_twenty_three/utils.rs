use std::collections::{BTreeSet, BTreeMap};

#[derive(Debug)]
pub struct MultiMap<K: Ord + Clone + Copy, V: Ord> {
    map: BTreeMap<K, BTreeSet<V>>,
}

impl<K: Ord + Clone + Copy, V: Ord> MultiMap<K, V> {
    pub fn new() -> Self {
        Self {
            map: BTreeMap::new(),
        }
    }

    pub fn insert(&mut self, key: K, value: V) -> bool {
        match self.map.get_mut(&key) {
            None => {
                let mut set = BTreeSet::new();
                set.insert(value);
                self.map.insert(key, set);
                true
            },
            Some(set) => {
                set.insert(value)
            }
        }
    }

    pub fn pop_first(&mut self) -> Option<(K, V)> {
        self.map.first_entry().and_then(|mut entry| {
            
            let set = entry.get_mut();
            let value = set.pop_first().unwrap();
            let key = entry.key().clone();

            if entry.get().is_empty() {
                entry.remove();
            }

            Some((key, value))
        })
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Location(isize, isize);

impl Location {
    pub fn step(&mut self, dir: &Direction) {
        match dir {
            Direction::Up => self.1 - 1,
            Direction::Down => self.1 + 1,
            Direction::Left => self.0 - 1,
            Direction::Right => self.0 + 1,
        };
    }
}