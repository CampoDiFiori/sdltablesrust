use sdl2::rect::Point;

pub trait Clickable {
    fn is_hovered(&self, clicked_point: &Point) -> bool; 
    fn move_by(&mut self, x_diff: i32, y_diff: i32) -> ();
}