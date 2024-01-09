mod twenty_twenty_three;
use advent_of_code::solved::Solved;
use crate::twenty_twenty_three::day_17::Solution;

fn main() {
    let solution = Solution::new("src/twenty_twenty_three/input/day_17.txt");
    solution.part_one();
    solution.part_two();
}