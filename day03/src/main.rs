use color_eyre::Result;
// Copilot coding

// Write a function "split_string" that takes a string and returns a tuple of two 52-element u8 vectors.
// Check every character and its index in the string.
// Convert the character to the position in the alphabet. Lower case characters are 1-26, upper case are 27-52.
// If the index into the string is below half the string length, use the position in the alphabet to index into the first vector and increase the value at that index by 1.
fn split_string(s: &str) -> (Vec<u8>, Vec<u8>) {
    let mut v1 = vec![0; 52];
    let mut v2 = vec![0; 52];
    for (i, c) in s.chars().enumerate() {
        let pos = match c {
            'a'..='z' => c as u8 - b'a',
            'A'..='Z' => c as u8 - b'A' + 26,
            _ => continue,
        };
        if i < s.len() / 2 {
            v1[pos as usize] += 1;
        } else {
            v2[pos as usize] += 1;
        }
    }
    (v1, v2)
}

// Write a function that takes a two u8 vectors and returns a u32 result.
// Compare the vectors element by element.
// If the elements are both non-zero add the current index + 1 to the result.
fn compare_letters(first_half: &[u8], second_half: &[u8]) -> u32 {
    let mut result = 0;
    for (i, (a, b)) in first_half.iter().zip(second_half.iter()).enumerate() {
        if *a != 0 && *b != 0 {
            result += i as u32 + 1;
        }
    }
    result
}

// Write a function "score_line" that uses split_string and compare_letters to return a u32 result.
fn score_line(s: &str) -> u32 {
    let (v1, v2) = split_string(s);
    compare_letters(&v1, &v2)
}

// Write a function "compare_lines" which takes a slice of string and returns a char.
// Check the shortest string in the slice and for every char check if the other strings contain that char.
// If so return that char.
fn compare_lines(lines: &[&str]) -> char {
    let shortest = lines.iter().min_by_key(|s| s.len()).unwrap();
    for c in shortest.chars() {
        if lines.iter().all(|s| s.contains(c)) {
            return c;
        }
    }
    unreachable!()
}

// write a function "part2" that takes a string and returns a u32 result.
// Split the string into lines.
// Group the lines into groups of 3.
// Use compare_lines to get the common letter in each group.
// Convert the letter to a u32 by positioning it in the alphabet. Lower case characters are 1-26, upper case are 27-52.
// Sum all the scores.
fn part2(input: &str) -> u32 {
    let lines = input.lines().collect::<Vec<_>>();
    lines
        .chunks(3)
        .map(|group| {
            let common = compare_lines(group);
            let pos = match common {
                'a'..='z' => common as u8 - b'a',
                'A'..='Z' => common as u8 - b'A' + 26,
                _ => unreachable!(),
            };
            pos as u32 + 1
        })
        .sum()
}

fn run() -> Result<(u32, u32)> {
    let input = include_str!("input.txt");
    let part1 = input.lines().map(score_line).sum();
    Ok((part1, part2(input)))
}

fn main() -> Result<()> {
    let (part1, part2) = run()?;
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    Ok(())
}

#[test]
fn score_rucksack() {
    let (first, second) = split_string("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL");
    assert_eq!(compare_letters(&first, &second), 38);
}

#[test]
fn snap_shot_test() -> Result<()> {
    insta::assert_debug_snapshot!(run()?);
    Ok(())
}
