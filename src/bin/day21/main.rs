use std::collections::HashMap;

#[derive(Clone)]
enum Job {
    Variable,
    Number(i64),
    Operation {
        key1: String,
        operator: String,
        key2: String,
    },
}

fn get_job_value(key: &String, jobs: &mut HashMap<String, Job>) -> i64 {
    match jobs.get(key).cloned().unwrap() {
        Job::Variable => panic!(),
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

fn fit_to_expected(
    key: &String,
    jobs: &mut HashMap<String, Job>,
    expected: Option<i64>,
) -> Option<i64> {
    match jobs.get(key).cloned().unwrap() {
        Job::Variable => {
            if let Some(value) = expected {
                jobs.insert(key.to_string(), Job::Number(value));
            }
            expected
        }
        Job::Number(value) => Some(value),
        Job::Operation {
            key1,
            operator,
            key2,
        } => {
            if let Some(exp_value) = expected {
                if let Some(c) = fit_to_expected(&key2, jobs, None) {
                    let x = match &operator[..] {
                        "+" => exp_value - c,
                        "-" => exp_value + c,
                        "*" => exp_value / c,
                        "/" => exp_value * c,
                        _ => panic!(),
                    };

                    let value = match &operator[..] {
                        "+" => fit_to_expected(&key1, jobs, Some(x))? + c,
                        "-" => fit_to_expected(&key1, jobs, Some(x))? - c,
                        "*" => fit_to_expected(&key1, jobs, Some(x))? * c,
                        "/" => fit_to_expected(&key1, jobs, Some(x))? / c,
                        _ => panic!(),
                    };

                    jobs.insert(key.to_string(), Job::Number(value));
                    Some(value)
                } else {
                    let c = fit_to_expected(&key1, jobs, None).unwrap();
                    let x = match &operator[..] {
                        "+" => exp_value - c,
                        "-" => c - exp_value,
                        "*" => exp_value / c,
                        "/" => c / exp_value,
                        _ => panic!(),
                    };

                    let value = match &operator[..] {
                        "+" => c + fit_to_expected(&key2, jobs, Some(x))?,
                        "-" => c - fit_to_expected(&key2, jobs, Some(x))?,
                        "*" => c * fit_to_expected(&key2, jobs, Some(x))?,
                        "/" => c / fit_to_expected(&key2, jobs, Some(x))?,
                        _ => panic!(),
                    };

                    jobs.insert(key.to_string(), Job::Number(value));
                    Some(value)
                }
            } else {
                let value = match &operator[..] {
                    "+" => {
                        fit_to_expected(&key1, jobs, None)? + fit_to_expected(&key2, jobs, None)?
                    }
                    "-" => {
                        fit_to_expected(&key1, jobs, None)? - fit_to_expected(&key2, jobs, None)?
                    }
                    "*" => {
                        fit_to_expected(&key1, jobs, None)? * fit_to_expected(&key2, jobs, None)?
                    }
                    "/" => {
                        fit_to_expected(&key1, jobs, None)? / fit_to_expected(&key2, jobs, None)?
                    }
                    _ => panic!(),
                };

                jobs.insert(key.to_string(), Job::Number(value));
                Some(value)
            }
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

fn part2() {
    let input = include_str!("input.txt");

    let mut jobs: HashMap<String, Job> = HashMap::new();

    for line in input.lines() {
        let (key, value_str) = line.split_once(": ").unwrap();

        if key == "humn" {
            jobs.insert(String::from(key), Job::Variable);
            continue;
        }

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

    match jobs.get_mut(&String::from("root")).unwrap() {
        Job::Operation { operator, .. } => *operator = String::from("-"),
        _ => panic!(),
    };

    fit_to_expected(&String::from("root"), &mut jobs, Some(0));

    println!(
        "{}",
        fit_to_expected(&String::from("humn"), &mut jobs, None).unwrap()
    );
}

fn main() {
    println!("Part 1:");
    part1();

    println!("");

    println!("Part 2:");
    part2();
}
