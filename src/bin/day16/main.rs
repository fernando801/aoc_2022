use priority_queue::PriorityQueue;
use std::collections::{BTreeSet, HashMap};

struct Valve {
    id: String,
    flow_rate: u32,
    connections: Vec<String>,
}

#[derive(Hash, PartialEq, Eq, Debug)]
struct State {
    mins_left: u32,
    closed_valves: BTreeSet<String>,
    total_preassure: u32,
    valve: String,
    limit: u32,
}

#[derive(Hash, PartialEq, Eq, Debug)]
struct State2 {
    mins_left: u32,
    closed_valves: BTreeSet<String>,
    total_preassure: u32,
    valve1: String,
    valve2: String,
    limit: u32,
}

fn max_preassure_release(graph: HashMap<String, Valve>) -> u32 {
    let initial_state = State {
        mins_left: 30,
        closed_valves: graph
            .iter()
            .filter(|&(_, valve)| valve.flow_rate > 0)
            .map(|(key, _)| key.clone())
            .collect(),
        total_preassure: 0,
        valve: String::from("AA"),
        limit: graph.iter().map(|(_, valve)| valve.flow_rate).sum::<u32>() * 30,
    };

    let mut best_limit: u32 = u32::MIN;

    let mut queue: PriorityQueue<State, u32> = PriorityQueue::new();

    let priority = initial_state.limit;

    queue.push(initial_state, priority);

    while !queue.is_empty() {
        let (current, _) = queue.pop().unwrap();

        if current.total_preassure > best_limit {
            best_limit = current.total_preassure;
        }

        if current.limit < best_limit {
            continue;
        }

        if current.mins_left == 0 || current.closed_valves.is_empty() {
            if current.limit > best_limit {
                best_limit = current.limit;
            }
            continue;
        }

        let valve = graph.get(&current.valve).unwrap();
        let mut connections = valve.connections.clone();
        connections.sort_by(|a, b| {
            let a = graph.get(a).unwrap().flow_rate;
            let b = graph.get(b).unwrap().flow_rate;
            b.cmp(&a)
        });

        for neighbor in connections {
            let new_state = State {
                mins_left: current.mins_left - 1,
                closed_valves: current.closed_valves.clone(),
                total_preassure: current.total_preassure,
                valve: neighbor,
                limit: current.total_preassure
                    + current
                        .closed_valves
                        .iter()
                        .map(|key| {
                            let v = graph.get(key).unwrap();
                            v.flow_rate
                        })
                        .sum::<u32>()
                        * (current.mins_left - 1),
            };

            let priority = new_state.limit;

            queue.push(new_state, priority);
        }

        if current.closed_valves.contains(&valve.id) {
            let mins_left = current.mins_left - 1;
            let total_preassure = current.total_preassure + valve.flow_rate * mins_left;
            let mut new_closed_valves = current.closed_valves.clone();
            new_closed_valves.remove(&valve.id);

            let new_state = State {
                mins_left,
                total_preassure,
                valve: current.valve,
                limit: total_preassure
                    + new_closed_valves
                        .iter()
                        .map(|key| {
                            let v = graph.get(key).unwrap();
                            v.flow_rate
                        })
                        .sum::<u32>()
                        * mins_left,
                closed_valves: new_closed_valves,
            };

            let priority = new_state.limit;

            queue.push(new_state, priority);
        }
    }

    return best_limit;
}

fn max_preassure_release2(graph: HashMap<String, Valve>) -> u32 {
    let mut ordered_effective_valves: Vec<(String, u32)> = graph
        .iter()
        .filter(|&(_, valve)| valve.flow_rate > 0)
        .map(|(key, valve)| (key.clone(), valve.flow_rate))
        .collect();

    ordered_effective_valves.sort_by(|a, b| {
        let a = graph.get(&a.0).unwrap().flow_rate;
        let b = graph.get(&b.0).unwrap().flow_rate;
        b.cmp(&a)
    });

    let mut lim = 0;

    for (i, time_left) in (0..=26).rev().step_by(2).enumerate() {
        if i < ordered_effective_valves.len() {
            lim += time_left * ordered_effective_valves[i].1;
        } else {
            break;
        }
    }

    let initial_state = State2 {
        mins_left: 26,
        closed_valves: graph
            .iter()
            .filter(|&(_, valve)| valve.flow_rate > 0)
            .map(|(key, _)| key.clone())
            .collect(),
        total_preassure: 0,
        valve1: String::from("AA"),
        valve2: String::from("AA"),
        limit: lim,
    };

    let effective_valves = graph
        .iter()
        .filter(|&(_, valve)| valve.flow_rate > 0)
        .count() as u32;

    let mut best_limit: u32 = u32::MIN;

    let mut queue: PriorityQueue<State2, u32> = PriorityQueue::new();

    let valves_opened = effective_valves - initial_state.closed_valves.len() as u32;
    let priority = initial_state.limit + initial_state.total_preassure + valves_opened;

    queue.push(initial_state, priority);

    while !queue.is_empty() {
        let (current, _) = queue.pop().unwrap();

        if current.total_preassure > best_limit {
            best_limit = current.total_preassure;
        }

        if current.limit < best_limit {
            continue;
        }

        if current.mins_left == 0 || current.closed_valves.is_empty() {
            if current.total_preassure > best_limit {
                best_limit = current.total_preassure;
            }
            continue;
        }

        let valve1 = graph.get(&current.valve1).unwrap();
        let mut connections1 = valve1.connections.clone();
        connections1.sort_by(|a, b| {
            let a = graph.get(a).unwrap().flow_rate;
            let b = graph.get(b).unwrap().flow_rate;
            b.cmp(&a)
        });

        let valve2 = graph.get(&current.valve2).unwrap();
        let mut connections2 = valve2.connections.clone();
        connections2.sort_by(|a, b| {
            let a = graph.get(a).unwrap().flow_rate;
            let b = graph.get(b).unwrap().flow_rate;
            b.cmp(&a)
        });

        let mut ord_left_valves: Vec<(String, u32)> = current
            .closed_valves
            .iter()
            .map(|a| (a.clone(), graph.get(a).unwrap().flow_rate))
            .collect();
        ord_left_valves.sort_by(|a, b| {
            let a = graph.get(&a.0).unwrap().flow_rate;
            let b = graph.get(&b.0).unwrap().flow_rate;
            b.cmp(&a)
        });

        let mut lim = 0;

        for (i, time_left) in (0..=current.mins_left + 1).rev().step_by(2).enumerate() {
            if i < ord_left_valves.len() {
                lim += time_left * ord_left_valves[i].1;
            } else {
                break;
            }
        }

        for neighbor1 in &connections1 {
            for neighbor2 in &connections2 {
                let new_state = State2 {
                    mins_left: current.mins_left - 1,
                    closed_valves: current.closed_valves.clone(),
                    total_preassure: current.total_preassure,
                    valve1: neighbor1.clone(),
                    valve2: neighbor2.clone(),
                    limit: current.total_preassure + lim,
                };

                let valves_opened = effective_valves - new_state.closed_valves.len() as u32;
                let priority = new_state.limit + new_state.total_preassure + valves_opened;

                queue.push(new_state, priority);
            }
        }

        if current.closed_valves.contains(&valve1.id) {
            let mins_left = current.mins_left - 1;
            let total_preassure = current.total_preassure + valve1.flow_rate * mins_left;
            let mut new_closed_valves = current.closed_valves.clone();
            new_closed_valves.remove(&valve1.id);

            let mut ord_left_valves: Vec<(String, u32)> = new_closed_valves
                .iter()
                .map(|a| (a.clone(), graph.get(a).unwrap().flow_rate))
                .collect();
            ord_left_valves.sort_by(|a, b| {
                let a = graph.get(&a.0).unwrap().flow_rate;
                let b = graph.get(&b.0).unwrap().flow_rate;
                b.cmp(&a)
            });

            let mut lim = 0;

            for (i, time_left) in (0..=current.mins_left + 1).rev().step_by(2).enumerate() {
                if i < ord_left_valves.len() {
                    lim += time_left * ord_left_valves[i].1;
                } else {
                    break;
                }
            }

            for neighbor2 in connections2 {
                let new_state = State2 {
                    mins_left,
                    total_preassure,
                    valve1: valve1.id.clone(),
                    valve2: neighbor2,
                    limit: total_preassure + lim,
                    closed_valves: new_closed_valves.clone(),
                };

                let valves_opened = effective_valves - new_state.closed_valves.len() as u32;
                let priority = new_state.limit + new_state.total_preassure + valves_opened;

                queue.push(new_state, priority);
            }
        }

        if current.closed_valves.contains(&valve2.id) {
            let mins_left = current.mins_left - 1;
            let total_preassure = current.total_preassure + valve2.flow_rate * mins_left;
            let mut new_closed_valves = current.closed_valves.clone();
            new_closed_valves.remove(&valve2.id);

            let mut ord_left_valves: Vec<(String, u32)> = new_closed_valves
                .iter()
                .map(|a| (a.clone(), graph.get(a).unwrap().flow_rate))
                .collect();
            ord_left_valves.sort_by(|a, b| {
                let a = graph.get(&a.0).unwrap().flow_rate;
                let b = graph.get(&b.0).unwrap().flow_rate;
                b.cmp(&a)
            });

            let mut lim = 0;

            for (i, time_left) in (0..=current.mins_left + 1).rev().step_by(2).enumerate() {
                if i < ord_left_valves.len() {
                    lim += time_left * ord_left_valves[i].1;
                } else {
                    break;
                }
            }

            for neighbor1 in connections1 {
                let new_state = State2 {
                    mins_left,
                    total_preassure,
                    valve1: neighbor1,
                    valve2: valve2.id.clone(),
                    limit: total_preassure + lim,
                    closed_valves: new_closed_valves.clone(),
                };

                let valves_opened = effective_valves - new_state.closed_valves.len() as u32;
                let priority = new_state.limit + new_state.total_preassure + valves_opened;

                queue.push(new_state, priority);
            }
        }

        if current.closed_valves.contains(&valve1.id)
            && current.closed_valves.contains(&valve2.id)
            && valve1.id != valve2.id
        {
            let mins_left = current.mins_left - 1;
            let total_preassure =
                current.total_preassure + (valve1.flow_rate + valve2.flow_rate) * mins_left;
            let mut new_closed_valves = current.closed_valves.clone();
            new_closed_valves.remove(&valve1.id);
            new_closed_valves.remove(&valve2.id);

            let mut ord_left_valves: Vec<(String, u32)> = new_closed_valves
                .iter()
                .map(|a| (a.clone(), graph.get(a).unwrap().flow_rate))
                .collect();
            ord_left_valves.sort_by(|a, b| {
                let a = graph.get(&a.0).unwrap().flow_rate;
                let b = graph.get(&b.0).unwrap().flow_rate;
                b.cmp(&a)
            });

            let mut lim = 0;

            for (i, time_left) in (0..=current.mins_left + 1).rev().step_by(2).enumerate() {
                if i < ord_left_valves.len() {
                    lim += time_left * ord_left_valves[i].1;
                } else {
                    break;
                }
            }

            let new_state = State2 {
                mins_left,
                total_preassure,
                valve1: current.valve1,
                valve2: current.valve2,
                limit: total_preassure + lim,
                closed_valves: new_closed_valves,
            };

            let valves_opened = effective_valves - new_state.closed_valves.len() as u32;
            let priority = new_state.limit + new_state.total_preassure + valves_opened;

            queue.push(new_state, priority);
        }
    }

    return best_limit;
}

fn part1() {
    let input = include_str!("input.txt");

    let mut graph: HashMap<String, Valve> = HashMap::new();

    for l in input.lines() {
        let trim = &l["Valve ".len()..];
        let (valve_flow_rate_str, connections_str) =
            match trim.split_once("; tunnels lead to valves ") {
                Some(pair) => pair,
                None => trim.split_once("; tunnel leads to valve ").unwrap(),
            };
        let (id, flow_rate) = valve_flow_rate_str.split_once(" has flow rate=").unwrap();
        let connections: Vec<String> = connections_str
            .split(", ")
            .map(|str| str.to_owned())
            .collect();

        let valve = Valve {
            id: id.to_owned(),
            flow_rate: flow_rate.parse().unwrap(),
            connections,
        };

        graph.insert(id.to_owned(), valve);
    }

    let max_preassure = max_preassure_release(graph);

    println!("{}", max_preassure);
}

fn part2() {
    let input = include_str!("input.txt");

    let mut graph: HashMap<String, Valve> = HashMap::new();

    for l in input.lines() {
        let trim = &l["Valve ".len()..];
        let (valve_flow_rate_str, connections_str) =
            match trim.split_once("; tunnels lead to valves ") {
                Some(pair) => pair,
                None => trim.split_once("; tunnel leads to valve ").unwrap(),
            };
        let (id, flow_rate) = valve_flow_rate_str.split_once(" has flow rate=").unwrap();
        let connections: Vec<String> = connections_str
            .split(", ")
            .map(|str| str.to_owned())
            .collect();

        let valve = Valve {
            id: id.to_owned(),
            flow_rate: flow_rate.parse().unwrap(),
            connections,
        };

        graph.insert(id.to_owned(), valve);
    }

    let max_preassure = max_preassure_release2(graph);

    println!("{}", max_preassure);
}

fn main() {
    println!("Part 1:");
    part1();

    println!("");

    println!("Part 2:");
    part2();
}
