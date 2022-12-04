use color_eyre::Report;
use std::cmp::Reverse;

fn main() -> Result<(), Report> {
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
    println!("Part 1: {:1}", elves[0]);
    println!("Part 2: {:?}", elves[0..=2].iter().sum::<i32>());
    Ok(())
}
