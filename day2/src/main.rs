use std::collections::HashMap;

// A Rock       1
// B Paper      2
// C Scissors   3

// X Rock       1
// Y Paper      2
// Z Scissors   3

// Lose     0
// Draw     3
// Win      6

fn main() {
    let selected = HashMap::from([
        ("X",1),
        ("Y",2),
        ("Z",3)
    ]);

    let equivalent = HashMap::from([
        ("A", "X"),
        ("B", "Y"),
        ("C", "Z")
    ]);

    let loses_to = HashMap::from([
        ("X", "C"),
        ("Y", "A"),
        ("Z", "B")
    ]);

    let rounds = include_str!("in.txt")
        .split("\n")
        .map(|line| (&line[..1],&line[line.len()-1..]));
        
    let points: i32 = rounds.map(|(oponent, you)| {
        let outcome = match loses_to[you] {
            loser if loser == oponent => 6,
            loser if loser == loses_to[equivalent[oponent]] => 3,
            _ => 0,
        };
        selected[you] + outcome
    }).sum();

    println!("{}", points);

    ()
}
