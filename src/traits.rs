use ggez::graphics::{Rect, Canvas, DrawParam};
use ggez::{Context, GameResult};
use ggez::glam::Vec2;
use std::fmt::Debug;
use crate::structs::Component;
use dyn_clone::{DynClone, clone_trait_object};

pub trait ComponentBuilder: Debug + DynClone {
    fn build(&mut self, ctx: &mut Context, size: Vec2) -> GameResult<Component>;
    fn update(&mut self, _ctx: &mut Context) -> GameResult {Ok(())}
    fn on_click(&mut self, _ctx: &mut Context) -> GameResult {Ok(())}
    fn on_hover(&mut self, _ctx: &mut Context) -> GameResult {Ok(())}
}
clone_trait_object!(ComponentBuilder);

impl<C: ComponentBuilder + 'static> From<C> for Box<dyn ComponentBuilder> {
    fn from(builder: C) -> Self {
        Box::new(builder)
    }
}

pub trait Drawable: Debug + DynClone {
    fn draw(&self, canvas: &mut Canvas, bound: Rect, offset: Vec2, param: DrawParam);
    fn size(&self, ctx: &Context) -> Vec2;
}
clone_trait_object!(Drawable);

impl<T: ggez::graphics::Drawable + Debug + Clone> Drawable for T {
    fn draw(&self, canvas: &mut Canvas, bound: Rect, offset: Vec2, param: DrawParam) {
        println!("draw bound: {:?}", bound);
        println!("draw offset: {:?}", offset);
        param.dest(offset);
        if bound.w > 0.0 && bound.h > 0.0 {
            canvas.set_scissor_rect(bound).unwrap();
            ggez::graphics::Drawable::draw(self, canvas, param)
        }
    }

    //Offset built into the drawable is included in its size
    fn size(&self, ctx: &Context) -> Vec2 {
        let rect = ggez::graphics::Drawable::dimensions(self, ctx).unwrap_or_default();
        Vec2::new(rect.w+rect.x, rect.h+rect.y)
    }
}

pub trait Pages: Default {}
