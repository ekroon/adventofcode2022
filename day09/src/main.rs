use std::collections::HashSet;

const INPUT: &str = include_str!("input.txt");

// function where
// "D 1" => [(0, -1)]
// "D 2" => [(0, -1), (0, -1)]
// "D 1\nL 1" => [(0, -1), (-1, 0)]
fn instructions_to_movements(input: &str) -> impl Iterator<Item = (i32, i32)> + '_ {
    input.lines().flat_map(|line| {
        let (direction, distance) = (&line[0..1], &line[2..]);
        let distance = distance
            .parse::<i32>()
            .unwrap_or_else(|_| panic!("<{}>", distance));
        let (dx, dy) = match direction {
            "U" => (0, 1),
            "D" => (0, -1),
            "L" => (-1, 0),
            "R" => (1, 0),
            _ => panic!("Invalid direction"),
        };
        std::iter::repeat((dx, dy)).take(distance as usize)
    })
}

// function where
// [(0,0), (0,0)], (0,1) => [(0,1),(0,0)]
// [(0,1), (0,0)], (0,1) => [(0,2),(0,1)]
// [(0,1), (0,0)], (1,0) => [(1,1),(0,0)]
// [(1,1), (0,0)], (1,0) => [(2,1),(1,1)]
fn move_rope(rope: &Vec<(i32, i32)>, instruction: (i32, i32)) -> Vec<(i32, i32)> {
    let mut result = Vec::with_capacity(rope.len());
    let first = rope[0];
    let new_head = (first.0 + instruction.0, first.1 + instruction.1);
    result.push(new_head);
    for i in 1..rope.len() {
        let current = rope[i];
        let prev = result[i - 1];
        let dist_0 = (current.0 - prev.0).abs();
        let dist_1 = (current.1 - prev.1).abs();
        if dist_0 > 1 || dist_1 > 1 {
            let (dx, dy) = (prev.0 - current.0, prev.1 - current.1);
            let new_instruction = (dx.clamp(-1, 1), dy.clamp(-1, 1));
            result.push((current.0 + new_instruction.0, current.1 + new_instruction.1));
        } else {
            result.push(current);
        }
    }
    result
}

fn run(input: &str, size: usize) -> usize {
    let rope = vec![(0, 0); size];
    let mut visited: HashSet<(i32, i32)> = instructions_to_movements(input)
        .scan(rope, |rope, instruction| {
            let new_rope = move_rope(rope, instruction);
            let result = *new_rope.last().unwrap();
            *rope = new_rope;
            Some(result)
        })
        .collect();
    visited.insert((0, 0));
    visited.len()
}

fn solve() -> (usize, usize) {
    let part1 = run(INPUT, 2);
    let part2 = run(INPUT, 10);
    (part1, part2)
}

fn main() {
    let (part1, part2) = solve();
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

// add test mod
#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    const SAMPLE_INPUT: &str = include_str!("sample1.txt");
    const SAMPLE_INPUT_2: &str = include_str!("sample2.txt");

    #[test_case("D 1" => vec![(0, -1)])]
    #[test_case("D 2" => vec![(0, -1), (0, -1)])]
    #[test_case("D 1\nL 1" => vec![(0, -1), (-1, 0)])]
    fn input_to_movements(input: &str) -> Vec<(i32, i32)> {
        instructions_to_movements(input).collect()
    }

    #[test_case(vec![(0,0), (0,0)], (0,1) => vec![(0,1),(0,0)])]
    #[test_case(vec![(0,1), (0,0)], (0,1) => vec![(0,2),(0,1)])]
    #[test_case(vec![(0,1), (0,0)], (1,0) => vec![(1,1),(0,0)])]
    #[test_case(vec![(1,1), (0,0)], (1,0) => vec![(2,1),(1,1)])]
    fn test_snake_move(rope: Vec<(i32, i32)>, instruction: (i32, i32)) -> Vec<(i32, i32)> {
        move_rope(&rope, instruction)
    }

    #[test_case(SAMPLE_INPUT => 13)]
    #[test_case(INPUT => 5874)]
    fn test_part1(input: &str) -> usize {
        run(input, 2)
    }

    #[test_case(SAMPLE_INPUT => 1)]
    #[test_case(SAMPLE_INPUT_2 => 36)]
    #[test_case("R 5\nU 8" => 1)]
    fn test_part2(input: &str) -> usize {
        run(input, 10)
    }

    #[test]
    fn snapshot_test() {
        insta::assert_debug_snapshot!(solve());
    }
}
