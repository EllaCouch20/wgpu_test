use ggez::graphics::{Color, DrawMode, Mesh, Rect};
use ggez::{Context, GameResult};
use super::traits::{Drawable, Component};

//TODO: move to own files

pub struct ABox(pub f32, pub f32);

impl Component for ABox {
    fn spawn(&self, ctx: &mut Context, bound: Rect) -> GameResult<Vec<Box<dyn Drawable>>> {
        let circle = Mesh::new_rounded_rectangle(
            ctx,
            DrawMode::fill(),
            Rect::new(0.0, 0.0, bound.w*self.0, bound.h*self.1),
            40.0,
            Color::WHITE,
        )?;

        Ok(vec![Box::new(circle)])
    }
}

pub struct Stack(pub Vec<Box<dyn Component>>);

impl Component for Stack {
    fn spawn(&self, ctx: &mut Context, drawable: Rect) -> GameResult<Vec<Box<dyn Drawable>>> {
        Ok(self.0.iter().map(|c| c.spawn(ctx, drawable)).collect::<GameResult<Vec<Vec<Box<dyn Drawable>>>>>()?.into_iter().flatten().collect())
    }

    fn update(&mut self, _ctx: &mut Context) -> GameResult {println!("Hello stack with {} len", self.0.len()); Ok(())}
}


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
