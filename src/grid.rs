use crate::cell::*;
use rand;
use rand::seq::SliceRandom;
use std::io::{self, Write};

pub struct Grid {
    pub width: i64,
    pub height: i64,
    pub grid: Vec<Vec<Cell>>,
    processed: Vec<Vec<bool>>,
}

impl Grid {
    // Helper function
    pub fn new(width: i64, height: i64) -> Self {
        Self {
            width,
            height,
            grid: Self::make_grid(width, height),
            processed: vec![vec![false; width as usize]; height as usize],
        }
    }

    // Retunes a grid
    fn make_grid(size_x: i64, size_y: i64) -> Vec<Vec<Cell>> {
        let mut grid: Vec<Vec<Cell>> = Vec::new();

        for y in 0..size_y {
            grid.push(Vec::new());

            for _x in 0..size_x {
                grid[y as usize].push(Cell::new_empty())
            }
        }

        grid
    }

    // Places a element in a circle based of the cords you want
    pub fn place_element(&mut self, x: i32, y: i32, selected_element: u8, brush_size: i32) {
        let positions = self.get_circle_positions(x, y, brush_size);

        for (xp, yp) in positions {
            if self.grid[yp as usize][xp as usize].cell_type != 0 && selected_element != 0 {
                continue;
            }

            self.grid[yp as usize][xp as usize] = match selected_element {
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
        // Clear processed flags
        for row in &mut self.processed {
            for cell in row {
                *cell = false;
            }
        }

        // Track all cells
        let mut sand_count: u32 = 0;
        let mut water_count: u32 = 0;
        let mut wet_sand_count: u32 = 0;
        let mut fire_count: u32 = 0;
        let mut smoke_count: u32 = 0;
        let mut steam_count: u32 = 0;

        for y in 0..self.height {
            for x in 0..self.width {
                if self.processed[y as usize][x as usize] {
                    continue;
                }
                let cell_type = self.grid[y as usize][x as usize].cell_type;
                match cell_type {
                    SAND_CELL => {
                        self.update_sand(x, y);
                        sand_count += 1;
                    }
                    WATER_CELL => {
                        self.update_water(x, y);
                        water_count += 1;
                    }
                    WET_SAND_CELL => {
                        self.update_wet_sand(x, y);
                        wet_sand_count += 1;
                    }
                    FIRE_CELL => {
                        self.update_fire(x, y);
                        fire_count += 1
                    }
                    SMOKE_CELL => {
                        self.update_smoke(x, y);
                        smoke_count += 1;
                    }
                    STEAM_CELL => {
                        self.update_steam(x, y);
                        steam_count += 1;
                    }
                    _ => {}
                }
            }
        }
        print!(
            "\rSAND: {}, WATER: {}, WET_SAND: {} FIRE: {} SMOKE: {} STEAM: {}",
            sand_count, water_count, wet_sand_count, fire_count, smoke_count, steam_count
        );
        io::stdout().flush().unwrap();
    }

    // Rules of sand
    // 1) It first tryes to move down
    // 2) Then diagonally left or right
    pub fn update_sand(&mut self, x: i64, y: i64) {
        let targets = [(x, y + 1), (x - 1, y + 1), (x + 1, y + 1)];

        for (tx, ty) in targets {
            match self
                .grid
                .get(ty as usize)
                .and_then(|row| row.get(tx as usize))
                .map(|cell| cell.cell_type)
            {
                Some(EMPTY_CELL) => {
                    self.move_particle(x, y, tx, ty);
                    return; // We dont want it to make more than one move a tick
                }
                _ => {}
            }
        }
    }

    // Rules of water
    // 1) Its first goal is to move down, if it can it will if it cant it wont
    // 2) Then it tries to move diagonally to try and move down
    // 3) It will try to move left and right
    pub fn update_water(&mut self, x: i64, y: i64) {
        self.water_to_wet_sand(x, y);

        // Priority 1: Fall straight down
        if self.try_move_water(x, y, x, y + 1) {
            return;
        }

        // Priority 2: Fall diagonally (randomize left/right)
        let mut diagonals = [(x - 1, y + 1), (x + 1, y + 1)];
        diagonals.shuffle(&mut rand::rng());
        for (tx, ty) in diagonals {
            if self.try_move_water(x, y, tx, ty) {
                return;
            }
        }

        // Priority 3: Spread horizontally
        let mut horizontals = [(x - 1, y), (x + 1, y)];
        horizontals.shuffle(&mut rand::rng());
        for (tx, ty) in horizontals {
            if self.try_move_water(x, y, tx, ty) {
                return;
            }
        }
    }

    // Gets all positions in the shape of a box that is in bound of the grid
    fn get_square_area(&self, x: i64, y: i64) -> Vec<(i64, i64)> {
        let mut cells: Vec<(i64, i64)> = Vec::new();

        for cy in -1..2 {
            for cx in -1..2 {
                if cx + x >= 0 && cy + y >= 0 && cx + x < self.width && cy + y < self.height {
                    cells.push((cx + x, cy + y));
                }
            }
        }

        cells
    }

    // Gets the cells of the at the water and then if they are sand it turns it into wet sand
    fn water_to_wet_sand(&mut self, x: i64, y: i64) {
        let cells = self.get_square_area(x, y);

        for (x, y) in cells {
            if self.grid[y as usize][x as usize].cell_type == SAND_CELL {
                self.grid[y as usize][x as usize] = Cell::new_wet_sand();
            }
        }
    }

    // tx: Target X
    // ty: Target Y
    // Tryes to move the water with the rules of moving water, if it cant move it will return false
    // if it can move it will return true
    fn try_move_water(&mut self, x: i64, y: i64, tx: i64, ty: i64) -> bool {
        if tx < 0 || ty < 0 {
            return false;
        }
        if self
            .grid
            .get(ty as usize)
            .and_then(|row| row.get(tx as usize))
            .map(|cell| cell.cell_type)
            == Some(0)
        {
            self.move_particle(x, y, tx, ty)
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

        match self
            .grid
            .get(ty as usize)
            .and_then(|row| row.get(tx as usize))
            .map(|cell| cell.cell_type)
        {
            Some(EMPTY_CELL) => self.move_particle(x, y, tx, ty),
            Some(WATER_CELL) => self.swap_particle(x, y, tx, ty),
            _ => {}
        }
    }

    // tx: Target X
    // ty: Target Y
    // Swaps a particle from position to taget position
    fn swap_particle(&mut self, x: i64, y: i64, tx: i64, ty: i64) {
        let cell_buffer = self.grid[ty as usize][tx as usize];
        self.grid[ty as usize][tx as usize] = self.grid[y as usize][x as usize];
        self.grid[y as usize][x as usize] = cell_buffer;
        self.processed[ty as usize][tx as usize] = true;
    }

    // tx: Target X
    // ty: Target Y
    // Moves a particle to target position
    // Note: Replaces the x and y position with a empty cell
    fn move_particle(&mut self, x: i64, y: i64, tx: i64, ty: i64) {
        self.grid[ty as usize][tx as usize] = self.grid[y as usize][x as usize];
        self.grid[y as usize][x as usize] = Cell::new_empty();
        self.processed[ty as usize][tx as usize] = true;
    }

    // Updates life time for a cell
    // If cell has lived the amount of its max life time it dies
    fn update_life_time(&mut self, x: i64, y: i64) {
        self.grid[y as usize][x as usize].life_time += 1;
        if self.grid[y as usize][x as usize].life_time
            >= self.grid[y as usize][x as usize].max_life_time
        {
            self.grid[y as usize][x as usize] = Cell::new_empty();
            return;
        }
    }

    // Updates cell based of grass partible rules
    fn update_fire(&mut self, x: i64, y: i64) {
        self.update_life_time(x, y);

        self.fire_to_glass(x, y);
        self.fire_make_smoke(x, y);
        self.fire_to_steam(x, y);

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

    // Turns any cells that are sand cells into glass at a givin point
    fn fire_to_glass(&mut self, x: i64, y: i64) {
        let cells: Vec<(i64, i64)> = self.get_square_area(x, y);

        for (x, y) in cells {
            if self.grid[y as usize][x as usize].cell_type == SAND_CELL {
                self.grid[y as usize][x as usize] = Cell::new_glass();
            }
        }
    }

    // Has a random chance to make smoke at a giving point
    fn fire_make_smoke(&mut self, x: i64, y: i64) {
        let random_number = rand::random_range(0..100);
        if random_number > 98 {
            let cells: Vec<(i64, i64)> = self.get_square_area(x, y);

            if !cells.is_empty() {
                let random_cell = cells[rand::random_range(0..cells.len())];
                let (sx, sy) = random_cell;

                self.grid[sy as usize][sx as usize] = Cell::new_smoke();
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
        if self
            .grid
            .get(ty as usize)
            .and_then(|row| row.get(tx as usize))
            .map(|cell| cell.cell_type)
            == Some(0)
        {
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
            if tx < 0 || ty < 0 || tx >= self.width || ty >= self.height {
                continue;
            }
            match self
                .grid
                .get(ty as usize)
                .and_then(|row| row.get(tx as usize))
                .map(|cell| cell.cell_type)
            {
                Some(EMPTY_CELL) => {
                    self.move_particle(x, y, tx, ty);
                    return;
                }
                _ => {}
            }
        }

        let mut targets = [(x - 1, y), (x + 1, y)];
        targets.shuffle(&mut rand::rng());

        for (tx, ty) in targets {
            if tx < 0 || ty < 0 || tx >= self.width || ty >= self.height {
                continue;
            }
            match self
                .grid
                .get(ty as usize)
                .and_then(|row| row.get(tx as usize))
                .map(|cell| cell.cell_type)
            {
                Some(EMPTY_CELL) => {
                    self.move_particle(x, y, tx, ty);
                    return;
                }
                _ => {}
            }
        }
    }

    // If theres any water around the giving cords it will turn into steam
    fn fire_to_steam(&mut self, x: i64, y: i64) {
        let cells: Vec<(i64, i64)> = self.get_square_area(x, y);

        for (x, y) in cells {
            if self.grid[y as usize][x as usize].cell_type == WATER_CELL {
                self.grid[y as usize][x as usize] = Cell::new_steam();
            }
        }
    }
}
