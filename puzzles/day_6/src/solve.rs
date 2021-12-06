use crate::{FishTracker, FISH_TRACKER_LENGTH};

fn get_counts(input: &[u8]) -> [u64; FISH_TRACKER_LENGTH] {
    let mut counts = [0; FISH_TRACKER_LENGTH];
    for x in input {
        counts[*x as usize] += 1;
    }
    counts
}

pub fn get_answer_1(input: &[u8]) -> u64 {
    let counts = get_counts(input);
    let mut tracker = FishTracker::new(&counts);
    (0..80).for_each(|_| tracker.update_values());
    tracker.get_sum()
}

pub fn get_answer_2(input: &[u8]) -> u64 {
    let counts = get_counts(input);
    let mut tracker = FishTracker::new(&counts);
    (0..256).for_each(|_| tracker.update_values());
    tracker.get_sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_counts() {
        let input = [3, 4, 3, 1, 2];
        let counts = get_counts(&input);
        assert_eq!(counts, [0, 1, 1, 2, 1, 0, 0, 0, 0]);
    }
}
