use super::clickable::Clickable;
use super::into_sdl_rect::IntoSdlRect;

impl Clickable for sdl2::rect::Point {
    fn is_hovered (&self, clicked_point: &sdl2::rect::Point) -> bool {
        return clicked_point.x <= self.x + 5
        && clicked_point.x >= self.x - 5
        && clicked_point.y <= self.y + 5
        && clicked_point.y >= self.y - 5 
    } 

    fn move_by(&mut self, x_diff: i32, y_diff: i32) -> () {
        *self = self.offset(x_diff, y_diff)
    }
}

impl IntoSdlRect for sdl2::rect::Point {
    fn into_sdl_rect(&self) -> sdl2::rect::Rect {
        sdl2::rect::Rect::new(
            self.x - 5,
            self.y - 5,
            10,
            10
        )  
    }
}

impl IntoSdlRect for [sdl2::rect::Point] {
    fn into_sdl_rect(&self) -> sdl2::rect::Rect {
        sdl2::rect::Rect::new(
            self[0].x,
            self[0].y,
            (self[1].x - self[0].x) as u32,
            (self[2].y - self[1].y) as u32
        )    
    }
}

impl Clickable for sdl2::rect::Rect {
    fn is_hovered (&self, clicked_point: &sdl2::rect::Point) -> bool {
        return clicked_point.x <= self.right()
        && clicked_point.x >= self.left()
        && clicked_point.y <= self.bottom() // bottom for some reason is the top of rect
        && clicked_point.y >= self.top() 
    } 

    fn move_by(&mut self, x_diff: i32, y_diff: i32) -> () {
        self.offset(x_diff, y_diff)
    }
}