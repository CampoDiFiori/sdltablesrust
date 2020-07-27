use sdl2::pixels::Color;
use super::point::Point;
use super::rect::Rect;
use super::clickable::Clickable;

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
        let selected_entry = self.rects.iter()
        .enumerate()
        .find(|(_, rect)| rect.is_hovered(&clicked_point));

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

    pub fn put_on_window_canvas(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) -> Result<(), String> {
        let sdl_rects: Vec<sdl2::rect::Rect> =
            self.rects.iter().map(|rect| rect.to_sdl_rect()).collect();

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas
            .draw_rects(sdl_rects.as_slice())?;

        if let Some(selected_idx) = self.selected {
            canvas.set_draw_color(Color::RGB(255, 0, 0));
            canvas.fill_rect(self.rects[selected_idx].to_sdl_rect())?;
        }

        Ok(())
    }
}
