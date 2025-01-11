use ggez::graphics::Rect;
use ggez::{Context, GameResult};
use crate::traits::{Drawable, Component};


pub struct Column(pub Vec<Box<dyn Component>>, pub f32);

impl Component for Column {
    fn spawn(&self, ctx: &mut Context, mut drawable: Rect) -> GameResult<Vec<Box<dyn Drawable>>> {
        Ok(self.0.iter().map(|c| {
            let children = c.spawn(ctx, drawable)?;
            let height = children.iter().fold(0.0, |height, c| {let rect = c.dimensions(); std::cmp::max(height, rect.x + rect.h)});
            Ok(children)
        }).collect::<GameResult<Vec<Vec<Box<dyn Drawable>>>>>()?.into_iter().flatten().collect())
    }

    fn update(&mut self, _ctx: &mut Context) -> GameResult {println!("Hello stack with {} len", self.0.len()); Ok(())}
}

//  impl Column {
//      fn get_bounding_size(children: &[Box<dyn Drawable>]) -> (f32, f32) {
//          let rects = children.iter().map(|c| c.dimensions()).collect::<Vec<_>>();

//          let width = rects.iter().fold(0.0, |width, rect| std::cmp::max(width, rect.y + rect.w));
//          let height = rects.iter().fold(0.0, |height, rect| std::cmp::max(height, rect.x + rect.h));
//          (width, height)
//      }
//  }
