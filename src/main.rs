mod cell;
mod color;
mod grid;
mod ui;

use std::time::{Duration, Instant};

use color::*;
use grid::Grid;

use piston_window::{
    PistonWindow, WindowSettings,
    graphics::{Context, Graphics, Transformed, clear, rectangle, text},
};

use piston_window::*;
use wgpu_graphics::TextureSettings;

use crate::cell::*;

fn main() {
    const CELL_SIZE: f64 = 7.0;
    const GRID_WIDTH: i64 = 170;
    const GRID_HEIGHT: i64 = 130;
    const _FPS: u16 = 60;

    let sand_box_height = (GRID_HEIGHT as f64 * CELL_SIZE) as u32;
    let sand_box_width = (GRID_WIDTH as f64 * CELL_SIZE) as u32;
    let window_width: u32 = sand_box_width + 200;
    let window_height: u32 = sand_box_height;

    let mut window: PistonWindow =
        WindowSettings::new("Particle sim", [window_width, window_height])
            .exit_on_esc(true)
            .resizable(false)
            .samples(0)
            .vsync(true)
            .build()
            .unwrap();

    let mut grid = Grid::new(GRID_WIDTH, GRID_HEIGHT);

    let mut board_x: i32 = 0;
    let mut board_y: i32 = 0;
    let mut mouse_held = false;

    let update_interval = Duration::from_millis(5);
    let mut last_update = Instant::now();

    let mut selected_element: u8 = 1;

    // Load font
    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .unwrap();

    let texture_settings = TextureSettings::new();
    let mut glyphs = window
        .load_font(assets.join("OpenSans-Bold.ttf"), texture_settings)
        .unwrap();

    // Draw grid
    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            // Handle one time press actions

            match key {
                Key::D1 => selected_element = SAND_CELL,
                Key::D2 => selected_element = STEEL_CELL,
                Key::D3 => selected_element = WATER_CELL,
                Key::D4 => selected_element = FIRE_CELL,
                _ => {}
            }
        }

        // Keeps track of mouse and board positions
        if let Some(cursor_pos) = event.mouse_cursor_args() {
            let mouse_x: f64 = cursor_pos[0];
            let mouse_y: f64 = cursor_pos[1];

            board_x = (mouse_x / CELL_SIZE) as i32;
            board_y = (mouse_y / CELL_SIZE) as i32;
        }

        if let Some(Button::Mouse(MouseButton::Left)) = event.press_args() {
            mouse_held = true;
        }

        if let Some(Button::Mouse(MouseButton::Left)) = event.release_args() {
            mouse_held = false;
        }

        // Place element
        if mouse_held
            && board_x >= 0
            && board_x < GRID_WIDTH as i32
            && board_y >= 0
            && board_y < GRID_HEIGHT as i32
        {
            grid.place_element(board_x, board_y, selected_element);
        }

        // Update grid
        if last_update.elapsed() >= update_interval {
            grid.update();
            last_update = Instant::now();
        }

        // Draw grid
        window.draw_2d(&event, |context, graphics, _device| {
            clear(LIGHT_BLUE_COLOR, graphics);
            draw_grid(
                &grid,
                CELL_SIZE,
                board_x,
                board_y,
                selected_element,
                &context,
                graphics,
            );

            // Draw text

            let current = match selected_element {
                SAND_CELL => "Sand",
                STEEL_CELL => "Steel",
                WATER_CELL => "Water",
                FIRE_CELL => "Fire",
                _ => "<element>",
            };

            text::Text::new_color([1.0, 1.0, 1.0, 1.0], 24)
                .draw(
                    &format!("Current: {}", current),
                    &mut glyphs,
                    &context.draw_state,
                    context.transform.trans(sand_box_width as f64 + 10.0, 25.0),
                    graphics,
                )
                .unwrap();
        });
    }
}

fn draw_grid<G: Graphics>(
    grid: &Grid,
    cell_size: f64,
    board_x: i32,
    board_y: i32,
    selected_element: u8,
    context: &Context,
    graphics: &mut G,
) {
    let mouse_hover = grid.get_circle_positions(board_x, board_y, 2);

    // Draw simple grid
    for y in 0..grid.height {
        for x in 0..grid.width {
            let x_pos = x as f64 * cell_size;
            let y_pos = y as f64 * cell_size;

            let cell_rect: [f64; 4] = [x_pos, y_pos, cell_size, cell_size];

            // Don't render if cell is empty
            if grid.grid[y as usize][x as usize].cell_type == 0 {
                continue;
            }

            let color = grid.grid[y as usize][x as usize].cell_color;

            rectangle(color, cell_rect, context.transform, graphics);
        }
    }

    // Draw mouse hover
    for (x, y) in mouse_hover {
        let x_pos = x as f64 * cell_size;
        let y_pos = y as f64 * cell_size;
        let cell_rect: [f64; 4] = [x_pos, y_pos, cell_size, cell_size];

        let color: [f32; 4] = match selected_element {
            SAND_CELL => SAND_COLOR,
            STEEL_CELL => STEEL_COLOR,
            WATER_CELL => WATER_COLOR,
            FIRE_CELL => FIRE_COLOR,
            SMOKE_CELL => SMOKE_COLOR,
            STEAM_CELL => STEAM_COLOR,
            _ => TRANSPAERNT_COLOR,
        };

        rectangle(color, cell_rect, context.transform, graphics);
    }
}
