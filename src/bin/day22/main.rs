use regex::Regex;
use std::cmp;
use std::collections::{HashMap, VecDeque};

fn part1() {
    let input = include_str!("input.txt");

    let (map_str, path) = input.split_once("\n\n").unwrap();

    let mut map: HashMap<(i32, i32), char> = HashMap::new();
    let mut row_limits: HashMap<i32, (i32, i32)> = HashMap::new();
    let mut col_limits: HashMap<i32, (i32, i32)> = HashMap::new();

    for (row, line) in map_str
        .lines()
        .into_iter()
        .enumerate()
        .map(|(i, l)| (i as i32, l))
    {
        for (col, c) in line
            .char_indices()
            .into_iter()
            .filter(|&(_, c)| c != ' ')
            .map(|(i, c)| (i as i32, c))
        {
            let row_lims = row_limits.entry(row).or_insert((i32::MAX, i32::MIN));
            let col_lims = col_limits.entry(col).or_insert((i32::MAX, i32::MIN));

            *row_lims = (cmp::min(row_lims.0, col), cmp::max(row_lims.1, col));
            *col_lims = (cmp::min(col_lims.0, row), cmp::max(col_lims.1, row));

            map.insert((row, col), c);
        }
    }

    let path = format!("R{path}");

    let re = Regex::new(r"\w(\d+)").unwrap();

    let mut directions: VecDeque<(i32, i32)> = VecDeque::from([(-1, 0), (0, 1), (1, 0), (0, -1)]);
    let mut curr = (0, row_limits.get(&0).copied().unwrap().0);

    for m in re.find_iter(&path).map(|m| m.as_str()) {
        let d = &m[0..1];
        let n: u32 = (&m[1..]).parse().unwrap();

        match d {
            "L" => {
                let l = directions.pop_back().unwrap();
                directions.push_front(l);
            }
            "R" => {
                let r = directions.pop_front().unwrap();
                directions.push_back(r);
            }
            _ => panic!(),
        }

        let dir = directions.front().unwrap();
        for _ in 0..n {
            let mut next = (curr.0 + dir.0, curr.1 + dir.1);

            if map.get(&next).is_none() {
                next = match dir {
                    (-1, 0) => (col_limits.get(&curr.1).copied().unwrap().1, next.1),
                    (1, 0) => (col_limits.get(&curr.1).copied().unwrap().0, next.1),
                    (0, -1) => (next.0, row_limits.get(&curr.0).copied().unwrap().1),
                    (0, 1) => (next.0, row_limits.get(&curr.0).copied().unwrap().0),
                    _ => panic!(),
                };
            }

            if map.get(&next).copied().unwrap() == '#' {
                break;
            }

            curr = next;
        }
    }

    let facing_value = match directions.front().unwrap() {
        (1, 0) => 1,
        (0, 1) => 0,
        (-1, 0) => 3,
        (0, -1) => 2,
        _ => panic!(),
    };

    let password = 1000 * (curr.0 + 1) + 4 * (curr.1 + 1) + facing_value;

    println!("{}", password);
}

fn part2() {}

fn main() {
    println!("Part 1:");
    part1();

    println!("");

    println!("Part 2:");
    part2();
}
