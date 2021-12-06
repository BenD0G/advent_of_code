use day_6::{get_answer_1, get_answer_2, input::INPUT};

#[test]
fn test_case_1() {
    let input = [3, 4, 3, 1, 2];
    let answer_1 = get_answer_1(&input);

    assert_eq!(answer_1, 5934);
}

#[test]
fn test_case_2() {
    let input = [3, 4, 3, 1, 2];
    let answer_1 = get_answer_2(&input);

    assert_eq!(answer_1, 26984457539);
}

#[test]
fn full_test_case_1() {
    let answer_1 = get_answer_1(&INPUT);
    assert_eq!(answer_1, 354564);
}

#[test]
fn full_test_case_2() {
    let answer_2 = get_answer_2(&INPUT);
    assert_eq!(answer_2, 1609058859115);
}
