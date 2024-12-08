mod guard_iter;
mod util;

use std::collections::HashSet;

use array2d::Array2D;
use guard_iter::GuardIterator;
use util::{MapCell, PuzzleInput};

fn part1(input: &PuzzleInput) -> usize {
    let visited_positions: HashSet<(usize, usize)> = input
        .guard_iter()
        .map(|guard_state| guard_state.position)
        .collect();

    visited_positions.len()
}

fn causes_loop_when_obstacle(
    map_grid: &mut Array2D<MapCell>,
    start_pos: (usize, usize),
    position: (usize, usize),
) -> bool {
    let prev = map_grid[position];
    map_grid[position] = MapCell::Obstacle;

    let result = GuardIterator::new(&map_grid, start_pos).is_infinite();

    map_grid[position] = prev;

    result
}

fn part2(input: &PuzzleInput) -> usize {
    let visited_positions: HashSet<(usize, usize)> = input
        .guard_iter()
        .map(|guard_state| guard_state.position)
        .collect();

    let mut map_grid_clone = input.map_grid.clone();

    visited_positions
        .into_iter()
        .filter(|&pos| {
            (pos != input.start_pos)
                && causes_loop_when_obstacle(&mut map_grid_clone, input.start_pos, pos)
        })
        .count()
}

fn main() {
    let file_contents = std::fs::read("input.txt").unwrap();
    let file_contents_as_str = std::str::from_utf8(&file_contents).unwrap();

    let input = PuzzleInput::from(file_contents_as_str);

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "....#.....\n\
                              .........#\n\
                              ..........\n\
                              ..#.......\n\
                              .......#..\n\
                              ..........\n\
                              .#..^.....\n\
                              ........#.\n\
                              #.........\n\
                              ......#...";

    #[test]
    fn test_part1() {
        let input = PuzzleInput::from(TEST_INPUT);
        assert_eq!(41, part1(&input))
    }

    #[test]
    fn test_part2() {
        let input = PuzzleInput::from(TEST_INPUT);
        assert_eq!(6, part2(&input))
    }
}
