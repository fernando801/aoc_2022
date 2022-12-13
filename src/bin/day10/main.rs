fn part1() {
    let input = include_str!("input.txt");

    let mut cycle = 1;
    let mut x = 1;

    let mut sum = 0;

    for line in input.lines() {
        if let Some((_, amount)) = line.split_once(" ") {
            for _ in 0..2 {
                if cycle <= 220 && (cycle + 20) % 40 == 0 {
                    sum += x * cycle;
                }
                cycle += 1;
            }
            x += amount.parse::<i32>().unwrap();
        } else {
            if cycle <= 220 && (cycle + 20) % 40 == 0 {
                sum += x * cycle;
            }
            cycle += 1;
        }
    }

    println!("{}", sum);
}

fn part2() {
    let input = include_str!("input.txt");

    let mut cycle: i32 = 1;
    let mut x: i32 = 1;

    let mut screen = [['.'; 40]; 6];

    for line in input.lines() {
        if let Some((_, amount)) = line.split_once(" ") {
            for _ in 0..2 {
                if ((cycle - 1) % 40 - x).abs() <= 1 {
                    let c = cycle as usize;
                    screen[(c - 1) / 40][(c - 1) % 40] = '#';
                }
                cycle += 1;
            }
            x += amount.parse::<i32>().unwrap();
        } else {
            if ((cycle - 1) % 40 - x).abs() <= 1 {
                let c = cycle as usize;
                screen[(c - 1) / 40][(c - 1) % 40] = '#';
            }
            cycle += 1;
        }
    }

    screen.iter().for_each(|line| {
        line.iter().for_each(|c| print!("{}", c));
        println!();
    });
}

fn main() {
    println!("Part 1:");
    part1();

    println!("");

    println!("Part 2:");
    part2();
}
