mod color;
mod grid;
mod ui;

use std::env;
use std::time::{Duration, Instant};

use color::*;
use grid::cells::*;
use grid::grid::Grid;
use ui::text::Label;

use piston_window::{
    PistonWindow, WindowSettings,
    graphics::{Context, Graphics, clear, rectangle},
};

use piston_window::*;
use wgpu_graphics::TextureSettings;

const CELL_SIZE: f64 = 7.0;
const GRID_WIDTH: i64 = 170;
const GRID_HEIGHT: i64 = 130;
const _FPS: u16 = 60;
const DEBUG: bool = false;

fn main() {
    let sand_box_height = (GRID_HEIGHT as f64 * CELL_SIZE) as u32;
    let sand_box_width = (GRID_WIDTH as f64 * CELL_SIZE) as u32;
    let window_width: u32 = sand_box_width + 300;
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

    let mut brush_size: i32 = 2;

    let update_interval = Duration::from_millis(10);
    let mut last_update = Instant::now();

    let mut selected_element: u8 = 1;

    // Load font
    let exe_path = env::current_exe().unwrap();
    let assets = exe_path
        .parent()
        .map(|p| p.join("assets"))
        .filter(|p| p.exists())
        .or_else(|| {
            env::current_dir()
                .ok()
                .map(|p| p.join("assets"))
                .filter(|p| p.exists())
        })
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
                Key::D6 => selected_element = EMPTY_CELL,
                Key::D1 => selected_element = SAND_CELL,
                Key::D2 => selected_element = STEEL_CELL,
                Key::D3 => selected_element = WATER_CELL,
                Key::D4 => selected_element = FIRE_CELL,
                Key::D5 => selected_element = ACID_CELL,
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

        if let Some(scroll) = event.mouse_scroll_args() {
            let scroll_y = scroll[1]; // Vertical scroll
            brush_size += scroll_y as i32;
            brush_size = brush_size.max(1).min(20);
        }

        // Place element
        if mouse_held
            && board_x >= 0
            && board_x < GRID_WIDTH as i32
            && board_y >= 0
            && board_y < GRID_HEIGHT as i32
        {
            grid.place_element(board_x, board_y, selected_element, brush_size);
        }

        // Update grid
        if last_update.elapsed() >= update_interval {
            grid.update();
            last_update = Instant::now();
        }

        // Draw grid
        window.draw_2d(&event, |context, graphics, _device| {
            clear(BLUE_COLOR, graphics);
            draw_grid(
                &grid,
                CELL_SIZE,
                window_width,
                window_height,
                board_x,
                board_y,
                selected_element,
                brush_size,
                &context,
                graphics,
            );

            // Draw text
            let current = match selected_element {
                EMPTY_CELL => "Eraser",
                SAND_CELL => "Sand",
                STEEL_CELL => "Steel",
                WATER_CELL => "Water",
                FIRE_CELL => "Fire",
                ACID_CELL => "Acid",
                _ => "<element>",
            };

            let current_brush = Label::new(
                sand_box_width as f64 + 10.0,
                25.0,
                format!("Current: {}", current),
            );

            let brush_size_label = Label::new(
                sand_box_width as f64 + 10.0,
                50.0,
                format!("Brush size: {}", brush_size),
            );

            current_brush.draw(&context, graphics, &mut glyphs);
            brush_size_label.draw(&context, graphics, &mut glyphs);

            let sand_label = Label::new(
                sand_box_width as f64 + 10.0,
                100.0,
                format!("Sand: {}", grid.debug_info.sand_count),
            );
            let water_label = Label::new(
                sand_box_width as f64 + 10.0,
                125.0,
                format!("Water: {}", grid.debug_info.water_count),
            );
            let wet_sand_label = Label::new(
                sand_box_width as f64 + 10.0,
                150.0,
                format!("Wet Sand: {}", grid.debug_info.wet_sand_count),
            );
            let fire_label = Label::new(
                sand_box_width as f64 + 10.0,
                175.0,
                format!("Fire: {}", grid.debug_info.fire_count),
            );
            let smoke_label = Label::new(
                sand_box_width as f64 + 10.0,
                200.0,
                format!("Smoke: {}", grid.debug_info.smoke_count),
            );
            let steam_label = Label::new(
                sand_box_width as f64 + 10.0,
                225.0,
                format!("Steam: {}", grid.debug_info.steam_count),
            );
            let time_label = Label::new(
                sand_box_width as f64 + 10.0,
                250.0,
                format!("Update: {:?}", grid.debug_info.time_to_update_cells),
            );

            sand_label.draw(&context, graphics, &mut glyphs);
            water_label.draw(&context, graphics, &mut glyphs);
            wet_sand_label.draw(&context, graphics, &mut glyphs);
            fire_label.draw(&context, graphics, &mut glyphs);
            smoke_label.draw(&context, graphics, &mut glyphs);
            steam_label.draw(&context, graphics, &mut glyphs);
            time_label.draw(&context, graphics, &mut glyphs);

            grid.debug_info.reset();
        });
    }
}

fn draw_grid<G: Graphics>(
    grid: &Grid,
    cell_size: f64,
    window_width: u32,
    window_height: u32,
    board_x: i32,
    board_y: i32,
    selected_element: u8,
    brush_size: i32,
    context: &Context,
    graphics: &mut G,
) {
    let mouse_hover = grid.get_circle_positions(board_x, board_y, brush_size);

    // Draw simple grid
    for y in 0..grid.height {
        for x in 0..grid.width {
            let x_pos = x as f64 * cell_size;
            let y_pos = y as f64 * cell_size;

            let cell_rect: [f64; 4] = [x_pos, y_pos, cell_size, cell_size];

            let idx = (y * grid.width + x) as usize;

            // Don't render if cell is empty
            if grid.grid[idx].cell_type != 0 {
                let color = grid.grid[idx].cell_color;
                rectangle(color, cell_rect, context.transform, graphics);
            }

            if DEBUG {
                let color: [f32; 4] = [(grid.temperature[idx] / MAX_TEMP) as f32, 0.0, 0.0, 0.9];
                rectangle(color, cell_rect, context.transform, graphics);
            }
        }
    }

    // Draw black boarder to the side of the grid
    let x_pos: f64 = grid.width as f64 * cell_size;
    let y_pos: f64 = 0.0;
    let cell_rect: [f64; 4] = [
        x_pos,
        y_pos,
        window_width as f64 - x_pos,
        window_height as f64,
    ];

    rectangle(BLACK_COLOR, cell_rect, context.transform, graphics);

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
            EMPTY_CELL => LIGHT_BLUE_COLOR,
            ACID_CELL => ACID_COLOR,
            _ => TRANSPAERNT_COLOR,
        };

        rectangle(color, cell_rect, context.transform, graphics);
    }
}
