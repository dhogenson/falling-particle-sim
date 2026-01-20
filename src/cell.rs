use crate::color::*;

pub const EMPTY_CELL: u8 = 0;
pub const SAND_CELL: u8 = 1;
pub const CLAY_CELL: u8 = 2;
pub const WATER_CELL: u8 = 3;
pub const WET_SAND_CELL: u8 = 4;

#[derive(Debug, Clone, Copy)]
pub struct Cell {
    pub cell_color: [f32; 4],
    pub cell_type: u8,
}

impl Cell {
    pub fn new_empty() -> Self {
        Self {
            cell_color: WHITE_COLOR,
            cell_type: EMPTY_CELL,
        }
    }

    pub fn new_sand() -> Self {
        Self {
            cell_color: random_color(SAND_COLOR),
            cell_type: SAND_CELL,
        }
    }

    pub fn new_clay() -> Self {
        Self {
            cell_color: random_color(CLAY_COLOR),
            cell_type: CLAY_CELL,
        }
    }

    pub fn new_water() -> Self {
        Self {
            cell_color: random_color(WATER_COLOR),
            cell_type: WATER_CELL,
        }
    }

    pub fn new_wet_sand() -> Self {
        Self {
            cell_color: random_color(WET_SAND_COLOR),
            cell_type: WET_SAND_CELL,
        }
    }
}
