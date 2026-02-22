use super::cells::*;
use std::time::Instant;

use super::debug::DebugInfo;

pub struct Grid {
    pub width: i64,
    pub height: i64,
    pub grid: Vec<Cell>,
    processed: Vec<bool>,
    pub temperature: Vec<i64>,
    pub debug_info: DebugInfo,
}

impl Grid {
    // Helper function to convert 2D coordinates to 1D index
    #[inline]
    pub(crate) fn idx(&self, x: i64, y: i64) -> usize {
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
    pub(crate) fn make_grid(size_x: i64, size_y: i64) -> Vec<Cell> {
        vec![Cell::new_empty(); (size_x * size_y) as usize]
    }

    // Places an element in a circle based of the cords you want
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
                ACID_CELL => Cell::new_acid(),
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

                if cell_type == EMPTY_CELL {
                    continue;
                }

                match cell_type {
                    SAND_CELL => {
                        self.debug_info.sand_count += 1;
                        self.update_sand(x, y);
                    }
                    WATER_CELL => {
                        self.debug_info.water_count += 1;
                        self.update_water(x, y);
                    }
                    WET_SAND_CELL => {
                        self.debug_info.wet_sand_count += 1;
                        self.update_wet_sand(x, y);
                    }
                    FIRE_CELL => {
                        self.debug_info.fire_count += 1;
                        self.update_fire(x, y);
                    }
                    SMOKE_CELL => {
                        self.debug_info.smoke_count += 1;
                        self.update_smoke(x, y);
                    }
                    STEAM_CELL => {
                        self.debug_info.steam_count += 1;
                        self.update_steam(x, y);
                    }
                    ACID_CELL => {
                        self.debug_info.acid_count += 1;
                        self.move_acid(x, y);
                    }
                    _ => {}
                }
            }
        }
        let duration = start.elapsed();
        self.debug_info.time_to_update_cells = duration;
        // self.debug_info.print_info();
    }

    // Gets all positions in the shape of a box that is in bound of the grid
    pub(crate) fn get_square_area(&self, x: i64, y: i64) -> impl Iterator<Item = (i64, i64)> + '_ {
        let min_x = (x - 1).max(0);
        let max_x = (x + 1).min(self.width - 1);
        let min_y = (y - 1).max(0);
        let max_y = (y + 1).min(self.height - 1);

        (min_y..=max_y).flat_map(move |cy| (min_x..=max_x).map(move |cx| (cx, cy)))
    }

    // tx: Target X
    // ty: Target Y
    // Swaps a particle from position to target position
    pub(crate) fn swap_particle(&mut self, x: i64, y: i64, tx: i64, ty: i64) {
        let src_idx = self.idx(x, y);
        let dst_idx = self.idx(tx, ty);
        self.grid.swap(src_idx, dst_idx);
        self.processed[dst_idx] = true;
    }

    // tx: Target X
    // ty: Target Y
    // Moves a particle to target position
    // Note: Replaces the x and y position with a empty cell
    pub(crate) fn move_particle(&mut self, x: i64, y: i64, tx: i64, ty: i64) {
        let src_idx = self.idx(x, y);
        let dst_idx = self.idx(tx, ty);
        self.grid[dst_idx] = self.grid[src_idx];
        self.grid[src_idx] = Cell::new_empty();
        self.processed[dst_idx] = true;
    }

    // Updates lifetime for a cell
    // If cell has lived the amount of its max lifetime it dies
    pub(crate) fn update_life_time(&mut self, x: i64, y: i64) {
        let idx = self.idx(x, y);
        self.grid[idx].life_time += 1;
        if self.grid[idx].life_time >= self.grid[idx].max_life_time {
            self.grid[idx] = Cell::new_empty();
            return;
        }
    }

    pub(crate) fn update_temperature(&mut self, x: i64, y: i64, temperature: i64) {
        let cells: Vec<_> = self.get_square_area(x, y).collect();

        // tx: Target X
        // ty: Target Y
        for (tx, ty) in cells {
            let idx = self.idx(tx, ty);
            self.temperature[idx] += temperature;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_grid() {
        let grid: Grid = Grid::new(10, 10);
        assert_eq!(grid.width, 10);
        assert_eq!(grid.height, 10);
    }

    #[test]
    fn test_swap_particle() {
        let mut grid: Grid = Grid::new(10, 10);
        grid.grid[0] = Cell::new_sand();
        grid.grid[1] = Cell::new_glass();
        grid.swap_particle(0, 0, 1, 0);

        assert_eq!(grid.grid[0].cell_type, GLASS_CELL);
        assert_eq!(grid.grid[1].cell_type, SAND_CELL);
    }

    #[test]
    fn test_move_particle() {
        let mut grid: Grid = Grid::new(10, 10);
        grid.grid[0] = Cell::new_sand();
        grid.move_particle(0, 0, 1, 1);

        assert_eq!(grid.grid[0].cell_type, EMPTY_CELL);
        assert_eq!(grid.grid[11].cell_type, SAND_CELL);
    }

    #[test]
    fn test_update_life_time() {
        let mut grid: Grid = Grid::new(10, 10);
        grid.grid[0] = Cell::new_fire();

        let cell_max_life_time: u64 = grid.grid[0].max_life_time;

        grid.update_life_time(0, 0);
        assert_eq!(grid.grid[0].life_time, 1);

        grid.grid[0].life_time = cell_max_life_time - 1;
        grid.update_life_time(0, 0);
        assert_eq!(grid.grid[0].cell_type, EMPTY_CELL);
    }

    #[test]
    fn test_get_circle_positions() {
        let grid: Grid = Grid::new(10, 10);

        let cells: Vec<(i32, i32)> = grid.get_circle_positions(3, 3, 1);
        assert_eq!(cells.len(), 5);
    }

    #[test]
    fn test_get_circle_positions_at_edge() {
        let grid: Grid = Grid::new(10, 10);

        let cells: Vec<(i32, i32)> = grid.get_circle_positions(0, 0, 1);
        assert_eq!(cells.len(), 3);
    }

    #[test]
    fn test_get_square_positions() {
        let grid: Grid = Grid::new(10, 10);

        let cells: Vec<_> = grid.get_square_area(3, 3).collect();
        assert_eq!(cells.len(), 9);
    }

    #[test]
    fn test_get_square_positions_at_edge() {
        let grid: Grid = Grid::new(10, 10);

        let cells: Vec<_> = grid.get_square_area(0, 0).collect();
        assert_eq!(cells.len(), 4);
    }
}
