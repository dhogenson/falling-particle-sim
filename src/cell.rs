use crate::color::*;

#[derive(Debug, Clone, Copy)]
pub struct Cell {
    pub cell_color: [f32; 4],
    pub cell_type: u8,
}

impl Cell {
    pub fn new_empty() -> Self {
        Self {
            cell_color: WHITE,
            cell_type: 0,
        }
    }

    pub fn new_sand() -> Self {
        Self {
            cell_color: random_color(SAND),
            cell_type: 1,
        }
    }

    pub fn new_clay() -> Self {
        Self {
            cell_color: random_color(CLAY),
            cell_type: 2,
        }
    }

    pub fn new_water() -> Self {
        Self {
            cell_color: random_color(WATER),
            cell_type: 3,
        }
    }
}
