mod input;

use input::INPUT;

fn make_input() -> [[u8; 99]; 99] {
    let mut ret = [[0; 99]; 99];
    for (row, l) in INPUT.lines().enumerate() {
        for (col, c) in l.chars().enumerate() {
            ret[row][col] = c.to_string().parse().unwrap();
        }
    }
    ret
}

/// Find the values which are greater than all values prior
fn get_visible<T>(values: T) -> Vec<bool>
where
    T: Iterator<Item = u8>,
{
    let mut max = 0;
    values
        .enumerate()
        .map(|(i, v)| {
            if i == 0 || v > max {
                max = v;
                true
            } else {
                false
            }
        })
        .collect()
}

/// For each value, find the number of values prior smaller than the current value and the last value
/// that was greater or equal to the current value.
fn get_scenic_scores<T>(values: T) -> Vec<u64>
where
    T: Iterator<Item = u8>,
{
    let mut latest_index_of_tree_with_height_at_least = [0u64; 10];
    values
        .enumerate()
        .map(|(i, height)| {
            let scenic_score =
                i as u64 - latest_index_of_tree_with_height_at_least[height as usize];
            for j in 0..(height + 1) {
                latest_index_of_tree_with_height_at_least[j as usize] = i as u64;
            }
            scenic_score
        })
        .collect()
}

fn is_visible_from_side<F>(grid: &[[u8; 99]; 99], index_mapper: F) -> [[bool; 99]; 99]
where
    F: Fn((usize, usize)) -> (usize, usize),
{
    let mut ret = [[false; 99]; 99];

    for i in 0..99 {
        let values = (0..99).map(|j| {
            let (i, j) = index_mapper((i, j));
            grid[i][j]
        });
        let visible = get_visible(values);
        for (j, v) in visible.iter().enumerate() {
            let (i, j) = index_mapper((i, j));
            ret[i][j] = *v;
        }
    }
    ret
}

fn scenic_scores_from_side<F>(grid: &[[u8; 99]; 99], index_mapper: F) -> [[u64; 99]; 99]
where
    F: Fn((usize, usize)) -> (usize, usize),
{
    let mut ret = [[0; 99]; 99];

    for i in 0..99 {
        let values = (0..99).map(|j| {
            let (i, j) = index_mapper((i, j));
            grid[i][j]
        });
        let visible = get_scenic_scores(values);
        for (j, v) in visible.iter().enumerate() {
            let (i, j) = index_mapper((i, j));
            ret[i][j] = *v;
        }
    }
    ret
}

fn solve_1() -> u64 {
    let mut total = 0;
    let grid = make_input();
    let left = is_visible_from_side(&grid, |(i, j)| (i, j));
    let right = is_visible_from_side(&grid, |(i, j)| (i, 98 - j));
    let top = is_visible_from_side(&grid, |(i, j)| (j, i));
    let bottom = is_visible_from_side(&grid, |(i, j)| (98 - j, i));

    for i in 0..99 {
        for j in 0..99 {
            if left[i][j] || right[i][j] || top[i][j] || bottom[i][j] {
                total += 1;
            }
        }
    }

    total
}

fn solve_2() -> u64 {
    let mut max = 0;
    let grid = make_input();
    let left = scenic_scores_from_side(&grid, |(i, j)| (i, j));
    let right = scenic_scores_from_side(&grid, |(i, j)| (i, 98 - j));
    let top = scenic_scores_from_side(&grid, |(i, j)| (j, i));
    let bottom = scenic_scores_from_side(&grid, |(i, j)| (98 - j, i));

    for i in 0..99 {
        for j in 0..99 {
            let score = left[i][j] * right[i][j] * top[i][j] * bottom[i][j];
            if score > max {
                max = score;
            }
        }
    }
    max
}

fn main() {
    println!("{}", solve_1());
    println!("{}", solve_2());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scenic_scores() {
        let values = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let scores = get_scenic_scores(values.iter().cloned());
        assert_eq!(scores, vec![0, 1, 2, 3, 4, 5, 6, 7, 8]);

        let values = vec![2, 5, 5, 1, 2];
        let scores = get_scenic_scores(values.iter().cloned());
        assert_eq!(scores, vec![0, 1, 1, 1, 2]);

        let values = vec![3, 5, 3, 5, 3];
        let scores = get_scenic_scores(values.iter().cloned());
        assert_eq!(scores, vec![0, 1, 1, 2, 1]);
    }
}
