use std::cmp;
use std::collections::{HashSet, VecDeque};

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

fn part2() {}

fn main() {
    println!("Part 1:");
    part1();

    println!("");

    println!("Part 2:");
    part2();
}
