use num::Integer;
use std::{
    cell::{Cell, RefCell},
    collections::{HashMap, VecDeque},
};

pub fn solve(input: &str) -> usize {
    let (mut modules, sources) = read_input(input);

    let cycle_targets = get_cycle_targets("rx", &modules, &sources);
    let mut cycles = vec![0; cycle_targets.len()];
    let mut n = 0;

    while cycles.iter().any(|x| *x == 0) {
        n += 1;
        run_cycle(&mut modules, &cycle_targets)
            .into_iter()
            .enumerate()
            .filter(|(_, x)| *x)
            .for_each(|(m, _)| cycles[m] = n)
    }

    cycles.into_iter().reduce(|a, b| a.lcm(&b)).unwrap()
}

fn read_input(input: &str) -> (HashMap<&str, Module>, HashMap<&str, Vec<&str>>) {
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
    let modules = modules
        .into_iter()
        .map(|module| {
            if let ModuleType::Conjunction(ref hashmap) = module.module_type {
                hashmap.borrow_mut().extend(
                    sources
                        .get(module.name)
                        .unwrap()
                        .iter()
                        .map(|source| (*source, false)),
                );
            }
            (module.name, module)
        })
        .collect();
    (modules, sources)
}

fn get_cycle_targets<'a>(
    name: &'a str,
    modules: &HashMap<&'a str, Module>,
    sources: &HashMap<&'a str, Vec<&'a str>>,
) -> Vec<(&'a str, bool)> {
    let src = sources.get(name).unwrap();

    if name == "rx" {
        get_cycle_targets(src.first().unwrap(), modules, sources)
    } else if src
        .iter()
        .all(|s| modules.get(s).unwrap().destinations.len() == 1)
    {
        src.iter()
            .flat_map(|s| get_cycle_targets(s, modules, sources))
            .map(|(s, x)| (s, !x))
            .collect()
    } else {
        vec![(name, false)]
    }
}

fn run_cycle(modules: &mut HashMap<&str, Module>, cycle_targets: &Vec<(&str, bool)>) -> Vec<bool> {
    let mut result = vec![false; cycle_targets.len()];
    let mut queue = VecDeque::from([Pulse {
        is_high: false,
        source: "button",
        destination: "broadcaster",
    }]);

    while let Some(pulse) = queue.pop_front() {
        if let Some(n) = cycle_targets
            .iter()
            .position(|(s, x)| *s == pulse.source && *x == pulse.is_high)
        {
            result[n] = true;
        }

        if let Some(module) = modules.get(pulse.destination) {
            queue.extend(module.process(pulse));
        }
    }

    result
}

enum ModuleType<'a> {
    Broadcaster,
    FlipFlop(Cell<bool>),
    Conjunction(RefCell<HashMap<&'a str, bool>>),
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
                module_type: ModuleType::FlipFlop(Cell::new(false)),
                destinations,
            },
            Some('&') => Module {
                name: &name[1..],
                module_type: ModuleType::Conjunction(RefCell::new(HashMap::new())),
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

    fn process(&self, pulse: Pulse) -> impl Iterator<Item = Pulse> {
        let next_pulse = match &self.module_type {
            ModuleType::Broadcaster => Some(pulse.is_high),
            ModuleType::Conjunction(memory) => {
                *memory.borrow_mut().get_mut(pulse.source).unwrap() = pulse.is_high;
                Some(!memory.borrow().iter().all(|(_, x)| *x))
            }
            ModuleType::FlipFlop(state) => {
                if !pulse.is_high {
                    state.set(!state.get());
                    Some(state.get())
                } else {
                    None
                }
            }
        };
        let source = self.name;
        self.destinations.iter().filter_map(move |destination| {
            next_pulse.map(|is_high| Pulse {
                is_high,
                source,
                destination,
            })
        })
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

    #[test]
    fn result() {
        let result = solve(INPUT);
        assert_eq!(result, 244151741342687);
    }
}
