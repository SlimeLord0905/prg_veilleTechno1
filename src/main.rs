use minifb::{Key, KeyRepeat, Window, WindowOptions};
use rand::Rng;
use rayon::prelude::*;
use std::thread;
use std::time::Duration;

const GRID_WIDTH: usize = 750; //nombre de cells de x
const GRID_HEIGHT: usize = 400; //nombre de cells y
const CELL_SIZE: usize = 1; //nombre de pixel occupé par une cellule
const WINDOW_WIDTH: usize = GRID_WIDTH * CELL_SIZE; //largeur de la grid
const WINDOW_HEIGHT: usize = GRID_HEIGHT * CELL_SIZE; //hauteur de la grid
const WINDOW_TITLE: &str = "Smooth Life Cellular Automaton Par Mathieu Mercier";
const FRAME_DELAY_MS: u64 = 50; //possible délais a appliquer entre chaque cycle default 50ms augmenter le frame delay réduit la charge sur le cpu

fn main() {
    let mut grid = create_grid();
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

    let mut rng = rand::thread_rng();
    randomize_grid(&mut grid, &mut rng);

    let mut restart = false;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        if window.is_key_pressed(Key::R, KeyRepeat::No) {
            randomize_grid(&mut grid, &mut rng);
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

//Crée la grid un vecteur deux dimmension c'est a dire une array a taille non fini qui contient d'autre array a taille non fini
fn create_grid() -> Vec<Vec<f64>> {
    vec![vec![0.0; GRID_WIDTH]; GRID_HEIGHT]
}

//Ajoute des cellules en vie aléatoire à la création de la grid
fn randomize_grid(grid: &mut Vec<Vec<f64>>, rng: &mut impl Rng) {
    for i in 0..GRID_HEIGHT {
        for j in 0..GRID_WIDTH {
            grid[i][j] = rng.gen_range(0.0..=1.0);
        }
    }
}
//Affiche le contenue du buffer de la windows pixel par pixel
fn print_grid(buffer: &mut Vec<u32>, grid: &Vec<Vec<f64>>) {
    for (i, row) in grid.iter().enumerate() {
        for (j, &value) in row.iter().enumerate() {
            let color_value = (value * 255.0) as u32; 

            let pixel_x_start = j * CELL_SIZE;
            let pixel_x_end = pixel_x_start + CELL_SIZE;
            let pixel_y_start = i * CELL_SIZE;
            let pixel_y_end = pixel_y_start + CELL_SIZE;

            for pixel_x in pixel_x_start..pixel_x_end {
                for pixel_y in pixel_y_start..pixel_y_end {
                    let index = pixel_y * WINDOW_WIDTH + pixel_x;

                    buffer[index] = (color_value << 16) | (color_value << 8) | color_value;
                }
            }
        }
    }
}
//Update le buffer de la window avec la valeur retourner par smooth_life_neighbors pour toutes les cellules
fn update_grid(grid: &mut Vec<Vec<f64>>) {
    let mut new_grid = create_grid();

    new_grid
        .par_iter_mut()
        .enumerate()
        .for_each(|(i, new_row)| {
            for j in 0..GRID_WIDTH {
                let new_value = smooth_life_neighbors(&grid, i, j);

                new_row[j] = new_value.max(0.0).min(1.0); 
            }
        });

    *grid = new_grid;
}

//Compte le nombre de voisin dans le radius fait les calculs du model smoothLife puis retourne la valeur de la cellule
fn smooth_life_neighbors(grid: &Vec<Vec<f64>>, row: usize, col: usize) -> f64 {
    let mut inner_count = 0;
    let mut outer_count = 0;
    let mut inner_count_cells = 0;
    let mut outer_count_cells = 0;

    let inner_radius = 1;
    let outer_radius = 8;

    for i in (row as isize - outer_radius as isize)..=(row as isize + outer_radius as isize) {
        for j in (col as isize - outer_radius as isize)..=(col as isize + outer_radius as isize) {
            if (i >= 0) && (i < GRID_HEIGHT as isize) && (j >= 0) && (j < GRID_WIDTH as isize) {
                let dx = (col as isize - j).abs() as f64;
                let dy = (row as isize - i).abs() as f64;
                let d = dx.max(dy);

                let cell_value = grid[i as usize][j as usize];

                if d < inner_radius as f64 {
                    inner_count += cell_value.round() as u32;
                    inner_count_cells += 1;
                } else if d < outer_radius as f64 {
                    outer_count += cell_value.round() as u32;
                    outer_count_cells += 1;
                }
            }
        }
    }

    let ui = if inner_count_cells > 0 {
        inner_count as f64 / inner_count_cells as f64
    } else {
        0.0
    };

    let uo = if outer_count_cells > 0 {
        outer_count as f64 / outer_count_cells as f64
    } else {
        0.0
    };

    if (ui >= 0.5 && (0.26 <= uo && uo <= 0.46)) || (ui < 0.5 && (0.27 <= uo && uo <= 0.36)) {
        1.0
    } else {
        0.0
    }
}
