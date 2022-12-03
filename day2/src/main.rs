use std::collections::HashMap;

// A Rock       1
// B Paper      2
// C Scissors   3

// X Lose     0
// Y Draw     3
// Z Win      6


fn main() {
    let selected = HashMap::from([
        ("A",1),
        ("B",2),
        ("C",3)
    ]);

    let loses_to = HashMap::from([
        ("A", "C"),
        ("B", "A"),
        ("C", "B")
    ]);

    let rounds = include_str!("in.txt")
        .split("\n")
        .map(|line| (&line[..1],&line[line.len()-1..]));
        
    let points: i32 = rounds.map(|(oponent, outcome)| {
        match outcome {
            "Z" => 6 + selected[loses_to[loses_to[oponent]]],
            "Y" => 3 + selected[oponent],
            _ => 0 + selected[loses_to[oponent]],
        }
    }).sum();

    println!("{}", points);

    ()
}
