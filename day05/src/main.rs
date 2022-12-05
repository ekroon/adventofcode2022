use regex::Regex;

#[derive(Debug)]
struct Instruction {
    from: u32,
    to: u32,
    amount: u32,
}

#[derive(Debug)]
struct Stacks {
    stacks: Vec<Vec<char>>,
}

fn parse(input: &str) -> (Stacks, Vec<Instruction>) {
    // split input on empty line
    let (first, second) = input.split_once("\n\n").unwrap();
    (parse_stacks(first), parse_instructions(second))
}

fn parse_stacks(input: &str) -> Stacks {
    // reverse input lines
    let input = input.lines().rev().collect::<Vec<_>>().join("\n");
    // get first line
    // indexed loop over first line, if number push index to vector stack_index
    let stack_index = input
        .lines()
        .next()
        .unwrap()
        .chars()
        .enumerate()
        .filter(|(_, c)| c.is_numeric())
        .map(|(i, _)| i)
        .collect::<Vec<_>>();

    // create Stacks with the length of stack_index
    let mut stacks = Stacks {
        stacks: vec![vec![]; stack_index.len()],
    };

    // line by line
    // using stack_index, get char at index and push to stack if upper case char
    // indexed loop over line and if upper case char, push to the vector at index in stacks
    for line in input.lines() {
        for (i, c) in line.chars().enumerate() {
            if c.is_uppercase() {
                stacks.stacks[stack_index.iter().position(|&x| x == i).unwrap()].push(c);
            }
        }
    }

    stacks
}

fn parse_instructions(input: &str) -> Vec<Instruction> {
    // for every line in input
    // use a regex to extract the from, to and amount in "move 1 from 2 to 1" format
    // substract 1 from the from and to
    // return vector of instructions
    input
        .lines()
        .map(|line| {
            let caps = Regex::new(r"move (\d+) from (\d+) to (\d+)")
                .unwrap()
                .captures(line)
                .unwrap();
            Instruction {
                from: caps[2].parse::<u32>().unwrap() - 1,
                to: caps[3].parse::<u32>().unwrap() - 1,
                amount: caps[1].parse::<u32>().unwrap(),
            }
        })
        .collect()
}

// function "run_instruction" which takes a reference to a Stacks and an Instruction
// pop the amount of chars from the from stack and push them to the to stack
fn run_instruction(stacks: &mut Stacks, instruction: &Instruction) {
    for _ in 0..instruction.amount {
        // get from chars from stack
        if let Some(c) = stacks.stacks[instruction.from as usize].pop() {
            // push to stack
            stacks.stacks[instruction.to as usize].push(c);
        }
    }
}

// function "run_instruction_9001" which takes a reference to a Stacks and an Instruction
// take amount of chars from the stack and append them to the to stack
fn run_instruction_9001(stacks: &mut Stacks, instruction: &Instruction) {
    let mut temp_stack = vec![];
    for _ in 0..instruction.amount {
        // get from chars from stack
        if let Some(c) = stacks.stacks[instruction.from as usize].pop() {
            // push to temp_stack
            temp_stack.push(c);
        }
    }
    // append temp_stack reversed to to stack
    stacks.stacks[instruction.to as usize].append(&mut temp_stack.into_iter().rev().collect());
}

fn part1(input: &str) -> String {
    // parse input
    let (mut stacks, instructions) = parse(input);
    // run instructions
    for instruction in instructions {
        run_instruction(&mut stacks, &instruction);
    }
    // pop a char from each stack and join them
    stacks
        .stacks
        .iter_mut()
        .filter_map(|stack| stack.pop())
        .collect()
}

fn part2(input: &str) -> String {
    // parse input
    let (mut stacks, instructions) = parse(input);
    // run instructions
    for instruction in instructions {
        run_instruction_9001(&mut stacks, &instruction);
    }
    // pop a char from each stack and join them
    stacks
        .stacks
        .iter_mut()
        .filter_map(|stack| stack.pop())
        .collect()
}

fn run() -> (String, String) {
    let input = include_str!("input.txt");
    (part1(input), part2(input))
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
