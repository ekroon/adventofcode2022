use color_eyre::eyre::eyre;
use color_eyre::Result;
use regex::Regex;

// Write a function "parse_line" that uses a regex to convert a string like "94-97,31-95" to a result containing a tuple of ((94, 97), (31, 95)).
fn parse_line(line: &str) -> Result<((usize, usize), (usize, usize))> {
    let re = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)")?;
    let caps = re.captures(line).ok_or_else(|| eyre!("No match"))?;
    let a = (caps[1].parse()?, caps[2].parse()?);
    let b = (caps[3].parse()?, caps[4].parse()?);
    Ok((a, b))
}

// Write a function "full_overlaps" that takes a tuple of tuples like ((begin, end), (begin, end)) and returns true if the two ranges overlap completely.
// Take into consideration that each of them can be larger than the other.
fn full_overlaps(ranges: &((usize, usize), (usize, usize))) -> bool {
    let ((a1, a2), (b1, b2)) = ranges;
    (a1 <= b1 && b2 <= a2) || (b1 <= a1 && a2 <= b2)
}

// For this one I had to write the fn def myself and then copilot filled in the body.
fn any_overlaps(ranges: &((usize, usize), (usize, usize))) -> bool {
    let ((a1, a2), (b1, b2)) = ranges;
    (a1 <= b1 && b1 <= a2)
        || (a1 <= b2 && b2 <= a2)
        || (b1 <= a1 && a1 <= b2)
        || (b1 <= a2 && a2 <= b2)
}

// Write a function "part1" which takes a string input and returns a u32.
// Split the input into lines and run parse_line and full_overlaps on each line.
// If true add to the result.
fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(parse_line)
        .filter_map(Result::ok)
        .filter(full_overlaps)
        .count() as u32
}

// Write a function "part2" which takes a string input and returns a u32.
// Split the input into lines and run parse_line and any_overlaps on each line.
// If true add to the result.
fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(parse_line)
        .filter_map(Result::ok)
        .filter(any_overlaps)
        .count() as u32
}

fn run() -> Result<(u32, u32)> {
    let input = include_str!("input.txt");
    let part1 = part1(input);
    let part2 = part2(input);
    Ok((part1, part2))
}

fn main() -> Result<()> {
    let (part1, part2) = run()?;
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    Ok(())
}

#[test]
fn snapshot_test() {
    // snapshot run function with insta debug snapshot
    insta::assert_debug_snapshot!(run());
}
