use std::str::FromStr;

use itertools::Itertools;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Play {
    Rock,
    Paper,
    Scissors,
}

impl Play {
    fn value(&self) -> usize {
        match self {
            Play::Rock => 1,
            Play::Paper => 2,
            Play::Scissors => 3,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Outcome {
    Lost,
    Draw,
    Won,
}

impl Outcome {
    fn value(&self) -> usize {
        match self {
            Outcome::Lost => 0,
            Outcome::Draw => 3,
            Outcome::Won => 6,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Round {
    elf: Play,
    you: Play,
}

impl Round {
    fn outcome(&self) -> Outcome {
        match (self.elf, self.you) {
            (Play::Rock, Play::Paper) => Outcome::Won,
            (Play::Rock, Play::Scissors) => Outcome::Lost,
            (Play::Paper, Play::Scissors) => Outcome::Won,
            (Play::Paper, Play::Rock) => Outcome::Lost,
            (Play::Scissors, Play::Rock) => Outcome::Won,
            (Play::Scissors, Play::Paper) => Outcome::Lost,
            (_, _) => Outcome::Draw,
        }
    }

    fn score(&self) -> usize {
        self.outcome().value() + self.you.value()
    }
}

impl FromStr for Round {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_ascii_whitespace().take(2);
        let elf_str = parts.next().unwrap();
        let you_str = parts.next().unwrap();

        let elf = match elf_str {
            "A" => Play::Rock,
            "B" => Play::Paper,
            "C" => Play::Scissors,
            _ => return Err(format!("Unexpected elf play: {}", elf_str))
        };

        let you = match you_str {
            "X" => Play::Rock,
            "Y" => Play::Paper,
            "Z" => Play::Scissors,
            _ => return Err(format!("Unexpected you play: {}", you_str))
        };

        Ok(Round { elf, you })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Round2 {
    elf: Play,
    outcome: Outcome,
}

impl Round2 {
    fn play(&self) -> Play {
        match (self.outcome, self.elf) {
            (Outcome::Lost, Play::Rock) => Play::Scissors,
            (Outcome::Lost, Play::Paper) => Play::Rock,
            (Outcome::Lost, Play::Scissors) => Play::Paper,
            (Outcome::Draw, Play::Rock) => Play::Rock,
            (Outcome::Draw, Play::Paper) => Play::Paper,
            (Outcome::Draw, Play::Scissors) => Play::Scissors,
            (Outcome::Won, Play::Rock) => Play::Paper,
            (Outcome::Won, Play::Paper) => Play::Scissors,
            (Outcome::Won, Play::Scissors) => Play::Rock,
        }
    }

    fn score(&self) -> usize {
        self.outcome.value() + self.play().value()
    }
}

impl FromStr for Round2 {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_ascii_whitespace().take(2);
        let elf_str = parts.next().unwrap();
        let outcome_str = parts.next().unwrap();

        let elf = match elf_str {
            "A" => Play::Rock,
            "B" => Play::Paper,
            "C" => Play::Scissors,
            _ => return Err(format!("Unexpected elf play: {}", elf_str))
        };

        let outcome = match outcome_str {
            "X" => Outcome::Lost,
            "Y" => Outcome::Draw,
            "Z" => Outcome::Won,
            _ => return Err(format!("Unexpected outcome: {}",outcome_str ))
        };

        Ok(Round2 { elf, outcome })
    }
}
fn score_strategy(strategy: Vec<Round>) -> usize {
    strategy.iter().map(|r| r.score()).sum()
}

fn parse_strategy(input: &str) -> Result<Vec<Round>, String> {
    input.lines().map(|line| Round::from_str(line)).collect()
}

fn part1(input: &str) -> Result<usize, String> {
    let strat = parse_strategy(input)?;
    Ok(score_strategy(strat))
}

fn parse_strategy2(input: &str) -> Result<Vec<Round2>, String> {
    input.lines().map(|line| Round2::from_str(line)).collect()
}

fn score_strategy2(strategy: Vec<Round2>) -> usize {
    strategy.iter().map(|r| r.score()).sum()
}

fn part2(input: &str) -> Result<usize, String> {
    let strat = parse_strategy2(input)?;
    Ok(score_strategy2(strat))
}

pub fn sol1() -> usize {
    let input = include_str!("day2.txt");
    let strat = parse_strategy(input).unwrap();
    score_strategy(strat)
}

pub fn sol2() -> usize {
    let input = include_str!("day2.txt");
    let strat = parse_strategy2(input).unwrap();
    score_strategy2(strat)
}

#[cfg(test)]
mod test {
    use super::{part1, part2};

    #[test]
    fn test_part1_example() {
        let input = 
        "A Y
B X
C Z";
        assert_eq!(part1(input), Ok(15));
    }

    #[test]
    fn test_part2_example() {
        let input = 
        "A Y
B X
C Z";
        assert_eq!(part2(input), Ok(12));
    }
}
