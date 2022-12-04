use crate::GameResult::{Draw, Loss, Win};
use crate::RockPaperScissors::{Paper, Rock, Scissors};
use color_eyre::eyre::eyre;
use color_eyre::Report;
use std::str::FromStr;

#[derive(Debug)]
struct Parsed(u8, u8);

impl FromStr for Parsed {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b) = s.split_once(' ').ok_or_else(|| eyre!("Invalid input"))?;
        Ok(Self(
            *a.as_bytes().first().unwrap(),
            *b.as_bytes().first().unwrap(),
        ))
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum RockPaperScissors {
    Rock,
    Paper,
    Scissors,
}

const ORDERING: [RockPaperScissors; 3] = [Rock, Paper, Scissors];

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum GameResult {
    Loss,
    Win,
    Draw,
}

#[derive(Debug)]
struct RockPaperScissorsGame {
    other: RockPaperScissors,
    me: RockPaperScissors,
}

impl RockPaperScissorsGame {
    fn my_result(&self) -> GameResult {
        let my_index = ORDERING.iter().position(|v| self.me == *v).unwrap();
        let other_index = ORDERING.iter().position(|v| self.other == *v).unwrap();
        let loss_index = (my_index + 1) % 3;
        if other_index == loss_index {
            Loss
        } else if other_index == my_index {
            Draw
        } else {
            Win
        }
    }
}

impl TryFrom<Parsed> for RockPaperScissorsGame {
    type Error = Report;

    fn try_from(value: Parsed) -> Result<Self, Self::Error> {
        let other = match value.0 {
            b'A' => Rock,
            b'B' => Paper,
            b'C' => Scissors,
            _ => return Err(eyre!("Invalid input")),
        };
        let me = match value.1 {
            b'X' => Rock,
            b'Y' => Paper,
            b'Z' => Scissors,
            _ => return Err(eyre!("Invalid input")),
        };
        Ok(Self { other, me })
    }
}

struct ParsedPart2(Parsed);

impl TryFrom<ParsedPart2> for RockPaperScissorsGame {
    type Error = Report;

    fn try_from(value: ParsedPart2) -> Result<Self, Self::Error> {
        let value = value.0;
        let other = match value.0 {
            b'A' => RockPaperScissors::Rock,
            b'B' => RockPaperScissors::Paper,
            b'C' => RockPaperScissors::Scissors,
            _ => return Err(eyre!("Invalid input")),
        };
        let result = match value.1 {
            b'X' => Loss,
            b'Y' => Draw,
            b'Z' => Win,
            _ => return Err(eyre!("Invalid input")),
        };

        let me = ORDERING
            .iter()
            .find(|v| {
                let game = Self { other, me: **v };
                game.my_result() == result
            })
            .unwrap();

        Ok(Self { other, me: *me })
    }
}

struct Scored(i32);

impl From<RockPaperScissorsGame> for Scored {
    fn from(value: RockPaperScissorsGame) -> Self {
        let mut result = 0;
        result += match value.me {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        };
        result += match value.my_result() {
            Win => 6,
            Draw => 3,
            Loss => 0,
        };
        Scored(result)
    }
}

fn run() -> Result<(i32, i32), Report> {
    let input = include_str!("input.txt");
    let part1 = input
        .lines()
        .map(|line| line.parse::<Parsed>().unwrap())
        .map(|parsed| RockPaperScissorsGame::try_from(parsed).unwrap())
        .map(|game| Scored::from(game).0)
        .sum();
    let part2 = input
        .lines()
        .map(|line| ParsedPart2(line.parse::<Parsed>().unwrap()))
        .map(|parsed| RockPaperScissorsGame::try_from(parsed).unwrap())
        .map(|game| Scored::from(game).0)
        .sum();
    Ok((part1, part2))
}

fn main() -> Result<(), Report> {
    let (part1, part2) = run()?;
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    Ok(())
}

#[test]
fn test() -> Result<(), Report> {
    insta::assert_debug_snapshot!(run()?);
    Ok(())
}
