use std::collections::HashMap;

fn main() {
    let rucksacks: Vec<&str> = include_str!("in.txt").split("\n").collect();

    let groups: u32 = rucksacks
        .chunks(3)
        .map(|x| x.to_vec())
        .map(|r| {
            let mut map1: HashMap<char, u32> = HashMap::new();
            let mut map2: HashMap<char, u32> = HashMap::new();

            for c in r[0].chars() {
                let ascii = c as u32;
                let ascii = if ascii > 96 { ascii - 96 } else { ascii - 38 };
                map1.insert(c, ascii);
            }

            for c in r[1].chars() {
                let ascii = c as u32;
                let ascii = if ascii > 96 { ascii - 96 } else { ascii - 38 };
                map2.insert(c, ascii);
            }

            for c in r[2].chars() {
                if let (Some(v), Some(_)) = (map1.get(&c).copied(), map2.get(&c).copied()) {
                    return v;
                }
            }

            0
        })
        .sum();

    println!("{}", groups);
}
