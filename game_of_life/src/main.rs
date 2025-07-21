use minifb::{Key, Window, WindowOptions};
use std::{thread, time};

const WIDTH: usize = 100;
const HEIGHT: usize = 100;
const SCALE: usize = 6;

const ALIVE: u32 = 0xFFFFFF; // Blanco
const DEAD: u32 = 0x0068a2;  // Negro

fn main() {
    let mut window = Window::new(
        "Conway's Game of Life",
        WIDTH * SCALE,
        HEIGHT * SCALE,
        WindowOptions {
            resize: false,
            scale: minifb::Scale::X1,
            ..WindowOptions::default()
        },
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let mut buffer = vec![DEAD; WIDTH * HEIGHT];
    let mut next = buffer.clone();

    init_pattern(&mut buffer);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        update(&buffer, &mut next);
        render_scaled(&mut window, &next);
        buffer.copy_from_slice(&next);
        thread::sleep(time::Duration::from_millis(100));
    }
}

//Funciones update y render 
fn get_index(x: usize, y: usize) -> usize {
    y * WIDTH + x
}

fn is_alive(color: u32) -> bool {
    color == ALIVE
}

fn count_alive_neighbors(buffer: &[u32], x: usize, y: usize) -> u8 {
    let mut count = 0;

    for dx in [-1, 0, 1] {
        for dy in [-1, 0, 1] {
            if dx == 0 && dy == 0 {
                continue;
            }

            let nx = x as isize + dx;
            let ny = y as isize + dy;

            if nx >= 0 && ny >= 0 && nx < WIDTH as isize && ny < HEIGHT as isize {
                let i = get_index(nx as usize, ny as usize);
                if is_alive(buffer[i]) {
                    count += 1;
                }
            }
        }
    }

    count
}

fn update(current: &[u32], next: &mut [u32]) {
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let idx = get_index(x, y);
            let alive = is_alive(current[idx]);
            let neighbors = count_alive_neighbors(current, x, y);

            next[idx] = match (alive, neighbors) {
                (true, 2) | (true, 3) => ALIVE,
                (false, 3) => ALIVE,
                _ => DEAD,
            };
        }
    }
}

//Renderizar la imagen escalada
fn render_scaled(window: &mut Window, buffer: &[u32]) {
    let mut scaled = vec![0; WIDTH * HEIGHT * SCALE * SCALE];

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let color = buffer[get_index(x, y)];
            for dy in 0..SCALE {
                for dx in 0..SCALE {
                    let sx = x * SCALE + dx;
                    let sy = y * SCALE + dy;
                    let idx = sy * WIDTH * SCALE + sx;
                    scaled[idx] = color;
                }
            }
        }
    }

    window
        .update_with_buffer(&scaled, WIDTH * SCALE, HEIGHT * SCALE)
        .unwrap();
}

//Crear patrones iniciales
fn set(buffer: &mut [u32], x: usize, y: usize, state: u32) {
    if x < WIDTH && y < HEIGHT {
        buffer[get_index(x, y)] = state;
    }
}

fn put_glider(buffer: &mut [u32], x: usize, y: usize) {
    set(buffer, x + 1, y, ALIVE);
    set(buffer, x + 2, y + 1, ALIVE);
    set(buffer, x, y + 2, ALIVE);
    set(buffer, x + 1, y + 2, ALIVE);
    set(buffer, x + 2, y + 2, ALIVE);
}

fn put_block(buffer: &mut [u32], x: usize, y: usize) {
    set(buffer, x, y, ALIVE);
    set(buffer, x + 1, y, ALIVE);
    set(buffer, x, y + 1, ALIVE);
    set(buffer, x + 1, y + 1, ALIVE);
}

fn put_blinker(buffer: &mut [u32], x: usize, y: usize) {
    set(buffer, x, y, ALIVE);
    set(buffer, x + 1, y, ALIVE);
    set(buffer, x + 2, y, ALIVE);
}

//Iniciar un patrón creativo
fn init_pattern(buffer: &mut [u32]) {
    for i in 0..8 {
        put_glider(buffer, 10 + i * 5, 10 + i * 5);
    }

    put_block(buffer, 50, 50);
    put_blinker(buffer, 20, 20);
}


