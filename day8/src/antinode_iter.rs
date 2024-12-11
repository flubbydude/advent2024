use std::iter::FusedIterator;

use num::Integer;

pub struct AntinodeIter {
    position: (usize, usize),
    delta: (isize, isize),
    num_rows: usize,
    num_columns: usize,
    complete: bool,
}

impl AntinodeIter {
    pub fn new(
        start_position: (usize, usize),
        from: (usize, usize),
        towards: (usize, usize),
        num_rows: usize,
        num_columns: usize,
    ) -> Self {
        let delta = if from.0 == towards.0 {
            (0, 1)
        } else if from.1 == towards.1 {
            (1, 0)
        } else {
            let gcd = towards.0.abs_diff(from.0).gcd(&towards.1.abs_diff(from.1));

            let delta_i = (towards.0 as isize - from.0 as isize) / (gcd as isize);
            let delta_j = (towards.1 as isize - from.1 as isize) / (gcd as isize);

            (delta_i, delta_j)
        };

        Self {
            position: start_position,
            delta,
            num_rows,
            num_columns,
            complete: false,
        }
    }
}

impl Iterator for AntinodeIter {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.complete {
            return None;
        }

        let result = self.position;
        match self.position.0.checked_add_signed(self.delta.0) {
            Some(i) => {
                if i >= self.num_rows {
                    self.complete = true;
                } else {
                    self.position.0 = i;
                }
            }
            None => self.complete = true,
        };
        match self.position.1.checked_add_signed(self.delta.1) {
            Some(j) => {
                if j >= self.num_columns {
                    self.complete = true;
                } else {
                    self.position.1 = j;
                }
            }
            None => self.complete = true,
        };

        Some(result)
    }
}

impl FusedIterator for AntinodeIter {}
