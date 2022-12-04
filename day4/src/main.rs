fn main() {
    let assignment_pairs: Vec<((usize, usize), (usize, usize))> = include_str!("in.txt")
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
            if range1.0 > range2.1 || range2.0 > range1.1
            {
                return 0;
            }

            1
        })
        .sum();

    println!("{}", overlaped_assignments);
}
