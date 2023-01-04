fn part1() {
    let input = include_str!("input.txt");

    let sum: i64 = input
        .lines()
        .flat_map(|l| {
            l.chars().rev().enumerate().map(|(exp, c)| {
                let dv = match c {
                    '=' => -2,
                    '-' => -1,
                    '0' => 0,
                    '1' => 1,
                    '2' => 2,
                    _ => panic!(),
                };

                dv * 5i64.pow(exp as u32)
            })
        })
        .sum();

    let digits = ['=', '-', '0', '1', '2'];
    let mut snafu = String::new();

    let mut exp = 0;
    let mut offset = 0;
    loop {
        let pow = 5i64.pow(exp);
        offset += 2 * pow;

        let i = (sum + offset) / pow % 5;
        snafu.insert(0, digits[i as usize]);

        if offset >= sum {
            break;
        }

        exp += 1;
    }

    println!("{}", snafu);
}

fn main() {
    println!("Part 1:");
    part1();
}
