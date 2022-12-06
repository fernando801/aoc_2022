fn part1() {
    let mut max: u32 = 0;
    let mut current: u32 = 0;
    let lines = include_str!("input.txt").split("\n");
    for line in lines {
        current = match line.parse::<u32>() {
            Ok(num) => num + current,
            _ => 0,
        };
        if current > max {
            max = current;
        }
    }
    println!("{}", &max);
}

fn part2() {
    let mut max: [u32; 3] = [0, 0, 0];
    let mut current: u32 = 0;
    let lines = include_str!("input.txt").split("\n");

    for line in lines {
        current = match line.parse::<u32>() {
            Ok(num) => num + current,
            _ => {
                if current > max[0] {
                    max[0] = current;
                    for i in 1..max.len() {
                        if max[i - 1] > max[i] {
                            current = max[i];
                            max[i] = max[i - 1];
                            max[i - 1] = current;
                        }
                    }
                }
                0
            }
        };
    }

    let sum: u32 = max.iter().sum();

    println!("{}", &sum);
}

fn main() {
    println!("Part 1:");
    part1();

    println!("");

    println!("Part 2:");
    part2();
}
