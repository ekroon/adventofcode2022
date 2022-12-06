use std::collections::HashSet;

fn find_network_start_marker(input: &str, n: usize) -> (String, usize) {
    // loop over the input string, n characters at a time, count the number of different characters
    // in the substring, and return the string and current index + n into the string
    for (i, c) in input.chars().enumerate() {
        let mut set = HashSet::new();
        set.insert(c);
        for j in 1..n {
            if let Some(c) = input.chars().nth(i + j) {
                set.insert(c);
            }
        }
        if set.len() == n {
            return (input[i..i + n].to_string(), i + n);
        }
    }
    ("".to_string(), 0)
}

fn run() -> (usize, usize) {
    let input = include_str!("input.txt");

    (
        find_network_start_marker(input, 4).1,
        find_network_start_marker(input, 14).1,
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
