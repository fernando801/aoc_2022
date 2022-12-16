use priority_queue::PriorityQueue;
use std::cmp;
use std::collections::HashSet;

fn part1() {
    let input = include_str!("input.txt");

    let row_y = 2000000;

    let pairs: Vec<((i64, i64), (i64, i64))> = input
        .lines()
        .map(|line| &line["Sensor at x=".len()..])
        .map(|trimmed| {
            let (s_str, b_str) = trimmed.split_once(": closest beacon is at x=").unwrap();

            let sensor: (i64, i64) = s_str
                .split_once(", y=")
                .map(|(x_str, y_str)| (x_str.parse().unwrap(), y_str.parse().unwrap()))
                .unwrap();

            let beacon: (i64, i64) = b_str
                .split_once(", y=")
                .map(|(x_str, y_str)| (x_str.parse().unwrap(), y_str.parse().unwrap()))
                .unwrap();

            (sensor, beacon)
        })
        .collect();

    let mut position_covered = HashSet::new();

    for &((s_x, s_y), (b_x, b_y)) in &pairs {
        let m_distance = (s_x - b_x).abs() + (s_y - b_y).abs();
        let d = m_distance - (s_y - row_y).abs();

        for x in s_x - d..s_x + d + 1 {
            position_covered.insert((x, row_y));
        }

        position_covered.remove(&(b_x, b_y));
    }

    println!("{}", &position_covered.len());
}

fn part2() {}

fn main() {
    println!("Part 1:");
    part1();

    println!("");

    println!("Part 2:");
    part2();
}
