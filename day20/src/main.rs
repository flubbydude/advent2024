mod direction;
mod grid_cell;
mod puzzle_input;

use std::cmp::Ordering;

use array2d::Array2D;
use enum_iterator::all;

// It can't use manhattan distance for within, instead checks a cube.
// But not sure if that's just this impl or if that's KD Trees in general
// Need to look into it more. Maybe can redo day 20 and 17 after.
// At least come back and do my own impl of KdTree which doesn't have
// doodoo code in it
// due to Dim type requirement in KdPoint, this library is but cheeks
// TODO: also can implement getting all 20 points nearby to a point,
// AND/OR a sliding window (with a 2d sliding window).
// sliding window would be tough when handling numbers outside of the bounds
use kd_tree::{KdPoint, KdTree};

use direction::Direction;
use grid_cell::GridCell;
use puzzle_input::PuzzleInput;

fn move_once_in_direction(
    (i, j): (usize, usize),
    direction: Direction,
    num_rows: usize,
    num_columns: usize,
) -> Option<(usize, usize)> {
    let (di, dj) = direction.as_tuple();

    let new_i = i.checked_add_signed(di)?;
    let new_j = j.checked_add_signed(dj)?;

    if new_i >= num_rows || new_j >= num_columns {
        None
    } else {
        Some((new_i, new_j))
    }
}

fn get_positions_two_away_from(
    position: (usize, usize),
    num_rows: usize,
    num_columns: usize,
) -> impl IntoIterator<Item = (usize, usize)> {
    //
    //   A
    //  H#B
    // G#.#C
    //  F#D
    //   E
    //
    [
        (Direction::North, Direction::North),
        (Direction::North, Direction::East),
        (Direction::East, Direction::East),
        (Direction::South, Direction::East),
        (Direction::South, Direction::South),
        (Direction::South, Direction::West),
        (Direction::West, Direction::West),
        (Direction::North, Direction::West),
    ]
    .into_iter()
    .filter_map(move |(d1, d2)| {
        move_once_in_direction(position, d1, num_rows, num_columns)
            .and_then(|p2| move_once_in_direction(p2, d2, num_rows, num_columns))
    })
}

fn get_distance_grid(puzzle_input: &PuzzleInput) -> Array2D<Option<usize>> {
    let &PuzzleInput {
        ref grid,
        start_position,
        end_position,
    } = puzzle_input;

    let mut dist_grid = Array2D::filled_with(None, grid.num_rows(), grid.num_columns());
    let mut cur_position = end_position;
    let mut prev_position = None;
    let mut distance = 0;

    'outer: while cur_position != start_position {
        'inner: for direction in all::<Direction>() {
            let Some(maybe_next_position) = move_once_in_direction(
                cur_position,
                direction,
                grid.num_rows(),
                grid.num_columns(),
            ) else {
                continue 'inner;
            };

            if grid[maybe_next_position] == GridCell::Track
                && Some(maybe_next_position) != prev_position
            {
                dist_grid[cur_position] = Some(distance);
                distance += 1;
                prev_position = Some(cur_position);
                cur_position = maybe_next_position;
                continue 'outer;
            }
        }
        panic!("No successor found at {cur_position:?}");
    }

    dist_grid[cur_position] = Some(distance);

    dist_grid
}

fn part1(puzzle_input: &PuzzleInput, steps_to_save: usize) -> usize {
    let distance_grid = &get_distance_grid(puzzle_input);

    // From instructions:
    // "cheats are uniquely identified by their start position and end position"
    // So, only need to check end positions distances
    // can ignore checking walls and stuff
    distance_grid
        .enumerate_row_major()
        .filter_map(|(position, maybe_dist)| maybe_dist.map(|dist| (position, dist)))
        .flat_map(|(position_before_cheat, dist_before_cheat)| {
            get_positions_two_away_from(
                position_before_cheat,
                distance_grid.num_rows(),
                distance_grid.num_columns(),
            )
            .into_iter()
            .filter_map(|position_after_cheat| distance_grid[position_after_cheat])
            .filter(move |&dist_after_cheat| {
                dist_after_cheat >= dist_before_cheat + steps_to_save + 2
            })
        })
        .count()
}

fn part2(puzzle_input: &PuzzleInput, steps_to_save: usize) -> usize {
    const RADIUS: usize = 20;

    let distance_grid = &get_distance_grid(puzzle_input);

    struct Item((usize, usize));

    impl KdPoint for Item {
        type Scalar = usize;
        type Dim = typenum::U2;

        fn at(&self, i: usize) -> Self::Scalar {
            match i {
                0 => self.0 .0,
                1 => self.0 .1,
                _ => panic!(),
            }
        }
    }

    let kdpoints = distance_grid
        .enumerate_row_major()
        .filter_map(|(position, maybe_dist)| maybe_dist.map(|_| Item(position)))
        .collect::<Vec<_>>();

    let kd_tree = KdTree::build(kdpoints);

    kd_tree
        .iter()
        .flat_map(|item_before_cheat| {
            let dist_before_cheat = distance_grid[item_before_cheat.0].unwrap();
            kd_tree
                .within_by_cmp(|maybe_arr_after_cheat, k| {
                    if maybe_arr_after_cheat.at(k) + RADIUS < item_before_cheat.at(k) {
                        Ordering::Less
                    } else if maybe_arr_after_cheat.at(k) > item_before_cheat.at(k) + RADIUS {
                        Ordering::Greater
                    } else {
                        Ordering::Equal
                    }
                })
                .into_iter()
                .filter(move |&&Item(position_after_cheat)| {
                    let cheat_dist = position_after_cheat.0.abs_diff(item_before_cheat.0 .0)
                        + position_after_cheat.1.abs_diff(item_before_cheat.0 .1);

                    if cheat_dist > RADIUS {
                        return false;
                    }

                    let dist_after_cheat = distance_grid[position_after_cheat].unwrap();
                    dist_after_cheat >= dist_before_cheat + steps_to_save + cheat_dist
                })
        })
        .count()
}

fn main() {
    const INPUT_STR: &str = include_str!("../input.txt");
    const STEPS_TO_SAVE: usize = 100;

    let puzzle_input = PuzzleInput::from_input(INPUT_STR);

    println!("{}", part1(&puzzle_input, STEPS_TO_SAVE));
    println!("{}", part2(&puzzle_input, STEPS_TO_SAVE));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_STR: &str = include_str!("../example.txt");

    /// See `../example_solution.txt`
    const EXAMPLE_SOLN_VALS_PART1: [(usize, usize); 11] = [
        (14, 2),
        (14, 4),
        (2, 6),
        (4, 8),
        (2, 10),
        (3, 12),
        (1, 20),
        (1, 36),
        (1, 38),
        (1, 40),
        (1, 64),
    ];

    /// See `../example_solution.txt`
    const EXAMPLE_SOLN_VALS_PART2: [(usize, usize); 14] = [
        (32, 50),
        (31, 52),
        (29, 54),
        (39, 56),
        (25, 58),
        (23, 60),
        (20, 62),
        (19, 64),
        (12, 66),
        (14, 68),
        (12, 70),
        (22, 72),
        (4, 74),
        (3, 76),
    ];

    #[test]
    pub fn test_part1() {
        let puzzle_input = PuzzleInput::from_input(TEST_INPUT_STR);

        for (i, &(_, num_steps)) in EXAMPLE_SOLN_VALS_PART1.iter().enumerate() {
            assert_eq!(
                part1(&puzzle_input, num_steps),
                EXAMPLE_SOLN_VALS_PART1[i..]
                    .iter()
                    .map(|&(num_ways, _)| num_ways)
                    .sum(),
                "num_steps = {num_steps}"
            );
        }
    }

    #[test]
    pub fn test_part2() {
        let puzzle_input = PuzzleInput::from_input(TEST_INPUT_STR);

        for (i, &(_, num_steps)) in EXAMPLE_SOLN_VALS_PART2.iter().enumerate().rev() {
            assert_eq!(
                part2(&puzzle_input, num_steps),
                EXAMPLE_SOLN_VALS_PART2[i..]
                    .iter()
                    .map(|&(num_ways, _)| num_ways)
                    .sum(),
                "num_steps = {num_steps}"
            );
        }
    }
}
