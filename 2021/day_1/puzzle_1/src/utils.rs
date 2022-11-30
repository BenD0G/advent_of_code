pub fn make_vec(input: &str) -> Vec<u32> {
    input.split('\n').map(|x| x.parse().unwrap()).collect()
}
