mod direction;
mod grid_cell;
mod puzzle_input;
mod search;
mod state;

use std::collections::HashSet;

use grid_cell::GridCell;
use smallvec::{smallvec, SmallVec};

use direction::Direction;
use puzzle_input::PuzzleInput;
use search::{best_cost_djikstra, best_paths};
use state::ReindeerState;

fn part1(input: &PuzzleInput) -> Option<u64> {
    let start_state = ReindeerState::new(input.start_position, Direction::East);

    let successors = |state: &ReindeerState| {
        let moved_forward = state.moved_forward_one();
        let result: SmallVec<[_; 3]> = if input.grid[moved_forward.position()] == GridCell::Empty {
            smallvec![
                (1, moved_forward),
                (1000, state.turned_ccw()),
                (1000, state.turned_cw())
            ]
        } else {
            smallvec![(1000, state.turned_ccw()), (1000, state.turned_cw())]
        };
        result.into_iter()
    };

    let is_goal = |state: &ReindeerState| state.position() == input.end_position;

    best_cost_djikstra([(0, start_state)], successors, is_goal)
}

fn part2(input: &PuzzleInput) -> Option<usize> {
    let start_state = ReindeerState::new(input.start_position, Direction::East);

    // for part 2: note we need to move forward first
    // to prune branches faster, since not using a prio queue
    let successors = |state: &ReindeerState| {
        let moved_forward = state.moved_forward_one();
        let result: SmallVec<[_; 3]> = if input.grid[moved_forward.position()] == GridCell::Empty {
            smallvec![
                (1, moved_forward),
                (1000, state.turned_ccw()),
                (1000, state.turned_cw())
            ]
        } else {
            smallvec![(1000, state.turned_ccw()), (1000, state.turned_cw())]
        };
        result.into_iter()
    };

    let is_goal = |state: &ReindeerState| state.position() == input.end_position;

    let best_path_cost = best_cost_djikstra([(0, start_state.clone())], successors, is_goal)?;

    let best_paths = best_paths(
        [(0, start_state.clone())],
        successors,
        is_goal,
        best_path_cost,
    );

    best_paths
        .into_iter()
        .flatten()
        .map(|s| s.position())
        .collect::<HashSet<_>>()
        .len()
        .into()
}

fn main() -> Result<(), puzzle_input::Error> {
    let file_contents_as_str = include_str!("../input.txt");

    let input = PuzzleInput::parse_from_input(file_contents_as_str)?;

    println!("{:?}", part1(&input));
    println!("{:?}", part2(&input));

    Ok(())
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use super::*;

    const TEST_INPUT_SMALL: &str = "###############\n\
                                    #.......#....E#\n\
                                    #.#.###.#.###.#\n\
                                    #.....#.#...#.#\n\
                                    #.###.#####.#.#\n\
                                    #.#.#.......#.#\n\
                                    #.#.#####.###.#\n\
                                    #...........#.#\n\
                                    ###.#.#####.#.#\n\
                                    #...#.....#.#.#\n\
                                    #.#.#.###.#.#.#\n\
                                    #.....#...#.#.#\n\
                                    #.###.#.#.#.#.#\n\
                                    #S..#.....#...#\n\
                                    ###############";

    const TEST_INPUT_BIG: &str = "#################\n\
                                  #...#...#...#..E#\n\
                                  #.#.#.#.#.#.#.#.#\n\
                                  #.#.#.#...#...#.#\n\
                                  #.#.#.#.###.#.#.#\n\
                                  #...#.#.#.....#.#\n\
                                  #.#.#.#.#.#####.#\n\
                                  #.#...#.#.#.....#\n\
                                  #.#.#####.#.###.#\n\
                                  #.#.#.......#...#\n\
                                  #.#.###.#####.###\n\
                                  #.#.#...#.....#.#\n\
                                  #.#.#.#####.###.#\n\
                                  #.#.#.........#.#\n\
                                  #.#.#.#########.#\n\
                                  #S#.............#\n\
                                  #################";

    #[test_case(TEST_INPUT_SMALL => Some(7036) ; "small example")]
    #[test_case(TEST_INPUT_BIG => Some(11048) ; "big example")]
    fn test_part1(input_str: &str) -> Option<u64> {
        let puzzle_input = PuzzleInput::parse_from_input(input_str).unwrap();
        part1(&puzzle_input)
    }

    #[test_case(TEST_INPUT_SMALL => Some(45) ; "small example")]
    #[test_case(TEST_INPUT_BIG => Some(64) ; "big example")]
    fn test_part2(input_str: &str) -> Option<usize> {
        let puzzle_input = PuzzleInput::parse_from_input(input_str).unwrap();
        part2(&puzzle_input)
    }
}
