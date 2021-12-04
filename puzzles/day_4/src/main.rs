use day_4::{
    get_answer_1, get_answer_2, get_boards,
    input::{BOARDS, CALLED_NUMBERS},
};

fn main() {
    let mut boards = get_boards(BOARDS);

    let answer_1 = get_answer_1(&CALLED_NUMBERS, &mut boards);
    println!("Answer 1 is {}!", answer_1);

    let mut boards2 = get_boards(BOARDS);
    let answer_2 = get_answer_2(&CALLED_NUMBERS, &mut boards2);
    println!("Answer 2 is {}!", answer_2);
}
