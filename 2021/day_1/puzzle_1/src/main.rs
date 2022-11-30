use puzzle_1::{input::INPUT, make_vec};

fn get_answer(input: &[u32]) -> u32 {
    input
        .iter()
        .zip(input.iter().skip(1))
        .map(|(x, y)| (y > x) as u32)
        .sum()
}

fn main() {
    let v = make_vec(INPUT);
    if v.first().unwrap() != &176 || v.last().unwrap() != &5596 {
        panic!("Unexpected vector")
    }

    let answer = get_answer(&v);

    println!("Answer is {}!", answer);
}

#[cfg(test)]
mod tests {
    use crate::{get_answer, make_vec};

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
        assert_eq!(v.first().unwrap(), &199);
        assert_eq!(v.last().unwrap(), &263);

        assert_eq!(get_answer(&v), 7);
    }
}
