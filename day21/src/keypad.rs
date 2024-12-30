use std::collections::{HashMap, HashSet};

use array2d::Array2D;
use enum_iterator::all;

use crate::direction::{move_once_bounded, Direction};

type Position = (usize, usize);
type Path = Vec<Direction>;

fn keypad_as_bool_array2d<T>(keypad: &Array2D<Option<T>>) -> Array2D<bool> {
    Array2D::from_iter_row_major(
        keypad.elements_row_major_iter().map(Option::is_some),
        keypad.num_rows(),
        keypad.num_columns(),
    )
    .unwrap()
}

// TODO: When building use HashMap<Position, Rc<RefCell<HashMap<Position, Vec<Path>>>> lol
// then can output either HashMap<(&'a T, &'a T), Box<[Box[Direction]>]>,
// or HashMap<(Position, Position), Vec<Path>>

// flip i just realized better to use Array2D<Position, Array2D<Position, Vec<Path>>> lol
pub struct KeyPadShortestPaths {
    shortest_paths: HashMap<Position, HashMap<Position, Vec<Path>>>,
}

impl KeyPadShortestPaths {
    pub fn new_from_keypad<T>(keypad: &Array2D<Option<T>>) -> Self {
        Self::new_from_bool_array2d(&keypad_as_bool_array2d(keypad))
    }

    fn new_from_bool_array2d(keypad: &Array2D<bool>) -> Self {
        let present_positions = keypad
            .enumerate_row_major()
            .filter_map(|(position, &is_present)| if is_present { Some(position) } else { None })
            .collect::<Vec<_>>();

        let successors = |&position| {
            all::<Direction>().filter_map(move |direction| {
                move_once_bounded(position, direction, keypad.num_rows(), keypad.num_columns())
                    .map(|new_position| (direction, new_position))
            })
        };

        // shortest_paths[to][from] = [Path1, Path2, ...]
        let mut shortest_paths: HashMap<Position, HashMap<Position, Vec<Path>>> = present_positions
            .iter()
            .map(|&position| (position, HashMap::from([(position, vec![vec![]])])))
            .collect();

        for prev_length in 0.. {
            let mut changed = false;

            for target in present_positions.iter() {
                let explored_sources_for_target = shortest_paths[target]
                    .keys()
                    .copied()
                    .collect::<HashSet<_>>();
                for (direction, neighbor) in successors(target) {
                    if !keypad[neighbor] {
                        continue;
                    }

                    dbg!(direction, neighbor);

                    let new_shortest_paths_to_target = shortest_paths[&neighbor]
                        .iter()
                        .filter(|&(source, _)| !explored_sources_for_target.contains(source))
                        .filter_map(|(&source, shortest_paths_to_neighbor)| {
                            let new_paths = shortest_paths_to_neighbor
                                .iter()
                                .filter(|&path| path.len() == prev_length)
                                .map(|path| {
                                    let mut path_clone = path.clone();
                                    path_clone.push(direction.opposite());
                                    path_clone
                                })
                                .collect::<Vec<_>>();

                            if new_paths.is_empty() {
                                None
                            } else {
                                Some((source, new_paths))
                            }
                        })
                        .collect::<Vec<_>>();

                    for (source, new_shortest_paths) in new_shortest_paths_to_target {
                        changed = true;
                        shortest_paths
                            .get_mut(target)
                            .unwrap()
                            .entry(source)
                            .or_default()
                            .extend(new_shortest_paths);
                    }
                }
            }

            if !changed {
                break;
            }
        }

        KeyPadShortestPaths { shortest_paths }
    }

    pub fn shortest_paths_between(&self, from: &Position, to: &Position) -> Option<&[Path]> {
        self.shortest_paths.get(to)?.get(from).map(|v| &**v)
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

        let x = KeyPadShortestPaths::new_from_keypad(&keypad);

        dbg!(x.shortest_paths);
    }
}
