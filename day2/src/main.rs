use std::ops::Deref;

trait LevelsExt {
    fn is_safe(&self) -> bool;

    fn is_safe_lenient(&self) -> bool;
}

impl<T: Deref<Target = [u8]>> LevelsExt for T {
    fn is_safe(&self) -> bool {
        let difference_helper = |window: &[u8]| (1..=3).contains(&window[0].abs_diff(window[1]));

        self.windows(2).all(difference_helper)
            && (self.windows(2).all(|window| window[0] < window[1])
                || self.windows(2).rev().all(|window| window[0] >= window[1]))
    }

    fn is_safe_lenient(&self) -> bool {
        if (&self[1..]).is_safe() || (&self[..self.len() - 1]).is_safe() {
            return true;
        }

        (1..self.len() - 1).any(|i| {
            self.iter()
                .copied()
                .enumerate()
                .filter_map(|(j, level)| if i == j { None } else { Some(level) })
                .collect::<Vec<_>>()
                .is_safe()
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
    input.iter().filter(|&level| level.is_safe()).count()
}

fn part2(input: &[Vec<u8>]) -> usize {
    input
        .iter()
        .filter(|&level| level.is_safe_lenient())
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
