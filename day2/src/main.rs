use std::ops::Deref;

trait LevelsExt {
    fn is_safe(self) -> bool;
}

trait LenientLevelsExt {
    fn is_safe_lenient(&self) -> bool;
}

impl<T: Iterator<Item = u8>> LevelsExt for T {
    fn is_safe(mut self) -> bool {
        let mut prev = match self.next() {
            Some(x) => x,
            None => return true,
        };

        let mut peekable = self.peekable();

        let increasing = match peekable.peek() {
            Some(&second) => second > prev,
            None => return true,
        };

        while let Some(cur) = peekable.next() {
            if !(1..=3).contains(&cur.abs_diff(prev)) || increasing != (cur > prev) {
                return false;
            }

            prev = cur;
        }

        true
    }
}

impl<T: Deref<Target = [u8]>> LenientLevelsExt for T {
    fn is_safe_lenient(&self) -> bool {
        (0..self.len()).any(|i| {
            self.iter()
                .copied()
                .enumerate()
                .filter_map(|(j, level)| if i == j { None } else { Some(level) })
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
    input
        .iter()
        .filter(|&levels| levels.iter().copied().is_safe())
        .count()
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
