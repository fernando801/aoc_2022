use priority_queue::DoublePriorityQueue;
use std::collections::HashMap;

fn a_star(
    adjacency_list: &HashMap<(usize, usize), Vec<(usize, usize)>>,
    start: (usize, usize),
    end: (usize, usize),
) -> Option<usize> {
    let mut queue: DoublePriorityQueue<(usize, usize), usize> = DoublePriorityQueue::new();
    queue.push(start, start.0.abs_diff(end.0) + start.1.abs_diff(end.1));

    let mut gscore: HashMap<(usize, usize), usize> =
        HashMap::from_iter(adjacency_list.keys().map(|&key| (key, usize::MAX)));

    gscore.insert(start, 0);

    let mut fscore: HashMap<(usize, usize), usize> =
        HashMap::from_iter(adjacency_list.keys().map(|&key| (key, usize::MAX)));

    fscore.insert(start, start.0.abs_diff(end.0) + start.1.abs_diff(end.1));

    while !queue.is_empty() {
        let (current, _) = queue.pop_min().unwrap();

        if current == end {
            return gscore.get(&current).copied();
        }

        for &neighbor in adjacency_list.get(&current).unwrap() {
            let tentative_gscore = gscore.get(&current).unwrap() + 1;
            if tentative_gscore < *gscore.get(&neighbor).unwrap() {
                gscore.insert(neighbor, tentative_gscore);
                fscore.insert(
                    neighbor,
                    tentative_gscore + neighbor.0.abs_diff(end.0) + neighbor.1.abs_diff(end.1),
                );
                queue.push(neighbor, *fscore.get(&neighbor).unwrap());
            }
        }
    }

    None
}

fn part1() {
    let input = include_str!("input.txt");

    let m = input.lines().count();
    let n = input.find("\n").unwrap();

    let mut start = (0, 0);
    let mut end = (0, 0);

    let hmap: Vec<Vec<char>> = input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(|(j, c)| match c {
                    'S' => {
                        start = (i, j);
                        'a'
                    }
                    'E' => {
                        end = (i, j);
                        'z'
                    }
                    l => l,
                })
                .collect()
        })
        .collect();

    let mut adjacency_list: HashMap<(usize, usize), Vec<(usize, usize)>> = HashMap::new();

    for i in 0..m {
        for j in 0..n {
            let current = (hmap[i][j] as u8 + 1) as char;
            let mut neighbors: Vec<(usize, usize)> = Vec::new();

            if i > 0 && hmap[i - 1][j] <= current {
                neighbors.push((i - 1, j));
            }

            if i < m - 1 && hmap[i + 1][j] <= current {
                neighbors.push((i + 1, j));
            }

            if j > 0 && hmap[i][j - 1] <= current {
                neighbors.push((i, j - 1));
            }

            if j < n - 1 && hmap[i][j + 1] <= current {
                neighbors.push((i, j + 1));
            }

            adjacency_list.insert((i, j), neighbors);
        }
    }

    let shortest_path_length = a_star(&adjacency_list, start, end).unwrap();

    println!("{}", shortest_path_length);
}

fn part2() {}

fn main() {
    println!("Part 1:");
    part1();

    println!("");

    println!("Part 2:");
    part2();
}
