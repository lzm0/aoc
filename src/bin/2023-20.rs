use std::collections::{HashMap, VecDeque};

#[derive(Clone, Debug, PartialEq)]
enum Module {
    FlipFlop {
        on: bool,
        destinations: Vec<String>,
    },
    Conjuction {
        memory: HashMap<String, Pulse>,
        destinations: Vec<String>,
    },
    Broadcast {
        destinations: Vec<String>,
    },
    Output,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Pulse {
    High,
    Low,
}

impl Module {
    fn destinations(&self) -> Vec<String> {
        match self {
            Module::FlipFlop { destinations, .. } => destinations.clone(),
            Module::Conjuction { destinations, .. } => destinations.clone(),
            Module::Broadcast { destinations } => destinations.clone(),
            Module::Output => vec![],
        }
    }
}

fn parse(input: &str) -> HashMap<String, Module> {
    let mut configuration = HashMap::new();

    for line in input.lines() {
        let (left, right) = line.split_once(" -> ").unwrap();
        let destinations = right.split(", ").map(|s| s.to_string()).collect();
        if left == "broadcaster" {
            configuration.insert(left.to_string(), Module::Broadcast { destinations });
            continue;
        }
        if left.starts_with('%') {
            configuration.insert(
                left.strip_prefix('%').unwrap().to_string(),
                Module::FlipFlop {
                    on: false,
                    destinations: destinations.clone(),
                },
            );
            continue;
        }
        if left.starts_with('&') {
            configuration.insert(
                left.strip_prefix('&').unwrap().to_string(),
                Module::Conjuction {
                    memory: HashMap::new(),
                    destinations: destinations.clone(),
                },
            );
            continue;
        }
        unreachable!();
    }

    for (name, module) in configuration.clone().iter_mut() {
        for destination in module.destinations() {
            if let Some(Module::Conjuction { memory, .. }) = configuration.get_mut(&destination) {
                memory.insert(name.clone(), Pulse::Low);
            }
            if !configuration.contains_key(&destination) {
                configuration.insert(destination.clone(), Module::Output);
            }
        }
    }
    configuration
}

fn part_one(configuration: &mut HashMap<String, Module>) -> u64 {
    let mut low_count = 0;
    let mut high_count = 0;
    for _ in 0..1000 {
        let mut queue =
            VecDeque::from([("button".to_string(), "broadcaster".to_string(), Pulse::Low)]);

        while let Some(item) = queue.pop_front() {
            let (input, name, pulse) = item;

            match pulse {
                Pulse::High => high_count += 1,
                Pulse::Low => low_count += 1,
            }

            let module = configuration.get_mut(&name).unwrap();
            let output = match module {
                Module::FlipFlop { on, .. } => {
                    if pulse == Pulse::Low {
                        *on = !*on;
                        if *on {
                            Some(Pulse::High)
                        } else {
                            Some(Pulse::Low)
                        }
                    } else {
                        None
                    }
                }
                Module::Conjuction { memory, .. } => {
                    memory.insert(input.clone(), pulse);
                    if memory.values().all(|&p| p == Pulse::High) {
                        Some(Pulse::Low)
                    } else {
                        Some(Pulse::High)
                    }
                }
                Module::Broadcast { .. } => Some(pulse),
                Module::Output => None,
            };
            if let Some(output) = output {
                for destination in module.destinations() {
                    queue.push_back((name.clone(), destination, output));
                }
            }
        }
    }
    low_count * high_count
}

fn part_two(configuration: &mut HashMap<String, Module>) -> u64 {
    for i in 1.. {
        let mut queue =
            VecDeque::from([("button".to_string(), "broadcaster".to_string(), Pulse::Low)]);
        while let Some(item) = queue.pop_front() {
            let (input, name, pulse) = item;
            if name == "xm" && pulse == Pulse::High {
                dbg!(i, &input);
            }

            let module = configuration.get_mut(&name).unwrap();
            let output = match module {
                Module::FlipFlop { on, .. } => {
                    if pulse == Pulse::Low {
                        *on = !*on;
                        if *on {
                            Some(Pulse::High)
                        } else {
                            Some(Pulse::Low)
                        }
                    } else {
                        None
                    }
                }
                Module::Conjuction { memory, .. } => {
                    memory.insert(input.clone(), pulse);
                    if memory.values().all(|&p| p == Pulse::High) {
                        Some(Pulse::Low)
                    } else {
                        Some(Pulse::High)
                    }
                }
                Module::Broadcast { .. } => Some(pulse),
                Module::Output => None,
            };
            if let Some(output) = output {
                for destination in module.destinations() {
                    queue.push_back((name.clone(), destination, output));
                }
            }
        }
    }
    unreachable!();
}

fn main() {
    let configuration = parse(include_str!("../input/2023-20.txt"));

    println!("Part one: {}", part_one(&mut configuration.clone()));
    println!("Part two: {}", part_two(&mut configuration.clone()));
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const EXAMPLE_ONE: &str = indoc! {"
        broadcaster -> a, b, c
        %a -> b
        %b -> c
        %c -> inv
        &inv -> a
    "};

    const EXAMPLE_TWO: &str = indoc! {"
        broadcaster -> a
        %a -> inv, con
        &inv -> b
        %b -> con
        &con -> output
"};

    #[test]
    fn test_part_one_example_one() {
        assert_eq!(part_one(&mut parse(EXAMPLE_ONE)), 32000000);
    }

    #[test]
    fn test_part_one_example_two() {
        assert_eq!(part_one(&mut parse(EXAMPLE_TWO)), 11687500);
    }
}
