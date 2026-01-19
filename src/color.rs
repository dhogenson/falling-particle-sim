use rand::{self, Rng};

pub const SAND: [f32; 4] = [0.7, 0.6, 0.4, 1.0];
pub const CLAY: [f32; 4] = [0.2, 0.2, 0.2, 1.0];
pub const WATER: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
pub const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

pub fn random_color(base_color: [f32; 4]) -> [f32; 4] {
    let mut rng = rand::rng();

    // Generate a random brightness multiplier (0.9 to 1.1 means ±10% brightness variation)
    let brightness_factor: f32 = rng.random_range(0.9..1.1);

    let mut random_base_color: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

    // Apply the same brightness factor to all RGB channels
    for color_index in 0..3 {
        let adjusted = base_color[color_index] * brightness_factor;
        random_base_color[color_index] = adjusted.clamp(0.0, 1.0);
    }

    random_base_color
}
