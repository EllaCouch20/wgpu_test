use ggez::graphics::{Color, DrawMode, Mesh, Rect};
use ggez::{Context, GameResult};
use crate::traits::{Drawable, Component};


pub struct Rectangle(pub f32, pub f32);

impl Component for Rectangle {
    fn spawn(&self, ctx: &mut Context, bound: Rect) -> GameResult<Vec<Box<dyn Drawable>>> {
        Ok(vec![Box::new(Mesh::new_rounded_rectangle(
            ctx,
            DrawMode::fill(),
            Rect::new(0.0, 0.0, bound.w*self.0, bound.h*self.1),
            40.0,
            Color::WHITE,
        )?)])
    }
}