use std::ops::{Index, IndexMut};

pub const FISH_TRACKER_LENGTH: usize = 9;

pub struct FishTracker {
    counts: [u64; FISH_TRACKER_LENGTH],
    zero_index: usize,
}

impl FishTracker {
    fn rotate(&mut self) {
        self.zero_index = (self.zero_index + 1) % FISH_TRACKER_LENGTH;
    }

    pub fn update_values(&mut self) {
        let zero_count = self[0];
        self.rotate();
        self[FISH_TRACKER_LENGTH - 1] = zero_count;
        self[6] += zero_count;
    }

    pub fn get_sum(&self) -> u64 {
        self.counts.iter().sum()
    }

    pub fn new(initial_values: &[u64; FISH_TRACKER_LENGTH]) -> Self {
        Self {
            counts: initial_values.clone(),
            zero_index: 0,
        }
    }
}

impl Index<usize> for FishTracker {
    type Output = u64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.counts[(self.zero_index + index) % FISH_TRACKER_LENGTH]
    }
}

impl IndexMut<usize> for FishTracker {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.counts[(self.zero_index + index) % FISH_TRACKER_LENGTH]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_rotated_counts(tracker: &FishTracker) -> Vec<u64> {
        (0..FISH_TRACKER_LENGTH).map(|i| tracker[i]).collect()
    }

    #[test]
    fn test_basic() {
        let counts = [0, 1, 1, 2, 1, 0, 0, 0, 0];
        let mut tracker = FishTracker::new(&counts);

        let counts_0 = get_rotated_counts(&tracker);
        assert_eq!(&counts_0, &counts);

        tracker.update_values();
        let counts_1 = get_rotated_counts(&tracker);
        assert_eq!(&counts_1, &[1, 1, 2, 1, 0, 0, 0, 0, 0]);

        tracker.update_values();
        let counts_2 = get_rotated_counts(&tracker);
        assert_eq!(&counts_2, &[1, 2, 1, 0, 0, 0, 1, 0, 1]);
    }
}
