use crate::cell::*;
use rand;
use rand::seq::SliceRandom;
use std::{
    io::{self, Write},
    time::{Duration, Instant},
};

pub struct DebugInfo {
    pub sand_count: i64,
    pub water_count: i64,
    pub wet_sand_count: i64,
    pub fire_count: i64,
    pub smoke_count: i64,
    pub steam_count: i64,
    pub time_to_update_cells: Duration,
}

pub struct Grid {
    pub width: i64,
    pub height: i64,
    pub grid: Vec<Cell>,
    processed: Vec<bool>,
    pub temperature: Vec<i64>,
    pub debug_info: DebugInfo,
}

impl DebugInfo {
    pub fn new() -> Self {
        Self {
            sand_count: 0,
            water_count: 0,
            wet_sand_count: 0,
            fire_count: 0,
            smoke_count: 0,
            steam_count: 0,
            time_to_update_cells: Duration::from_secs(0),
        }
    }

    pub fn print_info(&self) {
        print!(
            "\rSAND: {}, WATER: {}, WET_SAND: {} FIRE: {} SMOKE: {} STEAM: {} Times since last frame {:?}",
            self.sand_count,
            self.water_count,
            self.wet_sand_count,
            self.fire_count,
            self.smoke_count,
            self.steam_count,
            self.time_to_update_cells
        );
        io::stdout().flush().unwrap();
    }

    pub fn reset(&mut self) {
        self.sand_count = 0;
        self.water_count = 0;
        self.wet_sand_count = 0;
        self.fire_count = 0;
        self.smoke_count = 0;
        self.steam_count = 0;
        self.time_to_update_cells = Duration::from_secs(0);
    }
}

impl Grid {
    // Helper function to convert 2D coordinates to 1D index
    #[inline]
    fn idx(&self, x: i64, y: i64) -> usize {
        (y * self.width + x) as usize
    }

    // Helper function
    pub fn new(width: i64, height: i64) -> Self {
        Self {
            width,
            height,
            grid: Self::make_grid(width, height),
            processed: vec![false; (width * height) as usize],
            temperature: vec![0; (width * height) as usize],
            debug_info: DebugInfo::new(),
        }
    }

    // Returns a grid
    fn make_grid(size_x: i64, size_y: i64) -> Vec<Cell> {
        vec![Cell::new_empty(); (size_x * size_y) as usize]
    }

    // Places a element in a circle based of the cords you want
    pub fn place_element(&mut self, x: i32, y: i32, selected_element: u8, brush_size: i32) {
        let positions = self.get_circle_positions(x, y, brush_size);

        for (xp, yp) in positions {
            let idx = self.idx(xp as i64, yp as i64);
            if self.grid[idx].cell_type != 0 && selected_element != 0 {
                continue;
            }

            self.grid[idx] = match selected_element {
                SAND_CELL => Cell::new_sand(),
                STEEL_CELL => Cell::new_steel(),
                WATER_CELL => Cell::new_water(),
                FIRE_CELL => Cell::new_fire(),
                EMPTY_CELL => Cell::new_empty(),
                _ => Cell::new_empty(),
            };
        }
    }

    // Get all list elements in a circle
    pub fn get_circle_positions(
        &self,
        center_x: i32,
        center_y: i32,
        radius: i32,
    ) -> Vec<(i32, i32)> {
        let mut positions = Vec::new();

        for dy in -radius..=radius {
            for dx in -radius..=radius {
                // Check if point is within circle: x² + y² ≤ r²
                if dx * dx + dy * dy <= radius * radius {
                    let x = center_x + dx;
                    let y = center_y + dy;

                    // Bounds check
                    if x >= 0 && y >= 0 && x < self.width as i32 && y < self.height as i32 {
                        positions.push((x, y));
                    }
                }
            }
        }

        positions
    }

    // Main update function for cells
    pub fn update(&mut self) {
        let start = Instant::now();
        // Clear processed flags
        self.processed.fill(false);

        // Clear temp
        self.temperature.fill(0);

        for y in 0..self.height {
            for x in 0..self.width {
                let idx = self.idx(x, y);
                if self.processed[idx] {
                    continue;
                }
                let cell_type = self.grid[idx].cell_type;
                match cell_type {
                    SAND_CELL => {
                        self.update_sand(x, y);
                        self.debug_info.sand_count += 1;
                    }
                    WATER_CELL => {
                        self.update_water(x, y);
                        self.debug_info.water_count += 1;
                    }
                    WET_SAND_CELL => {
                        self.update_wet_sand(x, y);
                        self.debug_info.wet_sand_count += 1;
                    }
                    FIRE_CELL => {
                        self.update_fire(x, y);
                        self.debug_info.fire_count += 1
                    }
                    SMOKE_CELL => {
                        self.update_smoke(x, y);
                        self.debug_info.smoke_count += 1;
                    }
                    STEAM_CELL => {
                        self.update_steam(x, y);
                        self.debug_info.steam_count += 1;
                    }
                    _ => {}
                }
            }
        }
        let duration = start.elapsed();
        self.debug_info.time_to_update_cells = duration;
        // self.debug_info.print_info();
    }

    // Rules of sand
    // 1) It first tryes to move down
    // 2) Then diagonally left or right
    pub fn update_sand(&mut self, x: i64, y: i64) {
        let idx = self.idx(x, y);

        if self.temperature[idx] >= SAND_MELTING {
            self.grid[idx] = Cell::new_glass();
            return;
        }
        let targets = [(x, y + 1), (x - 1, y + 1), (x + 1, y + 1)];

        for (tx, ty) in targets {
            if tx < 0 || ty < 0 || tx >= self.width || ty >= self.height {
                continue;
            }
            let idx = self.idx(tx, ty);
            if self.grid[idx].cell_type == EMPTY_CELL {
                self.move_particle(x, y, tx, ty);

                return; // We dont want it to make more than one move a tick
            }
        }
    }

    // Rules of water
    // 1) Its first goal is to move down, if it can it will if it cant it wont
    // 2) Then it tries to move diagonally to try and move down
    // 3) It will try to move left and right
    pub fn update_water(&mut self, x: i64, y: i64) {
        let idx = self.idx(x, y);

        if self.temperature[idx] > WATER_MELTING {
            self.grid[idx] = Cell::new_steam();
            return;
        }

        let mut rng = rand::rng();

        self.water_to_wet_sand(x, y);

        // Priority 1: Fall straight down
        if self.try_move_water(x, y, x, y + 1) {
            return;
        }

        // Priority 2: Fall diagonally (randomize left/right)
        let mut diagonals = [(x - 1, y + 1), (x + 1, y + 1)];
        diagonals.shuffle(&mut rng);
        for (tx, ty) in diagonals {
            if self.try_move_water(x, y, tx, ty) {
                return;
            }
        }

        // Priority 3: Spread horizontally
        let mut horizontals = [(x - 1, y), (x + 1, y)];
        horizontals.shuffle(&mut rng);
        for (tx, ty) in horizontals {
            if self.try_move_water(x, y, tx, ty) {
                return;
            }
        }
    }

    // Gets all positions in the shape of a box that is in bound of the grid
    fn get_square_area(&self, x: i64, y: i64) -> impl Iterator<Item = (i64, i64)> + '_ {
        let min_x = (x - 1).max(0);
        let max_x = (x + 1).min(self.width - 1);
        let min_y = (y - 1).max(0);
        let max_y = (y + 1).min(self.height - 1);

        (min_y..=max_y).flat_map(move |cy| (min_x..=max_x).map(move |cx| (cx, cy)))
    }

    // Gets the cells of the at the water and then if they are sand it turns it into wet sand
    fn water_to_wet_sand(&mut self, x: i64, y: i64) {
        let cells: Vec<_> = self.get_square_area(x, y).collect();

        for (x, y) in cells {
            let idx = self.idx(x, y);
            if self.grid[idx].cell_type == SAND_CELL {
                self.grid[idx] = Cell::new_wet_sand();
            }
        }
    }

    // tx: Target X
    // ty: Target Y
    // Tryes to move the water with the rules of moving water, if it cant move it will return false
    // if it can move it will return true
    fn try_move_water(&mut self, x: i64, y: i64, tx: i64, ty: i64) -> bool {
        if tx < 0 || ty < 0 || tx >= self.width || ty >= self.height {
            return false;
        }
        let idx = self.idx(tx, ty);
        if self.grid[idx].cell_type == EMPTY_CELL {
            self.move_particle(x, y, tx, ty);
            return true;
        }
        false
    }

    // Rules of sand
    // 1) It can only move down
    // 2) If the wet sand is below any water it will swap with its positions with the water
    // to make a sinking effect
    fn update_wet_sand(&mut self, x: i64, y: i64) {
        let tx = x;
        let ty = y + 1;

        if ty >= self.height {
            return;
        }

        let idx = self.idx(tx, ty);
        match self.grid[idx].cell_type {
            EMPTY_CELL => self.move_particle(x, y, tx, ty),
            WATER_CELL => self.swap_particle(x, y, tx, ty),
            _ => {}
        }
    }

    // tx: Target X
    // ty: Target Y
    // Swaps a particle from position to taget position
    fn swap_particle(&mut self, x: i64, y: i64, tx: i64, ty: i64) {
        let src_idx = self.idx(x, y);
        let dst_idx = self.idx(tx, ty);
        self.grid.swap(src_idx, dst_idx);
        self.processed[dst_idx] = true;
    }

    // tx: Target X
    // ty: Target Y
    // Moves a particle to target position
    // Note: Replaces the x and y position with a empty cell
    fn move_particle(&mut self, x: i64, y: i64, tx: i64, ty: i64) {
        let src_idx = self.idx(x, y);
        let dst_idx = self.idx(tx, ty);
        self.grid[dst_idx] = self.grid[src_idx];
        self.grid[src_idx] = Cell::new_empty();
        self.processed[dst_idx] = true;
    }

    // Updates life time for a cell
    // If cell has lived the amount of its max life time it dies
    fn update_life_time(&mut self, x: i64, y: i64) {
        let idx = self.idx(x, y);
        self.grid[idx].life_time += 1;
        if self.grid[idx].life_time >= self.grid[idx].max_life_time {
            self.grid[idx] = Cell::new_empty();
            return;
        }
    }

    fn update_temperature(&mut self, x: i64, y: i64, temperature: i64) {
        let cells: Vec<_> = self.get_square_area(x, y).collect();

        // tx: Target X
        // ty: Target Y
        for (tx, ty) in cells {
            let idx = self.idx(tx, ty);
            self.temperature[idx] += temperature;
        }
    }

    // Updates cell based of grass partible rules
    fn update_fire(&mut self, x: i64, y: i64) {
        self.update_life_time(x, y);
        self.update_temperature(x, y, FIRE_TEMP);
        self.fire_make_smoke(x, y);

        let mut targets = [(x, y - 1), (x - 1, y - 1), (x + 1, y - 1)];
        targets.shuffle(&mut rand::rng());

        for (tx, ty) in targets {
            if !self.try_move_gass(x, y, tx, ty) {
                continue;
            }
        }

        let mut targets = [(x - 1, y), (x + 1, y)];
        targets.shuffle(&mut rand::rng());

        for (tx, ty) in targets {
            if !self.try_move_gass(x, y, tx, ty) {
                continue;
            }
        }
    }

    // Has a random chance to make smoke at a giving point
    fn fire_make_smoke(&mut self, x: i64, y: i64) {
        let random_number = rand::random_range(0..100);
        if random_number > 98 {
            let cells: Vec<_> = self.get_square_area(x, y).collect();

            if !cells.is_empty() {
                let random_cell = cells[rand::random_range(0..cells.len())];
                let (sx, sy) = random_cell;
                let idx = self.idx(sx, sy);

                self.grid[idx] = Cell::new_smoke();
            }
        }
    }

    // Moves smoke based of the gass particle rules
    //
    // First
    // 1) First it tries to move up or diagonally
    // 2) And then tries to move left or right
    fn update_smoke(&mut self, x: i64, y: i64) {
        self.update_life_time(x, y);

        // Priority 1: fall diagonally or upwards
        let mut targets = [(x, y - 1), (x - 1, y - 1), (x + 1, y - 1)];
        targets.shuffle(&mut rand::rng());

        for (tx, ty) in targets {
            if !self.try_move_gass(x, y, tx, ty) {
                continue;
            }
        }

        // Priority 2: move left/right

        let mut targets = [(x - 1, y), (x + 1, y)];
        targets.shuffle(&mut rand::rng());

        for (tx, ty) in targets {
            if !self.try_move_gass(x, y, tx, ty) {
                continue;
            }
        }
    }

    // tx: Target X
    // ty: Target Y
    // Trys to move a gass particle, if it cant it returns false, if it can it returns true
    fn try_move_gass(&mut self, x: i64, y: i64, tx: i64, ty: i64) -> bool {
        if tx < 0 || ty < 0 || tx >= self.width || ty >= self.height {
            return false;
        }
        let idx = self.idx(tx, ty);
        if self.grid[idx].cell_type == EMPTY_CELL {
            self.move_particle(x, y, tx, ty);
            return true;
        }
        false
    }

    // Moves the steam based of the gass particle rules
    fn update_steam(&mut self, x: i64, y: i64) {
        self.update_life_time(x, y);

        let mut targets = [(x, y - 1), (x - 1, y - 1), (x + 1, y - 1)];
        targets.shuffle(&mut rand::rng());

        for (tx, ty) in targets {
            if self.try_move_gass(x, y, tx, ty) {
                return;
            }
        }

        let mut targets = [(x - 1, y), (x + 1, y)];
        targets.shuffle(&mut rand::rng());

        for (tx, ty) in targets {
            if self.try_move_gass(x, y, tx, ty) {
                return;
            }
        }
    }
}
