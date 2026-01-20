mod cell;
mod color;
mod grid;

use std::time::{Duration, Instant};

use color::*;
use grid::Grid;

use std::io::{self, Write};

use piston_window::{
    PistonWindow, WindowSettings,
    graphics::{Context, Graphics, clear, rectangle},
};

use piston_window::*;

fn main() {
    const CELL_SIZE: f64 = 5.0;
    const GRID_WIDTH: u32 = 200;
    const GRID_HEIGHT: u32 = 200;
    const _FPS: u16 = 60;

    let window_width: u32 = (GRID_WIDTH as f64 * CELL_SIZE) as u32;
    let window_height: u32 = (GRID_HEIGHT as f64 * CELL_SIZE) as u32;

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

    // FPS tracking
    let mut frame_count = 0;
    let mut fps_timer = Instant::now();
    let fps_update_interval = Duration::from_secs(1);

    // Draw grid
    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            // Handle one time press actions

            match key {
                Key::D1 => selected_element = 1,
                Key::D2 => selected_element = 2,
                Key::D3 => selected_element = 3,
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
            clear(LIGHT_BLUE, graphics);
            draw_grid(&grid, CELL_SIZE, &context, graphics);
        });

        // Update FPS counter
        frame_count += 1;
        if fps_timer.elapsed() >= fps_update_interval {
            let fps = frame_count as f64 / fps_timer.elapsed().as_secs_f64();
            print!("\rFPS: {:.2}", fps);
            io::stdout().flush().unwrap();
            frame_count = 0;
            fps_timer = Instant::now();
        }
    }
}

fn draw_grid<G: Graphics>(grid: &Grid, cell_size: f64, context: &Context, graphics: &mut G) {
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
}
