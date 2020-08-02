use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect as SDL_Rect;
// use sdl2::render::{Canvas, TextureQuery};
// use sdl2::ttf::{Font, FontError};
// use sdl2::video::Window;
use std::time::Duration;

mod config;
mod shapes;

use config::{WINDOW_HEIGHT, WINDOW_WIDTH};
use shapes::rects::Rects;

pub fn main() -> Result<(), String> {

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let ttf_context = sdl2::ttf::init().unwrap();
    let font = ttf_context
        .load_font("fonts/times-new-roman.ttf", 50)
        .unwrap();

    let window = video_subsystem
        .window("Tables", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let mut mouse_position = sdl2::rect::Point::new(0, 0);

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut is_running = true;

    let rect = SDL_Rect::new(0, 0, 60, 30);

    // let mut time1: std::time::Instant;
    // let mut time2: std::time::Instant;
    // let mut time_diff_sum: std::time::Duration = std::time::Duration::new(0, 0);
    // let mut time_diff_count: u128 = 0;

    let mut rects = Rects::new();

    rects.add_table(sdl2::rect::Rect::new(500, 500, 100, 100 ));
    rects.add_table(sdl2::rect::Rect::new(300, 300, 100, 100 ));


    'running: loop {
        // time1 = std::time::Instant::now();

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
                Event::MouseMotion { x, y, .. } => {
                    let (x_diff, y_diff) = (x - mouse_position.x, y - mouse_position.y);
                    mouse_position = sdl2::rect::Point::new(x, y);
                    rects.move_selected_rect(x_diff, y_diff)
                }
                Event::MouseButtonDown { x, y, .. } => rects.select_rect(sdl2::rect::Point::new(x, y)),
                Event::MouseButtonUp { .. } => rects.unselect_any_rect(),
                _ => {}
            }
        }

        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();

        let texture_creator = canvas.texture_creator();
        let text_texture = texture_creator
            .create_texture_from_surface(font
                .render(format!("{}, {}", mouse_position.x, mouse_position.y).as_str())
                .blended(Color::RGBA(0, 0, 0, 255))
                .unwrap())
            .unwrap();

        // canvas.fill_rect(rect).unwrap();
        canvas
            .copy(&text_texture, None, Some(rect))
            .unwrap();

        rects.put_on_window_canvas(&mut canvas)?;

        canvas.present();
        // The rest of the game loop goes here...

        // time2 = std::time::Instant::now();

        // time_diff_sum = time_diff_sum + (time2 - time1);
        // time_diff_count = time_diff_count + 1;

        // if time_diff_count == 50 {
        //     println!("Average render: {}", time_diff_sum.as_micros() / time_diff_count);
        //     time_diff_sum = std::time::Duration::new(0, 0);
        //     time_diff_count = 0;
        // }

        ::std::thread::sleep(Duration::from_millis(1000 / 60) - Duration::from_micros(1000));
    }

    return Ok(());
}
