use num::integer;
use std::collections::{HashSet, VecDeque};

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

    let exit = (m, n - 1);
    let lcm = integer::lcm(m, n);
    let mut queue: VecDeque<(i32, i32, i32)> = VecDeque::from([(0, -1, 0)]);
    let mut seen: HashSet<(i32, i32, i32)> = HashSet::new();

    'bfs: while !queue.is_empty() {
        let (mut time, r, c) = queue.pop_front().unwrap();

        time += 1;

        'directions: for (dr, dc) in [(-1, 0), (1, 0), (0, -1), (0, 1), (0, 0)].iter() {
            let (nr, nc) = (r + dr, c + dc);

            if (nr, nc) == exit {
                println!("{}", time);
                break 'bfs;
            }

            if (nr < 0 || nr >= m || nc < 0 || nc >= n) && (nr, nc) != (-1, 0) {
                continue;
            }

            if nr != -1 {
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
}

fn part2() {}

fn main() {
    println!("Part 1:");
    part1();

    println!("");

    println!("Part 2:");
    part2();
}
