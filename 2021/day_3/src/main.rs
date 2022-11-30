use day_3::INPUT;

const MAX_LENGTH: usize = 12;

fn parse_binary(s: &str) -> u64 {
    s.chars()
        .rev()
        .enumerate()
        .map(|(i, c)| match c {
            '0' => 0,
            '1' => 2u64.pow(i as u32),
            _ => panic!("Unexpected char {}", c),
        })
        .sum()
}

fn get_vec(s: &str) -> Vec<u64> {
    s.split('\n').map(parse_binary).collect()
}

/// Given an array in little-endian, and 1 for each 1 in our number.
fn update_count(n: &u64, counts: &mut [u64]) {
    for i in 0..MAX_LENGTH {
        counts[i] += (n >> i) % 2;
    }
}

fn get_counts(inputs: &[u64]) -> [u64; MAX_LENGTH] {
    let mut counts = [0; 12];
    for n in inputs {
        update_count(n, &mut counts);
    }
    counts
}

fn make_gamma(num_inputs: u64, counts: &[u64; MAX_LENGTH]) -> u64 {
    let mid = num_inputs / 2;
    counts
        .iter()
        .enumerate()
        .map(|(i, c)| {
            if c > &mid {
                2u64.pow(i as u32)
            } else if c < &mid {
                0
            } else {
                panic!("Ambiguous")
            }
        })
        .sum()
}

fn get_most_common_bit(numbers: &[u64], shift: usize) -> Option<u64> {
    let new_shift = MAX_LENGTH - shift - 1;
    let num_ones: u64 = numbers.iter().map(|x| (x >> new_shift) % 2).sum();
    let num_zeroes = numbers.len() as u64 - num_ones;

    if num_ones > num_zeroes {
        return Some(1);
    } else if num_ones < num_zeroes {
        return Some(0);
    } else {
        return None;
    }
}

fn get_answer_2(o2: &[u64], co2: &[u64], position: usize) -> u64 {
    if o2.len() == 1 && co2.len() == 1 {
        return co2[0] * o2[0];
    } else if o2.len() == 1 {
        let comparison = 2u64.pow((MAX_LENGTH - position - 1) as u32);
        // Need to sort CO2
        let most_common_bit = get_most_common_bit(&co2, position);
        let need_one = match most_common_bit {
            Some(1) => false,
            Some(0) => true,
            None => false,
            _ => panic!(),
        };

        let mut new_co2: Vec<u64> = vec![];
        for number in co2.into_iter() {
            let has_one = (number & comparison) != 0;
            match (has_one, need_one) {
                (true, true) => new_co2.push(*number),
                (false, false) => new_co2.push(*number),
                _ => (),
            }
        }
        return get_answer_2(&o2, &new_co2, position + 1);
    } else if co2.len() == 1 {
        let comparison = 2u64.pow((MAX_LENGTH - position - 1) as u32);
        // Need to sort O2
        let most_common_bit = get_most_common_bit(&o2, position);
        let need_one = match most_common_bit {
            Some(1) => true,
            Some(0) => false,
            None => true,
            _ => panic!(),
        };

        let mut new_o2: Vec<u64> = vec![];
        for number in o2.into_iter() {
            let has_one = (number & comparison) != 0;
            match (has_one, need_one) {
                (true, true) => new_o2.push(*number),
                (false, false) => new_o2.push(*number),
                _ => (),
            }
        }
        return get_answer_2(&new_o2, &co2, position + 1);
    } else {
        let comparison = 2u64.pow((MAX_LENGTH - position - 1) as u32);
        // Need to sort O2
        let most_common_bit = get_most_common_bit(&o2, position);
        let need_one = match most_common_bit {
            Some(1) => true,
            Some(0) => false,
            None => true,
            _ => panic!(),
        };

        let mut new_o2: Vec<u64> = vec![];
        for number in o2.into_iter() {
            let has_one = (number & comparison) != 0;
            match (has_one, need_one) {
                (true, true) => new_o2.push(*number),
                (false, false) => new_o2.push(*number),
                _ => (),
            }
        }

        // Need to sort CO2
        let most_common_bit = get_most_common_bit(&co2, position);
        let need_one = match most_common_bit {
            Some(1) => false,
            Some(0) => true,
            None => false,
            _ => panic!(),
        };

        let mut new_co2: Vec<u64> = vec![];
        for number in co2.into_iter() {
            let has_one = (number & comparison) != 0;
            match (has_one, need_one) {
                (true, true) => new_co2.push(*number),
                (false, false) => new_co2.push(*number),
                _ => (),
            }
        }
        return get_answer_2(&new_o2, &new_co2, position + 1);
    }
}

fn main() {
    let numbers = get_vec(INPUT);
    let counts = get_counts(&numbers);
    let gamma = make_gamma(numbers.len() as u64, &counts);
    let epsilon = 2u64.pow(MAX_LENGTH as u32) - 1 - gamma;

    let answer_1 = gamma * epsilon;
    println!("Answer 1 is {}!", answer_1);

    let answer_2 = get_answer_2(&numbers, &numbers, 0);
    println!("Answer 2 is {}!", answer_2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let s = "000000010011";
        let s2 = "111111111111";

        assert_eq!(parse_binary(s), 19);
        assert_eq!(parse_binary(s2), 2u64.pow(12) - 1);
    }

    #[test]
    fn test_bit_logic() {
        let f = |n: u64, i: u32| (n >> i) % 2;

        assert_eq!(f(19, 0), 1);
        assert_eq!(f(19, 1), 1);
        assert_eq!(f(19, 2), 0);
        assert_eq!(f(19, 3), 0);
        assert_eq!(f(19, 4), 1);

        assert_eq!(f(2u64.pow(12) - 1, 11), 1);
    }

    #[test]
    fn test_case() {
        let input = r"00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

        let numbers = get_vec(input);
        let counts = get_counts(&numbers);
        let gamma = make_gamma(12, &counts);
        let epsilon = 2u64.pow(5 as u32) - 1 - gamma;
        let answer_1 = gamma * epsilon;

        assert_eq!(gamma, 22);
        assert_eq!(epsilon, 9);
        assert_eq!(answer_1, 198);
    }
}
