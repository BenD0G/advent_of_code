use puzzle_1::{input::INPUT, make_vec};

fn get_answer(input: &[u32]) -> u32 {
    input
        .iter()
        .zip(input.iter().skip(3))
        .map(|(x, y)| (y > x) as u32)
        .sum()
}

fn main() {
    let v = make_vec(INPUT);
    let answer = get_answer(&v);
    println!("Answer is {}!", answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case() {
        let input = r"199
200
208
210
200
207
240
269
260
263";
        let v = make_vec(input);
        let answer = get_answer(&v);
        assert_eq!(answer, 5);
    }
}
