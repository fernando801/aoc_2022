use std::collections::HashSet;

fn part1() {
    let datastream = include_str!("input.txt");

    let mut start = 0;

    for i in 3..datastream.len() {
        let set: HashSet<char> = datastream[i - 3..i + 1].chars().collect();

        if set.len() == 4 {
            start = i + 1;
            break;
        }
    }

    println!("{}", &start);
}

fn part2() {
    let datastream = include_str!("input.txt");

    let mut start = 0;

    for i in 13..datastream.len() {
        let set: HashSet<char> = datastream[i - 13..i + 1].chars().collect();

        if set.len() == 14 {
            start = i + 1;
            break;
        }
    }

    println!("{}", &start);
}

fn main() {
    println!("Part 1:");
    part1();

    println!("");

    println!("Part 2:");
    part2();
}
