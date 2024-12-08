use std::collections::HashSet;

use super::util::{Direction, MapCell, PositionExt};
use array2d::Array2D;

pub struct GuardIterator<'a> {
    map_grid: &'a Array2D<MapCell>,
    position: Option<(usize, usize)>,
    direction: Direction,
}

impl<'a> GuardIterator<'a> {
    pub fn new(map_grid: &'a Array2D<MapCell>, start_position: (usize, usize)) -> Self {
        GuardIterator {
            map_grid: &map_grid,
            position: Some(start_position),
            direction: Direction::North,
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct GuardState {
    pub position: (usize, usize),
    pub direction: Direction,
}

impl Iterator for GuardIterator<'_> {
    type Item = GuardState;

    fn next(&mut self) -> Option<Self::Item> {
        let Some(cur_pos) = self.position else {
            return None;
        };

        let result = Some(GuardState {
            position: cur_pos,
            direction: self.direction,
        });

        let in_front = match cur_pos.bounded_move_in_direction(
            self.direction,
            self.map_grid.num_rows(),
            self.map_grid.num_columns(),
        ) {
            Some(pos) => pos,
            None => {
                self.position = None;
                return result;
            }
        };

        if self.map_grid[in_front] == MapCell::Obstacle {
            self.direction = self.direction.turn_cw();
        } else {
            self.position = Some(in_front);
        }

        result
    }
}

impl GuardIterator<'_> {
    pub fn is_infinite(self) -> bool {
        let mut visited_states: HashSet<GuardState> = HashSet::new();

        for state in self {
            if visited_states.contains(&state) {
                return true;
            }
            visited_states.insert(state);
        }

        false
    }
}
