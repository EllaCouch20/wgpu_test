use ggez::graphics::Rect;
use ggez::{Context, GameResult};
use crate::traits::{Drawable, Component};


pub struct Stack(pub Vec<Box<dyn Component>>);

impl Component for Stack {
    fn spawn(&self, ctx: &mut Context, drawable: Rect) -> GameResult<Vec<Box<dyn Drawable>>> {
        Ok(self.0.iter().map(|c| c.spawn(ctx, drawable)).collect::<GameResult<Vec<Vec<Box<dyn Drawable>>>>>()?.into_iter().flatten().collect())
    }

    fn update(&mut self, _ctx: &mut Context) -> GameResult {println!("Hello stack with {} len", self.0.len()); Ok(())}
}

