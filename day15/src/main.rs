mod direction;
mod part1_cell;
mod part2_cell;
mod puzzle_input;

use std::collections::HashSet;

use array2d::Array2D;
use direction::Direction;
use part1_cell::Part1Cell;
use part2_cell::{to_part2_grid, Part2Cell};
use puzzle_input::PuzzleInput;

fn gps((i, j): (usize, usize)) -> usize {
    100 * i + j
}

fn move_once_in_direction((i, j): (usize, usize), direction: Direction) -> (usize, usize) {
    // assume no overflow or underflow since walls are around the whole thang
    match direction {
        Direction::North => (i - 1, j),
        Direction::East => (i, j + 1),
        Direction::South => (i + 1, j),
        Direction::West => (i, j - 1),
    }
}

fn move_and_push_part1(
    grid: &mut Array2D<Part1Cell>,
    robot_position: &mut (usize, usize),
    direction: Direction,
) {
    let next_robot_position = move_once_in_direction(*robot_position, direction);
    match grid[next_robot_position] {
        Part1Cell::Empty => *robot_position = next_robot_position,
        Part1Cell::Wall => (),
        Part1Cell::Box => {
            let mut next_box_position = move_once_in_direction(next_robot_position, direction);
            loop {
                match grid[next_box_position] {
                    Part1Cell::Empty => {
                        grid[next_box_position] = Part1Cell::Box;
                        grid[next_robot_position] = Part1Cell::Empty;
                        *robot_position = next_robot_position;
                        return;
                    }
                    Part1Cell::Wall => return,
                    Part1Cell::Box => {
                        next_box_position = move_once_in_direction(next_box_position, direction)
                    }
                }
            }
        }
    }
}

fn try_push_part2_north_south(
    grid: &mut Array2D<Part2Cell>,
    box_position: (usize, usize),
    direction: Direction,
    seen: &mut HashSet<(usize, usize)>,
) -> bool {
    todo!()
}

fn try_push_part2_east_west(
    grid: &mut Array2D<Part2Cell>,
    box_position: (usize, usize),
    direction: Direction,
) -> bool {
    todo!()
}

fn try_push_part2(
    grid: &mut Array2D<Part2Cell>,
    box_position: (usize, usize),
    direction: Direction,
) -> bool {
    match direction {
        Direction::North | Direction::South => {
            try_push_part2_north_south(grid, box_position, direction, &mut HashSet::new())
        }
        Direction::East | Direction::West => {
            try_push_part2_east_west(grid, box_position, direction)
        }
    }
}

fn move_and_push_part2(
    grid: &mut Array2D<Part2Cell>,
    robot_position: &mut (usize, usize),
    direction: Direction,
) {
    let next_robot_position = move_once_in_direction(*robot_position, direction);
    match grid[next_robot_position] {
        Part2Cell::Empty => *robot_position = next_robot_position,
        Part2Cell::Wall => (),
        Part2Cell::LeftBox | Part2Cell::RightBox => {
            if try_push_part2(grid, *robot_position, direction) {
                *robot_position = next_robot_position;
            }
        }
    }
}

fn part1(input: PuzzleInput) -> usize {
    let PuzzleInput {
        mut grid,
        start_position: mut cur_position,
        directions,
    } = input;

    for direction in directions {
        move_and_push_part1(&mut grid, &mut cur_position, direction);
    }

    grid.enumerate_row_major()
        .filter(|&(_, &cell)| matches!(cell, Part1Cell::Box))
        .map(|(position, _)| gps(position))
        .sum()
}

fn part2(input: PuzzleInput) -> usize {
    let PuzzleInput {
        grid: part1_grid,
        start_position: mut cur_position,
        directions,
    } = input;

    let mut grid = to_part2_grid(&part1_grid);

    for direction in directions {
        move_and_push_part2(&mut grid, &mut cur_position, direction);
    }

    grid.enumerate_row_major()
        .filter(|&(_, &cell)| matches!(cell, Part2Cell::LeftBox))
        .map(|(position, _)| gps(position))
        .sum()
}

fn main() {
    let file_contents_as_str = include_str!("../input.txt");

    let input = PuzzleInput::parse_input(file_contents_as_str);

    println!("{}", part1(input.clone()));
    println!("{}", part2(input));
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use super::*;

    const TEST_INPUT_SMALL_PART_1: &str = "########\n\
                                           #..O.O.#\n\
                                           ##@.O..#\n\
                                           #...O..#\n\
                                           #.#.O..#\n\
                                           #...O..#\n\
                                           #......#\n\
                                           ########\n\n\
                                           <^^>>>vv<v>>v<<";

    const TEST_INPUT_BIG: &str = include_str!("../example.txt");

    #[test_case(TEST_INPUT_SMALL_PART_1 => 2028 ; "small example")]
    #[test_case(TEST_INPUT_BIG => 10092 ; "big example")]
    fn test_part1(input: &str) -> usize {
        let puzzle_input = PuzzleInput::parse_input(input);
        part1(puzzle_input)
    }

    #[test_case(TEST_INPUT_BIG => 9021 ; "big example")]
    fn test_part2(input: &str) -> usize {
        let puzzle_input = PuzzleInput::parse_input(input);
        part2(puzzle_input)
    }
}
