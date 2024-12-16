mod direction;

use std::array;

use array2d::Array2D;

use direction::Direction;
use enum_iterator::{all, cardinality};

fn parse_input(input: &str) -> Array2D<u8> {
    let num_rows = input.lines().count();
    let num_columns = input.lines().next().unwrap().chars().count();

    Array2D::from_iter_row_major(
        input
            .lines()
            .flat_map(|line| line.as_bytes().iter().copied()),
        num_rows,
        num_columns,
    )
    .unwrap()
}

pub fn get_neighbor_in_direction(
    position: (usize, usize),
    direction: Direction,
    num_rows: usize,
    num_columns: usize,
) -> Option<(usize, usize)> {
    use Direction::*;
    match direction {
        North => position.0.checked_sub(1).map(|row| (row, position.1)),
        East => {
            if position.1 + 1 < num_columns {
                Some((position.0, position.1 + 1))
            } else {
                None
            }
        }
        South => {
            if position.0 + 1 < num_rows {
                Some((position.0 + 1, position.1))
            } else {
                None
            }
        }
        West => position.1.checked_sub(1).map(|column| (position.0, column)),
    }
}

fn get_neighbors(
    position: (usize, usize),
    num_rows: usize,
    num_columns: usize,
) -> impl Iterator<Item = (usize, usize)> {
    all::<Direction>().flat_map(move |direction| {
        get_neighbor_in_direction(position, direction, num_rows, num_columns).into_iter()
    })
}

fn part1(grid: &Array2D<u8>) -> usize {
    let mut seen = Array2D::filled_with(false, grid.num_rows(), grid.num_columns());
    let mut result = 0;

    for start in grid.indices_row_major() {
        if seen[start] {
            continue;
        }

        seen[start] = true;

        let region_plant = grid[start];
        let mut stack = vec![start];
        let mut area = 0;
        let mut perimeter = 0;

        while let Some(position) = stack.pop() {
            area += 1;
            perimeter += 4;

            for neighbor in get_neighbors(position, grid.num_rows(), grid.num_columns()) {
                if grid[neighbor] == region_plant {
                    perimeter -= 1;

                    if !seen[neighbor] {
                        seen[neighbor] = true;
                        stack.push(neighbor);
                    }
                }
            }
        }

        result += area * perimeter;
    }

    result
}

fn part2(grid: &Array2D<u8>) -> usize {
    let mut seen = Array2D::filled_with(false, grid.num_rows(), grid.num_columns());
    let mut seen_fence_in_direction: [Array2D<bool>; cardinality::<Direction>()] =
        array::from_fn(|_| Array2D::filled_with(false, grid.num_rows(), grid.num_columns()));
    let mut result = 0;

    let is_fence_in_direction = |position, direction| match get_neighbor_in_direction(
        position,
        direction,
        grid.num_rows(),
        grid.num_columns(),
    ) {
        Some(neighbor) => grid[neighbor] != grid[position],
        None => true,
    };

    for start in grid.indices_row_major() {
        if seen[start] {
            continue;
        }

        seen[start] = true;

        let region_plant = grid[start];
        let mut stack = vec![start];
        let mut area = 0;
        let mut num_sides = 0;

        while let Some(position) = stack.pop() {
            area += 1;

            for direction in all::<Direction>() {
                let seen_fence = &mut seen_fence_in_direction[direction as usize];
                if !seen_fence[position] && is_fence_in_direction(position, direction) {
                    num_sides += 1;
                    seen_fence[position] = true;
                    for perp_direction in [direction.turn_ccw(), direction.turn_cw()] {
                        let mut cur_position = position;

                        loop {
                            cur_position = match get_neighbor_in_direction(
                                cur_position,
                                perp_direction,
                                grid.num_rows(),
                                grid.num_columns(),
                            ) {
                                Some(pos) => pos,
                                None => break,
                            };

                            if grid[cur_position] == region_plant
                                && is_fence_in_direction(cur_position, direction)
                            {
                                seen_fence[cur_position] = true;
                            } else {
                                break;
                            }
                        }
                    }
                }
            }

            for neighbor in get_neighbors(position, grid.num_rows(), grid.num_columns()) {
                if grid[neighbor] == region_plant && !seen[neighbor] {
                    seen[neighbor] = true;
                    stack.push(neighbor);
                }
            }
        }

        result += area * num_sides;
    }

    result
}

fn main() {
    let file_contents_as_str = include_str!("../input.txt");

    let input = parse_input(file_contents_as_str);

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use super::*;

    const TEST_INPUT_SMALL: &str = "AAAA\n\
                                    BBCD\n\
                                    BBCC\n\
                                    EEEC";

    const TEST_INPUT_MEDIUM: &str = "OOOOO\n\
                                     OXOXO\n\
                                     OOOOO\n\
                                     OXOXO\n\
                                     OOOOO";

    const TEST_INPUT_BIG: &str = "RRRRIICCFF\n\
                                  RRRRIICCCF\n\
                                  VVRRRCCFFF\n\
                                  VVRCCCJFFF\n\
                                  VVVVCJJCFE\n\
                                  VVIVCCJJEE\n\
                                  VVIIICJJEE\n\
                                  MIIIIIJJEE\n\
                                  MIIISIJEEE\n\
                                  MMMISSJEEE";

    const TEST_INPUT_E: &str = "EEEEE\n\
                                EXXXX\n\
                                EEEEE\n\
                                EXXXX\n\
                                EEEEE";

    const TEST_INPUT_AB: &str = "AAAAAA\n\
                                 AAABBA\n\
                                 AAABBA\n\
                                 ABBAAA\n\
                                 ABBAAA\n\
                                 AAAAAA";

    #[test_case(TEST_INPUT_SMALL => 140 ; "small")]
    #[test_case(TEST_INPUT_MEDIUM => 772 ; "medium")]
    #[test_case(TEST_INPUT_BIG => 1930 ; "big")]
    fn test_part1(input_str: &str) -> usize {
        part1(&parse_input(input_str))
    }

    #[test_case(TEST_INPUT_SMALL => 80 ; "small")]
    #[test_case(TEST_INPUT_MEDIUM => 436 ; "medium")]
    #[test_case(TEST_INPUT_E => 236 ; "big e")]
    #[test_case(TEST_INPUT_AB => 368 ; "ab")]
    #[test_case(TEST_INPUT_BIG => 1206 ; "big")]
    fn test_part2(input_str: &str) -> usize {
        part2(&parse_input(input_str))
    }
}
