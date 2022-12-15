use std::cmp;
use std::collections::hash_map::Entry;
use std::collections::HashMap;

fn part1() {
    let input = include_str!("input.txt");

    let mut map: HashMap<(i32, i32), char> = HashMap::new();

    let mut min_x = i32::MAX;
    let mut max_x = i32::MIN;
    let mut max_y = i32::MIN;

    for line in input.lines() {
        let vrtx: Vec<(i32, i32)> = line
            .split(" -> ")
            .map(|pair| {
                let (x, y) = pair.split_once(",").unwrap();
                (x.parse().unwrap(), y.parse().unwrap())
            })
            .collect();

        for i in 1..vrtx.len() {
            // let x_min = vec![vrtx[i - 1].0, vrtx[i].0, min_x];
            // let x_max = vec![vrtx[i - 1].0, vrtx[i].0, max_x];
            // min_x = x_min.iter().min().copied().unwrap();
            // max_x = x_max.iter().max().copied().unwrap();

            min_x = cmp::min(cmp::min(vrtx[i - 1].0, vrtx[i].0), min_x);
            max_x = cmp::max(cmp::max(vrtx[i - 1].0, vrtx[i].0), max_x);
            max_y = cmp::max(cmp::max(vrtx[i - 1].1, vrtx[i].1), max_y);

            if vrtx[i - 1].0 == vrtx[i].0 {
                let x = vrtx[i].0;
                let y1 = vrtx[i - 1].1;
                let y2 = vrtx[i].1;

                for y in cmp::min(y1, y2)..cmp::max(y1, y2) + 1 {
                    map.insert((x, y), '#');
                }
            } else {
                let y = vrtx[i].1;
                let x1 = vrtx[i - 1].0;
                let x2 = vrtx[i].0;

                for x in cmp::min(x1, x2)..cmp::max(x1, x2) + 1 {
                    map.insert((x, y), '#');
                }
            }
        }
    }

    let mut sand_units = 0;

    'sand_drop: loop {
        let mut sand = (500, 0);

        loop {
            if sand.0 < min_x || sand.0 > max_x || sand.1 > max_y {
                break 'sand_drop;
            }

            if let Entry::Vacant(_) = map.entry((sand.0, sand.1 + 1)) {
                sand = (sand.0, sand.1 + 1);
                continue;
            }

            if let Entry::Vacant(_) = map.entry((sand.0 - 1, sand.1 + 1)) {
                sand = (sand.0 - 1, sand.1 + 1);
                continue;
            }

            if let Entry::Vacant(_) = map.entry((sand.0 + 1, sand.1 + 1)) {
                sand = (sand.0 + 1, sand.1 + 1);
                continue;
            }

            map.insert(sand, 'o');
            sand_units += 1;
            break;
        }
    }

    println!("{}", sand_units);
}

fn part2() {
    let input = include_str!("input.txt");

    let mut map: HashMap<(i32, i32), char> = HashMap::new();

    let mut max_y = i32::MIN;

    for line in input.lines() {
        let vrtx: Vec<(i32, i32)> = line
            .split(" -> ")
            .map(|pair| {
                let (x, y) = pair.split_once(",").unwrap();
                (x.parse().unwrap(), y.parse().unwrap())
            })
            .collect();

        for i in 1..vrtx.len() {
            max_y = cmp::max(cmp::max(vrtx[i - 1].1, vrtx[i].1), max_y);

            if vrtx[i - 1].0 == vrtx[i].0 {
                let x = vrtx[i].0;
                let y1 = vrtx[i - 1].1;
                let y2 = vrtx[i].1;

                for y in cmp::min(y1, y2)..cmp::max(y1, y2) + 1 {
                    map.insert((x, y), '#');
                }
            } else {
                let y = vrtx[i].1;
                let x1 = vrtx[i - 1].0;
                let x2 = vrtx[i].0;

                for x in cmp::min(x1, x2)..cmp::max(x1, x2) + 1 {
                    map.insert((x, y), '#');
                }
            }
        }
    }

    let mut sand_units = 0;

    'sand_drop: loop {
        let mut sand = (500, 0);

        loop {
            if sand.1 > max_y {
                map.insert(sand, 'o');
                sand_units += 1;
                if sand == (500, 0) {
                    break 'sand_drop;
                } else {
                    break;
                }
            }

            if let Entry::Vacant(_) = map.entry((sand.0, sand.1 + 1)) {
                sand = (sand.0, sand.1 + 1);
                continue;
            }

            if let Entry::Vacant(_) = map.entry((sand.0 - 1, sand.1 + 1)) {
                sand = (sand.0 - 1, sand.1 + 1);
                continue;
            }

            if let Entry::Vacant(_) = map.entry((sand.0 + 1, sand.1 + 1)) {
                sand = (sand.0 + 1, sand.1 + 1);
                continue;
            }

            map.insert(sand, 'o');
            sand_units += 1;
            if sand == (500, 0) {
                break 'sand_drop;
            } else {
                break;
            }
        }
    }

    println!("{}", sand_units);
}

fn main() {
    println!("Part 1:");
    part1();

    println!("");

    println!("Part 2:");
    part2();
}
