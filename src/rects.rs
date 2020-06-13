use sdl2::pixels::Color;
use sdl2::surface::Surface;
use sdl2::ttf::Font;

pub const WINDOW_WIDTH: u32 = 1000;
pub const WINDOW_HEIGHT: u32 = 1000;

#[derive(Copy, Clone)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new() -> Point {
        Point { x: 0, y: 0 }
    }

    pub fn from_xy(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    pub fn to_surface(&self, font: &Font) -> Surface {
        return font
            .render(format!("{}, {}", self.x, self.y).as_str())
            .blended(Color::RGBA(0, 0, 0, 255))
            .expect("Font to surface failed");
    }

    pub fn update_and_get_diff(&mut self, x: i32, y: i32) -> (i32, i32) {
        let ret = (x - self.x, y - self.y);
        self.x = x;
        self.y = y;
        ret
    }
}

pub struct Rect {
    p1: Point,
    p2: Point,
}

impl Rect {
    pub fn new(x1: i32, y1: i32, x2: i32, y2: i32) -> Rect {
        Rect {
            p1: Point { x: x1, y: y1 },
            p2: Point { x: x2, y: y2 },
        }
    }

    pub fn move_by(&mut self, x_diff: i32, y_diff: i32) {
        self.p1.x += x_diff;
        self.p1.y += y_diff;
        self.p2.x += x_diff;
        self.p2.y += y_diff;
    }

    pub fn to_SDL_rect(&self) -> sdl2::rect::Rect {
        sdl2::rect::Rect::new(
            self.p1.x,
            self.p1.y,
            (self.p2.x - self.p1.x) as u32,
            (self.p2.y - self.p1.y) as u32,
        )
    }
}

pub struct Rects {
    rects: Vec<Rect>,
    selected: Option<usize>,
}

impl Rects {
    pub fn new() -> Rects {
        Rects {
            rects: Vec::new(),
            selected: None,
        }
    }

    pub fn add_rect(&mut self, rect: Rect) {
        self.rects.push(rect);
    }

    pub fn select_rect(&mut self, clicked_point: Point) {
        let selected_entry = self.rects.iter().enumerate().find(|(_, rect)| {
            return clicked_point.x <= (*rect).p2.x
                && clicked_point.x >= (*rect).p1.x
                && clicked_point.y <= (*rect).p2.y
                && clicked_point.y >= (*rect).p1.y
        });
        self.selected = match selected_entry {
            Some((idx, _)) => Some(idx),
            None => None,
        };
    }

    pub fn unselect_rect(&mut self) {
        self.selected = None;
    }

    pub fn move_selected(&mut self, x_diff: i32, y_diff: i32) {
        if let Some(selected_idx) = self.selected {
            self.rects[selected_idx].move_by(x_diff, y_diff);
        }
    }

    pub fn put_on_window_canvas(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        let sdl_rects: Vec<sdl2::rect::Rect> =
            self.rects.iter().map(|rect| rect.to_SDL_rect()).collect();

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas
            .draw_rects(sdl_rects.as_slice())
            .expect("Couldn't draw rects");
    }
}
