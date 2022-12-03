use std::{str::FromStr, collections::HashSet};

use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Rucksack {
    lhs: String,
    rhs: String,
}

impl FromStr for Rucksack {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        assert_eq!(s.len() % 2, 0);
        let mut lhs = s.to_owned();
        let rhs = lhs.split_off(s.len() / 2);
        Ok(Rucksack { lhs, rhs })
    }
}

impl Rucksack {
    fn find_error(&self) -> u8 {
        let lhs_bytes = self.lhs.as_bytes();
        self.rhs.bytes().find(|r| lhs_bytes.contains(r)).expect("Found no error in rucksack")
    }

    fn priority(item: u8) -> usize {
        (match item {
            b'a'..=b'z' => item - b'a' + 1,
            b'A'..=b'Z' => item - b'A' + 27,
            _ => panic!("Unexpected item: {}", item)
        }) as usize
    }

    fn items(&self) -> HashSet<u8> {
        self.lhs.bytes().chain(self.rhs.bytes()).collect()
    }
}

fn parse_input(input: &str) -> Result<Vec<Rucksack>, String> {
    input.lines().map(Rucksack::from_str).collect()
}

fn part1_calc(rucksacks: Vec<Rucksack>) -> usize {
    rucksacks.iter().map(|r| r.find_error()).map(Rucksack::priority).sum()
}

fn part2_calc(rucksacks: Vec<Rucksack>) -> usize {
    rucksacks.into_iter().chunks(3).into_iter().map(|group| {
        let (a, b, c) = group.collect_tuple().expect("Expected 3 elves per group");
        let a_b = a.items().intersection(&b.items()).copied().collect::<HashSet<_>>();
        a_b.intersection(&c.items()).next().copied().expect("Expected one shared item to be badge")

    }).map(Rucksack::priority).sum()
}

pub fn sol1() -> usize {
    let input = include_str!("day3.txt");
    let rs = parse_input(input).expect("invalid inputs");
    part1_calc(rs)
}

pub fn sol2() -> usize {
    let input = include_str!("day3.txt");
    let rs = parse_input(input).expect("invalid inputs");
    part2_calc(rs)
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn test_part1_example() {
        let rs = parse_input(EXAMPLE_INPUT).unwrap();
        assert_eq!(part1_calc(rs), 157);
    }

    #[test]
    fn test_part2_example() {
        let rs = parse_input(EXAMPLE_INPUT).unwrap();
        assert_eq!(part2_calc(rs), 70);
    }

}