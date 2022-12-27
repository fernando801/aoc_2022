#[derive(Debug, Clone, Copy)]
struct Node {
    value: i64,
    prev: usize,
    next: usize,
}

fn part1() {
    let input = include_str!("input.txt");

    let mut list: Vec<Node> = Vec::new();

    let mut index_0 = 0;

    for (i, value) in input
        .lines()
        .map(str::parse::<i64>)
        .map(|r| r.unwrap())
        .enumerate()
    {
        if value == 0 {
            index_0 = i;
        }

        list.push(Node {
            value,
            prev: if i > 0 { i - 1 } else { i },
            next: i + 1,
        })
    }

    let n = list.len();

    list[n - 1].next = 0;
    list[0].prev = n - 1;

    for i in 0..n {
        let node = list[i];
        let steps = node.value.abs() as usize;

        if steps % (n - 1) == 0 {
            continue;
        }

        list[node.prev].next = node.next;
        list[node.next].prev = node.prev;

        let mut new = i;

        if node.value > 0 {
            for _ in 0..(steps % (n - 1)) {
                new = list[new].next;
            }

            list[i].prev = new;
            list[i].next = list[new].next;

            list[new].next = i;
            new = list[i].next;
            list[new].prev = i;
        } else {
            for _ in 0..(steps % (n - 1)) {
                new = list[new].prev;
            }

            list[i].next = new;
            list[i].prev = list[new].prev;

            list[new].prev = i;
            new = list[i].prev;
            list[new].next = i;
        }
    }

    let mut current = index_0;
    let mut coor_sum = 0;

    for i in 0..=3000 {
        if i % 1000 == 0 {
            coor_sum += list[current].value;
        }
        current = list[current].next;
    }

    println!("{}", coor_sum);
}

fn part2() {
    let input = include_str!("input.txt");

    let mut list: Vec<Node> = Vec::new();

    let mut index_0 = 0;

    for (i, value) in input
        .lines()
        .map(str::parse::<i64>)
        .map(|r| r.unwrap())
        .enumerate()
    {
        if value == 0 {
            index_0 = i;
        }

        list.push(Node {
            value: value * 811589153,
            prev: if i > 0 { i - 1 } else { i },
            next: i + 1,
        })
    }

    let n = list.len();

    list[n - 1].next = 0;
    list[0].prev = n - 1;

    for _ in 0..10 {
        for i in 0..n {
            let node = list[i];
            let steps = node.value.abs() as usize;

            if steps % (n - 1) == 0 {
                continue;
            }

            list[node.prev].next = node.next;
            list[node.next].prev = node.prev;

            let mut new = i;

            if node.value > 0 {
                for _ in 0..(steps % (n - 1)) {
                    new = list[new].next;
                }

                list[i].prev = new;
                list[i].next = list[new].next;

                list[new].next = i;
                new = list[i].next;
                list[new].prev = i;
            } else {
                for _ in 0..(steps % (n - 1)) {
                    new = list[new].prev;
                }

                list[i].next = new;
                list[i].prev = list[new].prev;

                list[new].prev = i;
                new = list[i].prev;
                list[new].next = i;
            }
        }
    }

    let mut current = index_0;
    let mut coor_sum = 0;

    for i in 0..=3000 {
        if i % 1000 == 0 {
            coor_sum += list[current].value;
        }
        current = list[current].next;
    }

    println!("{}", coor_sum);
}

fn main() {
    println!("Part 1:");
    part1();

    println!("");

    println!("Part 2:");
    part2();
}
