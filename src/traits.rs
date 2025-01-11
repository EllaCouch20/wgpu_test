use ggez::graphics::{Rect, Canvas, DrawParam};
use ggez::{Context, GameResult};
use crate::structs::SpawnResult;
use std::fmt::Debug;

pub trait Component {
    fn spawn(&self, ctx: &mut Context, bound: Rect) -> SpawnResult;
    fn update(&mut self, _ctx: &mut Context) -> GameResult {Ok(())}
    fn on_click(&mut self, _ctx: &mut Context) -> GameResult {Ok(())}
    fn on_hover(&mut self, _ctx: &mut Context) -> GameResult {Ok(())}
}

pub trait Drawable: Debug {
    fn draw(&self, canvas: &mut Canvas, param: DrawParam);
    fn dimensions(&self, gfx: &Context) -> Option<Rect>;
}

impl<T: ggez::graphics::Drawable + Debug> Drawable for T {
    fn draw(&self, canvas: &mut Canvas, param: DrawParam) {
        ggez::graphics::Drawable::draw(self, canvas, param)
    }
    fn dimensions(&self, gfx: &Context) -> Option<Rect> {
        ggez::graphics::Drawable::dimensions(self, gfx)
    }
}

pub trait Pages: Default {}
