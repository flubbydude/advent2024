mod grid_cell;
mod search;

use array2d::Array2D;

use grid_cell::GridCell;
use search::best_cost_a_star;

fn parse_coordinates(value: &str) -> (usize, usize) {
    let (i_str, j_str) = value.split_once(',').unwrap();
    (i_str.parse().unwrap(), j_str.parse().unwrap())
}

fn part1(
    byte_coordinates: impl IntoIterator<Item = (usize, usize)>,
    num_bytes_to_fall: usize,
    num_rows: usize,
    num_columns: usize,
) -> Option<u64> {
    let mut grid = Array2D::filled_with(GridCell::Safe, num_rows, num_columns);

    for coord in byte_coordinates.into_iter().take(num_bytes_to_fall) {
        grid[coord] = GridCell::Corrupted;
    }

    let grid = grid;
    let grid_ref = &grid;

    let goal = (num_rows - 1, num_columns - 1);

    let start_states = [(0, (0, 0))];
    let successors = |&(i, j): &(usize, usize)| {
        [(-1, 0), (0, 1), (1, 0), (0, -1)].into_iter().filter_map(
            move |(di, dj): (isize, isize)| {
                let next_position = (i.checked_add_signed(di)?, j.checked_add_signed(dj)?);
                if next_position.0 < num_rows
                    && next_position.1 < num_columns
                    && grid_ref[next_position] == GridCell::Safe
                {
                    Some((1, next_position))
                } else {
                    None
                }
            },
        )
    };
    let is_goal = |state: &(usize, usize)| *state == goal;
    let heuristic =
        |state: &(usize, usize)| (state.0.abs_diff(goal.0) + state.1.abs_diff(goal.1)) as u64;

    best_cost_a_star(start_states, successors, is_goal, heuristic)
}

fn main() {
    let input_str = include_str!("../input.txt");
    let num_rows = 71;
    let num_columns = 71;
    let num_bytes_to_fall = 1024;

    let coordinates_iter = input_str.lines().map(parse_coordinates);

    println!(
        "{:?}",
        part1(coordinates_iter, num_bytes_to_fall, num_rows, num_columns,)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../example.txt");

    #[test]
    fn test_part1() {
        let coordinates_iter = TEST_INPUT.lines().map(parse_coordinates);
        let num_rows = 7;
        let num_columns = 7;
        let num_bytes_to_fall = 12;

        assert_eq!(
            part1(coordinates_iter, num_bytes_to_fall, num_rows, num_columns),
            Some(22)
        );
    }
}
