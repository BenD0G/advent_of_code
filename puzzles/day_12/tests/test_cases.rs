use day_12::get_answer_1;

#[test]
fn test_case_1() {
    let input = r"start-A
start-b
A-c
A-b
b-d
A-end
b-end";
    let answer_1 = get_answer_1(input);
    assert_eq!(answer_1, 10);
}

#[test]
fn test_case_2() {
    let input = r"dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc";
    let answer_1 = get_answer_1(input);
    assert_eq!(answer_1, 19);
}

#[test]
fn test_case_3() {
    let input = r"fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW";
    let answer_1 = get_answer_1(input);
    assert_eq!(answer_1, 226);
}
