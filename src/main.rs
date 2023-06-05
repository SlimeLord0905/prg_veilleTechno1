use minifb::{Key, KeyRepeat, Window, WindowOptions};
use rand::Rng;
use std::thread;
use std::time::Duration;

const GRID_WIDTH: usize = 375;
const GRID_HEIGHT: usize = 200;
const CELL_SIZE: usize = 2;
const WINDOW_WIDTH: usize = GRID_WIDTH * CELL_SIZE;
const WINDOW_HEIGHT: usize = GRID_HEIGHT * CELL_SIZE;
const WINDOW_TITLE: &str = "Conway's Game of Life";
const FRAME_DELAY_MS: u64 = 0; // Delay between each frame (50ms in this case)

fn main() {
    let mut grid = create_grid();

    // Set initial state with random alive cells
    let mut rng = rand::thread_rng();
    for _ in 0..(GRID_WIDTH * GRID_HEIGHT / 10) {
        let row = rng.gen_range(0..GRID_HEIGHT);
        let col = rng.gen_range(0..GRID_WIDTH);
        grid[row][col] = true;
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
            for _ in 0..(GRID_WIDTH * GRID_HEIGHT / 10) {
                let row = rng.gen_range(0..GRID_HEIGHT);
                let col = rng.gen_range(0..GRID_WIDTH);
                grid[row][col] = true;
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

fn create_grid() -> Vec<Vec<bool>> {
    vec![vec![false; GRID_WIDTH]; GRID_HEIGHT]
}

fn print_grid(buffer: &mut Vec<u32>, grid: &Vec<Vec<bool>>) {
    for (i, row) in grid.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            let color = if cell { 0xFFFFFF } else { 0x000000 };

            for x in 0..CELL_SIZE {
                for y in 0..CELL_SIZE {
                    let pixel_x = j * CELL_SIZE + x;
                    let pixel_y = i * CELL_SIZE + y;
                    let index = pixel_y * WINDOW_WIDTH + pixel_x;

                    buffer[index] = color;
                }
            }
        }
    }
}

fn update_grid(grid: &mut Vec<Vec<bool>>) {
    let mut new_grid = create_grid();

    for (i, row) in grid.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            let alive_neighbors = count_alive_neighbors(&grid, i, j);

            new_grid[i][j] = match (cell, alive_neighbors) {
                (true, 2) | (true, 3) | (false, 3) => true,
                _ => false,
            };
        }
    }

    *grid = new_grid;
}

fn count_alive_neighbors(grid: &Vec<Vec<bool>>, row: usize, col: usize) -> u8 {
    let mut count = 0;

    for i in (row as isize - 1)..=(row as isize + 1) {
        for j in (col as isize - 1)..=(col as isize + 1) {
            if (i >= 0) && (i < GRID_HEIGHT as isize) && (j >= 0) && (j < GRID_WIDTH as isize) {
                if !(i == row as isize && j == col as isize) && grid[i as usize][j as usize] {
                    count += 1;
                }
            }
        }
    }

    count
}
