use std::collections::{HashMap, VecDeque};

pub fn solve(input: &str) -> usize {
    let mut modules = read_input(input);
    let mut low_total = 0;
    let mut high_total = 0;

    for _ in 0..1000 {
        let (low_count, high_count) = run_cycle(&mut modules);
        low_total += low_count;
        high_total += high_count;
    }

    low_total * high_total
}

fn read_input(input: &str) -> HashMap<&str, Module> {
    let modules = input.lines().map(Module::parse).collect::<Vec<_>>();
    let sources = modules.iter().fold(
        HashMap::new(),
        |mut hashmap: HashMap<&str, Vec<&str>>, module| {
            module.destinations.iter().for_each(|destination| {
                hashmap.entry(*destination).or_default().push(module.name);
            });
            hashmap
        },
    );
    modules
        .into_iter()
        .map(|mut module| {
            if let ModuleType::Conjunction(ref mut hashmap) = module.module_type {
                hashmap.extend(
                    sources
                        .get(module.name)
                        .unwrap()
                        .iter()
                        .map(|source| (*source, false)),
                );
            }
            (module.name, module)
        })
        .collect()
}

fn run_cycle(modules: &mut HashMap<&str, Module>) -> (usize, usize) {
    let mut low_count = 0;
    let mut high_count = 0;
    let mut queue = VecDeque::from([Pulse {
        is_high: false,
        source: "button",
        destination: "broadcaster",
    }]);

    while let Some(pulse) = queue.pop_front() {
        if pulse.is_high {
            high_count += 1
        } else {
            low_count += 1
        }

        if let Some(module) = modules.get_mut(pulse.destination) {
            match &mut module.module_type {
                ModuleType::Broadcaster => {
                    for destination in &module.destinations {
                        queue.push_back(Pulse {
                            is_high: pulse.is_high,
                            source: module.name,
                            destination,
                        })
                    }
                }
                ModuleType::Conjunction(ref mut inputs) => {
                    *inputs.get_mut(pulse.source).unwrap() = pulse.is_high;
                    let is_high = !inputs.iter().all(|(_, x)| *x);
                    for destination in &module.destinations {
                        queue.push_back(Pulse {
                            is_high,
                            source: module.name,
                            destination,
                        })
                    }
                }
                ModuleType::FlipFlop(ref mut is_on) => {
                    if !pulse.is_high {
                        *is_on = !*is_on;
                        for destination in &module.destinations {
                            queue.push_back(Pulse {
                                is_high: *is_on,
                                source: module.name,
                                destination,
                            })
                        }
                    }
                }
            }
        }
    }

    (low_count, high_count)
}

enum ModuleType<'a> {
    Broadcaster,
    FlipFlop(bool),
    Conjunction(HashMap<&'a str, bool>),
}

struct Module<'a> {
    name: &'a str,
    module_type: ModuleType<'a>,
    destinations: Vec<&'a str>,
}

impl<'a> Module<'a> {
    fn parse(line: &str) -> Module {
        let (name, destinations) = line.split_once(" -> ").unwrap();
        let destinations = destinations.split(", ").collect::<Vec<_>>();
        match name.chars().next() {
            Some('%') => Module {
                name: &name[1..],
                module_type: ModuleType::FlipFlop(false),
                destinations,
            },
            Some('&') => Module {
                name: &name[1..],
                module_type: ModuleType::Conjunction(HashMap::new()),
                destinations,
            },
            _ if name == "broadcaster" => Module {
                name,
                module_type: ModuleType::Broadcaster,
                destinations,
            },
            _ => panic!("Unknown module type: {}", line),
        }
    }
}

struct Pulse<'a> {
    is_high: bool,
    source: &'a str,
    destination: &'a str,
}

#[cfg(test)]
mod tests {
    use super::super::INPUT;
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");
    const EXAMPLE2: &str = include_str!("../example2.txt");

    #[test]
    fn example() {
        let result = solve(EXAMPLE);
        assert_eq!(result, 32000000);
    }

    #[test]
    fn example2() {
        let result = solve(EXAMPLE2);
        assert_eq!(result, 11687500);
    }

    #[test]
    fn result() {
        let result = solve(INPUT);
        assert_eq!(result, 743871576);
    }
}
