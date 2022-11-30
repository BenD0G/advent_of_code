use crate::Board;

pub fn get_boards(input: &str) -> Vec<Board> {
    input.split("\n\n").map(Board::new_from_str).collect()
}
