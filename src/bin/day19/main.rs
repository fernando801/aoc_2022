use regex::Regex;
use std::cmp;
use std::collections::HashMap;

struct State {
    time_left: i32,
    robots: [i32; 4],
    resources: [i32; 4],
}

fn dfs(
    blueprint: &Vec<Vec<(i32, usize)>>,
    max_spend: [i32; 3],
    current: State,
    cache: &mut HashMap<(i32, [i32; 4], [i32; 4]), i32>,
) -> i32 {
    if current.time_left == 0 {
        return current.resources[3];
    }

    let key = (current.time_left, current.robots, current.resources);
    // println!("{:?}", key);

    // let mut guess = String::new();
    // std::io::stdin()
    //     .read_line(&mut guess);

    if let Some(cached_value) = cache.get(&key).copied() {
        return cached_value;
    }

    let mut max_value = current.resources[3] + current.robots[3] * current.time_left;

    'robots: for (mineral, recipe) in blueprint.iter().enumerate() {
        if mineral != 3 && current.robots[mineral] >= max_spend[mineral] {
            continue;
        }

        let mut wait = 0;
        for &(req_amount, req_mineral) in recipe.iter() {
            if current.robots[req_mineral] == 0 {
                // wait will never end
                continue 'robots;
            }

            let resource_wait = ((req_amount - current.resources[req_mineral]) as f64
                / current.robots[req_mineral] as f64)
                .ceil() as i32;

            wait = cmp::max(wait, resource_wait);
        }

        let time_left = current.time_left - wait - 1;
        if time_left <= 0 {
            continue;
        }

        let mut robots = current.robots.clone();
        robots[mineral] += 1;

        let mut resources = current.resources.clone();
        for (res_type, bot_amount) in current.robots.iter().enumerate() {
            resources[res_type] += bot_amount * (wait + 1);
        }
        for &(req_amount, req_mineral) in recipe.iter() {
            resources[req_mineral] -= req_amount;
        }

        for i in 0..3 {
            resources[i] = cmp::min(resources[i], max_spend[i] * time_left);
        }

        let new_state = State {
            time_left,
            robots,
            resources,
        };

        max_value = cmp::max(max_value, dfs(blueprint, max_spend, new_state, cache));
    }

    cache.insert(key, max_value);
    return max_value;
}

fn part1() {
    let input = include_str!("input.txt");

    let re = Regex::new(r"(\d+) (\w+)").unwrap();

    let mut max_spend = [0, 0, 0];

    let mut quality_level_sum = 0;

    for (id, line) in input.lines().enumerate() {
        let mut blueprint: Vec<Vec<(i32, usize)>> = Vec::new();
        for robot_costs in line.split(". ") {
            let mut recipe: Vec<(i32, usize)> = Vec::new();
            for (amount, mineral) in re
                .find_iter(robot_costs)
                .map(|s| s.as_str().split_once(" ").unwrap())
            {
                let amount: i32 = amount.parse().unwrap();
                let mineral: usize = match mineral {
                    "ore" => 0,
                    "clay" => 1,
                    "obsidian" => 2,
                    _ => panic!("No such mineral"),
                };

                max_spend[mineral] = cmp::max(max_spend[mineral], amount);
                recipe.push((amount, mineral));
            }

            blueprint.push(recipe);
        }

        let mut cache = HashMap::new();

        let start = State {
            time_left: 24,
            robots: [1, 0, 0, 0],
            resources: [0, 0, 0, 0],
        };

        quality_level_sum += dfs(&blueprint, max_spend, start, &mut cache) * (id as i32 + 1);
    }

    println!("{}", quality_level_sum);
}

fn part2() {}

fn main() {
    println!("Part 1:");
    part1();

    println!("");

    println!("Part 2:");
    part2();
}
