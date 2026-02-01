use super::super::cell::*;
use super::super::grid::Grid;

impl Grid {
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

    // Rules of wet sand
    // 1) It can only move down
    // 2) If the wet sand is below any water it will swap with its positions with the water
    // to make a sinking effect
    pub fn update_wet_sand(&mut self, x: i64, y: i64) {
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
}
