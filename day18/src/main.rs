mod a_star;
mod binary_search;
mod grid_cell;

use array2d::Array2D;

use a_star::best_cost_a_star;
use binary_search::BinarySearchExt;
use grid_cell::GridCell;

fn parse_coordinates(value: &str) -> (usize, usize) {
    let (i_str, j_str) = value.split_once(',').unwrap();
    (i_str.parse().unwrap(), j_str.parse().unwrap())
}

fn part1(
    byte_coordinates: impl IntoIterator<Item = (usize, usize)>,
    num_rows: usize,
    num_columns: usize,
    num_bytes_to_fall: usize,
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

fn part2(
    coordinates: &[(usize, usize)],
    num_rows: usize,
    num_columns: usize,
) -> Option<(usize, usize)> {
    let pp = (0..coordinates.len()).partition_point(|&i| {
        part1(coordinates.iter().copied(), num_rows, num_columns, i).is_none()
    });
    coordinates.get(pp.checked_sub(1).unwrap()).cloned()
}

fn main() {
    const INPUT: &str = include_str!("../input.txt");
    const NUM_ROWS: usize = 71;
    const NUM_COLUMNS: usize = 71;
    const NUM_BYTES_TO_FALL_PART1: usize = 1024;

    println!(
        "{:?}",
        part1(
            INPUT.lines().map(parse_coordinates),
            NUM_ROWS,
            NUM_COLUMNS,
            NUM_BYTES_TO_FALL_PART1,
        )
    );

    let coordinates = INPUT.lines().map(parse_coordinates).collect::<Vec<_>>();

    println!("{:?}", part2(&coordinates, NUM_ROWS, NUM_COLUMNS));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../example.txt");
    const TEST_NUM_ROWS: usize = 7;
    const TEST_NUM_COLUMNS: usize = 7;
    const TEST_NUM_BYTES_TO_FALL: usize = 12;

    #[test]
    fn test_part1() {
        let coordinates_iter = TEST_INPUT.lines().map(parse_coordinates);

        assert_eq!(
            part1(
                coordinates_iter,
                TEST_NUM_ROWS,
                TEST_NUM_COLUMNS,
                TEST_NUM_BYTES_TO_FALL
            ),
            Some(22)
        );
    }

    #[test]
    fn test_part2() {
        let coordinates = TEST_INPUT
            .lines()
            .map(parse_coordinates)
            .collect::<Vec<_>>();

        assert_eq!(
            Some((6, 1)),
            part2(&coordinates, TEST_NUM_ROWS, TEST_NUM_COLUMNS)
        );
    }
}
