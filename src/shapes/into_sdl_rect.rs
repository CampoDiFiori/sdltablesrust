use sdl2::rect::Rect;

pub trait IntoSdlRect {
    fn into_sdl_rect(&self) -> Rect;
}