use minifb::{Key, KeyRepeat, Window, WindowOptions};
use rand::Rng;
use std::thread;
use std::time::Duration;

const GRID_WIDTH: usize = 375;
const GRID_HEIGHT: usize = 200;
const CELL_SIZE: usize = 2;
const WINDOW_WIDTH: usize = GRID_WIDTH * CELL_SIZE;
const WINDOW_HEIGHT: usize = GRID_HEIGHT * CELL_SIZE;
const WINDOW_TITLE: &str = "Smooth Life Cellular Automaton";
const FRAME_DELAY_MS: u64 = 50;

fn main() {
    let mut grid = create_grid();

    // Set initial state with random cell values
    let mut rng = rand::thread_rng();

    for i in 0..GRID_HEIGHT {
        for j in 0..GRID_WIDTH {
            let value = rng.gen_range(1.0..10.0);
            grid[i][j] = if value > 9.0 { 1.0 } else { 0.0 }; // Set the cell value based on the condition
        }
    }

    let mut buffer: Vec<u32> = vec![0; WINDOW_WIDTH * WINDOW_HEIGHT];

    let mut window = Window::new(
        WINDOW_TITLE,
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
        WindowOptions {
            scale: minifb::Scale::X2,
            ..WindowOptions::default()
        },
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let mut restart = false;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        if window.is_key_pressed(Key::R, KeyRepeat::No) {
            grid = create_grid();
            for i in 0..GRID_HEIGHT {
                for j in 0..GRID_WIDTH {
                    grid[i][j] = rng.gen_range(0.0..1.0);
                }
            }
            restart = true;
        }

        if restart {
            buffer.iter_mut().for_each(|pixel| *pixel = 0);
            restart = false;
        }

        print_grid(&mut buffer, &grid);
        window.update_with_buffer_size(&buffer, WINDOW_WIDTH, WINDOW_HEIGHT).unwrap();
        update_grid(&mut grid);

        thread::sleep(Duration::from_millis(FRAME_DELAY_MS));
    }
}

fn create_grid() -> Vec<Vec<f64>> {
    vec![vec![0.0; GRID_WIDTH]; GRID_HEIGHT]
}

fn print_grid(buffer: &mut Vec<u32>, grid: &Vec<Vec<f64>>) {
    for (i, row) in grid.iter().enumerate() {
        for (j, &value) in row.iter().enumerate() {
            let color_value = (value * 255.0) as u32; // Map cell value to grayscale value

            for x in 0..CELL_SIZE {
                for y in 0..CELL_SIZE {
                    let pixel_x = j * CELL_SIZE + x;
                    let pixel_y = i * CELL_SIZE + y;
                    let index = pixel_y * WINDOW_WIDTH + pixel_x;

                    buffer[index] = (color_value << 16) | (color_value << 8) | color_value;
                }
            }
        }
    }
}

fn update_grid(grid: &mut Vec<Vec<f64>>) {
    let mut new_grid = create_grid();

    for i in 0..GRID_HEIGHT {
        for j in 0..GRID_WIDTH {
            let value = grid[i][j];
            let alive_neighbors = smooth_life_neighbors(&grid, i, j);

            let new_value = value + (alive_neighbors - value) * 0.05; // Apply smooth life rules

            new_grid[i][j] = new_value.max(0.0).min(1.0); // Ensure value is within range [0, 1]
        }
    }

    *grid = new_grid;
}

fn smooth_life_neighbors(grid: &Vec<Vec<f64>>, row: usize, col: usize) -> f64 {
    let mut sum = 0.0;
    let mut count = 0;

    for i in (row as isize - 1)..=(row as isize + 1) {
        for j in (col as isize - 1)..=(col as isize + 1) {
            if (i >= 0) && (i < GRID_HEIGHT as isize) && (j >= 0) && (j < GRID_WIDTH as isize) {
                if !(i == row as isize && j == col as isize) {
                    sum += grid[i as usize][j as usize];
                    count += 1;
                }
            }
        }
    }

    sum / (count as f64)
}
