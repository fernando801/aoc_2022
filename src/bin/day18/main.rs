use std::collections::HashSet;

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

fn part2() {}

fn main() {
    println!("Part 1:");
    part1();

    println!("");

    println!("Part 2:");
    part2();
}
