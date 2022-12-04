mod input;
use input::INPUT;

fn solve_1() -> u64 {
    let mut total = 0;
    for line in INPUT.lines() {
        let (left_bounds, right_bounds) = line.split_once(',').unwrap();
        let (left_lower, left_upper) = left_bounds.split_once('-').unwrap();
        let (right_lower, right_upper) = right_bounds.split_once('-').unwrap();

        let left_lower = left_lower.parse::<u64>().unwrap();
        let left_upper = left_upper.parse::<u64>().unwrap();
        let right_lower = right_lower.parse::<u64>().unwrap();
        let right_upper = right_upper.parse::<u64>().unwrap();

        if left_lower <= right_lower && left_upper >= right_upper
            || right_lower <= left_lower && right_upper >= left_upper
        {
            total += 1;
        }
    }
    total
}

fn solve_2() -> u64 {
    let mut total = 0;
    for line in INPUT.lines() {
        let (left_bounds, right_bounds) = line.split_once(',').unwrap();
        let (left_lower, left_upper) = left_bounds.split_once('-').unwrap();
        let (right_lower, right_upper) = right_bounds.split_once('-').unwrap();

        let left_lower = left_lower.parse::<u64>().unwrap();
        let left_upper = left_upper.parse::<u64>().unwrap();
        let right_lower = right_lower.parse::<u64>().unwrap();
        let right_upper = right_upper.parse::<u64>().unwrap();

        if left_upper >= right_lower && left_upper <= right_upper
            || right_upper >= left_lower && right_upper <= left_upper
        {
            total += 1;
        }
    }
    total
}

fn main() {
    println!("{}", solve_1());
    println!("{}", solve_2());
}
