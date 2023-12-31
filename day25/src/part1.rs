use itertools::Itertools;
use pathfinding::prelude::components;
use rayon::prelude::*;
use std::collections::{HashMap, HashSet, VecDeque};

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

    let graph_map = build_graph_hashmap(graph.iter().copied());

    // Graph sorted by detour length (longest first)
    // The edges with longest detour length are the ones that are most likely to split the graph
    let candidates: Vec<(&str, &str)> = graph
        .par_iter()
        .map(|edge| {
            (
                detour_length(edge, &graph_map).expect("Single edge can split graph"),
                edge,
            )
        })
        .collect::<Vec<_>>()
        .into_iter()
        .sorted_by_key(|(length, _)| *length)
        .rev()
        .map(|(_, edge)| *edge)
        .collect();

    // Brute-force search of all possible combinations of 3 edges
    // Stops early when it finds a combination that splits the graph into two components
    (0..graph.len() - 2)
        .flat_map(|a| (a + 1..graph.len() - 1).map(move |b| (a, b)))
        .flat_map(|(a, b)| (b + 1..graph.len()).map(move |c| (a, b, c)))
        .find_map(|(a, b, c)| {
            let groups = graph_map
                .iter()
                .map(|(node, linked)| {
                    let mut linked = linked.clone();
                    linked.retain(|x| {
                        candidates[a] != (*node, *x)
                            && candidates[a] != (*x, *node)
                            && candidates[b] != (*node, *x)
                            && candidates[b] != (*x, *node)
                            && candidates[c] != (*node, *x)
                            && candidates[c] != (*x, *node)
                    });
                    linked
                })
                .collect::<Vec<_>>();
            let groups = components(&groups);
            if groups.len() == 2 {
                Some(groups[0].len() * groups[1].len())
            } else {
                None
            }
        })
        .unwrap()
}

// Build a hashmap of all the nodes and their neighbors
fn build_graph_hashmap<'a>(
    edges: impl Iterator<Item = (&'a str, &'a str)>,
) -> HashMap<&'a str, Vec<&'a str>> {
    edges.fold(HashMap::new(), |mut graph, (l, r)| {
        graph.entry(l).or_insert_with(|| vec![l]).push(r);
        graph.entry(r).or_insert_with(|| vec![r]).push(l);
        graph
    })
}

// Number of edges in the shortest path between two nodes that does not include the direct edge
fn detour_length(
    &(start, end): &(&str, &str),
    graph_map: &HashMap<&str, Vec<&str>>,
) -> Option<usize> {
    let mut queue = VecDeque::from(vec![(start, 0)]);
    let mut visited = HashSet::new();

    while let Some((node, distance)) = queue.pop_front() {
        if visited.insert(node) {
            for &next in graph_map.get(node).unwrap() {
                if next == end {
                    if node != start {
                        return Some(distance + 1);
                    }
                } else if node != next {
                    queue.push_back((next, distance + 1));
                }
            }
        }
    }

    None
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
    fn result() {
        let result = solve(INPUT);
        assert_eq!(result, 543036);
    }
}
