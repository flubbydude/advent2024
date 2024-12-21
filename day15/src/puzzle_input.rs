use array2d::Array2D;

use crate::{direction::Direction, part1_cell::Part1Cell};

#[derive(Debug, Clone)]
pub struct PuzzleInput {
    pub grid: Array2D<Part1Cell>,
    pub start_position: (usize, usize),
    pub directions: Vec<Direction>,
}

impl PuzzleInput {
    pub fn parse_input(input_str: &str) -> PuzzleInput {
        let mut lines = input_str.lines();

        let input_grid_vecs = (&mut lines)
            .take_while(|line| !line.is_empty())
            .map(|line| {
                line.chars()
                    .map(|c| InputCell::try_from(c).unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let start_position = input_grid_vecs
            .iter()
            .enumerate()
            .flat_map(|(i, row)| row.iter().enumerate().map(move |(j, cell)| ((i, j), cell)))
            .find(|&(_, &cell)| matches!(cell, InputCell::Robot))
            .unwrap()
            .0;

        let grid = Array2D::from_iter_row_major(
            input_grid_vecs
                .iter()
                .flat_map(|row| row.iter().copied().map(Part1Cell::from)),
            input_grid_vecs.len(),
            input_grid_vecs[0].len(),
        )
        .unwrap();

        let directions = lines
            .flat_map(str::chars)
            .map(|c| Direction::try_from(c).unwrap())
            .collect();

        PuzzleInput {
            grid,
            start_position,
            directions,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum InputCell {
    Robot,
    GridCell(Part1Cell),
}

impl TryFrom<char> for InputCell {
    type Error = char;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        if value == '@' {
            Ok(InputCell::Robot)
        } else {
            Ok(InputCell::GridCell(value.try_into()?))
        }
    }
}

impl From<InputCell> for Part1Cell {
    fn from(value: InputCell) -> Self {
        match value {
            InputCell::Robot => Part1Cell::Empty,
            InputCell::GridCell(grid_cell) => grid_cell,
        }
    }
}
