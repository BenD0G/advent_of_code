use std::collections::BTreeMap;

pub fn get_answer_1(input: &[u32]) -> u32 {
    let mut crabs_in_positions: BTreeMap<u32, u32> = BTreeMap::new();
    for n in input {
        if crabs_in_positions.contains_key(&n) {
            crabs_in_positions.insert(*n, crabs_in_positions[&n] + 1);
        } else {
            crabs_in_positions.insert(*n, 1);
        }
    }

    let total_crabs = input.len() as u32;

    let latest_l1_sum: u32 = input.iter().sum();
    let mut latest_l1_sum = latest_l1_sum as i32;
    let mut last_position = 0;
    let mut num_crabs_to_left = 0;

    let mut best_l1_sum = latest_l1_sum;

    for (position, num_crabs) in crabs_in_positions.iter() {
        let position_delta = position - last_position;
        latest_l1_sum += position_delta as i32 * num_crabs_to_left as i32
            - position_delta as i32 * (total_crabs as i32 - num_crabs_to_left as i32);

        if latest_l1_sum < best_l1_sum {
            best_l1_sum = latest_l1_sum;
        }

        num_crabs_to_left += num_crabs;
        last_position = *position;
    }
    best_l1_sum as u32
}

pub fn get_answer_2(input: &[u32]) -> u32 {
    let mut crabs_in_positions: BTreeMap<u32, u32> = BTreeMap::new();
    for n in input {
        if crabs_in_positions.contains_key(&n) {
            crabs_in_positions.insert(*n, crabs_in_positions[&n] + 1);
        } else {
            crabs_in_positions.insert(*n, 1);
        }
    }

    let total_crabs = input.len() as u32;

    let latest_l1_sum: i32 = crabs_in_positions
        .iter()
        .map(|(pos, count)| (count * pos * (pos + 1) / 2) as i32)
        .sum();
    let mut latest_l1_sum = latest_l1_sum as i32;
    let mut last_position = 0;
    let mut num_crabs_to_left: u32 = 0;

    let mut best_l1_sum = latest_l1_sum;

    let mut latest_left_delta = 0;
    let mut latest_right_delta: i32 = crabs_in_positions
        .iter()
        .map(|(pos, count)| (pos * count) as i32)
        .sum();

    for (position, num_crabs) in crabs_in_positions.iter() {
        let position_delta = position - last_position;

        // From just after last position up current position
        for _offset in 1..=position_delta {
            latest_left_delta += num_crabs_to_left as i32;

            latest_l1_sum += latest_left_delta;
            latest_l1_sum -= latest_right_delta;

            latest_right_delta -= total_crabs as i32 - num_crabs_to_left as i32;

            if latest_l1_sum < best_l1_sum {
                best_l1_sum = latest_l1_sum;
            }
        }

        num_crabs_to_left += num_crabs;
        last_position = *position;
    }
    best_l1_sum as u32
}
