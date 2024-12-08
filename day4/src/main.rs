use array2d::Array2D;

#[derive(Clone)]
struct DiagonalIter {
    num_cols: usize,
    // internally, always a southeast diagonal.
    // flip horizontally when flag is true
    top_left: (usize, usize),
    bottom_right: (usize, usize),
    flip_horizontally: bool,
}

impl DiagonalIter {
    fn new_southeast_from_left(
        num_rows: usize,
        num_cols: usize,
        start_row_index: usize,
    ) -> DiagonalIter {
        if start_row_index >= num_rows {
            panic!("start_row_index = {start_row_index} greater than num_rows = {num_rows}");
        }

        let count = num_cols.min(num_rows - start_row_index);

        DiagonalIter {
            num_cols,
            top_left: (start_row_index, 0),
            bottom_right: (start_row_index + count, count),
            flip_horizontally: false,
        }
    }

    fn new_southeast_from_top(
        num_rows: usize,
        num_cols: usize,
        start_col_index: usize,
    ) -> DiagonalIter {
        if start_col_index >= num_cols {
            panic!("start_col_index = {start_col_index} greater than num_cols = {num_cols}");
        }

        let count = num_rows.min(num_cols - start_col_index);

        DiagonalIter {
            num_cols,
            top_left: (0, start_col_index),
            bottom_right: (count, start_col_index + count),
            flip_horizontally: false,
        }
    }

    fn flipped(mut self) -> Self {
        self.flip_horizontally = !self.flip_horizontally;
        self
    }
}

impl Iterator for DiagonalIter {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.top_left == self.bottom_right {
            return None;
        }

        let mut result = self.top_left;
        if self.flip_horizontally {
            result.1 = self.num_cols - result.1 - 1;
        }

        self.top_left = (self.top_left.0 + 1, self.top_left.1 + 1);

        Some(result)
    }
}

impl DoubleEndedIterator for DiagonalIter {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.top_left == self.bottom_right {
            return None;
        }

        self.bottom_right = (self.bottom_right.0 - 1, self.bottom_right.1 - 1);

        let mut result = self.bottom_right;
        if self.flip_horizontally {
            result.1 = self.num_cols - result.1 - 1;
        }
        Some(result)
    }
}

trait DiagonalsIterExt<T> {
    fn diagonals_iter<'a>(
        &'a self,
    ) -> impl Iterator<Item = impl DoubleEndedIterator<Item = &'a T> + Clone> + Clone
    where
        T: 'a;
}

impl<T> DiagonalsIterExt<T> for Array2D<T> {
    fn diagonals_iter<'a>(
        &'a self,
    ) -> impl Iterator<Item = impl DoubleEndedIterator<Item = &'a T> + Clone> + Clone
    where
        T: 'a,
    {
        let southeast_diags = (0..self.num_columns())
            .map(|index| {
                DiagonalIter::new_southeast_from_top(self.num_rows(), self.num_columns(), index)
            })
            .chain((1..self.num_rows()).map(|index| {
                DiagonalIter::new_southeast_from_left(self.num_rows(), self.num_columns(), index)
            }));

        let all_diags = southeast_diags
            .clone()
            .map(DiagonalIter::flipped)
            .chain(southeast_diags);

        all_diags.map(|diag| diag.map(|pos| &self[pos]))
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
