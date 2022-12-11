use std::collections::VecDeque;

const INPUT: &str = include_str!("input.txt");

#[derive(Debug)]
struct Monkey {
    items: VecDeque<u64>,
    operation: Operation,
    test: Test,
    items_inspected: i64,
}

#[derive(Debug)]
enum Operation {
    Add(u64),
    Multiple(u64),
    MultipleSelf
}

#[derive(Debug)]
struct Test {
    divisor: u64,
    if_true: usize,
    if_false: usize,
}

fn parse_monkey(input: &str) -> Monkey {
    let mut lines = input.lines();
    lines.next();
    let items: VecDeque<u64> = lines.next().unwrap().trim().strip_prefix("Starting items:").unwrap().trim().split(", ").map(|v| v.parse().unwrap()).collect();
    let operation_str = lines.next().unwrap().trim();
    let operation = if operation_str == "Operation: new = old * old" {
        Operation::MultipleSelf
    } else if let Some(n) = operation_str.strip_prefix("Operation: new = old * ") {
        Operation::Multiple(n.parse().unwrap())
    } else if let Some(n) = operation_str.strip_prefix("Operation: new = old + ") {
        Operation::Add(n.parse().unwrap())
    } else {
        unreachable!();
    };

    let divisor = lines.next().unwrap().trim().strip_prefix("Test: divisible by ").unwrap().parse().unwrap();
    let if_true = lines.next().unwrap().trim().strip_prefix("If true: throw to monkey ").unwrap().parse().unwrap();
    let if_false = lines.next().unwrap().trim().strip_prefix("If false: throw to monkey ").unwrap().parse().unwrap();

    Monkey {
        items,
        operation,
        test: Test {
            divisor,
            if_true,
            if_false,
        },
        items_inspected: 0,
    }
}

fn parse_monkeys(input: &str) -> Vec<Monkey> {
    input.split("\n\n").map(|v| parse_monkey(v)).collect()
}

fn run_round(monkeys: &mut Vec<Monkey>, divide_with: u64) {
    let multiple: u64 = monkeys.iter().map(|v| v.test.divisor).product();
    for i in 0..monkeys.len() {
        let monkey = monkeys.get_mut(i).unwrap();
        let mut add_to_monkeys = vec![];
        while let Some(item) = monkey.items.pop_front() {
            monkey.items_inspected += 1;
            let mut new_value = match monkey.operation {
                Operation::Add(v) => item + v,
                Operation::Multiple(v) => item * v,
                Operation::MultipleSelf => item * item
            };
            new_value = new_value / divide_with;
            new_value = new_value % multiple;
            if new_value % monkey.test.divisor == 0 {
                add_to_monkeys.push((monkey.test.if_true, new_value))
            } else {
                add_to_monkeys.push((monkey.test.if_false, new_value))
            }
        }
        for (index, value) in add_to_monkeys {
            monkeys.get_mut(index).unwrap().items.push_back(value);
        }
    }
}

fn top_two_active_multiplied(monkeys: Vec<Monkey>) -> i64 {
    let mut highest = 0;
    let mut second_highest = 0;
    for monkey in monkeys {
        if monkey.items_inspected > highest {
            second_highest = highest;
            highest = monkey.items_inspected;
        } else if monkey.items_inspected > second_highest {
            second_highest = monkey.items_inspected;
        }
    }
    highest * second_highest
}

fn part1(input: &str) -> i64 {
    let mut monkeys = parse_monkeys(input);
    for _ in 0..20 {
        run_round(&mut monkeys, 3);
    }
    top_two_active_multiplied(monkeys)
}

fn part2(input: &str) -> i64 {
    let mut monkeys = parse_monkeys(input);
    for _ in 0..10_000 {
        run_round(&mut monkeys, 1);
    }
    top_two_active_multiplied(monkeys)
}

fn run(input: &str) -> (i64, i64) {
    (part1(input), part2(input))
}

fn main() {
    let (part1, part2) = run(INPUT);
    println!("Part1: {}", part1);
    println!("Part1: {}", part2);
}

#[cfg(test)]
mod tests {
    use test_case::test_case;
    use super::*;

    const SAMPLE: &str = include_str!("sample.txt");

    #[test_case(INPUT, "input"; "input")]
    #[test_case(SAMPLE, "sample"; "sample")]
    fn snapshot_test(input: &str, name: &str) {
        insta::assert_debug_snapshot!(name, run(input));
    }
}