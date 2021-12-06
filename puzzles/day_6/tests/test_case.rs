use day_6::{get_answer_1, get_answer_2};

#[test]
fn test_case_1() {
    let input = [3,4,3,1,2];
    let answer_1 = get_answer_1(&input);

    assert_eq!(answer_1, 5934);
}

#[test]
fn test_case_2() {
    let input = [3,4,3,1,2];
    let answer_1 = get_answer_2(&input);

    assert_eq!(answer_1, 26984457539);
}