use array2d::Array2D;

#[derive(Clone)]
struct Array2DDiagonalIter<'a, T> {
    array2d: &'a Array2D<T>,
    top_left: (usize, usize),
    bottom_right: (usize, usize),
}

impl<'a, T> Iterator for Array2DDiagonalIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.top_left == self.bottom_right {
            return None;
        }

        let result = &self.array2d[self.top_left];
        self.top_left = (self.top_left.0 + 1, self.top_left.1 + 1);
        Some(result)
    }
}

impl<T> DoubleEndedIterator for Array2DDiagonalIter<'_, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.top_left == self.bottom_right {
            return None;
        }

        self.bottom_right = (self.bottom_right.0 - 1, self.bottom_right.1 - 1);
        let result = &self.array2d[self.bottom_right];
        Some(result)
    }
}

trait DiagonalIterExt<T> {
    fn left_column_diagonal_iter(&self, start_row_index: usize) -> Array2DDiagonalIter<T>;
    fn top_row_diagonal_iter(&self, start_col_index: usize) -> Array2DDiagonalIter<T>;
}

trait DiagonalsIterExt<T> {
    fn diagonals_iter<'a>(&'a self) -> impl Iterator<Item = Array2DDiagonalIter<'a, T>> + Clone
    where
        T: 'a;
}

impl<T> DiagonalIterExt<T> for Array2D<T> {
    fn left_column_diagonal_iter(&self, start_row_index: usize) -> Array2DDiagonalIter<T> {
        if start_row_index >= self.num_rows() {
            panic!("start_row_index {start_row_index} too large");
        }

        let count = self.num_columns().min(self.num_rows() - start_row_index);

        Array2DDiagonalIter {
            array2d: &self,
            top_left: (start_row_index, 0),
            bottom_right: (start_row_index + count, count),
        }
    }

    fn top_row_diagonal_iter(&self, start_col_index: usize) -> Array2DDiagonalIter<T> {
        if start_col_index >= self.num_columns() {
            panic!("start_col_index {start_col_index} too large");
        }

        let count = self.num_rows().min(self.num_columns() - start_col_index);

        Array2DDiagonalIter {
            array2d: &self,
            top_left: (0, start_col_index),
            bottom_right: (count, start_col_index + count),
        }
    }
}

impl<T> DiagonalsIterExt<T> for Array2D<T> {
    fn diagonals_iter<'a>(&'a self) -> impl Iterator<Item = Array2DDiagonalIter<T>> + Clone
    where
        T: 'a,
    {
        (0..self.num_columns())
            .map(|index| self.top_row_diagonal_iter(index))
            .chain((1..self.num_rows()).map(|index| self.left_column_diagonal_iter(index)))
    }
}

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

// fn part2(input: &str) -> usize {
//     todo!()
// }

fn main() {
    let file_contents = std::fs::read("input.txt").unwrap();
    let file_contents_as_str = std::str::from_utf8(&file_contents).unwrap();

    let input = parse_input(file_contents_as_str);

    println!("{}", part1(&input));
    // println!("{}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SMALL_TEST_INPUT: &str = "..X...\n\
                                    .SAMX.\n\
                                    .A..A.\n\
                                    XMAS.S\n\
                                    .X....";

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
        let input = parse_input(SMALL_TEST_INPUT);
        assert_eq!(4, part1(&input))
    }

    #[test]
    fn test_part1() {
        let input = parse_input(TEST_INPUT);
        assert_eq!(18, part1(&input))
    }

    // #[test]
    // fn test_part2() {
    //     let input = parse_input(TEST_INPUT);
    //     assert_eq!(0, part2(input))
    // }
}
