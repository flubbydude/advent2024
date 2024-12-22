mod a_star;
mod direction;
mod grid_cell;
mod puzzle_input;
mod state;

use grid_cell::GridCell;
use smallvec::{smallvec, SmallVec};

use a_star::a_star;
use direction::Direction;
use puzzle_input::PuzzleInput;
use state::Part1State;

fn part1(input: &PuzzleInput) -> u64 {
    let start_state = Part1State::new(input.start_position, Direction::East);

    let successors = |state: &Part1State| {
        let mut result: SmallVec<[_; 3]> =
            smallvec![(1000, state.turned_ccw()), (1000, state.turned_cw())];
        let moved_forward = state.moved_forward_one();
        if input.grid[moved_forward.position()] == GridCell::Empty {
            result.push((1, moved_forward));
        }
        result.into_iter()
    };

    let is_goal = |state: &Part1State| state.position() == input.end_position;

    let heuristic = |state: &Part1State| {
        let (i, j) = state.position();
        let (ip, jp) = input.end_position;
        (i.abs_diff(ip) + j.abs_diff(jp)) as u64
    };

    a_star(start_state, successors, is_goal, heuristic)
        .map(|out| out.cost)
        .unwrap_or(u64::MAX)
}

fn main() -> Result<(), puzzle_input::Error> {
    let file_contents_as_str = include_str!("../input.txt");

    let input = PuzzleInput::parse_from_input(file_contents_as_str)?;

    println!("{}", part1(&input));

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

    #[test_case(TEST_INPUT_SMALL => 7036 ; "small example")]
    #[test_case(TEST_INPUT_BIG => 11048 ; "big example")]
    fn test_part1(input_str: &str) -> u64 {
        let puzzle_input = PuzzleInput::parse_from_input(input_str).unwrap();
        part1(&puzzle_input)
    }
}
