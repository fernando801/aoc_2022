use std::collections::HashMap;

// A Rock       1
// B Paper      2
// C Scissors   3

// X Lose     0
// Y Draw     3
// Z Win      6

fn part1() {
    let selected = HashMap::from([("X", 1), ("Y", 2), ("Z", 3)]);

    let equivalent = HashMap::from([("A", "X"), ("B", "Y"), ("C", "Z")]);

    let loses_to = HashMap::from([("X", "C"), ("Y", "A"), ("Z", "B")]);

    let rounds = include_str!("input.txt")
        .split("\n")
        .map(|line| (&line[..1], &line[line.len() - 1..]));

    let points: i32 = rounds
        .map(|(oponent, you)| {
            let outcome = match loses_to[you] {
                loser if loser == oponent => 6,
                loser if loser == loses_to[equivalent[oponent]] => 3,
                _ => 0,
            };
            selected[you] + outcome
        })
        .sum();

    println!("{}", points);
}

fn part2() {
    let selected = HashMap::from([("A", 1), ("B", 2), ("C", 3)]);

    let loses_to = HashMap::from([("A", "C"), ("B", "A"), ("C", "B")]);

    let rounds = include_str!("input.txt")
        .split("\n")
        .map(|line| (&line[..1], &line[line.len() - 1..]));

    let points: i32 = rounds
        .map(|(oponent, outcome)| match outcome {
            "Z" => 6 + selected[loses_to[loses_to[oponent]]],
            "Y" => 3 + selected[oponent],
            _ => 0 + selected[loses_to[oponent]],
        })
        .sum();

    println!("{}", points);
}

fn main() {
    println!("Part 1:");
    part1();

    println!("");

    println!("Part 2:");
    part2();
}
