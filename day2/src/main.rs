use itertools::Itertools;
use std::ops::Deref;

trait LevelsExt {
    fn is_safe(&self) -> bool;
    fn is_safe_lenient(&self) -> bool;
}

trait IterLevelsExt {
    fn is_safe_ordered(self, increasing: bool) -> bool;
}

fn passes_difference_rule(first: u8, second: u8) -> bool {
    first.abs_diff(second) <= 3
}

fn passes_ordering_rule(first: u8, second: u8, increasing: bool) -> bool {
    first != second && (increasing == (first < second))
}

fn is_safe_adjacent(first: u8, second: u8, increasing: bool) -> bool {
    passes_difference_rule(first, second) && passes_ordering_rule(first, second, increasing)
}

impl<T: Iterator<Item = u8>> IterLevelsExt for T {
    fn is_safe_ordered(self, increasing: bool) -> bool {
        self.tuple_windows()
            .all(|(a, b)| is_safe_adjacent(a, b, increasing))
    }
}

impl<T: Deref<Target = [u8]>> LevelsExt for T {
    fn is_safe(&self) -> bool {
        if self.len() < 2 {
            return true;
        }

        self.iter().copied().is_safe_ordered(self[0] < self[1])
    }

    fn is_safe_lenient(&self) -> bool {
        [false, true].into_iter().any(|increasing| {
            let problem_index = match self
                .iter()
                .copied()
                .tuple_windows()
                .position(|(first, second)| !is_safe_adjacent(first, second, increasing))
            {
                Some(index) => index,
                None => return true,
            };

            [problem_index, problem_index + 1]
                .into_iter()
                .any(|index_to_remove| {
                    let before_index_to_remove = if index_to_remove > 0 {
                        Some(index_to_remove - 1)
                    } else {
                        None
                    };

                    before_index_to_remove
                        .map(|i| self[i])
                        .into_iter()
                        .chain(self[index_to_remove + 1..].iter().copied())
                        .is_safe_ordered(increasing)
                })
        })
    }
}

fn parse_line(line: &str) -> Vec<u8> {
    line.split_ascii_whitespace()
        .map(|s| s.parse::<u8>().unwrap())
        .collect()
}

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input.lines().map(parse_line).collect()
}

fn part1(input: &[Vec<u8>]) -> usize {
    input.iter().filter(|&levels| levels.is_safe()).count()
}

fn part2(input: &[Vec<u8>]) -> usize {
    input
        .iter()
        .filter(|&levels| levels.is_safe_lenient())
        .count()
}

fn main() {
    let file_contents = std::fs::read("input.txt").unwrap();
    let file_contents_as_str = std::str::from_utf8(&file_contents).unwrap();

    let input = parse_input(file_contents_as_str);

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "7 6 4 2 1\n\
                              1 2 7 8 9\n\
                              9 7 6 2 1\n\
                              1 3 2 4 5\n\
                              8 6 4 4 1\n\
                              1 3 6 7 9";

    #[test]
    fn test_parse_input() {
        assert_eq!(
            vec![
                vec![7, 6, 4, 2, 1],
                vec![1, 2, 7, 8, 9],
                vec![9, 7, 6, 2, 1],
                vec![1, 3, 2, 4, 5],
                vec![8, 6, 4, 4, 1],
                vec![1, 3, 6, 7, 9]
            ],
            parse_input(TEST_INPUT)
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(2, part1(&parse_input(TEST_INPUT)));
    }

    #[test]
    fn test_part2() {
        assert_eq!(4, part2(&parse_input(TEST_INPUT)));
    }
}
