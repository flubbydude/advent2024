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

pub trait DiagonalsIterExt<T> {
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
