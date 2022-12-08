use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("input.txt");

fn parse(input: &str) -> HashMap<(usize, usize), usize> {
    // consider input as a grid of numbers of size 1. Parse it into a map from (line, position) to number.
    input
        .lines()
        .enumerate()
        .flat_map(|(line, line_str)| {
            line_str
                .chars()
                .enumerate()
                .map(move |(pos, c)| ((line, pos), c.to_digit(10).unwrap() as usize))
        })
        .collect()
}

// "solve" takes the parsed input from "parse" and in all directions (up, down, left, right) finds all the numbers that are higher than the previous number and counts them
fn solve(input: HashMap<(usize, usize), usize>) -> HashSet<(usize, usize)>  {
    // consider the input representing a grid of numbers of size sqrt(input.len())
    // line by line, remember the highest number seen in the row and if the current number is higher, add the key to a set
    // Consider 0 as valid input so use an Option<usize> to represent the highest number seen so far
    let mut valid_keys: HashSet<(usize, usize)> = HashSet::new();
    let grid_size = (input.len() as f64).sqrt() as usize;

    for line in 0..(input.len() as f64).sqrt() as usize {
        let mut highest_number_seen_from_left = None;
        let mut highest_number_seen_from_right = None;
        let mut highest_number_seen_from_top = None;
        let mut highest_number_seen_from_bottom = None;
        for pos in 0..(input.len() as f64).sqrt() as usize {
            // using line and pos, create coordinates for the grid starting from the left, right, top, bottom
            let mut coords = [
                (line, pos, &mut highest_number_seen_from_left),
                (
                    line,
                    grid_size - pos - 1,
                    &mut highest_number_seen_from_right,
                ),
                (
                    pos,
                    line,
                    &mut highest_number_seen_from_top,
                ),
                (
                    grid_size - pos - 1,
                    line,
                    &mut highest_number_seen_from_bottom,
                ),
            ];
            // for all the coords in the grid, check if the number is higher than the highest number seen so far
            for (line, pos, highest_number_seen_option) in coords.iter_mut() {
                let current_number = input.get(&(*line, *pos)).unwrap();
                if let Some(highest_number_seen) = highest_number_seen_option {
                    if current_number > highest_number_seen {
                        valid_keys.insert((*line, *pos));
                        highest_number_seen_option.replace(*current_number);
                    }
                } else {
                    valid_keys.insert((*line, *pos));
                    highest_number_seen_option.replace(*current_number);
                }
            }
        }
    }

    valid_keys
}

// function "solve2" finds the visible numbers from every position in the grid in all the directions (up, down, left, right) and returns the highest multiple of those numbers
fn solve2(input: HashMap<(usize, usize), usize>) -> usize {
    let grid_size = (input.len() as f64).sqrt() as usize;
    let mut highest_multiple = 0;
    for line in 0..(input.len() as f64).sqrt() as usize {
        for pos in 0..(input.len() as f64).sqrt() as usize {
            let current_number = input.get(&(line, pos)).unwrap();
            // count the numbers left of the current number which are lower than the current number
            let mut count_left = 0;
            for i in 0..pos {
                if input.get(&(line, pos - i - 1)).unwrap() < current_number {
                    count_left += 1;
                } else {
                    count_left += 1;
                    break;
                }
            }
            // count the numbers right of the current number which are lower than the current number
            let mut count_right = 0;
            for i in 0..(grid_size - pos - 1) {
                if input.get(&(line, pos + i + 1)).unwrap() < current_number {
                    count_right += 1;
                } else {
                    count_right += 1;
                    break;
                }
            }
            // count the numbers above the current number which are lower than the current number
            let mut count_top = 0;
            for i in 0..line {
                if input.get(&(line - i - 1, pos)).unwrap() < current_number {
                    count_top += 1;
                } else {
                    count_top += 1;
                    break;
                }
            }
            // count the numbers below the current number which are lower than the current number
            let mut count_bottom = 0;
            for i in 0..(grid_size - line - 1) {
                if input.get(&(line + i + 1, pos)).unwrap() < current_number {
                    count_bottom += 1;
                } else {
                    count_bottom += 1;
                    break;
                }
            }
            let multiple = count_left * count_right * count_top * count_bottom;
            if multiple > highest_multiple {
                highest_multiple = multiple;
            }
        }
    }
    highest_multiple
}

fn main() {
    println!("Part 1: {}", solve(parse(INPUT)).len());
    println!("Part 2: {}", solve2(parse(INPUT)));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn test_total() {
        assert_eq!(solve(parse(TEST_INPUT)).len(), 21);
    }

    #[test]
    fn test_items() {
        let expected: HashSet<(usize, usize)> =
            vec![(0, 0), (0, 1), (0, 2), (0, 3), (0, 4),
                 (1, 0), (1, 1), (1, 2), (1, 4),
                 (2, 0), (2, 1), (2, 3), (2, 4),
                 (3, 0), (3, 2), (3, 4),
                 (4, 0), (4, 1), (4, 2), (4, 3), (4, 4)].into_iter().collect();
        let result = solve(parse(TEST_INPUT));
        for (line, pos) in expected.iter() {
            assert_eq!(result.contains(&(*line, *pos)), true, "expected {:?} to be in result", (*line, *pos));
        }
    }

    #[test]
    fn test_total2() {
        assert_eq!(solve2(parse(TEST_INPUT)), 8);
    }
}