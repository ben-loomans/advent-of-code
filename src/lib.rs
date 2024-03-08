
pub mod solved {
    pub trait Solved {
        fn new(path: &str) -> Self;
        fn part_one(&mut self);
        fn part_two(&mut self);
    }
}
