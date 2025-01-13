use ggez::graphics::{Color, Rect, Text, TextFragment, PxScale, Image};
use ggez::{Context};
use crate::primitives::*;
use crate::traits::{Drawable};
use crate::Vec2;
use crate::ComponentBuilder;
use crate::GameResult;
use crate::graphics::Mesh;
use crate::graphics::DrawMode;
use crate::structs::Component;

use crate::theme::color::{
    ButtonColors,
    ButtonSchemes,
    hex,
};


#[derive(Debug, Clone)]
pub struct Button(pub ButtonStyle, pub Size);

impl ComponentBuilder for Button {
    fn build(&mut self, ctx: &mut Context, size: Vec2) -> GameResult<Component> {
        let palette = ButtonColors::new(ButtonSchemes::default());

        let colors = palette.colors_from(self.0, ButtonState::Default);

        let (text_size, height) = match self.1 {
            Size::Medium => (32.0, px(32.0)),
            Size::Large => (48.0, px(48.0))
        };

        let text = ;
        println!("size: {:?}", text.size(ctx));

        Ok(Component::from(vec![
            (
                Rectangle {
                    height,
                    width: size.x,
                    radius: 50.0,
                    color: colors.background,
                }.build(ctx, size)?,
                Rect::new(0.0, 0.0, size.x, size.y)
            ),
            (
                Center(CustomText("Continue", text_size)).build(ctx, size)?,
                Rect::new(0.0, 0.0, size.x, size.y),
            )
        ]))
    }
}


#[derive(Clone, Debug)]
pub struct CustomText(&'static str, f32);

impl ComponentBuilder for CustomText {
    fn build(&mut self, ctx: &mut Context, size: Vec2) -> GameResult<Component> {
        Ok(Component::from(vec![(
            Text::new(
                TextFragment {
                    text: self.0.to_string(),
                    scale: Some(PxScale::from(self.1)),
                    font: Some("Label".into()),
                    color: Some(Color::WHITE)
                }
            ),
            Rect::new(0.0, 0.0, size.x, size.y)
        )]))
    }
}


// impl Component for Button {
//     fn spawn(&self, ctx: &mut Context, mut bound: Rect) -> SpawnResult {
//         let palette = ButtonColors::new(
//             ButtonSchemes::default(),
//         );

//         let colors = palette.colors_from(self.style, ButtonState::Default);

//         let (text_size, height) = match self.size {
//             Size::Medium => (32.0, 32.0),
//             Size::Large => (48.0, 48.0)
//         };

//         // let label = CustomText::new(ctx, "Continue", text_size);

//         Stack(vec![
//             Box::new(BorderedRectangle::new(bound.w, px(text_size), colors.background, colors.outline, 50.0, px(1.0))),
//             Box::new(CustomText::new(ctx, "Continue", text_size)),
//             Box::new(Icon::new("/profile_picture.png", 32.0))
//         ]).spawn(ctx, bound)
//     }
// }

// pub struct Icon(pub String, pub f32);

// impl Icon {
//     pub fn new(p: &str, s: f32) -> Self {
//         Self(p.to_string(), s)
//     }
// }

// impl Component for Icon {
//     fn spawn(&self, ctx: &mut Context, mut bound: Rect) -> SpawnResult {
//         bound.h = 100.0;
//         bound.w = 100.0;
//         Ok(vec![Spawned::new(
//             Image::from_path(ctx, self.0.clone())?,
//             bound,
//             None
//         )])
//     }
// }

// // pub struct Label(String, f32);

// // impl Label {
// //     pub fn new(t: &str, s: f32) -> Self {
// //         Self(t.to_string(), s)
// //     }
// // }

// #[derive(Clone)]
// pub struct CustomText(Text, Rect, Alignment);

// impl CustomText {
//     pub fn new(ctx: &mut Context, t: &str, s: f32) -> Self {
//         let txt = Text::new(
//             TextFragment {
//                 text: t.to_string(),
//                 scale: Some(PxScale::from(s)),
//                 font: Some("Label".into()),
//                 color: Some(Color::WHITE)
//             }
//         );

//         Self(txt.clone(), txt.dimensions(ctx).unwrap_or_default(), Alignment::Center)
//     }
// }

// impl Component for CustomText {
//     fn spawn(&self, ctx: &mut Context, mut bound: Rect) -> SpawnResult {
//         if self.2 == Alignment::Center {
//             bound.y += self.1.h / 2.0;
//             bound.x += (bound.w - self.1.w) / 2.0;
//         }
//         Ok(vec![Spawned::new(
//             self.0.clone(),
//             bound,
//             None
//         )])
//     }
// }

pub fn px(a: f32) -> f32 {
    a * 2.0
}

#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
pub enum ButtonStyle {
    Primary,
    Secondary,
    Ghost
}

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
pub enum ButtonState {
    Default,
    Disabled,
    Selected,
    Hover,
}

#[derive(Debug, Clone)]
pub enum Size {
    Large,
    Medium,
}

#[derive(PartialEq, Eq, Clone)]
pub enum Alignment {
    Left,
    Right,
    Top,
    Bottom,
    Center,
}