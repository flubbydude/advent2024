use std::{collections::HashMap, hash::Hash};

use array2d::Array2D;
use enum_iterator::all;

use crate::direction::{self, move_once_bounded, Direction};

type Position = (usize, usize);
type Path = Vec<Direction>;

pub struct KeyPadShortestPaths<'a, T: Hash + Eq> {
    shortest_paths: HashMap<(&'a T, &'a T), Vec<Path>>,
}

fn enumerate_present_keys<T>(
    keypad: &Array2D<Option<T>>,
) -> impl DoubleEndedIterator<Item = (Position, &T)> + Clone {
    keypad
        .enumerate_row_major()
        .filter_map(|(position, maybe_key)| maybe_key.as_ref().map(|key| (position, key)))
}

// fn iter_present_keys<T>(
//     keypad: &Array2D<Option<T>>,
// ) -> impl DoubleEndedIterator<Item = &T> + Clone {
//     keypad
//         .elements_row_major_iter()
//         .filter_map(|maybe_key| maybe_key.as_ref())
// }

fn get_neighboring_keys<T>(
    keypad: &Array2D<Option<T>>,
    position: (usize, usize),
) -> impl Iterator<Item = (Direction, &T)> {
    all::<Direction>().filter_map(move |direction| {
        move_once_bounded(position, direction, keypad.num_rows(), keypad.num_columns())
            .and_then(|next_position| (direction, keypad[next_position].as_ref()?).into())
    })
}

impl<'a, T: Hash + Eq> KeyPadShortestPaths<'a, T> {
    pub fn new(keypad: &'a Array2D<Option<T>>) -> Self {
        let enumerated_keys = enumerate_present_keys(keypad).collect::<Vec<_>>();
        let keys_iter = || enumerated_keys.iter().map(|&(_, e)| e);

        let edges = enumerated_keys.iter().flat_map(|&(position, source)| {
            get_neighboring_keys(keypad, position)
                .map(move |(direction, neighbor)| (source, neighbor, direction))
        });

        let mut shortest_paths_to: HashMap<&T, HashMap<&T, Vec<Path>>> = HashMap::new();
        for (source, neighbor, direction) in edges {
            shortest_paths_to
                .entry(neighbor)
                .or_default()
                .entry(source)
                .or_default()
                .push(vec![direction])
        }

        loop {
            let mut changed = false;

            for &(position, target) in enumerated_keys.iter() {
                let shortest_paths_to_target = shortest_paths_to.get_mut(source);
                for target in keys_iter() {
                    // the shortest paths to target from any source
                    // are the shortest paths that aren't there
                    // to any neighbor from any source
                    // and append the direction from neighbor to the instructions
                }
            }

            if !changed {
                break;
            }
        }

        let shortest_paths = shortest_paths_to
            .into_iter()
            .flat_map(|(source, shortest_paths_from_source)| {
                shortest_paths_from_source
                    .into_iter()
                    .map(move |(target, paths)| ((source, target), paths))
            })
            .collect();

        KeyPadShortestPaths { shortest_paths }
    }

    pub fn shortest_paths_between<'b>(&'b self, from: &'a T, to: &'a T) -> Option<&'b [Path]> {
        self.shortest_paths.get(&(from, to)).map(|v| &**v)
    }
}
