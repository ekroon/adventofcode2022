use std::collections::HashSet;

fn find_network_start_marker(input: &str, n: usize) -> usize {
    // loop over the input string, n characters at a time, count the number of different characters
    // in the substring, and return the current index + n into the string.
    input
        .chars()
        .collect::<Vec<char>>()
        .windows(n)
        .enumerate()
        .find(|(_, window)| {
            window
                .iter()
                .collect::<HashSet<&char>>()
                .len() == window.len()
        })
        .map(|(i, _)| i + n)
        .unwrap()
}

fn run() -> (usize, usize) {
    let input = include_str!("input.txt");

    (
        find_network_start_marker(input, 4),
        find_network_start_marker(input, 14),
    )
}

fn main() {
    let (part1, part2) = run();
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

#[test]
fn snapshot_test() {
    insta::assert_debug_snapshot!(run());
}
