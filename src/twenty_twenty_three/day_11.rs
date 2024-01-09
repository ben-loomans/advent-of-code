use std::{fs::File, io::{BufReader, BufRead}, fmt::Display};
use advent_of_code::solved::Solved;

pub struct Solution {
    input: File,
}

#[derive(Debug)]
struct KDTree {
    root: Node,
}

impl KDTree {
    fn new(value: [isize; 2]) -> Self {
        Self {
            root: Node::new(value, 0)
        }
    }

    fn insert(&mut self, value: [isize; 2]) {
        self.root.insert(value);
    }

    fn min(&self, dim: usize) -> isize {
        self.root.min(dim)
    } 

    fn vert_voids(&self) -> Vec<isize> {
        let voids = Vec::new();



        voids
    }
}

#[derive(Debug)]
struct Node {
    value: [isize; 2],
    depth: usize,
    left_node: Option<Box<Node>>,
    right_node: Option<Box<Node>>,
}

impl Node {
    fn new(value: [isize; 2], depth: usize) -> Self {
        Self {
            value,
            depth,
            left_node: None,
            right_node: None
        }
    }

    fn insert(&mut self, value: [isize; 2]) {
        let dim = self.depth / 2;

        if value[dim] < self.value[dim] {
            match &mut self.left_node {
                Some(ref mut left) => {
                    left.insert(value);
                },
                None => {
                    self.left_node = Some(Box::new(Node::new(value, self.depth + 1)));
                },
            }
        } else {
            match &mut self.right_node {
                Some(ref mut right) => {
                    right.insert(value);
                },
                None => {
                    self.right_node = Some(Box::new(Node::new(value, self.depth + 1)));
                },
            }
        }
    }

    fn min(&self, dim: usize) -> isize {
        if self.depth / 2 == dim { // i.e. current node is split on the correct dimension
            match &self.left_node {
                Some(node) => {
                    node.min(dim)
                },
                None => {
                    self.value[dim]
                },
            }
        } else { // if not split on correct dimension, need to check both child nodes
            let left = &self.left_node;
            let right = &self.right_node;

            match (left, right) {
                (None, None) => {
                    self.value[dim]
                },
                (None, Some(node)) => {
                    if self.value[dim] < node.value[dim] {
                        self.value[dim]
                    } else {
                        node.min(dim)
                    }
                },
                (Some(node), None) => {
                    if self.value[dim] < node.value[dim] {
                        self.value[dim]
                    } else {
                        node.min(dim)
                    }
                },
                (Some(left), Some(right)) => {
                    
                },
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
        let mut tree = KDTree::new([5,5]);

        tree.insert([5,5]);
        tree.insert([6,7]);
        tree.insert([8,9]);
        tree.insert([4,3]);
        tree.insert([2,1]);

        let vec = vec![1,2,3,4,5];
        println!("tree: {:?}", tree);
    }

    fn part_two(&self) {
        
    }
}