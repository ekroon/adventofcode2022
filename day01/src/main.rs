use color_eyre::Report;
use std::cmp::Reverse;

fn run() -> Result<(i32, i32), Report> {
    let lines = include_str!("input.txt").lines();
    let mut elf = 0;
    let mut elves = vec![];
    for line in lines {
        if line.is_empty() {
            elves.push(elf);
            elf = 0;
        } else {
            let value = line.parse::<i32>()?;
            elf += value;
        }
    }
    elves.push(elf);
    elves.sort_by_key(|&e| Reverse(e));
    Ok((elves[0], elves[0..=2].iter().sum()))
}

fn main() -> Result<(), Report> {
    let (part1, part2) = run()?;
    println!("Part 1: {:1}", part1);
    println!("Part 2: {:?}", part2);
    Ok(())
}

#[test]
fn test() -> Result<(), Report> {
    insta::assert_debug_snapshot!(run()?);
    Ok(())
}
