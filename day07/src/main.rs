use std::collections::HashMap;

fn parse(input: &str) -> HashMap<Vec<String>, usize> {
    let mut paths = vec![];
    let mut filesystem = HashMap::<Vec<String>, _>::new();

    for line in input.lines() {
        if line.starts_with("$ cd ..") {
            paths.pop();
        } else if let Some(path) = line.strip_prefix("$ cd ") {
            paths.push(path.to_string());
        } else if line.starts_with("$ ls") || line.starts_with("dir ") {
            // ignore
        } else {
            let size: usize = line.split_whitespace().next().unwrap().parse().unwrap();
            let mut insert_path = vec![];
            for path in paths.iter() {
                insert_path.push(path.clone());
                *filesystem.entry(insert_path.clone()).or_insert(0) += size;
            }
        }
    }
    filesystem
}

fn part1(parsed: &HashMap<Vec<String>, usize>) -> usize {
    let mut total = 0;
    for (_, size) in parsed.iter() {
        if *size <= 100000 {
            total += size;
        }
    }
    total
}

fn part2(parsed: &HashMap<Vec<String>, usize>) -> usize {
    let total = 70000000;
    let needed = 30000000;
    let current = parsed.get(&vec!["/".to_string()]).unwrap();

    // find the smallest element usize in parsed where total - current + size >= needed
    let smallest = parsed
        .iter()
        .filter(|(_, &size)| total - current + size >= needed)
        .min_by_key(|(_, &size)| size)
        .unwrap()
        .1;

    *smallest
}

fn run() -> (usize, usize) {
    let parsed = parse(include_str!("input.txt"));
    (part1(&parsed), part2(&parsed))
}

fn main() {
    let (part1, part2) = run();
    println!("Part1: {}", part1);
    println!("Part2: {}", part2);
}

#[test]
fn snapshot_test() {
    insta::assert_debug_snapshot!(run());
}
