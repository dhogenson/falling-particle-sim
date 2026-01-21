use crate::color::*;

pub const EMPTY_CELL: u8 = 0;
pub const SAND_CELL: u8 = 1;
pub const CLAY_CELL: u8 = 2;
pub const WATER_CELL: u8 = 3;
pub const WET_SAND_CELL: u8 = 4;
pub const FIRE_CELL: u8 = 5;
pub const GLASS_CELL: u8 = 6;
pub const SMOKE_CELL: u8 = 7;
pub const STEAM_CELL: u8 = 8;

#[derive(Debug, Clone, Copy)]
pub struct Cell {
    pub cell_color: [f32; 4],
    pub cell_type: u8,
    pub max_life_time: u64,
    pub life_time: u64,
}

impl Cell {
    pub fn new_empty() -> Self {
        Self {
            cell_color: WHITE_COLOR,
            cell_type: EMPTY_CELL,
            max_life_time: 0,
            life_time: 0,
        }
    }

    pub fn new_sand() -> Self {
        Self {
            cell_color: random_color(SAND_COLOR),
            cell_type: SAND_CELL,
            max_life_time: 0,
            life_time: 0,
        }
    }

    pub fn new_clay() -> Self {
        Self {
            cell_color: random_color(CLAY_COLOR),
            cell_type: CLAY_CELL,
            max_life_time: 0,
            life_time: 0,
        }
    }

    pub fn new_water() -> Self {
        Self {
            cell_color: random_color(WATER_COLOR),
            cell_type: WATER_CELL,
            max_life_time: 0,
            life_time: 0,
        }
    }

    pub fn new_wet_sand() -> Self {
        Self {
            cell_color: random_color(WET_SAND_COLOR),
            cell_type: WET_SAND_CELL,
            max_life_time: 0,
            life_time: 0,
        }
    }

    pub fn new_fire() -> Self {
        Self {
            cell_color: random_color(FIRE_COLOR),
            cell_type: FIRE_CELL,
            max_life_time: 30,
            life_time: 0,
        }
    }

    pub fn new_glass() -> Self {
        Self {
            cell_color: random_color(GLASS_COLOR),
            cell_type: GLASS_CELL,
            max_life_time: 30,
            life_time: 0,
        }
    }

    pub fn new_smoke() -> Self {
        Self {
            cell_color: random_color(SMOKE_COLOR),
            cell_type: SMOKE_CELL,
            max_life_time: 400,
            life_time: 0,
        }
    }

    pub fn new_steam() -> Self {
        Self {
            cell_color: random_color(STEAM_COLOR),
            cell_type: STEAM_CELL,
            max_life_time: 200,
            life_time: 0,
        }
    }
}
