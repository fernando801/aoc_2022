use std::cmp;
use std::collections::{HashSet, VecDeque};

fn part1() {
    let input = include_str!("input.txt");

    let mut cubes: HashSet<(i32, i32, i32)> = HashSet::new();

    let mut surface_squares = 0;

    for line in input.lines() {
        let mut coord = line.splitn(3, ",");
        let x = coord.next().unwrap().parse().unwrap();
        let y = coord.next().unwrap().parse().unwrap();
        let z = coord.next().unwrap().parse().unwrap();

        cubes.insert((x, y, z));
    }

    for &(x, y, z) in cubes.iter() {
        for i in 1..=3 {
            let mut dx = 0;
            let mut dy = 0;
            let mut dz = 0;

            match i {
                1 => dx += 1,
                2 => dy += 1,
                3 => dz += 1,
                _ => panic!(),
            }

            for sign in [1, -1] {
                dx *= sign;
                dy *= sign;
                dz *= sign;

                if !cubes.contains(&(x + dx, y + dy, z + dz)) {
                    surface_squares += 1;
                }
            }
        }
    }

    println!("{}", surface_squares);
}

fn part2() {
    let input = include_str!("input.txt");

    let mut rock: HashSet<(i32, i32, i32)> = HashSet::new();
    let mut exterior: HashSet<(i32, i32, i32)> = HashSet::new();

    let mut surface_area = 0;

    let mut x_range = (i32::MAX, i32::MIN);
    let mut y_range = (i32::MAX, i32::MIN);
    let mut z_range = (i32::MAX, i32::MIN);

    for line in input.lines() {
        let mut coord = line.splitn(3, ",");
        let x = coord.next().unwrap().parse().unwrap();
        let y = coord.next().unwrap().parse().unwrap();
        let z = coord.next().unwrap().parse().unwrap();

        x_range = (cmp::min(x_range.0, x), cmp::max(x_range.1, x));
        y_range = (cmp::min(y_range.0, y), cmp::max(y_range.1, y));
        z_range = (cmp::min(z_range.0, z), cmp::max(z_range.1, z));

        rock.insert((x, y, z));
    }

    x_range = (x_range.0 - 1, x_range.1 + 1);
    y_range = (y_range.0 - 1, y_range.1 + 1);
    z_range = (z_range.0 - 1, z_range.1 + 1);

    let mut queue: VecDeque<(i32, i32, i32)> = VecDeque::from([(x_range.1, y_range.1, z_range.1)]);
    exterior.insert((x_range.1, y_range.1, z_range.1));

    while !queue.is_empty() {
        let (x, y, z) = queue.pop_front().unwrap();

        for i in 1..=3 {
            let mut dx = 0;
            let mut dy = 0;
            let mut dz = 0;

            match i {
                1 => dx += 1,
                2 => dy += 1,
                3 => dz += 1,
                _ => panic!(),
            }

            for sign in [1, -1] {
                dx *= sign;
                dy *= sign;
                dz *= sign;

                if rock.contains(&(x + dx, y + dy, z + dz)) {
                    surface_area += 1;
                } else if !exterior.contains(&(x + dx, y + dy, z + dz))
                    && x_range.0 <= x + dx
                    && x + dx <= x_range.1
                    && y_range.0 <= y + dy
                    && y + dy <= y_range.1
                    && z_range.0 <= z + dz
                    && z + dz <= z_range.1
                {
                    exterior.insert((x + dx, y + dy, z + dz));
                    queue.push_back((x + dx, y + dy, z + dz));
                }
            }
        }
    }

    println!("{}", surface_area);
}

fn main() {
    println!("Part 1:");
    part1();

    println!("");

    println!("Part 2:");
    part2();
}
