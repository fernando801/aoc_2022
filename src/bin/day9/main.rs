use std::collections::HashSet;

struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn from(x: i32, y: i32) -> Self {
        Position { x, y }
    }
}

fn part1() {
    let input = include_str!("input.txt");

    let mut head = Position::from(0, 0);
    let mut tail = Position::from(0, 0);

    let mut visited = HashSet::from([(0, 0)]);

    for instruction in input.lines() {
        let (direction, amount) = instruction.split_once(" ").unwrap();

        for _ in 0..amount.parse::<i32>().unwrap() {
            let prev = Position::from(head.x, head.y);

            match direction {
                "U" => head.y += 1,
                "R" => head.x += 1,
                "D" => head.y -= 1,
                "L" => head.x -= 1,
                _ => (),
            }

            if (head.x - tail.x).abs() >= 2 || (head.y - tail.y).abs() >= 2 {
                tail.x = prev.x;
                tail.y = prev.y;
                visited.insert((tail.x, tail.y));
            }
        }
    }

    println!("{:?}", visited.len());
}

fn part2() {}

fn main() {
    println!("Part 1:");
    part1();

    println!("");

    println!("Part 2:");
    part2();
}
