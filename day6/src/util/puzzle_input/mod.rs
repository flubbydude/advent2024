mod cells;
pub use cells::MapCell;

use array2d::Array2D;

use crate::guard_iter::GuardIterator;
use cells::InputCell;

pub struct PuzzleInput {
    pub map_grid: Array2D<MapCell>,
    pub start_pos: (usize, usize),
}

impl PuzzleInput {
    pub fn guard_iter(&self) -> GuardIterator<'_> {
        GuardIterator::new(&self.map_grid, self.start_pos)
    }
}

fn parse_input_grid(input_str: &str) -> Array2D<InputCell> {
    let num_rows = input_str.lines().count();
    let num_columns = input_str.lines().next().unwrap().chars().count();

    Array2D::from_iter_row_major(
        input_str
            .chars()
            .filter(|c| !c.is_ascii_whitespace())
            .map(|c| match c {
                '#' => InputCell::Obstacle,
                '.' => InputCell::Empty,
                '^' => InputCell::Start,
                _ => panic!("Unknown character {c} in input"),
            }),
        num_rows,
        num_columns,
    )
    .unwrap()
}

impl From<&str> for PuzzleInput {
    fn from(value: &str) -> Self {
        let input_grid = parse_input_grid(value);
        let start_pos = input_grid
            .enumerate_row_major()
            .find_map(|((i, j), cell)| {
                if matches!(cell, &InputCell::Start) {
                    Some((i, j))
                } else {
                    None
                }
            })
            .expect("Puzzle doesn't have start position");

        let map_grid = Array2D::from_iter_row_major(
            input_grid
                .elements_row_major_iter()
                .cloned()
                .map(MapCell::from),
            input_grid.num_rows(),
            input_grid.num_columns(),
        )
        .unwrap();

        PuzzleInput {
            map_grid,
            start_pos,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let grid = parse_input_grid(".#\n^#");
        let expected_grid = Array2D::from_rows(&[
            vec![InputCell::Empty, InputCell::Obstacle],
            vec![InputCell::Start, InputCell::Obstacle],
        ])
        .unwrap();

        assert_eq!(expected_grid, grid);
    }
}
