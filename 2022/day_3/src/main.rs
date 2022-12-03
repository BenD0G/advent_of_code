mod input;
use input::INPUT;

fn priority_for_c(c: &char) -> u64 {
    let num = *c as u64;
    if num >= 97 {
        num - 96
    } else {
        num - 38
    }
}

fn register_character(priority: u64, bits: &mut u64) {
    *bits |= 2u64.pow(priority as u32);
}

fn solve_1() -> u64 {
    let mut total = 0;
    for line in INPUT.lines() {
        let (left, right) = line.split_at(line.len() / 2);
        let mut bits = 0;
        left.chars()
            .map(|c| priority_for_c(&c))
            .for_each(|p| register_character(p, &mut bits));

        for c in right.chars() {
            let prio = priority_for_c(&c);
            let bit = 2u64.pow(prio as u32);
            if bit & bits != 0 {
                total += prio;
                break;
            }
        }
    }
    total
}

fn bits_for_line(s: &str) -> u64 {
    let mut bits = 0;
    for c in s.chars() {
        bits |= 2u64.pow(priority_for_c(&c) as u32);
    }
    bits
}

fn solve_2() -> u64 {
    let mut total = 0;
    let mut all_bits = INPUT.lines().map(bits_for_line);
    for _ in 0..100 {
        let b1 = all_bits.next().unwrap();
        let b2 = all_bits.next().unwrap();
        let b3 = all_bits.next().unwrap();

        total += 63 - (b1 & b2 & b3).leading_zeros() as u64
    }
    total
}

fn main() {
    println!("{}", solve_1());
    println!("{}", solve_2());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prio() {
        assert_eq!(priority_for_c(&'a'), 1);
        assert_eq!(priority_for_c(&'A'), 27);
        assert_eq!(priority_for_c(&'z'), 26);
        assert_eq!(priority_for_c(&'Z'), 52);
    }

    #[test]
    fn test_bits_for_line() {
        assert_eq!(bits_for_line("a"), 2);
        assert_eq!(bits_for_line("b"), 4);
        assert_eq!(63 - 1048u64.leading_zeros(), 10);
    }
}
