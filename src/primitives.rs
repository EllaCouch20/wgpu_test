use ggez::graphics::{Color, DrawMode, Mesh, Rect};
use ggez::{Context, GameResult};
use ggez::glam::Vec2;
use crate::traits::ComponentBuilder;
use crate::structs::Component;
use std::fmt::Debug;

pub use crate::Column;

#[derive(Debug, Clone)]
pub struct Rectangle {
    pub width: f32, 
    pub height: f32,
    pub color: Color,
    pub radius: f32,
}

impl ComponentBuilder for Rectangle {
    fn build(&mut self, ctx: &mut Context, size: Vec2) -> GameResult<Component> {
        Ok(Component::from(vec![
            (
                Mesh::new_rounded_rectangle(
                    ctx,
                    DrawMode::fill(),
                    Rect::new(0.0, 0.0, self.width, self.height),
                    self.radius,
                    self.color,
                )?,
                Rect::new(0.0, 0.0, size.x, size.y)
            )
        ]))
    }
}



// #[derive(Debug, Clone)]
// pub struct BorderedRectangle(pub f32, pub f32, );

// impl ComponentBuilder for BorderedRectangle {
//     fn build(&mut self, ctx: &mut Context, size: Vec2) -> GameResult<Component> {
//         Ok(Component::from(vec![
//             (
//                 Mesh::new_rounded_rectangle(
//                     ctx,
//                     DrawMode::fill(),
//                     Rect::new(0.0, 0.0, self.0*size.x, self.1*size.y),
//                     40.0,
//                     Color::WHITE,
//                 )?,
//                 Rect::new(20.0, 20.0, size.x, size.y)
//             )
//             (
//                 Mesh::new_rounded_rectangle(
//                     ctx,
//                     DrawMode::fill(),
//                     Rect::new(0.0, 0.0, self.0*size.x, self.1*size.y),
//                     40.0,
//                     Color::WHITE,
//                 )?,
//                 Rect::new(20.0, 20.0, size.x, size.y)
//             )
//         ]))
//     }
// }

#[derive(Debug, Clone)]
pub struct ExtRectangle(pub f32, pub f32);

impl ComponentBuilder for ExtRectangle {
    fn build(&mut self, ctx: &mut Context, size: Vec2) -> GameResult<Component> {
        Ok(Component::from(vec![
            (
                Mesh::new_rounded_rectangle(
                    ctx,
                    DrawMode::fill(),
                    Rect::new(0.0, 0.0, self.0, self.1),
                    40.0,
                    Color::WHITE,
                )?,
                Rect::new(0.0, 0.0, size.x, size.y)
            )
        ]))
    }
}

//  pub struct Stack(Vec<Component>);

//  impl<V: Into<Component>> From<Vec<V>> for Stack {
//      fn from(v: Vec<V>) -> Self {
//          Stack(v.into_iter().map(|v| v.into()).collect())
//      }
//  }

//  impl ComponentBuilder for Stack {
//      fn build(&self, ctx: &mut Context, size: Vec2) -> GameResult<Component> {
//          Ok(Component::from(self.0.clone()))
//      }
//  }

#[derive(Debug, Clone)]
pub struct Center<C: ComponentBuilder + Clone>(pub C);

impl<C: ComponentBuilder + Clone> ComponentBuilder for Center<C> {
    fn build(&mut self, ctx: &mut Context, size: Vec2) -> GameResult<Component> {
        let component = self.0.build(ctx, Vec2::new(size.x, size.y))?;
        let c_size = component.size(ctx);
        Ok(Component::from(vec![
            (
                component,
                Rect::new((size.x-c_size.x)/2.0, (size.y-c_size.y)/2.0, size.x, size.y)
            )
        ]))
    }
}

#[derive(Debug, Clone)]
pub struct Container<C: ComponentBuilder + Clone>(pub C, pub f32, pub f32);

impl<C: ComponentBuilder + Clone> ComponentBuilder for Container<C> {
    fn build(&mut self, ctx: &mut Context, size: Vec2) -> GameResult<Component> {
        Ok(Component::from(vec![
            (
                self.0.build(ctx, Vec2::new(size.x*self.1, size.y*self.2))?,
                Rect::new(0.0, 0.0, size.x, size.y)
            )
        ]))
    }
}



#[derive(Debug, Clone)]
pub struct Column(pub Vec<Box<dyn ComponentBuilder>>, pub f32);

impl ComponentBuilder for Column {
    fn build(&mut self, ctx: &mut Context, size: Vec2) -> GameResult<Component> {
        let mut bound = Rect::new(0.0, 0.0, size.x, size.y);
        Ok(Component::from(
            self.0.clone().into_iter().map(|mut builder| {
                let child = builder.build(ctx, Vec2::new(bound.w, bound.h))?;
                let height = child.size(ctx).y;
                let res = (child, bound);
                bound.h -= height as f32;
                bound.y += self.1 + height as f32;
                Ok(res)
            }).collect::<GameResult<Vec<(Component, Rect)>>>()?
        ))
    }
}

#[macro_export]
macro_rules! Column {
    ($x:literal, $($i:expr),*) => {{
        Column(vec![
            $(Box::new($i) as Box<dyn ramp_ds::traits::ComponentBuilder>),*
        ], $x)
    }}
}


impl<V: Into<Box<dyn ComponentBuilder>>> From<Vec<V>> for Column {
    fn from(v: Vec<V>) -> Self {
        Column(v.into_iter().map(|v| v.into()).collect(), 32.0)
    }
}


//  //  pub struct Stack(pub Vec<Box<dyn Component>>);

//  //  impl Component for Stack {
//  //      fn spawn(&self, ctx: &mut Context, bound: Rect) -> ComponentResult {
//  //          Ok(self.0.iter().map(|c| c.spawn(ctx, bound)).collect::<GameResult<Vec<Vec<SpawnedComponent>>>>()?.into_iter().flatten().collect())
//  //      }

//  //      fn update(&mut self, _ctx: &mut Context) -> GameResult {println!("Hello stack with {} len", self.0.len()); Ok(())}
//  //  }

//  //  pub struct Row(pub Vec<Box<dyn Component>>, pub f32);

//  //  impl Component for Row {
//  //      fn spawn(&self, ctx: &mut Context, mut bound: Rect) -> ComponentResult {
//  //          Ok(self.0.iter().map(|c| {
//  //              let children = c.spawn(ctx, bound)?;
//  //              let width = children.iter().fold(0, |width, (c, _)| {let rect = c.dimensions(ctx).unwrap(); std::cmp::max(width, (rect.x + rect.w) as u32)});
//  //              bound.w -= width as f32;
//  //              bound.x += self.1 + width as f32;
//  //              Ok(children)
//  //          }).collect::<GameResult<Vec<Vec<SpawnedComponent>>>>()?.into_iter().flatten().collect())
//  //      }
//  //  }

//  pub struct Scrollable{
//      pub component: Box<dyn Component>,
//      pub scroll: f32
//  }

//  impl Scrollable {
//      pub fn new(component: impl Component + 'static) -> Self {
//          Scrollable{component: Box::new(component), scroll: 50.0}
//      }
//  }

//  impl Component for Scrollable {
//      fn spawn(&self, ctx: &mut Context, mut bound: Rect) -> SpawnResult {
//          let mut children = self.component.spawn(ctx, bound)?;
//          let content_height = children.iter().fold(0, |height, c| {let rect = c.drawable.dimensions(ctx).unwrap(); std::cmp::max(height, (rect.y + rect.h) as u32)});
//          let max_scroll = std::cmp::max(0, content_height-bound.h as u32);
//          let scroll = std::cmp::min(max_scroll, self.scroll as u32);

//          children.iter_mut().for_each(|s| match s.param.transform {
//              Transform::Values{dest, ..} => {
//                  s.param = s.param.dest(Vec2::new(dest.x, dest.y-scroll as f32));
//              },
//              _ => {}
//          });
//          Ok(children)
//      }

//      fn update(&mut self, _ctx: &mut Context) -> GameResult {Ok(())}
//  }
