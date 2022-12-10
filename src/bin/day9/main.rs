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

fn part2() {
    let input = include_str!("input.txt");

    let mut rope = Vec::new();

    for _ in 0..10 {
        rope.push(Position::from(0, 0));
    }

    let mut visited = HashSet::from([(0, 0)]);

    for instruction in input.lines() {
        let (direction, amount) = instruction.split_once(" ").unwrap();

        for _ in 0..amount.parse::<i32>().unwrap() {
            match direction {
                "U" => rope[0].y += 1,
                "R" => rope[0].x += 1,
                "D" => rope[0].y -= 1,
                "L" => rope[0].x -= 1,
                _ => (),
            }

            for i in 1..rope.len() {
                let dx = rope[i - 1].x - rope[i].x;
                let dy = rope[i - 1].y - rope[i].y;

                if dx.abs() >= 2 || dy.abs() >= 2 {
                    rope[i].x += if dx == 0 { 0 } else { dx / dx.abs() };
                    rope[i].y += if dy == 0 { 0 } else { dy / dy.abs() };
                }
            }

            visited.insert((rope.last().unwrap().x, rope.last().unwrap().y));
        }
    }

    println!("{:?}", visited.len());
}

fn main() {
    println!("Part 1:");
    part1();

    println!("");

    println!("Part 2:");
    part2();
}
