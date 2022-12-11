fn part1() {
    let input = include_str!("input.txt");

    let mut cicle = 1;
    let mut x = 1;

    let mut sum = 0;

    for line in input.lines() {
        if let Some((_, amount)) = line.split_once(" ") {
            for _ in 0..2 {
                if cicle <= 220 && (cicle + 20) % 40 == 0 {
                    sum += x * cicle;
                }
                cicle += 1;
            }
            x += amount.parse::<i32>().unwrap();
        } else {
            if cicle <= 220 && (cicle + 20) % 40 == 0 {
                sum += x * cicle;
            }
            cicle += 1;
        }
    }

    println!("{}", sum);
}

fn part2() {}

fn main() {
    println!("Part 1:");
    part1();

    println!("");

    println!("Part 2:");
    part2();
}
