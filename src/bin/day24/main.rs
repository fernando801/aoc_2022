use num::integer;
use std::collections::{HashSet, VecDeque};

fn min_travel_time(
    blizzards: &[HashSet<(i32, i32)>; 4],
    m: i32,
    n: i32,
    start_case: (i32, i32, i32),
    target: (i32, i32),
) -> Option<i32> {
    let lcm = integer::lcm(m, n);
    let mut queue: VecDeque<(i32, i32, i32)> = VecDeque::from([start_case]);
    let mut seen: HashSet<(i32, i32, i32)> = HashSet::new();

    while !queue.is_empty() {
        let (mut time, r, c) = queue.pop_front().unwrap();

        time += 1;

        'directions: for (dr, dc) in [(-1, 0), (1, 0), (0, -1), (0, 1), (0, 0)].iter() {
            let (nr, nc) = (r + dr, c + dc);

            if (nr, nc) == target {
                return Some(time);
            }

            if (nr < 0 || nr >= m || nc < 0 || nc >= n)
                && (nr, nc) != (-1, 0)
                && (nr, nc) != (m, n - 1)
            {
                continue;
            }

            if nr != -1 && nr != m {
                for (i, rd, cd) in [(0, -1, 0), (1, 1, 0), (2, 0, -1), (3, 0, 1)] {
                    if blizzards[i].contains(&(
                        integer::mod_floor(nr - rd * time, m),
                        integer::mod_floor(nc - cd * time, n),
                    )) {
                        continue 'directions;
                    }
                }
            }

            if !seen.insert((time % lcm, nr, nc)) {
                continue;
            }

            queue.push_back((time, nr, nc));
        }
    }

    None
}

fn part1() {
    let input = include_str!("input.txt");

    let mut blizzards: [HashSet<(i32, i32)>; 4] = [
        HashSet::new(),
        HashSet::new(),
        HashSet::new(),
        HashSet::new(),
    ];

    let mut m = 0;
    let mut n = 0;

    for (row, line) in input.lines().enumerate().map(|(i, l)| (i as i32 - 1, l)) {
        m = row;
        for (col, c) in line.char_indices().map(|(i, c)| (i as i32 - 1, c)) {
            n = col;
            match c {
                '^' => blizzards[0].insert((row, col)),
                'v' => blizzards[1].insert((row, col)),
                '<' => blizzards[2].insert((row, col)),
                '>' => blizzards[3].insert((row, col)),
                _ => false,
            };
        }
    }

    let start = (-1, 0);
    let end = (m, n - 1);

    println!(
        "{}",
        min_travel_time(&blizzards, m, n, (0, start.0, start.1), end).unwrap()
    );
}

fn part2() {
    let input = include_str!("input.txt");

    let mut blizzards: [HashSet<(i32, i32)>; 4] = [
        HashSet::new(),
        HashSet::new(),
        HashSet::new(),
        HashSet::new(),
    ];

    let mut m = 0;
    let mut n = 0;

    for (row, line) in input.lines().enumerate().map(|(i, l)| (i as i32 - 1, l)) {
        m = row;
        for (col, c) in line.char_indices().map(|(i, c)| (i as i32 - 1, c)) {
            n = col;
            match c {
                '^' => blizzards[0].insert((row, col)),
                'v' => blizzards[1].insert((row, col)),
                '<' => blizzards[2].insert((row, col)),
                '>' => blizzards[3].insert((row, col)),
                _ => false,
            };
        }
    }

    let start = (-1, 0);
    let end = (m, n - 1);

    let time_1 = min_travel_time(&blizzards, m, n, (0, start.0, start.1), end).unwrap();
    let time_2 = min_travel_time(&blizzards, m, n, (time_1, end.0, end.1), start).unwrap();
    let time_3 = min_travel_time(&blizzards, m, n, (time_2, start.0, start.1), end).unwrap();

    println!("{}", time_3);
}

fn main() {
    println!("Part 1:");
    part1();

    println!("");

    println!("Part 2:");
    part2();
}
