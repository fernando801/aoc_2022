use std::{cmp::Ordering, str};

#[derive(Clone)]
enum Packet {
    Int(i32),
    List(Vec<Packet>),
}

#[derive(Debug)]
enum PacketError {
    ParseError,
}

fn packet_cmp(a: &Packet, b: &Packet) -> Ordering {
    let pair = (a, b);

    match pair {
        (Packet::Int(left), Packet::Int(right)) => left.cmp(right),
        (Packet::Int(int), Packet::List(_)) => {
            packet_cmp(&Packet::List(Vec::from([Packet::Int(*int)])), &pair.1)
        }
        (Packet::List(_), Packet::Int(int)) => {
            packet_cmp(&pair.0, &Packet::List(Vec::from([Packet::Int(*int)])))
        }
        (Packet::List(left), Packet::List(right)) => {
            for i in 0..left.len() {
                if i >= right.len() {
                    return Ordering::Greater;
                }

                let ord = packet_cmp(&left[i], &right[i]);

                if ord != Ordering::Equal {
                    return ord;
                }
            }

            if left.len() == right.len() {
                return Ordering::Equal;
            }

            Ordering::Less
        }
    }
}

fn str_to_packet(s: &str) -> Result<Packet, PacketError> {
    if let Ok(int) = s.parse::<i32>() {
        return Ok(Packet::Int(int));
    }

    if s.starts_with("[") {
        let new_str = &s[1..s.len() - 1];

        if new_str.len() == 0 {
            return Ok(Packet::List(Vec::new()));
        }

        let mut packets_strs: Vec<&str> = Vec::new();

        let mut open_brackets = 0;
        let mut slice_start = 0;

        for i in 0..new_str.len() {
            match new_str.chars().nth(i).unwrap() {
                ',' => {
                    if open_brackets == 0 {
                        packets_strs.push(&new_str[slice_start..i]);
                        slice_start = i + 1;
                    }
                }
                '[' => open_brackets += 1,
                ']' => open_brackets -= 1,
                _ => (),
            }
        }

        packets_strs.push(&new_str[slice_start..]);

        let mut packet_list: Vec<Packet> = Vec::new();

        for &packet_str in packets_strs.iter() {
            match str_to_packet(packet_str) {
                Ok(packet) => packet_list.push(packet),
                Err(err) => return Err(err),
            }
        }

        return Ok(Packet::List(packet_list));
    }

    Err(PacketError::ParseError)
}

fn part1() {
    let input = include_str!("input.txt");

    let pairs: Vec<(Packet, Packet)> = input
        .split("\n\n")
        .map(|pair_str| {
            let (left, right) = pair_str.split_once("\n").unwrap();
            (str_to_packet(left).unwrap(), str_to_packet(right).unwrap())
        })
        .collect();

    let ordered_indices_sum: usize = pairs
        .iter()
        .enumerate()
        .filter(|(_, (left, right))| packet_cmp(left, right) != Ordering::Greater)
        .map(|(i, _)| i + 1)
        .sum();

    println!("{}", ordered_indices_sum);
}

fn part2() {
    let input = include_str!("input.txt");

    let mut packets: Vec<Packet> = input
        .split("\n\n")
        .flat_map(|pair_str| {
            pair_str
                .split("\n")
                .map(|packet_str| str_to_packet(packet_str).unwrap())
        })
        .collect();

    packets.push(str_to_packet("[[2]]").unwrap());
    packets.push(str_to_packet("[[6]]").unwrap());

    packets.sort_by(packet_cmp);

    let divider_indices_product: usize = packets
        .iter()
        .enumerate()
        .filter(|(_, packet)| {
            packet_cmp(packet, &str_to_packet("[[2]]").unwrap()) == Ordering::Equal
                || packet_cmp(packet, &str_to_packet("[[6]]").unwrap()) == Ordering::Equal
        })
        .map(|(i, _)| i + 1)
        .product();

    println!("{}", divider_indices_product);
}

fn main() {
    println!("Part 1:");
    part1();

    println!("");

    println!("Part 2:");
    part2();
}
