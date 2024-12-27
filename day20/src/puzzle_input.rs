use array2d::Array2D;

use crate::grid_cell::GridCell;

pub struct PuzzleInput {
    pub grid: Array2D<GridCell>,
    pub start_position: (usize, usize),
    pub end_position: (usize, usize),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum InputCell {
    Start,
    End,
    GridCell(GridCell),
}

impl From<char> for InputCell {
    fn from(value: char) -> Self {
        match value {
            '.' => InputCell::GridCell(GridCell::Track),
            '#' => InputCell::GridCell(GridCell::Wall),
            'S' => InputCell::Start,
            'E' => InputCell::End,
            _ => panic!("Unknown character in input: '{value}'"),
        }
    }
}

impl From<InputCell> for GridCell {
    fn from(value: InputCell) -> Self {
        match value {
            InputCell::Start | InputCell::End => GridCell::Track,
            InputCell::GridCell(grid_cell) => grid_cell,
        }
    }
}

impl PuzzleInput {
    pub fn from_input(input_str: &str) -> PuzzleInput {
        let num_rows = input_str.lines().count();
        let num_columns = input_str.lines().next().unwrap().len();

        let input_grid = Array2D::from_iter_row_major(
            input_str.lines().flat_map(str::chars).map(InputCell::from),
            num_rows,
            num_columns,
        )
        .unwrap();

        let start_position = input_grid
            .enumerate_row_major()
            .find(|&(_, cell)| matches!(cell, InputCell::Start))
            .unwrap()
            .0;

        let end_position = input_grid
            .enumerate_row_major()
            .find(|&(_, cell)| matches!(cell, InputCell::End))
            .unwrap()
            .0;

        let grid = Array2D::from_iter_row_major(
            input_grid
                .elements_row_major_iter()
                .copied()
                .map(GridCell::from),
            num_rows,
            num_columns,
        )
        .unwrap();

        PuzzleInput {
            grid,
            start_position,
            end_position,
        }
    }
}
