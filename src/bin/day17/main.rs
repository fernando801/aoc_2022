use std::cmp;
use std::collections::{HashMap, HashSet, VecDeque};

struct Rock {
    positions: HashSet<(i32, i32)>,
    altitude: i32,
}

impl Rock {
    fn left_neighbors(&self) -> HashSet<(i32, i32)> {
        self.positions.iter().map(|&(x, y)| (x - 1, y)).collect()
    }

    fn right_neighbors(&self) -> HashSet<(i32, i32)> {
        self.positions.iter().map(|&(x, y)| (x + 1, y)).collect()
    }

    fn bottom_neighbors(&self) -> HashSet<(i32, i32)> {
        self.positions.iter().map(|&(x, y)| (x, y - 1)).collect()
    }

    fn from(positions: HashSet<(i32, i32)>) -> Self {
        let altitude = positions.iter().map(|(_, y)| y).max().copied().unwrap();

        Rock {
            positions,
            altitude,
        }
    }

    fn h_line((x, y): (i32, i32)) -> Self {
        let positions: HashSet<(i32, i32)> = [(x, y), (x + 1, y), (x + 2, y), (x + 3, y)]
            .iter()
            .copied()
            .collect();

        Rock::from(positions)
    }

    fn cross((x, y): (i32, i32)) -> Self {
        let positions: HashSet<(i32, i32)> = [
            (x + 1, y),
            (x, y + 1),
            (x + 1, y + 1),
            (x + 2, y + 1),
            (x + 1, y + 2),
        ]
        .iter()
        .copied()
        .collect();

        Rock::from(positions)
    }

    fn j_block((x, y): (i32, i32)) -> Self {
        let positions: HashSet<(i32, i32)> = [
            (x, y),
            (x + 1, y),
            (x + 2, y),
            (x + 2, y + 1),
            (x + 2, y + 2),
        ]
        .iter()
        .copied()
        .collect();

        Rock::from(positions)
    }

    fn v_line((x, y): (i32, i32)) -> Self {
        let positions: HashSet<(i32, i32)> = [(x, y), (x, y + 1), (x, y + 2), (x, y + 3)]
            .iter()
            .copied()
            .collect();

        Rock::from(positions)
    }

    fn square((x, y): (i32, i32)) -> Self {
        let positions: HashSet<(i32, i32)> = [(x, y), (x + 1, y), (x, y + 1), (x + 1, y + 1)]
            .iter()
            .copied()
            .collect();

        Rock::from(positions)
    }
}

fn part1() {
    let input = include_str!("input.txt");

    let mut instructions: VecDeque<char> = input.chars().collect();

    let mut tower: HashSet<(i32, i32)> = HashSet::with_capacity(2022 * 3 * 7);
    let mut tower_height = 0;

    for i in 0..2022 {
        let base = (2, tower_height + 4);

        let mut rock = match i % 5 {
            0 => Rock::h_line(base),
            1 => Rock::cross(base),
            2 => Rock::j_block(base),
            3 => Rock::v_line(base),
            4 => Rock::square(base),
            _ => panic!(),
        };

        loop {
            let dir = instructions.pop_front().unwrap();
            instructions.push_back(dir);

            let neighbors = match dir {
                '<' => rock.left_neighbors(),
                '>' => rock.right_neighbors(),
                _ => panic!(),
            };

            let mut blocked = false;

            for coor in neighbors.difference(&rock.positions) {
                if coor.0 < 0 || coor.0 >= 7 || tower.contains(coor) {
                    blocked = true;
                    break;
                }
            }

            if !blocked {
                rock.positions = neighbors;
            }

            let neighbors = rock.bottom_neighbors();

            let mut blocked = false;

            for coor in neighbors.difference(&rock.positions) {
                if coor.1 <= 0 || tower.contains(coor) {
                    blocked = true;
                    break;
                }
            }

            if !blocked {
                rock = Rock::from(neighbors);
            } else {
                for coor in rock.positions {
                    tower.insert(coor);
                }

                tower_height = cmp::max(rock.altitude, tower_height);
                break;
            }
        }
    }

    println!("{}", tower_height);
}

fn part2() {
    let input = include_str!("input.txt");

    let mut instructions: VecDeque<(usize, char)> = input.chars().enumerate().collect();

    let mut tower: HashSet<(i32, i32)> = HashSet::new();
    let mut tower_height = 0;
    let mut cycle_data: HashMap<String, (u64, u64, u64, u64)> = HashMap::new();

    let mut stop = 1000000000000u64;
    let mut fit = 0;

    for i in 0..1000000000000u64 {
        if i == stop {
            break;
        }

        let base = (2, tower_height as i32 + 4);

        let mut rock = match i % 5 {
            0 => Rock::h_line(base),
            1 => Rock::cross(base),
            2 => Rock::j_block(base),
            3 => Rock::v_line(base),
            4 => Rock::square(base),
            _ => panic!(),
        };

        loop {
            let dir = instructions.pop_front().unwrap();
            if dir.0 == 0 && stop == 1000000000000u64 {
                let mut key = String::new();

                for i in 0..7 {
                    if !tower.contains(&(i, tower_height as i32)) {
                        key.push('.');
                    } else {
                        key.push('#');
                    }
                }

                if let Some(prev) = cycle_data.get(&key) {
                    let data = (tower_height, tower_height - prev.0, i, i - prev.2);

                    if data.1 == prev.1 && data.3 == prev.3 {
                        stop -= data.2;
                        fit += (stop / data.3) * data.1;
                        stop = (stop % data.3) + data.2;
                    }

                    cycle_data.insert(key, data);
                } else {
                    cycle_data.insert(key, (tower_height, 0, i, 0));
                }
            }
            instructions.push_back(dir);

            let neighbors = match dir.1 {
                '<' => rock.left_neighbors(),
                '>' => rock.right_neighbors(),
                _ => panic!(),
            };

            let mut blocked = false;

            for coor in neighbors.difference(&rock.positions) {
                if coor.0 < 0 || coor.0 >= 7 || tower.contains(coor) {
                    blocked = true;
                    break;
                }
            }

            if !blocked {
                rock.positions = neighbors;
            }

            let neighbors = rock.bottom_neighbors();

            let mut blocked = false;

            for coor in neighbors.difference(&rock.positions) {
                if coor.1 <= 0 || tower.contains(coor) {
                    blocked = true;
                    break;
                }
            }

            if !blocked {
                rock = Rock::from(neighbors);
            } else {
                for coor in rock.positions {
                    tower.insert(coor);
                }

                tower_height = cmp::max(rock.altitude as u64, tower_height);
                break;
            }
        }
    }

    tower_height += fit;

    println!("{}", tower_height);
}

fn main() {
    println!("Part 1:");
    part1();

    println!("");

    println!("Part 2:");
    part2();
}
