use day_4::get_boards;

#[test]
fn test_make_new_board() {
    let input = r"22 13 17 11  0
8  2 23  4 24
21  9 14 16  7
6 10  3 18  5
1 12 20 15 19

3 15  0  2 22
9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6";
    let boards = get_boards(input);

    let board_0_row_2 = [Some(21), Some(9), Some(14), Some(16), Some(7)];

    assert_eq!(boards[0].board[2], board_0_row_2);

    assert_eq!(boards[1].numbers_remaining.len(), 25);
}