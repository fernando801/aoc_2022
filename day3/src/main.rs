use std::collections::HashMap;

fn main() {
    let rucksacks: u32 = include_str!("in.txt")
        .split("\n")
        .map(|line| {
            (&line[..line.len()/2], &line[line.len()/2..])
        })
        .map(|(compartment1, compartment2)| {
            let mut map: HashMap<char, u32> = HashMap::new();

            for c in compartment1.chars() {
                let ascii = c as u32;
                let ascii = if ascii > 96 {ascii - 96} else {ascii - 38};
                map.insert(c, ascii);
            }

            for c in compartment2.chars() {
                if let Some(v) = map.get(&c).copied() {
                    return v
                }
            }

            0
        })
        .sum();
    
    println!("{}", rucksacks);
}
