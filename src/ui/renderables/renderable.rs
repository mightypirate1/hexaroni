use crate::geometry::ScreenCoord;


pub trait Renderable {
    fn render(&self, screen_coord: &ScreenCoord, time: f32);
}
