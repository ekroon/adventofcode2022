use itertools::Itertools;

const INPUT: &str = include_str!("input.txt");
const CYCLES: [usize; 6] = [20, 60, 100, 140, 180, 220];

enum Instruction {
    Noop,
    AddX(i64)
}

fn cycle_values(input: &str) -> impl Iterator<Item = (usize, i64)> + '_ {
    let mut x = 1i64;
    let mut cycle = 0usize;
    input.lines().map(|line| {
        let mut parts = line.split_whitespace();
        match parts.next().unwrap() {
            "noop" => Instruction::Noop,
            "addx" => Instruction::AddX(parts.next().unwrap().parse().unwrap()),
            _ => panic!("invalid instruction")
        }
    }).flat_map(move |instruction| {
        match instruction {
            Instruction::Noop => {
                cycle += 1;
                vec![(cycle, x)]
            },
            Instruction::AddX(value) => {
                let result = vec![(cycle + 1, x), (cycle + 2, x)];
                x += value;
                cycle += 2;
                result
            }
        }
    })
}

fn part1(input: &str, cycles: &[usize]) -> Vec<i64> {
    cycle_values(input).fold(vec![], |mut v, (cycle, value)| {
        if cycles.contains(&cycle) {
            v.push(cycle as i64 * value);
        }
        v
    })
}

fn part2(input: &str) -> Vec<String> {
    let chars = cycle_values(input).fold(vec![], |mut v, (cycle, value)| {
        let column = ((cycle - 1) % 40) as i64;
        let xx = (-1..1).contains(&0);
        if ((value -1)..=(value +1)).contains(&column) {
            v.push('#')
        } else {
            v.push('.')
        }
        v
    });

    let mut result = vec![];
    for chunk in &chars.into_iter().chunks(40) {
        result.push(chunk.collect());
    }
    result
}

fn main() {
    let part1 = part1(INPUT, &CYCLES).into_iter().sum::<i64>();
    println!("Part 1: {part1}");
    println!("Part 2: ");
    let part2 = part2(INPUT);
    for line in part2 {
        println!("{line}");
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("sample.txt");

    #[test_case(SAMPLE_INPUT, &[20] => 420)]
    #[test_case(SAMPLE_INPUT, &CYCLES => 13140)]
    #[test_case(INPUT, &CYCLES => 15020)]
    fn test_part1(input: &str, cycles: &[usize]) -> i64 {
        part1(input, cycles).into_iter().sum()
    }

    #[test_case(SAMPLE_INPUT, "sample" ; "sample")]
    #[test_case(INPUT, "input" ; "input")]
    fn test_part2(input: &str, name: &str) {
        insta::assert_debug_snapshot!(name, part2(input));
    }
}