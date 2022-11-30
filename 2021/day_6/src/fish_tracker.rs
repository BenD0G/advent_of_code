use std::ops::{Index, IndexMut};

pub const FISH_TRACKER_LENGTH: usize = 9;

pub struct FishTracker {
    counts: [u64; FISH_TRACKER_LENGTH],
    zero_index: usize,
    scratch_space: [u64; FISH_TRACKER_LENGTH],
}

impl FishTracker {
    fn rotate(&mut self, n: usize) {
        self.zero_index = (self.zero_index + n) % FISH_TRACKER_LENGTH;
    }

    pub fn update_values(&mut self) {
        let zero_count = self[0];
        self.rotate(1);
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
            scratch_space: [0; FISH_TRACKER_LENGTH],
        }
    }

    /// Advance 8 days!
    /// You'll just have to trust me (and the unit tests).
    pub fn update_values_8(&mut self) {
        (0..5).for_each(|i| self.scratch_space[i] = self[i + 2]);
        self.scratch_space[5] = self[0] + self[7];
        self.scratch_space[6] = 0;
        self.scratch_space[7] = self[0];
        self.scratch_space[8] = self[1];

        for i in 0..FISH_TRACKER_LENGTH {
            self[i] += self.scratch_space[i];
        }

        self.rotate(8);
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
    use crate::get_counts;

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

    #[test]
    fn test_update_values_8() {
        let counts = [0, 1, 1, 2, 1, 0, 0, 0, 0];
        let mut tracker = FishTracker::new(&counts);

        tracker.update_values_8();
        let counts_8 = get_rotated_counts(&tracker);
        assert_eq!(&counts_8, &[1, 1, 3, 2, 2, 1, 0, 0, 0]);

        let expected_counts_16 = get_counts(&[
            1, 2, 1, 6, 0, 1, 2, 3, 3, 4, 1, 2, 3, 3, 3, 4, 4, 5, 5, 6, 8,
        ]);
        tracker.update_values_8();
        assert_eq!(&get_rotated_counts(&tracker), &expected_counts_16);
    }
}
