fn main() {
    let input: Vec<&str> = include_str!("in.txt").split("\n\n").collect();

    let (ship, instructions) = (input[0], input[1]);

    let ship: Vec<&str> = ship.split("\n").collect();

    let number_of_stacks = ship
        .last()
        .copied()
        .unwrap()
        .split_whitespace()
        .map(str::parse::<usize>)
        .filter(|r| r.is_ok())
        .last()
        .unwrap()
        .unwrap();

    let mut stacks: Vec<Vec<char>> = Vec::new();

    for _ in 0..number_of_stacks {
        stacks.push(Vec::new())
    }

    ship[..ship.len() - 1]
        .iter()
        .copied()
        .rev()
        .for_each(|line| {
            for (i, c) in line
                .chars()
                .enumerate()
                .filter(|(i, _)| (i + 3) % 4 == 0)
                .map(|(_, c)| c)
                .enumerate()
                .filter(|(_, c)| c.is_ascii_uppercase())
            {
                stacks[i].push(c);
            }
        });

    instructions.split("\n").for_each(|line| {
        let instruction_parts: Vec<usize> = line
            .split_whitespace()
            .map(str::parse::<usize>)
            .filter(|r| r.is_ok())
            .map(|r| r.unwrap())
            .collect();

        let mut buff: Vec<char> = Vec::new();

        for _ in 0..instruction_parts[0] {
            let value = stacks[instruction_parts[1] - 1].pop();
            buff.push(value.unwrap());
        }

        for _ in 0..instruction_parts[0] {
            stacks[instruction_parts[2] - 1].push(buff.pop().unwrap());
        }
    });

    for i in 0..number_of_stacks {
        print!("{}", stacks[i].pop().unwrap());
    }
    println!("");
}
