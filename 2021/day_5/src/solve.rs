use std::collections::BTreeSet;

use crate::Line;

/// Keeps track of how many lines have passed through each point
#[derive(Default)]
struct PointStatus {
    one_passed_through: BTreeSet<(u32, u32)>,
    at_least_two_passed_through: BTreeSet<(u32, u32)>,
}


pub fn get_answer_1(input: &str) -> usize {
    let lines: Vec<Line> = input.split('\n').map(|s| Line::parse(s)).collect();
    let mut point_status: PointStatus = PointStatus::default();
    for line in lines {
        if !line.is_horiz_or_vert() {
            continue;
        }
        let points = line.get_points();
        for point in points {
            if point_status.one_passed_through.contains(&point) {
                point_status.one_passed_through.remove(&point);
                point_status.at_least_two_passed_through.insert(point);
            } else if !point_status.at_least_two_passed_through.contains(&point) {
                point_status.one_passed_through.insert(point);
            }
        }
    }
    point_status.at_least_two_passed_through.len()
}

pub fn get_answer_2(input: &str) -> usize {
    let lines: Vec<Line> = input.split('\n').map(|s| Line::parse(s)).collect();
    let mut point_status: PointStatus = PointStatus::default();
    for line in lines {
        let points = line.get_points();
        for point in points {
            if point_status.one_passed_through.contains(&point) {
                point_status.one_passed_through.remove(&point);
                point_status.at_least_two_passed_through.insert(point);
            } else if !point_status.at_least_two_passed_through.contains(&point) {
                point_status.one_passed_through.insert(point);
            }
        }
    }
    point_status.at_least_two_passed_through.len()
}