use rand;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, TextureQuery};
use sdl2::ttf::Font;
use sdl2::video::Window;
use std::time::Duration;

const WINDOW_WIDTH: i32 = 1000;
const WINDOW_HEIGHT: i32 = 1000;

const GRID_WIDTH: i32 = 100;
const GRID_HEIGHT: i32 = 100;

const SQUARE_WIDTH: i32 = WINDOW_WIDTH / GRID_WIDTH;
const SQUARE_HEIGHT: i32 = WINDOW_HEIGHT / GRID_HEIGHT;

#[derive(Copy, Clone)]
enum CellState {
    Dead,
    Alive,
}

struct GameState {
    grid: [[CellState; GRID_WIDTH as usize]; GRID_HEIGHT as usize],
    neighbors: [[u8; GRID_WIDTH as usize]; GRID_HEIGHT as usize],
}

impl GameState {
    pub fn draw(&self, canvas: &mut Canvas<Window>, font: &Font) {
        // let TextureQuery { width, height, .. } = text_texture.query();

        // set entire background to black
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        for (i, row) in self.grid.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                let text_surface = font
                    .render(self.neighbors[i][j].to_string().as_str())
                    .blended(Color::RGBA(255, 0, 0, 255))
                    .expect("Font to surface failed");

                let texture_creator = canvas.texture_creator();
                let text_texture = texture_creator
                    .create_texture_from_surface(&text_surface)
                    .expect("Surface to texture failed");

                match *cell {
                    CellState::Alive => {
                        let rect = Rect::new(
                            SQUARE_WIDTH * i as i32,
                            SQUARE_HEIGHT * j as i32,
                            SQUARE_WIDTH as u32,
                            SQUARE_HEIGHT as u32,
                        );
                        canvas.fill_rect(rect).unwrap();
                        canvas
                            .copy(&text_texture, None, Some(rect))
                            .expect("Texture copy failed")
                    }
                    _ => {
                        let rect = Rect::new(
                            SQUARE_WIDTH * i as i32,
                            SQUARE_HEIGHT * j as i32,
                            SQUARE_WIDTH as u32,
                            SQUARE_HEIGHT as u32,
                        );
                        canvas.draw_rect(rect).unwrap();
                        canvas
                            .copy(&text_texture, None, Some(rect))
                            .expect("Texture copy failed")
                    }
                }
            }
        }
        canvas.present();
    }

    fn update_cell_neighbors(&mut self, row: i32, col: i32) {
        for i in -1..1 {
            for j in -1..1 {
                match (i, j) {
                    (0, 0) => continue,
                    _ => {
                        if row + i < 0
                            || col + j < 0
                            || row + i == GRID_HEIGHT
                            || col + j == GRID_WIDTH
                        {
                            continue;
                        }
                        self.neighbors[(row + i) as usize][(col + j) as usize] += 1;
                    }
                }
            }
        }
    }

    pub fn update_neighbors(&mut self) {
        for row in self.neighbors.iter_mut() {
            for cell in row.iter_mut() {
                *cell = 0;
            }
        }

        for row in 0..GRID_WIDTH {
            for col in 0..GRID_HEIGHT {
                match self.grid[row as usize][col as usize] {
                    CellState::Alive => self.update_cell_neighbors(row, col),
                    _ => (),
                }
            }
        }
    }

    pub fn update_grid(&mut self) {
        for row in 0..(GRID_WIDTH as usize) {
            for col in 0..(GRID_HEIGHT as usize) {
                self.grid[row][col] = match (self.grid[row][col], self.neighbors[row][col]) {
                    (CellState::Alive, 0) | (CellState::Alive, 1) => CellState::Dead,
                    (CellState::Dead, 0) | (CellState::Dead, 1) | (CellState::Dead, 2) => {
                        CellState::Dead
                    }
                    (CellState::Dead, _) => CellState::Alive,
                    _ => CellState::Alive,
                }
            }
        }
    }
}

fn init_game_state() -> GameState {
    let mut grid = [[CellState::Dead; GRID_WIDTH as usize]; GRID_HEIGHT as usize];

    for row in grid.iter_mut() {
        for cell in row.iter_mut() {
            *cell = if rand::random::<u8>() < 150 {
                CellState::Alive
            } else {
                CellState::Dead
            };
        }
    }

    return GameState {
        grid,
        neighbors: [[0_u8; GRID_WIDTH as usize]; GRID_HEIGHT as usize],
    };
}

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let ttf_context = sdl2::ttf::init().unwrap();
    let font = ttf_context
        .load_font("/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf", 12)
        .expect("Couldn't import the font");

    let window = video_subsystem
        .window("Game of Life", WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let mut game_state = init_game_state();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut is_running = true;

    'running: loop {
        if is_running {
            game_state.update_neighbors();
            // game_state.update_grid();
            game_state.draw(&mut canvas, &font);
        }

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    ..
                } => is_running = !is_running,
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        ::std::thread::sleep(Duration::from_millis(200));
    }
}
