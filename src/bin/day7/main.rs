fn part1() {
    let input = include_str!("input.txt");
    let input = &format!("{}{}", input, "\n$ cd ..")[..];

    let mut stack: Vec<usize> = Vec::new();
    let mut total = 0;

    for line in input.lines() {
        if line == "$ ls" {
            continue;
        }

        if line.starts_with("$ cd ") {
            let dir = &line[5..];

            if dir == ".." {
                let size = stack.pop().unwrap();
                *stack.last_mut().unwrap() += size;
                if size <= 100_000 {
                    total += size;
                }
            } else {
                stack.push(0);
            }
            continue;
        }

        if let Ok(size) = line.split_once(" ").unwrap().0.parse::<usize>() {
            *stack.last_mut().unwrap() += size;
        }
    }

    let size = stack.pop().unwrap();
    if size <= 100_000 {
        total += size;
    }

    println!("{}", &total);
}

fn part2() {}

fn main() {
    println!("Part 1:");
    part1();

    println!("");

    println!("Part 2:");
    part2();
}
