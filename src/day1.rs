use itertools::Itertools;
use std::num::ParseIntError;

fn get_foods(input: &str) -> Vec<usize> {
    let mut food = Vec::new();
    let mut curr = None;
    for line in input.lines() {
        if line.is_empty() {
            if let Some(curr) = curr {
                food.push(curr);
            }
            curr = None
        } else {
            let cals: usize = line.parse().unwrap();
            *curr.get_or_insert(0) += cals;
        }
    }
    food
}

fn solve(input: &str) -> usize {
    *get_foods(input).iter().max().unwrap()
}

pub fn answer1() -> usize {
    let input = include_str!("day1.txt");
    return solve(input)
}

pub fn answer2() -> usize {
    let input = include_str!("day1.txt");
    let mut foods = get_foods(input);
    foods.sort();
    foods.iter().rev().take(3).copied().sum()
}

#[cfg(test)]
mod test {
    use super::solve;

    #[test]
    fn test_example() {
        let example =
        "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

        assert_eq!(solve(example), 24000);
    }
}