use std::collections::HashMap;

#[derive(Clone)]
enum Job {
    Number(i64),
    Operation {
        key1: String,
        operator: String,
        key2: String,
    },
}

fn get_job_value(key: &String, jobs: &mut HashMap<String, Job>) -> i64 {
    match jobs.get(key).cloned().unwrap() {
        Job::Number(value) => value,
        Job::Operation {
            key1,
            operator,
            key2,
        } => {
            let value = match &operator[..] {
                "+" => get_job_value(&key1, jobs) + get_job_value(&key2, jobs),
                "-" => get_job_value(&key1, jobs) - get_job_value(&key2, jobs),
                "*" => get_job_value(&key1, jobs) * get_job_value(&key2, jobs),
                "/" => get_job_value(&key1, jobs) / get_job_value(&key2, jobs),
                _ => panic!(),
            };

            jobs.insert(key.to_string(), Job::Number(value));
            value
        }
    }
}

fn part1() {
    let input = include_str!("input.txt");

    let mut jobs: HashMap<String, Job> = HashMap::new();

    for line in input.lines() {
        let (key, value_str) = line.split_once(": ").unwrap();

        if let Ok(int) = value_str.parse::<i64>() {
            jobs.insert(String::from(key), Job::Number(int));
        } else {
            let mut split = value_str.split_whitespace();

            let key1 = split.next().unwrap().to_string();
            let operator = split.next().unwrap().to_string();
            let key2 = split.next().unwrap().to_string();

            jobs.insert(
                String::from(key),
                Job::Operation {
                    key1,
                    operator,
                    key2,
                },
            );
        }
    }

    let root_value = get_job_value(&String::from("root"), &mut jobs);

    println!("{}", root_value);
}

fn part2() {}

fn main() {
    println!("Part 1:");
    part1();

    println!("");

    println!("Part 2:");
    part2();
}
