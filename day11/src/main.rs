use std::iter::once;

use memoize::memoize;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Rock(u64);

impl Rock {
    fn get_successors(self) -> (Rock, Option<Rock>) {
        if self.0 == 0 {
            return (Rock(1), None);
        }

        let rock_str = self.0.to_string();
        let num_digits = rock_str.as_bytes().len();

        if num_digits % 2 == 0 {
            let (left, right) = rock_str.split_at(num_digits / 2);
            return (
                Rock(left.parse().unwrap()),
                Some(Rock(right.parse().unwrap())),
            );
        }

        (Rock(2024 * self.0), None)
    }
}

fn parse_input(input: &str) -> Vec<Rock> {
    input
        .split_ascii_whitespace()
        .map(|nstr| Rock(nstr.parse().unwrap()))
        .collect()
}

#[memoize]
fn num_rocks_from_rock(rock: Rock, num_steps: usize) -> usize {
    if num_steps == 0 {
        return 1;
    }

    let (left, right) = rock.get_successors();
    once(left)
        .chain(right.into_iter())
        .map(|the_rock| num_rocks_from_rock(the_rock, num_steps - 1))
        .sum()
}

fn num_rocks_from_rocks(rocks: &Vec<Rock>, num_steps: usize) -> usize {
    rocks
        .iter()
        .cloned()
        .map(|rock| num_rocks_from_rock(rock, num_steps))
        .sum()
}

fn part1(rocks: &Vec<Rock>) -> usize {
    const PART1_NUM_STEPS: usize = 25;
    num_rocks_from_rocks(rocks, PART1_NUM_STEPS)
}

fn part2(rocks: &Vec<Rock>) -> usize {
    const PART2_NUM_STEPS: usize = 75;
    num_rocks_from_rocks(rocks, PART2_NUM_STEPS)
}

fn main() {
    let file_contents_as_str = include_str!("../input.txt");

    let input = parse_input(file_contents_as_str);

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "125 17";

    #[test]
    fn test_parse_input() {
        let input = parse_input(TEST_INPUT);
        assert_eq!(vec![Rock(125), Rock(17)], input);
    }

    #[test]
    fn test_part1_a() {
        let input = parse_input(TEST_INPUT);
        assert_eq!(22, num_rocks_from_rocks(&input, 6));
    }

    #[test]
    fn test_part1_b() {
        let input = parse_input(TEST_INPUT);
        assert_eq!(55312, part1(&input));
    }
}
