use ggez::graphics::{Canvas, DrawParam};
use ggez::{Context, GameResult};
use std::fmt::Debug;
use crate::structs::{Component, Child, Vec2, Rect};
use dyn_clone::{DynClone, clone_trait_object};

//  pub trait Child: Debug + DynClone {
//      pub fn size(&self, ctx: &Context) -> Vec2;
//      //pub fn draw(&self, ctx: &Context) -> Vec2;
//  }
//  clone_trait_object!(Child);

pub trait ComponentBuilder: Debug + DynClone {
    fn build_children(&mut self, ctx: &mut Context, window_size: Vec2) -> GameResult<Vec<Child>>;

    fn build(&mut self, ctx: &mut Context, window: Rect, shrink_to_fit: bool) -> GameResult<Component> {
        Ok(Component(self.build_children(ctx, window.size())?, window.position(), window.size(), shrink_to_fit))
    }

    fn build_child(&mut self, ctx: &mut Context, window: Rect, shrink_to_fit: bool) -> GameResult<Child> {
        Ok(Child::new_component(self.build(ctx, window, shrink_to_fit)?))
    }

    //events
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
    fn offset(&self, ctx: &Context) -> Vec2;
}
clone_trait_object!(Drawable);

impl<T: ggez::graphics::Drawable + Debug + Clone> Drawable for T {
    fn draw(&self, canvas: &mut Canvas, window: Rect, offset: Vec2, param: DrawParam) {
        println!("window: {:?}", window);
        println!("offset: {:?}", offset);
        if canvas.set_scissor_rect(window.into()).is_ok() {
            println!("OK");
            ggez::graphics::Drawable::draw(self, canvas, param.dest(offset))
        }
    }

    fn size(&self, ctx: &Context) -> Vec2 {
        let rect = ggez::graphics::Drawable::dimensions(self, ctx).unwrap_or_default();
        Vec2::new(rect.w, rect.h)
    }

    fn offset(&self, ctx: &Context) -> Vec2 {
        let rect = ggez::graphics::Drawable::dimensions(self, ctx).unwrap_or_default();
        Vec2::new(rect.x, rect.y)
    }
}

pub trait Pages: Default {}
