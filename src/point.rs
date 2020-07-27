use sdl2::surface::Surface;
use sdl2::ttf::Font;
use sdl2::pixels::Color;
use super::rect::Rect;

#[derive(Copy, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new() -> Point {
        Point { x: 0, y: 0 }
    }

    pub fn from_xy(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    pub fn move_by(&mut self, x_diff: i32, y_diff: i32) -> () {
        self.x += x_diff;
        self.y += y_diff;
    }

    pub fn to_surface(&self, font: &Font) -> Surface {
        return font
            .render(format!("{}, {}", self.x, self.y).as_str())
            .blended(Color::RGBA(0, 0, 0, 255))
            .unwrap();
    }

    pub fn update_and_get_diff(&mut self, x: i32, y: i32) -> (i32, i32) {
        let ret = (x - self.x, y - self.y);
        self.x = x;
        self.y = y;
        ret
    }

    pub fn to_rect (&self) -> Rect {
        Rect::new(self.x - 5, self.y - 5, self.x + 5, self.y + 5)
    }
}
