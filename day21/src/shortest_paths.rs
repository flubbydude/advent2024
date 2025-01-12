use std::{
    array,
    iter::{once, repeat_n, Flatten, RepeatN},
};

use crate::direction::{move_once_checked, Direction};

fn shortest_path_zero_turns(
    start: (usize, usize),
    target: (usize, usize),
) -> Option<RepeatN<Direction>> {
    if start == target {
        Some(repeat_n(Direction::North, 0))
    } else if start.0 == target.0 {
        if target.1 > start.1 {
            Some(repeat_n(Direction::East, target.1 - start.1))
        } else {
            Some(repeat_n(Direction::West, start.1 - target.1))
        }
    } else if start.1 == target.1 {
        if start.0 > target.0 {
            Some(repeat_n(Direction::North, start.0 - target.0))
        } else {
            Some(repeat_n(Direction::South, target.0 - start.0))
        }
    } else {
        None
    }
}

pub enum BestPaths {
    One(Vec<Direction>), // start and target share row or col
    Two(Vec<Direction>, Vec<Direction>),
}

impl BestPaths {
    pub fn with_at_most_one_turn(start: (usize, usize), target: (usize, usize)) -> Self {
        let straight_line = shortest_path_zero_turns(start, target);
        if let Some(p) = straight_line {
            return BestPaths::One(p.collect());
        }

        let helper = |midstop| {
            shortest_path_zero_turns(start, midstop)
                .unwrap()
                .chain(shortest_path_zero_turns(midstop, target).unwrap())
                .collect()
        };

        BestPaths::Two(helper((start.0, target.1)), helper((target.0, start.1)))
    }
}

pub fn iter_positions(
    path: &[Direction],
    mut start: (usize, usize),
) -> impl '_ + IntoIterator<Item = (usize, usize)> {
    once(start).chain(path.iter().map(move |&d| {
        start = move_once_checked(start, d).unwrap();
        start
    }))
}

impl IntoIterator for BestPaths {
    type Item = Vec<Direction>;
    type IntoIter = Flatten<array::IntoIter<Option<Self::Item>, 2>>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            BestPaths::One(vec) => [Some(vec), None],
            BestPaths::Two(vec, vec1) => [Some(vec), Some(vec1)],
        }
        .into_iter()
        .flatten()
    }
}
