use super::super::cell::*;
use super::super::grid::Grid;
use rand;
use rand::seq::SliceRandom;

impl Grid {
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

    // Gets the cells of the at the water and then if they are sand it turns it into wet sand
    pub(crate) fn water_to_wet_sand(&mut self, x: i64, y: i64) {
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
    pub(crate) fn try_move_water(&mut self, x: i64, y: i64, tx: i64, ty: i64) -> bool {
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
