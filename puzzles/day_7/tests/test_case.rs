use day_7::{get_answer_1, get_answer_2};

#[test]
fn test_case_1() {
    let input = [16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
    let answer_1 = get_answer_1(&input);

    assert_eq!(answer_1, 37);
}

#[test]
fn test_case_2() {
    let input = [16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
    let answer_1 = get_answer_2(&input);

    assert_eq!(answer_1, 168);
}
