use super::point::Point;

pub trait Clickable {
    fn is_hovered(&self, clicked_point: &Point) -> bool; 
}