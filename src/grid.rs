use crate::cell::{Cell, EMPTY_CELL, SAND_CELL, WATER_CELL, WET_SAND_CELL};
use rand;
use rand::seq::SliceRandom;
use std::io::{self, Write};

pub struct Grid {
    pub width: u32,
    pub height: u32,
    pub grid: Vec<Vec<Cell>>,
    processed: Vec<Vec<bool>>,
}

impl Grid {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            grid: Self::make_grid(width, height),
            processed: vec![vec![false; width as usize]; height as usize],
        }
    }

    fn make_grid(size_x: u32, size_y: u32) -> Vec<Vec<Cell>> {
        let mut grid: Vec<Vec<Cell>> = Vec::new();

        for y in 0..size_y {
            grid.push(Vec::new());

            for _x in 0..size_x {
                grid[y as usize].push(Cell::new_empty())
            }
        }

        grid
    }

    pub fn place_element(&mut self, x: i32, y: i32, selected_element: u8) {
        let positions = self.get_circle_positions(x as i32, y as i32, 2);

        for (xp, yp) in positions {
            if self.grid[yp as usize][xp as usize].cell_type != 0 {
                continue;
            }

            self.grid[yp as usize][xp as usize] = match selected_element {
                1 => Cell::new_sand(),
                2 => Cell::new_clay(),
                3 => Cell::new_water(),
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

    pub fn update(&mut self) {
        // Clear processed flags
        for row in &mut self.processed {
            for cell in row {
                *cell = false;
            }
        }

        let mut sand_count: u32 = 0;
        let mut water_count: u32 = 0;
        let mut wet_sand_count: u32 = 0;

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
                    _ => {}
                }
            }
        }
        print!(
            "\rSAND: {}, WATER: {}, WET_SAND: {}",
            sand_count, water_count, wet_sand_count
        );
        io::stdout().flush().unwrap();
    }

    pub fn update_sand(&mut self, x: u32, y: u32) {
        let x = x as i32;
        let y = y as i32;
        let targets = [(x, y + 1), (x - 1, y + 1), (x + 1, y + 1)];

        for (tx, ty) in targets {
            if tx < 0 || ty < 0 {
                continue;
            }
            match self
                .grid
                .get(ty as usize)
                .and_then(|row| row.get(tx as usize))
                .map(|cell| cell.cell_type)
            {
                Some(EMPTY_CELL) => {
                    self.grid[ty as usize][tx as usize] = self.grid[y as usize][x as usize];
                    self.grid[y as usize][x as usize] = Cell::new_empty();
                    self.processed[ty as usize][tx as usize] = true;
                    // return;
                }
                _ => {}
            }
        }
    }

    pub fn update_water(&mut self, x: u32, y: u32) {
        let x = x as i32;
        let y = y as i32;

        self.water_to_wet_sand(x, y);

        // Priority 1: Fall straight down
        if self.try_move_water(x, y, x, y + 1) {
            // self.water_to_wet_sand(x, y - 1);
            return;
        }

        // Priority 3: Spread horizontally
        let mut horizontals = [(x - 1, y), (x + 1, y)];
        horizontals.shuffle(&mut rand::rng());
        for (tx, ty) in horizontals {
            if self.try_move_water(x, y, tx, ty) {
                // self.water_to_wet_sand(tx, ty);
                return;
            }
        }

        // Priority 2: Fall diagonally (randomize left/right)
        let mut diagonals = [(x - 1, y + 1), (x + 1, y + 1)];
        diagonals.shuffle(&mut rand::rng());
        for (tx, ty) in diagonals {
            if self.try_move_water(x, y, tx, ty) {
                // self.water_to_wet_sand(tx, ty);
                return;
            }
        }
    }

    fn water_to_wet_sand(&mut self, x: i32, y: i32) {
        let mut cells: Vec<(i32, i32)> = Vec::new();

        for cy in -1..2 {
            for cx in -1..2 {
                if cx + x >= 0
                    && cy + y >= 0
                    && cx + x < self.width as i32
                    && cy + y < self.height as i32
                {
                    cells.push((cx + x, cy + y));
                }
            }
        }

        for (x, y) in cells {
            if self.grid[y as usize][x as usize].cell_type == SAND_CELL {
                self.grid[y as usize][x as usize] = Cell::new_wet_sand();
            }
        }
    }

    fn try_move_water(&mut self, from_x: i32, from_y: i32, to_x: i32, to_y: i32) -> bool {
        if to_x < 0 || to_y < 0 {
            return false;
        }
        if self
            .grid
            .get(to_y as usize)
            .and_then(|row| row.get(to_x as usize))
            .map(|cell| cell.cell_type)
            == Some(0)
        {
            self.grid[to_y as usize][to_x as usize] = self.grid[from_y as usize][from_x as usize];
            self.grid[from_y as usize][from_x as usize] = Cell::new_empty();
            self.processed[to_y as usize][to_x as usize] = true;
            return true;
        }
        false
    }

    fn update_wet_sand(&mut self, x: u32, y: u32) {
        let tx = x;
        let ty = y + 1;

        match self
            .grid
            .get(ty as usize)
            .and_then(|row| row.get(tx as usize))
            .map(|cell| cell.cell_type)
        {
            Some(EMPTY_CELL) => {
                self.grid[ty as usize][tx as usize] = self.grid[y as usize][x as usize];
                self.grid[y as usize][x as usize] = Cell::new_empty();
                self.processed[ty as usize][tx as usize] = true;
            }
            Some(WATER_CELL) => {
                let cell_buffer = self.grid[ty as usize][tx as usize];
                self.grid[ty as usize][tx as usize] = self.grid[y as usize][x as usize];
                self.grid[y as usize][x as usize] = cell_buffer;
                self.processed[ty as usize][tx as usize] = true;
            }
            _ => {}
        }
    }
}
