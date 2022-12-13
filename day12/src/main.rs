use petgraph::algo::dijkstra;
use petgraph::Graph;
use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("input.txt");

fn solve(input: &str) -> (i32, i32) {
    let mut points = HashMap::new();
    let mut graph = Graph::new();
    let mut a_nodes = HashSet::new();
    let mut start = None;
    let mut end = None;

    for (i, line) in input.lines().enumerate() {
        for (j, ch) in line.chars().enumerate() {
            let mut is_start = false;
            let mut is_end = false;
            let mut ch = ch;
            if ch == 'S' {
                is_start = true;
                ch = 'a';
            } else if ch == 'E' {
                is_end = true;
                ch = 'z';
            }
            let idx= graph.add_node(());
            if is_start {
                start.replace(idx);
            }
            if is_end {
                end.replace(idx);
            }
            if ch == 'a' {
                a_nodes.insert(idx);
            }
            points.insert((i,j), (ch, idx));
        }
    }

    for (&(i, j), &(ch, node1)) in points.iter() {
        let neighbours = [
            (i.wrapping_sub(1), j),
            (i.wrapping_add(1), j),
            (i, j.wrapping_sub(1)),
            (i, j.wrapping_add(1)),
        ];

        for (k, l) in neighbours {
            if let Some(&(ch2, node2)) = points.get(&(k, l)) {
                if ch as u8 + 1 >= ch2 as u8 {
                    graph.add_edge(node2, node1, 1);
                }
            }
        }
    }

    let results = dijkstra(&graph, end.unwrap(), None, |_| 1);

    let part1 = *results.get(&start.unwrap()).unwrap();
    let part2 = *a_nodes
        .iter()
        .filter_map(|idx| results.get(idx))
        .min()
        .unwrap();
    (part1, part2)
}

fn main() {
    let (part1, part2) = solve(INPUT);
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    const SAMPLE: &str = include_str!("sample.txt");

    #[test_case(SAMPLE, "sample" ; "sample")]
    #[test_case(INPUT, "input" ; "input")]
    fn snapshot_test(input: &str, name: &str) {
        insta::assert_debug_snapshot!(name, solve(input));
    }
}
