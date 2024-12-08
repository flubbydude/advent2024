mod diagonals;

use array2d::Array2D;
use diagonals::DiagonalsIterExt;

trait XmasCounter {
    fn count_xmas(self) -> u64;
}

impl<'a, T: Iterator<Item = &'a u8>> XmasCounter for T {
    fn count_xmas(self) -> u64 {
        const XMAS: [u8; 4] = [b'X', b'M', b'A', b'S'];
        let mut i = 0;
        let mut count = 0;

        for &c in self {
            if c == XMAS[i] {
                i += 1;
                if i == 4 {
                    i = 0;
                    count += 1;
                }
            } else if c == XMAS[0] {
                i = 1;
            } else {
                i = 0;
            }
        }

        count
    }
}

fn parse_input(input: &str) -> Array2D<u8> {
    let num_rows = input.lines().count();
    let num_cols = input.lines().next().unwrap().len();

    Array2D::from_iter_row_major(
        input
            .chars()
            .filter(|c| c.is_ascii() && !c.is_ascii_whitespace())
            .map(|c| c as u8),
        num_rows,
        num_cols,
    )
    .unwrap()
}

fn part1(input: &Array2D<u8>) -> u64 {
    let mut result = 0;

    // Rows
    result += input.rows_iter().map(|row| row.count_xmas()).sum::<u64>();
    result += input
        .rows_iter()
        .map(|row| row.rev().count_xmas())
        .sum::<u64>();

    // Columns
    result += input
        .columns_iter()
        .map(|col| col.count_xmas())
        .sum::<u64>();
    result += input
        .columns_iter()
        .map(|col| col.rev().count_xmas())
        .sum::<u64>();

    // Diagonals
    result += input.diagonals_iter().map(|d| d.count_xmas()).sum::<u64>();
    result += input
        .diagonals_iter()
        .map(|d| d.rev().count_xmas())
        .sum::<u64>();

    result
}

fn part2(input: &Array2D<u8>) -> usize {
    if input.num_rows() < 3 || input.num_columns() < 3 {
        return 0;
    }

    let is_xmas_cross = |(i, j)| {
        if input[(i, j)] != b'A' {
            return false;
        }

        let crosses = [
            [input[(i - 1, j - 1)], input[(i + 1, j + 1)]],
            [input[(i - 1, j + 1)], input[(i + 1, j - 1)]],
        ];

        crosses
            .into_iter()
            .all(|arr| arr == [b'M', b'S'] || arr == [b'S', b'M'])
    };

    (1..input.num_rows() - 1)
        .flat_map(|i| (1..input.num_columns() - 1).map(move |j| (i, j)))
        .filter(|&pos| is_xmas_cross(pos))
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

    const SMALL_TEST_INPUT_1: &str = "..X...\n\
                                    .SAMX.\n\
                                    .A..A.\n\
                                    XMAS.S\n\
                                    .X....";

    const SMALL_TEST_INPUT_2: &str = "M.M\n\
                                      .A.\n\
                                      S.S";

    const TEST_INPUT: &str = "MMMSXXMASM\n\
                              MSAMXMSMSA\n\
                              AMXSXMAAMM\n\
                              MSAMASMSMX\n\
                              XMASAMXAMM\n\
                              XXAMMXXAMA\n\
                              SMSMSASXSS\n\
                              SAXAMASAAA\n\
                              MAMMMXMMMM\n\
                              MXMXAXMASX";

    #[test]
    fn test_parse_input() {
        let expected = Array2D::from_rows(&[vec![b'X', b'M'], vec![b'A', b'S']]).unwrap();

        assert_eq!(expected, parse_input("XM\nAS"));
    }

    #[test]
    fn test_part1_small() {
        let input = parse_input(SMALL_TEST_INPUT_1);
        assert_eq!(4, part1(&input))
    }

    #[test]
    fn test_part1() {
        let input = parse_input(TEST_INPUT);
        assert_eq!(18, part1(&input))
    }

    #[test]
    fn test_part2_small() {
        let input = parse_input(SMALL_TEST_INPUT_2);
        assert_eq!(1, part2(&input))
    }

    #[test]
    fn test_part2() {
        let input = parse_input(TEST_INPUT);
        assert_eq!(9, part2(&input))
    }
}
