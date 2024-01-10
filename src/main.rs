mod twenty_twenty_three;
use advent_of_code::solved::Solved;
use crate::twenty_twenty_three::day_18::Solution;

fn main() {
    let solution = Solution::new("src/twenty_twenty_three/input/day_18.txt");
    solution.part_one();
    solution.part_two();
}