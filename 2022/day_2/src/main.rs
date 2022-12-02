mod input;
use input::INPUT;

fn solve_1() -> i32 {
    let mut total_score = 0;
    for line in INPUT.lines() {
        let mut chars = line.chars();
        // villain is {0,1,2}; hero is {1,2,3}
        let villain = chars.next().unwrap() as i32 - 65;
        let hero = chars.skip(1).next().unwrap() as i32 - 87;

        total_score += hero + ((hero - villain).rem_euclid(3) * 3);
    }
    total_score
}

fn solve_2() -> i32 {
    let mut total_score = 0;
    for line in INPUT.lines() {
        let mut chars = line.chars();
        // villain is {0,1,2}; outcome is {0,1,2}
        let villain = chars.next().unwrap() as i32 - 65;
        let outcome = chars.skip(1).next().unwrap() as i32 - 88;

        let hero = (outcome + villain - 1).rem_euclid(3);

        total_score += outcome * 3 + hero + 1;
    }
    total_score
}

fn main() {
    let solution_1 = solve_1();
    println!("{}", solution_1);

    let solution_2 = solve_2();
    println!("{}", solution_2);
}
