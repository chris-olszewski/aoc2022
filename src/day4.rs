use std::str::FromStr;

use itertools::Itertools;

struct Pair(Range, Range);

struct Range {
    start: usize,
    end: usize,
}

impl FromStr for Pair {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (first_str, second_str) = s
            .split_once(',')
            .ok_or_else(|| format!("',' does not split {} into two parts", s))?;
        let first = Range::from_str(first_str)?;
        let second = Range::from_str(second_str)?;
        Ok(Pair(first, second))
    }
}

impl Range {
    fn includes(&self, section: usize) -> bool {
        self.start <= section && section <= self.end
    }
}

impl FromStr for Range {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (first_str, second_str) = s
            .split_once('-')
            .ok_or_else(|| format!("'-' does not split {} into two parts", s))?;
        let start = usize::from_str(first_str).unwrap();
        let end = usize::from_str(second_str).unwrap();
        Ok(Range { start, end })
    }
}

impl Range {
    fn is_subset(&self, other: &Self) -> bool {
        other.start <= self.start && self.end <= other.end
    }

    // |--|
    //  |--|
    //
    //  |---|
    //   |-|
    //
    //   |--|
    //  |--|
    // 
    //   |--|
    // |------|
    fn overlaps(&self, other: &Self) -> bool {
        (other.start <= self.start && self.start <= other.end) || (other.start <= self.end && self.end <= other.end) || other.is_subset(self)
    }
}

fn parse_input(input: &str) -> Result<Vec<Pair>, String> {
    input.lines().map(Pair::from_str).collect()
}

fn calc_part1(pairs: Vec<Pair>) -> usize {
    pairs
        .iter()
        .filter(|Pair(a, b)| a.is_subset(b) || b.is_subset(a))
        .count()
}

fn calc_part2(pairs: Vec<Pair>) -> usize {
    pairs
        .iter()
        .filter(|Pair(a, b)| a.overlaps(b))
        .count()
}

pub fn sol1() -> usize {
    let input = include_str!("day4.txt");
    let pairs = parse_input(input).unwrap();
    calc_part1(pairs)
}

pub fn sol2() -> usize {
    let input = include_str!("day4.txt");
    let pairs = parse_input(input).unwrap();
    calc_part2(pairs)
}
#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn test_part1_example() {
        let pairs = parse_input(EXAMPLE_INPUT).unwrap();
        assert_eq!(calc_part1(pairs), 2);
    }

    #[test]
    fn test_part2_example() {
        let pairs = parse_input(EXAMPLE_INPUT).unwrap();
        assert_eq!(calc_part2(pairs), 4);
    }
}
