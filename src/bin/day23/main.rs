use std::cmp;
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug)]
struct Grove {
    directions: VecDeque<(i32, i32)>,
    elves: HashSet<(i32, i32)>,
    next_loc: HashMap<(i32, i32), (i32, i32)>,
    canceled: HashSet<(i32, i32)>,
}

impl Grove {
    fn round(&mut self) -> bool {
        // Frist Half
        self.next_loc = HashMap::new();
        self.canceled = HashSet::new();

        for &elf in &self.elves {
            let mut next = elf;
            let mut movement = false;
            for &dir in &self.directions {
                let neighbors = [
                    (elf.0 + dir.0, elf.1 + dir.1),
                    (elf.0 + dir.0 + dir.1, elf.1 + dir.1 + dir.0),
                    (elf.0 + dir.0 - dir.1, elf.1 + dir.1 - dir.0),
                ];

                if self.elves.contains(&neighbors[0])
                    || self.elves.contains(&neighbors[1])
                    || self.elves.contains(&neighbors[2])
                {
                    movement = true;
                } else if next == elf {
                    next = neighbors[0];
                }
            }

            if !movement && next != elf {
                continue;
            }

            if self.next_loc.get(&next).is_none() {
                self.next_loc.insert(next, elf);
            } else {
                self.canceled.insert(next);
            }
        }

        let f = self
            .directions
            .pop_front()
            .expect("there should always be directions");
        self.directions.push_back(f);

        // Second Half
        for loc in &self.canceled {
            self.next_loc.remove(loc);
        }

        if self.next_loc.is_empty() {
            return false;
        }

        for (&next, &curr) in &self.next_loc {
            self.elves.remove(&curr);
            self.elves.insert(next);
        }

        true
    }

    fn do_rounds(&mut self, n: usize) {
        for _ in 0..n {
            self.round();
        }
    }

    fn do_rounds_with_movement(&mut self) -> usize {
        let mut round = 1;
        loop {
            if !self.round() {
                return round;
            }
            round += 1;
        }
    }

    fn get_empty_ground_count(&self) -> i32 {
        let mut row_lims = (i32::MAX, i32::MIN);
        let mut col_lims = (i32::MAX, i32::MIN);

        for &elf in &self.elves {
            row_lims = (cmp::min(row_lims.0, elf.0), cmp::max(row_lims.1, elf.0));
            col_lims = (cmp::min(col_lims.0, elf.1), cmp::max(col_lims.1, elf.1));
        }

        (row_lims.1 - row_lims.0 + 1) * (col_lims.1 - col_lims.0 + 1) - self.elves.len() as i32
    }
}

fn part1() {
    let input = include_str!("input.txt");

    let mut elves: HashSet<(i32, i32)> = HashSet::new();

    for (row, line) in input.lines().enumerate().map(|(i, l)| (i as i32, l)) {
        for (col, c) in line.char_indices().map(|(i, c)| (i as i32, c)) {
            if c == '#' {
                elves.insert((row, col));
            }
        }
    }

    let mut grove = Grove {
        directions: VecDeque::from([(-1, 0), (1, 0), (0, -1), (0, 1)]),
        next_loc: HashMap::new(),
        canceled: HashSet::new(),
        elves,
    };

    grove.do_rounds(10);
    let a = grove.get_empty_ground_count();

    println!("{}", a);
}

fn part2() {
    let input = include_str!("input.txt");

    let mut elves: HashSet<(i32, i32)> = HashSet::new();

    for (row, line) in input.lines().enumerate().map(|(i, l)| (i as i32, l)) {
        for (col, c) in line.char_indices().map(|(i, c)| (i as i32, c)) {
            if c == '#' {
                elves.insert((row, col));
            }
        }
    }

    let mut grove = Grove {
        directions: VecDeque::from([(-1, 0), (1, 0), (0, -1), (0, 1)]),
        next_loc: HashMap::new(),
        canceled: HashSet::new(),
        elves,
    };

    let a = grove.do_rounds_with_movement();

    println!("{}", a);
}

fn main() {
    println!("Part 1:");
    part1();

    println!("");

    println!("Part 2:");
    part2();
}
