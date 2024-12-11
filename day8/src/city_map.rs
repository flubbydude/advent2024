use std::collections::HashMap;

use array2d::Array2D;

use crate::{antenna::Antenna, grid_cell::GridCell};

pub trait CityMap {
    fn antennae_iter(&self) -> impl Iterator<Item = Antenna> + '_;

    fn get_antenna_positions_by_freq(&self) -> HashMap<u8, Vec<(usize, usize)>> {
        let mut antenna_positions_by_freq: HashMap<u8, Vec<(usize, usize)>> = HashMap::new();
        for Antenna {
            frequency,
            position,
        } in self.antennae_iter()
        {
            antenna_positions_by_freq
                .entry(frequency)
                .or_default()
                .push(position);
        }

        antenna_positions_by_freq
    }
}

impl CityMap for Array2D<GridCell> {
    fn antennae_iter(&self) -> impl Iterator<Item = Antenna> {
        self.enumerate_row_major().filter_map(|(position, cell)| {
            if let &GridCell::Antenna { frequency } = cell {
                Some(Antenna {
                    frequency,
                    position,
                })
            } else {
                None
            }
        })
    }
}
