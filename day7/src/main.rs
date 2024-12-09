use itertools::Itertools;
use std::ops;

#[derive(Debug, PartialEq, Eq)]
struct Equation {
    left: u64,
    right: Vec<u64>,
}

impl From<&str> for Equation {
    fn from(value: &str) -> Self {
        let (left_str, right_str) = value.split_once(": ").unwrap();
        let left = left_str.parse().unwrap();
        let right = right_str.split(' ').map(|s| s.parse().unwrap()).collect();

        Equation { left, right }
    }
}

impl Equation {
    fn equals_with_operators(
        &self,
        mut operators_iter: impl Iterator<Item = fn(u64, u64) -> u64>,
    ) -> bool {
        self.left
            == self
                .right
                .iter()
                .copied()
                .reduce(|acc, val| operators_iter.next().unwrap()(acc, val))
                .unwrap()
    }

    fn is_possible_with_operators(&self, operators: &[fn(u64, u64) -> u64]) -> bool {
        (0..self.right.len() - 1)
            .map(|_| operators.iter().copied())
            .multi_cartesian_product()
            .any(|operators_set| self.equals_with_operators(operators_set.iter().copied()))
    }
}

fn parse_input(input: &str) -> Vec<Equation> {
    input.lines().map(Equation::from).collect()
}

fn part1(input: &[Equation]) -> u64 {
    input
        .iter()
        .filter(|&eqn| eqn.is_possible_with_operators(&[ops::Add::add, ops::Mul::mul]))
        .map(|eqn| eqn.left)
        .sum()
}

fn part2(input: &[Equation]) -> u64 {
    fn my_concat(mut a: u64, b: u64) -> u64 {
        let mut b_copy = b;
        while b_copy >= 10 {
            a *= 10;
            b_copy /= 10;
        }

        a *= 10;

        a + b
    }

    input
        .iter()
        .filter(|&eqn| eqn.is_possible_with_operators(&[ops::Add::add, ops::Mul::mul, my_concat]))
        .map(|eqn| eqn.left)
        .sum()
}

fn main() {
    let file_contents = include_str!("../input.txt");

    let input = parse_input(file_contents);

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "190: 10 19\n\
                              3267: 81 40 27\n\
                              83: 17 5\n\
                              156: 15 6\n\
                              7290: 6 8 6 15\n\
                              161011: 16 10 13\n\
                              192: 17 8 14\n\
                              21037: 9 7 18 13\n\
                              292: 11 6 16 20";

    #[test]
    fn test_parse_input() {
        let expected = vec![
            Equation {
                left: 190,
                right: vec![10, 19],
            },
            Equation {
                left: 3267,
                right: vec![81, 40, 27],
            },
        ];
        let first_two_lines = TEST_INPUT.lines().take(2).collect::<Vec<_>>().join("\n");

        assert_eq!(expected, parse_input(&first_two_lines));
    }

    #[test]
    fn test_part1() {
        let input = parse_input(TEST_INPUT);
        assert_eq!(3749, part1(&input))
    }

    #[test]
    fn test_part2() {
        let input = parse_input(TEST_INPUT);
        assert_eq!(11387, part2(&input))
    }
}
