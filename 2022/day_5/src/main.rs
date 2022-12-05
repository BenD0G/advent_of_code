mod input;
use input::{CRATES, MOVES};

fn moves() -> impl Iterator<Item = (usize, usize, usize)> {
    MOVES.lines().map(|l| {
        let split: Vec<_> = l.split(" ").collect();
        (
            split[1].parse().unwrap(),
            split[3].parse::<usize>().unwrap() - 1,
            split[5].parse::<usize>().unwrap() - 1,
        )
    })
}

fn init_crates() -> [Vec<char>; 9] {
    let mut crates: [Vec<char>; 9] = Default::default();
    for line in CRATES.lines().rev() {
        let chars = line.as_bytes();
        for i in 0usize..9 {
            let index = 1 + 4 * i;
            let c = std::char::from_u32(chars[index] as u32).unwrap();
            if c != ' ' {
                crates[i].push(c);
            }
        }
    }
    crates
}

fn move_crates_1<T>(crates: &mut [Vec<char>; 9], moves: T)
where
    T: Iterator<Item = (usize, usize, usize)>,
{
    for (count, from, to) in moves {
        for _ in 0..count {
            let c = crates[from].pop().unwrap();
            crates[to].push(c);
        }
    }
}

fn move_crates_2<T>(crates: &mut [Vec<char>; 9], moves: T)
where
    T: Iterator<Item = (usize, usize, usize)>,
{
    for (count, from, to) in moves {
        let mut chars = vec![];
        for _ in 0..count {
            let c = crates[from].pop().unwrap();
            chars.push(c);
        }
        for c in chars.into_iter().rev() {
            crates[to].push(c);
        }
    }
}

fn make_answer(crates: &[Vec<char>; 9]) -> String {
    let mut s = String::new();
    for i in 0..9 {
        s.push(*crates[i].last().unwrap());
    }
    s
}

fn solve_1() -> String {
    let mut crates = init_crates();
    let moves = moves();
    move_crates_1(&mut crates, moves);
    make_answer(&crates)
}

fn solve_2() -> String {
    let mut crates = init_crates();
    let moves = moves();
    move_crates_2(&mut crates, moves);
    make_answer(&crates)
}

fn main() {
    println!("{}", solve_1());
    println!("{}", solve_2());
}
