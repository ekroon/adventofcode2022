use std::collections::HashSet;
use petgraph::algo::dijkstra;
use petgraph::Graph;

const INPUT: &str = include_str!("input.txt");

fn solve(input: &str) -> (i32, i32) {

    let mut points = Vec::new();

    let mut start = None;
    let mut end = None;
    for (i, line) in input.lines().enumerate() {
        for (j, ch) in line.chars().enumerate() {
            if ch == 'S' {
                points.push((i, j, 'a'));
                start.replace((i,j,'a'));
            }
            if ch == 'E' {
                points.push((i, j, 'z'));
                end.replace((i,j,'z'));
            } else {
                points.push((i, j, ch));
            }
        }
    }

    let mut graph = Graph::new();
    let mut a_nodes = HashSet::new();

    for &(i, j, ch) in points.iter() {
        let idx = graph.add_node((i, j, ch));
        if ch == 'a' {
            a_nodes.insert(idx);
        }
    }

    for &(i, j, ch) in points.iter() {
        for &(k, l, ch2) in points.iter() {
            if (i, j) == (k, l) {
                continue;
            }

            // compare if ch is one lower than c2

            if ch as u8 + 1 >= ch2 as u8 {
                // Check if (k, l) is a neighbor of (i, j)
                if  (i, j+1) == (k, l) ||
                    (j > 0) && (i, j-1) == (k, l) ||
                    (i+1, j) == (k, l) ||
                    (i > 0) && (i-1, j) == (k, l) {
                    // get node (i, j) from graph
                    let node1 = graph.node_indices().find(|&n| graph[n] == (i, j, ch)).unwrap();
                    let node2 = graph.node_indices().find(|&n| graph[n] == (k, l, ch2)).unwrap();
                    graph.add_edge(node1, node2, 1);
                }
            }
        }
    }

    let start = graph.node_indices().find(|&n| graph[n] == start.unwrap());
    let end = graph.node_indices().find(|&n| graph[n] == end.unwrap());

    let part1 = *dijkstra(&graph, start.unwrap(), end, |_| 1).get(&end.unwrap()).unwrap();

    let part2 = a_nodes.iter().filter_map(|&idx| {
        dijkstra(&graph, idx, end, |_| 1).get(&end.unwrap()).copied()
    }).min().unwrap();
    (part1, part2)
}

fn main() {
    let (part1, part2) = solve(INPUT);
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

#[cfg(test)]
mod tests {
    use test_case::test_case;
    use super::*;

    const SAMPLE: &str = include_str!("sample.txt");

    #[test_case(SAMPLE, "sample" ; "sample")]
    #[test_case(INPUT, "input" ; "input")]
    fn snapshot_test(input: &str, name: &str) {
        insta::assert_debug_snapshot!(name, solve(input));
    }
}
