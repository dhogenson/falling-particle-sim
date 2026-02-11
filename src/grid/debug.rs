use std::time::Duration;

pub struct DebugInfo {
    pub sand_count: i64,
    pub water_count: i64,
    pub wet_sand_count: i64,
    pub fire_count: i64,
    pub smoke_count: i64,
    pub steam_count: i64,
    pub acid_count: i64,
    pub time_to_update_cells: Duration,
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
            acid_count: 0,
            time_to_update_cells: Duration::from_secs(0),
        }
    }

    pub fn reset(&mut self) {
        self.sand_count = 0;
        self.water_count = 0;
        self.wet_sand_count = 0;
        self.fire_count = 0;
        self.smoke_count = 0;
        self.steam_count = 0;
        self.acid_count = 0;
        self.time_to_update_cells = Duration::from_secs(0);
    }
}
