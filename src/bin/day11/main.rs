use std::collections::VecDeque;

enum Operation {
    Add(u128),
    Mul(u128),
    Square,
    NA,
}

struct Monkey {
    items: VecDeque<u128>,
    operation: Operation,
    divisor: u128,
    options: (usize, usize),
    count: usize,
}

impl Monkey {
    fn new() -> Self {
        Monkey {
            items: VecDeque::new(),
            operation: Operation::NA,
            divisor: 1,
            options: (0, 0),
            count: 0,
        }
    }

    fn throw_item(&mut self, reliever: Box<dyn Fn(u128) -> u128>) -> (usize, u128) {
        self.count += 1;

        let item = self.items.pop_front().unwrap();
        let value = match self.operation {
            Operation::Add(v) => item + v,
            Operation::Mul(v) => item * v,
            Operation::Square => item * item,
            Operation::NA => item,
        };

        let value = reliever(value);

        if value % self.divisor == 0 {
            return (self.options.0, value);
        } else {
            return (self.options.1, value);
        }
    }
}

fn part1() {
    let input = include_str!("input.txt");
    let monkey_data = input.split("\n\n");

    let mut monkeys = Vec::new();

    for info in monkey_data {
        let mut m = Monkey::new();

        let lines: Vec<&str> = info.lines().collect();

        m.items = lines[1]["  Starting items: ".len()..]
            .split(", ")
            .map(|x| x.parse().unwrap())
            .collect();

        m.operation = lines[2]["  Operation: new = old * ".len()..]
            .parse()
            .map(|v| {
                if lines[2].contains('+') {
                    Operation::Add(v)
                } else {
                    Operation::Mul(v)
                }
            })
            .unwrap_or(Operation::Square);

        m.divisor = lines[3]["  Test: divisible by ".len()..].parse().unwrap();

        m.options = (
            lines[4]["    If true: throw to monkey ".len()..]
                .parse()
                .unwrap(),
            lines[5]["    If false: throw to monkey ".len()..]
                .parse()
                .unwrap(),
        );

        monkeys.push(m);
    }

    for _ in 0..20 {
        for from in 0..monkeys.len() {
            while !monkeys[from].items.is_empty() {
                let (to, item) = monkeys[from].throw_item(Box::new(|i| i / 3));
                monkeys[to].items.push_back(item);
            }
        }
    }

    let mut inspections: Vec<usize> = monkeys.iter().map(|m| m.count).collect();
    inspections.sort_by(|a, b| b.cmp(a));

    println!("{}", inspections[0] * inspections[1]);
}

fn part2() {
    let input = include_str!("input.txt");
    let monkey_data = input.split("\n\n");

    let mut monkeys = Vec::new();

    for info in monkey_data {
        let mut m = Monkey::new();

        let lines: Vec<&str> = info.lines().collect();

        m.items = lines[1]["  Starting items: ".len()..]
            .split(", ")
            .map(|x| x.parse().unwrap())
            .collect();

        m.operation = lines[2]["  Operation: new = old * ".len()..]
            .parse()
            .map(|v| {
                if lines[2].contains('+') {
                    Operation::Add(v)
                } else {
                    Operation::Mul(v)
                }
            })
            .unwrap_or(Operation::Square);

        m.divisor = lines[3]["  Test: divisible by ".len()..].parse().unwrap();

        m.options = (
            lines[4]["    If true: throw to monkey ".len()..]
                .parse()
                .unwrap(),
            lines[5]["    If false: throw to monkey ".len()..]
                .parse()
                .unwrap(),
        );

        monkeys.push(m);
    }

    let prod: u128 = monkeys.iter().map(|m| m.divisor).product();

    for _ in 0..10_000 {
        for from in 0..monkeys.len() {
            while !monkeys[from].items.is_empty() {
                let (to, item) = monkeys[from].throw_item(Box::new(move |i| i % prod));
                monkeys[to].items.push_back(item);
            }
        }
    }

    let mut inspections: Vec<usize> = monkeys.iter().map(|m| m.count).collect();
    inspections.sort_by(|a, b| b.cmp(a));

    println!("{}", inspections[0] * inspections[1]);
}

fn main() {
    println!("Part 1:");
    part1();

    println!("");

    println!("Part 2:");
    part2();
}
