use ggez::graphics::{Color, DrawMode, Mesh, Rect};
use ggez::{Context, GameResult};
use super::traits::{Drawable, Component};

//TODO: move to own files

pub struct ABox(pub f32, pub f32);

pub type SpawnedComponent = (Box<dyn Drawable>, Rect);
pub type ComponentResult = GameResult<Vec<SpawnedComponent>>;

impl Component for ABox {
    fn spawn(&self, ctx: &mut Context, bound: Rect) -> ComponentResult {
        let circle = Mesh::new_rounded_rectangle(
            ctx,
            DrawMode::fill(),
            Rect::new(0.0, 0.0, self.0, self.1),
            40.0,
            Color::WHITE,
        )?;

        Ok(vec![(Box::new(circle), bound)])
    }
}

pub struct Stack(pub Vec<Box<dyn Component>>);

impl Component for Stack {
    fn spawn(&self, ctx: &mut Context, bound: Rect) -> ComponentResult {
        Ok(self.0.iter().map(|c| c.spawn(ctx, bound)).collect::<GameResult<Vec<Vec<SpawnedComponent>>>>()?.into_iter().flatten().collect())
    }

    fn update(&mut self, _ctx: &mut Context) -> GameResult {println!("Hello stack with {} len", self.0.len()); Ok(())}
}


pub struct Column(pub Vec<Box<dyn Component>>, pub f32);

impl Component for Column {
    fn spawn(&self, ctx: &mut Context, mut bound: Rect) -> ComponentResult {
        Ok(self.0.iter().map(|c| {
            let children = c.spawn(ctx, bound)?;
            let height = children.iter().fold(0, |height, (c, _)| {let rect = c.dimensions(ctx).unwrap(); std::cmp::max(height, (rect.y + rect.h) as u32)});
            bound.h -= height as f32;
            bound.y += self.1 + height as f32;
            Ok(children)
        }).collect::<GameResult<Vec<Vec<SpawnedComponent>>>>()?.into_iter().flatten().collect())
    }

    fn update(&mut self, _ctx: &mut Context) -> GameResult {Ok(())}
}

//  impl Column {
//      fn get_bounding_size(children: &[Box<dyn Drawable>]) -> (f32, f32) {
//          let rects = children.iter().map(|c| c.dimensions()).collect::<Vec<_>>();

//          let width = rects.iter().fold(0.0, |width, rect| std::cmp::max(width, rect.y + rect.w));
//          let height = rects.iter().fold(0.0, |height, rect| std::cmp::max(height, rect.x + rect.h));
//          (width, height)
//      }
//  }
