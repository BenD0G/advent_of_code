use std::collections::BTreeSet;

use crate::Board;
use crate::HasWon;

pub fn get_answer_1(called_numbers: &[u8], boards: &mut [Board]) -> u32 {
    for number in called_numbers {
        for board in boards.iter_mut() {
            let has_won = board.call(*number);
            if let HasWon::True = has_won {
                return board.construct_final_answer(*number);
            }
        }
    }
    panic!("Reached the end")
}

pub fn get_answer_2(called_numbers: &[u8], boards: &mut [Board]) -> u32 {
    let mut won_indicies = BTreeSet::new();
    let num_boards = boards.len();
    for number in called_numbers {
        for (i, board) in boards.iter_mut().enumerate() {
            if won_indicies.contains(&i) {
                continue;
            }
            let has_won = board.call(*number);
            if let HasWon::True = has_won {
                if num_boards - won_indicies.len() == 1 {
                    // Last board
                    return board.construct_final_answer(*number);
                }
                won_indicies.insert(i);
            }
        }
    }
    panic!("Reached the end")
}