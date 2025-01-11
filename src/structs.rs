use ggez::graphics::{DrawParam, Rect};
use ggez::GameResult;
use ggez::glam::Vec2;
use crate::traits::Drawable;

#[derive(Debug)]
pub struct Spawned {
    pub drawable: Box<dyn Drawable>,
    pub bound: Rect,
    pub param: DrawParam
}
impl Spawned {
    pub fn new(drawable: impl Drawable + 'static, bound: Rect, param: Option<DrawParam>) -> Self {
        Spawned{
            drawable: Box::new(drawable),
            bound,
            param: param.unwrap_or(Vec2::new(bound.x, bound.y).into())
        }
    }
}
pub type SpawnResult = GameResult<Vec<Spawned>>;
