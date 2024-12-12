use std::collections::HashMap;

use array2d::Array2D;

use crate::{antenna::Antenna, grid_cell::GridCell};

pub struct CityMap(Array2D<GridCell>);

impl From<Array2D<GridCell>> for CityMap {
    fn from(value: Array2D<GridCell>) -> Self {
        CityMap(value)
    }
}

impl CityMap {
    pub fn antennae_iter(&self) -> impl Iterator<Item = Antenna> + '_ {
        self.0.enumerate_row_major().filter_map(|(position, cell)| {
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

    pub fn get_antenna_positions_by_freq(&self) -> HashMap<u8, Vec<(usize, usize)>> {
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

    pub fn num_rows(&self) -> usize {
        self.0.num_rows()
    }

    pub fn num_columns(&self) -> usize {
        self.0.num_columns()
    }
}
