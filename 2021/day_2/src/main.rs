use day_2::INPUT;

#[derive(std::cmp::PartialEq, Debug)]
enum Move {
    Forward(u32),
    Up(u32),
    Down(u32),
}

impl Move {
    fn from_str(s: &str) -> Self {
        let parts: Vec<&str> = s.split(' ').collect();
        let d: u32 = parts.last().unwrap().parse().unwrap();
        match parts.first().unwrap() {
            &"forward" => Move::Forward(d),
            &"up" => Move::Up(d),
            &"down" => Move::Down(d),
            foo => panic!("Unexpected text {}", foo),
        }
    }
}

struct Position {
    x: u32,
    depth: u32,
    aim: u32,
}

impl Position {
    fn move_sub(&mut self, movement: &Move) {
        match movement {
            &Move::Forward(d) => self.x += d,
            &Move::Up(d) => self.depth = self.depth.checked_sub(d).unwrap(),
            &Move::Down(d) => self.depth += d,
        }
    }

    fn move_sub_v2(&mut self, movement: &Move) {
        match movement {
            &Move::Forward(d) => {
                self.x += d;
                self.depth += d * self.aim;
            }
            &Move::Up(d) => self.aim = self.aim.checked_sub(d).unwrap(),
            &Move::Down(d) => self.aim += d,
        }
    }
}

fn parse(input: &str) -> Vec<Move> {
    input.split('\n').map(|s| Move::from_str(s)).collect()
}

fn get_answer_1(moves: &[Move]) -> u32 {
    let mut position = Position { x: 0, depth: 0, aim: 0 };
    for movement in moves {
        position.move_sub(movement);
    }
    position.depth * position.x
}

fn get_answer_2(moves: &[Move]) -> u32 {
    let mut position = Position { x: 0, depth: 0, aim: 0 };
    for movement in moves {
        position.move_sub_v2(movement);
    }
    position.depth * position.x
}

fn main() {
    let moves = parse(INPUT);
    let answer_1 = get_answer_1(&moves);
    println!("Answer 1 is {}!", answer_1);

    let answer_2 = get_answer_2(&moves);
    println!("Answer 2 is {}!", answer_2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case() {
        let input = r"forward 5
down 5
forward 8
up 3
down 8
forward 2";
        let moves = parse(input);
        let answer = get_answer_1(&moves);

        assert_eq!(moves.first().unwrap(), &Move::Forward(5));
        assert_eq!(moves.last().unwrap(), &Move::Forward(2));
        assert_eq!(answer, 150);

        let answer_2 = get_answer_2(&moves);
        assert_eq!(answer_2, 900);
    }
}
