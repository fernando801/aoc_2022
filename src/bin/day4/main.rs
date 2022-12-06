fn part1() {
    let assignment_pairs: Vec<((usize, usize), (usize, usize))> = include_str!("input.txt")
        .split("\n")
        .map(|line| {
            let elves: Vec<(usize, usize)> = line
                .split(",")
                .map(|elf| {
                    let range: Vec<&str> = elf.split("-").collect();
                    (
                        range[0].parse::<usize>().unwrap(),
                        range[1].parse::<usize>().unwrap(),
                    )
                })
                .collect();
            (elves[0], elves[1])
        })
        .collect();

    let overlaped_assignments: usize = assignment_pairs
        .iter()
        .map(|(range1, range2)| {
            if range1.0 <= range2.0 && range1.1 >= range2.1
                || range2.0 <= range1.0 && range2.1 >= range1.1
            {
                return 1;
            }

            0
        })
        .sum();

    println!("{}", overlaped_assignments);
}

fn part2() {
    let assignment_pairs: Vec<((usize, usize), (usize, usize))> = include_str!("input.txt")
        .split("\n")
        .map(|line| {
            let elves: Vec<(usize, usize)> = line
                .split(",")
                .map(|elf| {
                    let range: Vec<&str> = elf.split("-").collect();
                    (
                        range[0].parse::<usize>().unwrap(),
                        range[1].parse::<usize>().unwrap(),
                    )
                })
                .collect();
            (elves[0], elves[1])
        })
        .collect();

    let overlaped_assignments: usize = assignment_pairs
        .iter()
        .map(|(range1, range2)| {
            if range1.0 > range2.1 || range2.0 > range1.1 {
                return 0;
            }

            1
        })
        .sum();

    println!("{}", overlaped_assignments);
}

fn main() {
    println!("Part 1:");
    part1();

    println!("");

    println!("Part 2:");
    part2();
}
