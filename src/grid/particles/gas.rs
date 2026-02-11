use super::super::cell::*;
use super::super::grid::Grid;
use rand;
use rand::seq::SliceRandom;

impl Grid {
    // Updates cell based of fire particle rules
    pub fn update_fire(&mut self, x: i64, y: i64) {
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
    pub(crate) fn fire_make_smoke(&mut self, x: i64, y: i64) {
        let random_number = rand::random_range(0..1000);
        if random_number > 998 {
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
    pub fn update_smoke(&mut self, x: i64, y: i64) {
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

    // Moves the steam based of the gass particle rules
    pub fn update_steam(&mut self, x: i64, y: i64) {
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

    // tx: Target X
    // ty: Target Y
    // Trys to move a gass particle, if it cant it returns false, if it can it returns true
    pub(crate) fn try_move_gass(&mut self, x: i64, y: i64, tx: i64, ty: i64) -> bool {
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
}
