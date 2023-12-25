use indicatif::ParallelProgressIterator;
use pathfinding::prelude::components;
use rayon::prelude::*;
use std::collections::HashMap;

pub fn solve(input: &str) -> usize {
    let graph: Vec<(&str, &str)> = input
        .lines()
        .flat_map(|line| {
            let (component, others) = line.split_once(": ").unwrap();
            others
                .split_ascii_whitespace()
                .map(move |other| (component, other))
        })
        .collect();

    #[cfg(debug_assertions)]
    print_graph(&graph);

    let count = graph.len() * graph.len() * graph.len() / 6;

    (0..graph.len() - 2)
        .flat_map(|a| (a + 1..graph.len() - 1).map(move |b| (a, b)))
        .flat_map(|(a, b)| (b + 1..graph.len()).map(move |c| (a, b, c)))
        .par_bridge()
        .progress_count(count as u64)
        .filter_map(|(a, b, c)| {
            let groups = graph
                .iter()
                .enumerate()
                .filter(|(i, _)| *i != a && *i != b && *i != c)
                .fold(HashMap::new(), |mut graph, (_, component)| {
                    graph
                        .entry(component.0)
                        .or_insert_with(|| vec![component.0])
                        .push(component.1);
                    graph
                        .entry(component.1)
                        .or_insert_with(|| vec![component.1])
                        .push(component.0);
                    graph
                })
                .into_values()
                .collect::<Vec<_>>();
            let groups = components(&groups);
            if groups.len() == 2 {
                Some(groups[0].len() * groups[1].len())
            } else {
                None
            }
        })
        .find_any(|_| true)
        .unwrap()
}

#[allow(dead_code)]
fn print_graph(graph: &Vec<(&str, &str)>) {
    println!("graph {{");
    for (component, other) in graph {
        println!("   {component} -- {other}");
    }
    println!("}}");
}

#[cfg(test)]
mod tests {
    use super::super::INPUT;
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        let result = solve(EXAMPLE);
        assert_eq!(result, 54);
    }

    #[test]
    #[ignore = "not done yet"]
    fn result() {
        let result = solve(INPUT);
        assert_eq!(result, 42);
    }
}
