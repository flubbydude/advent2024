use array2d::Array2D;

use crate::part1_cell::Part1Cell;

#[derive(Debug, Clone, Copy)]
pub enum Part2Cell {
    Empty,
    LeftBox,
    RightBox,
    Wall,
}

pub fn to_part2_grid(part1_grid: &Array2D<Part1Cell>) -> Array2D<Part2Cell> {
    Array2D::from_iter_row_major(
        part1_grid
            .elements_row_major_iter()
            .flat_map(|cell| match cell {
                Part1Cell::Box => [Part2Cell::LeftBox, Part2Cell::RightBox],
                Part1Cell::Empty => [Part2Cell::Empty, Part2Cell::Empty],
                Part1Cell::Wall => [Part2Cell::Wall, Part2Cell::Wall],
            }),
        part1_grid.num_rows(),
        part1_grid.num_columns() * 2,
    )
    .unwrap()
}
