
pub mod solved {
    pub trait Solved {
        fn new(path: &str) -> Self;
        fn part_one(&self);
        fn part_two(&self);
    }
}
