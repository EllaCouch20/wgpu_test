use ggez::graphics::{Color, DrawParam, DrawMode, Mesh, Rect, Transform};
use ggez::{Context, GameResult};
use ggez::glam::Vec2;
use crate::traits::Component;
use crate::structs::{SpawnResult, Spawned};

pub struct Rectangle(pub f32, pub f32);

impl Component for Rectangle {
    fn spawn(&self, ctx: &mut Context, bound: Rect) -> SpawnResult {
        Ok(vec![Spawned::new(
            Mesh::new_rounded_rectangle(
                ctx,
                DrawMode::fill(),
                Rect::new(0.0, 0.0, bound.w*self.0, bound.h*self.1),
                40.0,
                Color::WHITE,
            )?,
            bound,
            None
        )])
    }
}

pub struct ExtRectangle(pub f32, pub f32);

impl Component for ExtRectangle {
    fn spawn(&self, ctx: &mut Context, bound: Rect) -> SpawnResult {
        Ok(vec![Spawned::new(
            Mesh::new_rounded_rectangle(
                ctx,
                DrawMode::fill(),
                Rect::new(0.0, 0.0, self.0, self.1),
                40.0,
                Color::WHITE,
            )?,
            bound,
            None
        )])
    }
}

pub struct Column(pub Vec<Box<dyn Component>>, pub f32);

impl Component for Column {
    fn spawn(&self, ctx: &mut Context, mut bound: Rect) -> SpawnResult {
        Ok(self.0.iter().map(|c| {
            let children = c.spawn(ctx, bound)?;
            let height = children.iter().fold(0, |height, c| {let rect = c.drawable.dimensions(ctx).unwrap(); std::cmp::max(height, (rect.y + rect.h) as u32)});
            bound.h -= height as f32;
            bound.y += self.1 + height as f32;
            Ok(children)
        }).collect::<GameResult<Vec<Vec<Spawned>>>>()?.into_iter().flatten().collect())
    }

    fn update(&mut self, _ctx: &mut Context) -> GameResult {Ok(())}
}

//  pub struct Stack(pub Vec<Box<dyn Component>>);

//  impl Component for Stack {
//      fn spawn(&self, ctx: &mut Context, bound: Rect) -> ComponentResult {
//          Ok(self.0.iter().map(|c| c.spawn(ctx, bound)).collect::<GameResult<Vec<Vec<SpawnedComponent>>>>()?.into_iter().flatten().collect())
//      }

//      fn update(&mut self, _ctx: &mut Context) -> GameResult {println!("Hello stack with {} len", self.0.len()); Ok(())}
//  }

//  pub struct Row(pub Vec<Box<dyn Component>>, pub f32);

//  impl Component for Row {
//      fn spawn(&self, ctx: &mut Context, mut bound: Rect) -> ComponentResult {
//          Ok(self.0.iter().map(|c| {
//              let children = c.spawn(ctx, bound)?;
//              let width = children.iter().fold(0, |width, (c, _)| {let rect = c.dimensions(ctx).unwrap(); std::cmp::max(width, (rect.x + rect.w) as u32)});
//              bound.w -= width as f32;
//              bound.x += self.1 + width as f32;
//              Ok(children)
//          }).collect::<GameResult<Vec<Vec<SpawnedComponent>>>>()?.into_iter().flatten().collect())
//      }

//      fn update(&mut self, _ctx: &mut Context) -> GameResult {Ok(())}
//  }

pub struct Container {
    pub component: Box<dyn Component>,
    pub width: f32,
    pub height: f32,
}

impl Container {
    pub fn new(component: impl Component + 'static, width: f32, height: f32) -> Self {
        Container{component: Box::new(component), width, height}
    }
}

impl Component for Container {
    fn spawn(&self, ctx: &mut Context, mut bound: Rect) -> SpawnResult {
        bound.w = std::cmp::min(bound.w as u32, self.width as u32) as f32;
        bound.h = std::cmp::min(bound.h as u32, self.height as u32) as f32;
        let mut spawns = self.component.spawn(ctx, bound)?;
        spawns.push(Spawned::new(
            Mesh::new_rectangle(
                ctx,
                DrawMode::fill(),
                Rect::new(0.0, 0.0, self.width, self.height),
                Color::GREEN,
            )?,
            bound,
            Some(<Vec2 as Into<DrawParam>>::into(Vec2::new(bound.x, bound.y)).z(-1))
        ));
        Ok(spawns)
    }

    fn update(&mut self, _ctx: &mut Context) -> GameResult {Ok(())}
}

pub struct Scrollable{
    pub component: Box<dyn Component>,
    pub scroll: f32
}

impl Scrollable {
    pub fn new(component: impl Component + 'static) -> Self {
        Scrollable{component: Box::new(component), scroll: 50.0}
    }
}

impl Component for Scrollable {
    fn spawn(&self, ctx: &mut Context, mut bound: Rect) -> SpawnResult {
        let mut children = self.component.spawn(ctx, bound)?;
        let content_height = children.iter().fold(0, |height, c| {let rect = c.drawable.dimensions(ctx).unwrap(); std::cmp::max(height, (rect.y + rect.h) as u32)});
        let max_scroll = std::cmp::max(0, content_height-bound.h as u32);
        let scroll = std::cmp::min(max_scroll, self.scroll as u32);

        children.iter_mut().for_each(|s| match s.param.transform {
            Transform::Values{dest, ..} => {
                println!("test: {}", dest.y);
                s.param = s.param.dest(Vec2::new(dest.x, dest.y-scroll as f32));
            },
            _ => {}
        });
        Ok(children)
    }

    fn update(&mut self, _ctx: &mut Context) -> GameResult {Ok(())}
}
