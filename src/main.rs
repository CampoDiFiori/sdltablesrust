use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use sdl2::rect::Rect;
use rand;
use sdl2::render::{Canvas};
use sdl2::video::Window;

const WINDOW_WIDTH: i32 = 800;
const WINDOW_HEIGHT: i32 = 600;

const HOR_SQUARES: i32 = 80;
const VER_SQUARES: i32 = 60;

const SQUARE_WIDTH: i32 = WINDOW_WIDTH / HOR_SQUARES;
const SQUARE_HEIGHT: i32 = WINDOW_HEIGHT / VER_SQUARES;

#[derive(Copy, Clone)]
enum CellState {
    Dead,
    Alive
}

struct GameState {
    grid: [[CellState; VER_SQUARES as usize] ; HOR_SQUARES as usize],
    neighbors: [[u8; VER_SQUARES as usize]; HOR_SQUARES as usize]
} 

impl GameState {
    pub fn draw (&self, canvas: &mut Canvas<Window>) {
        // set entire background to black
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();
        
        
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        for (i, row) in self.grid.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                match *cell {
                    CellState::Alive => {
                        canvas.fill_rect(Rect::new(
                            SQUARE_WIDTH * i as i32,
                            SQUARE_HEIGHT * j as i32, 
                            SQUARE_WIDTH as u32, 
                            SQUARE_HEIGHT as u32
                            )).unwrap();
                    },
                    _ => {
                        canvas.draw_rect(Rect::new(
                            SQUARE_WIDTH * i as i32,
                            SQUARE_HEIGHT * j as i32, 
                            SQUARE_WIDTH as u32, 
                            SQUARE_HEIGHT as u32
                            )).unwrap();
                    }
                }
            }   
        }
        canvas.present();
    }

    fn update_cell_neighbors (&mut self, row: i32, col: i32) {
        for i in -1..1 {
            for j in -1..1 {
                match (i, j) {
                    (0, 0) => {continue},
                    _ => {
                        if row + i < 0 || col + j < 0 || row + i == VER_SQUARES || col + j == HOR_SQUARES {
                            continue;
                        } 
                        self.neighbors[(row + i) as usize][(col + j) as usize] += 1;
                    }
                }
            }
        }
    }

    pub fn update_neighbors (&mut self) {
        for row in self.neighbors.iter_mut() {
            for cell in row.iter_mut() {
                *cell = 0;
            }
        }

        for row in 0..HOR_SQUARES {
            for col in 0..VER_SQUARES {
                match self.grid[row as usize][col as usize] {
                    CellState::Alive => self.update_cell_neighbors(row, col),
                    _ => ()
                }
            }
        }
    }

    pub fn update_grid (&mut self) {
        for row in 0..(HOR_SQUARES as usize) {
            for col in 0..(VER_SQUARES as usize) {
                self.grid[row][col] = match (self.grid[row][col], self.neighbors[row][col]) {
                    (CellState::Alive, 0) | (CellState::Alive, 1) => CellState::Dead,
                    (CellState::Dead, 0) | (CellState::Dead, 1) | (CellState::Dead, 2) => CellState::Dead,
                    (CellState::Dead, _) => CellState::Alive,
                    _ => CellState::Alive
                }
            }
        } 
    }
}

fn init_game_state () -> GameState {

    let mut grid = [[CellState::Dead; VER_SQUARES as usize] ; HOR_SQUARES as usize];

    for row in grid.iter_mut() {
        for cell in row.iter_mut() {
            *cell = if rand::random::<u8>() < 150 {CellState::Alive} else {CellState::Dead};
        }   
    }

    return GameState {
        grid,
        neighbors: [[0_u8; VER_SQUARES as usize]; HOR_SQUARES as usize]
    }
}
 
pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
 
    let window = video_subsystem.window("Game of Life", WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32)
        .position_centered()
        .build()
        .unwrap();
 
    let mut canvas = window.into_canvas().build().unwrap();

    let mut game_state = init_game_state();
    
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        game_state.update_neighbors();
        game_state.update_grid();
        game_state.draw(&mut canvas);

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        ::std::thread::sleep(Duration::from_millis(1000));
    }
}