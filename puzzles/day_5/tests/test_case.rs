use day_5::{get_answer_1, get_answer_2};

#[test]
fn test_case_1() {
    let input = r"0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";
    let answer_1 = get_answer_1(input);

    assert_eq!(answer_1, 5);
}

#[test]
fn test_case_2() {
    let input = r"0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";
    let answer_2 = get_answer_2(input);

    assert_eq!(answer_2, 12);
}
