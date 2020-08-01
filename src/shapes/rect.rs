use super::point::Point;
use super::clickable::Clickable;

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

    pub fn to_sdl_rect(&self) -> sdl2::rect::Rect {
        sdl2::rect::Rect::new(
            self.p1.x,
            self.p1.y,
            (self.p2.x - self.p1.x) as u32,
            (self.p2.y - self.p1.y) as u32,
        )
    }

    /// Although we only need two points to generate a Rect, sometimes
    /// we're interested in all four. Mainly, when we want to create 4 control points
    /// for a Rect.
    pub fn get_all_points(& self) -> [Point; 4] {
        [
            self.p1,
            Point {
                x: self.p2.x,
                y: self.p1.y
            },
            self.p2,
            Point {
                x: self.p1.x,
                y: self.p2.y
            },
        ]
    }
}

impl Clickable for Rect {
    fn is_hovered (&self, clicked_point: &Point) -> bool {
        return clicked_point.x <= self.p2.x
        && clicked_point.x >= self.p1.x
        && clicked_point.y <= self.p2.y
        && clicked_point.y >= self.p1.y
    } 
}