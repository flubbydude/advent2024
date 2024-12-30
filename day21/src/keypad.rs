use std::collections::HashSet;

use array2d::Array2D;
use enum_iterator::all;

use crate::direction::{move_once_bounded, Direction};

type Path = Vec<Direction>;

pub struct KeyPadShortestPaths {
    shortest_paths: Array2D<Array2D<Vec<Path>>>,
}

impl KeyPadShortestPaths {
    pub fn new_from_keypad<T>(keypad: &Array2D<Option<T>>) -> Self {
        let successors = |position| {
            all::<Direction>().filter_map(move |direction| {
                move_once_bounded(position, direction, keypad.num_rows(), keypad.num_columns())
                    .filter(|&new_position| keypad[new_position].is_some())
                    .map(|new_position| (direction, new_position))
            })
        };

        // shortest_paths[to][from] = [Path1, Path2, ...]
        let mut shortest_paths: Array2D<Array2D<Vec<Path>>> = Array2D::filled_by_row_major(
            || Array2D::filled_with(Vec::new(), keypad.num_rows(), keypad.num_columns()),
            keypad.num_rows(),
            keypad.num_columns(),
        );

        let present_positions = keypad
            .enumerate_row_major()
            .filter(|(_, maybe_key)| maybe_key.is_some())
            .map(|(position, _)| position)
            .collect::<Vec<_>>();

        for &present_position in present_positions.iter() {
            shortest_paths[present_position][present_position].push(vec![]);
        }

        let mut next_frontier = present_positions
            .into_iter()
            .map(|position| (position, position))
            .collect::<HashSet<_>>();

        let mut prev_length = 0;

        while !next_frontier.is_empty() {
            let frontier = next_frontier;
            next_frontier = HashSet::new();

            for (source, prev_target) in frontier {
                for (direction, target) in successors(prev_target) {
                    // if already did the source to target in a previous iteration
                    if shortest_paths[source][target]
                        .first()
                        .is_some_and(|path| path.len() <= prev_length)
                    {
                        continue;
                    }

                    next_frontier.insert((source, target));

                    let paths_to_add = shortest_paths[source][prev_target]
                        .iter()
                        .map(|path| {
                            let mut path_clone = path.clone();
                            path_clone.push(direction);
                            path_clone
                        })
                        .collect::<Vec<_>>();

                    shortest_paths[source][target].extend(paths_to_add);
                }
            }

            prev_length += 1;
        }

        KeyPadShortestPaths { shortest_paths }
    }

    pub fn shortest_paths_between(
        &self,
        source: (usize, usize),
        target: (usize, usize),
    ) -> &[Path] {
        let result = &self.shortest_paths[source][target];
        assert!(!result.is_empty());
        result
    }
}

#[cfg(test)]
mod tests {
    use crate::instruction::Instruction;

    use super::*;

    #[test]
    fn test_shortest_paths() {
        let keypad = Array2D::from_rows(&[
            vec![
                None,
                Some(Direction::North.into()),
                Some(Instruction::Activate),
            ],
            vec![
                Some(Direction::West.into()),
                Some(Direction::South.into()),
                Some(Direction::East.into()),
            ],
        ])
        .unwrap();

        let shortest_paths = KeyPadShortestPaths::new_from_keypad(&keypad);

        let shortest_paths_east_to_north = shortest_paths.shortest_paths_between((1, 2), (0, 1));
        assert_eq!(shortest_paths_east_to_north.len(), 2);
        assert!(shortest_paths_east_to_north.contains(&vec![Direction::North, Direction::West]));
        assert!(shortest_paths_east_to_north.contains(&vec![Direction::West, Direction::North]));
    }
}
