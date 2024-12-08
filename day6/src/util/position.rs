pub trait PositionExt
where
    Self: Sized,
{
    fn bounded_move_in_direction(
        &self,
        direction: Direction,
        num_rows: usize,
        num_columns: usize,
    ) -> Option<Self>;
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn as_tuple(&self) -> (isize, isize) {
        use Direction::*;
        match self {
            North => (-1, 0),
            East => (0, 1),
            South => (1, 0),
            West => (0, -1),
        }
    }

    pub fn turn_cw(&self) -> Direction {
        use Direction::*;
        match self {
            North => East,
            East => South,
            South => West,
            West => North,
        }
    }
}

impl PositionExt for (usize, usize) {
    fn bounded_move_in_direction(
        &self,
        direction: Direction,
        num_rows: usize,
        num_columns: usize,
    ) -> Option<Self> {
        let (delta_i, delta_j) = direction.as_tuple();
        let new_i = self.0 as isize + delta_i;
        let new_j = self.1 as isize + delta_j;

        if new_i < 0 || new_j < 0 {
            return None;
        }

        let new_i = new_i as usize;
        let new_j = new_j as usize;

        if new_i >= num_rows || new_j >= num_columns {
            return None;
        }

        Some((new_i, new_j))
    }
}
