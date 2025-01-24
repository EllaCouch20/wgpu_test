use ggez::graphics::{DrawParam};
use ggez::{Context};
use crate::primitives::*;
use crate::traits::Drawable;
use crate::Vec2;
use crate::ComponentBuilder;
use crate::structs::{Child, Rect, px};
use crate::theme::*;
use super::CustomText;
use crate::GameResult;

#[derive(Debug, Clone)]
pub struct Button(pub ButtonStyle, pub Size, pub &'static str);

impl ComponentBuilder for Button {
    fn build_children(&mut self, ctx: &mut Context, window_size: Vec2) -> GameResult<Vec<Child>> {
        let colors = palette().button.colors_from(self.0, ButtonState::Default);

        let (text_size, height) = match self.1 {
            Size::Medium => (32.0, px(ctx, 32.0)),
            Size::Large => (48.0, px(ctx, 48.0))
        };

        let mut label = CustomText::label(self.2, text_size)
            .build(ctx, Rect::new(0.0, 0.0, window_size.x, window_size.y), true)?;

        let label_size = label.size(ctx);

        let width = match self.1 {
            Size::Medium => label_size.x + 48.0,
            Size::Large => window_size.x
        };

        label.1.x = (width - label_size.x) / 2.0;
        label.1.y = (height - label_size.y) / 2.0;

        Ok(vec![
            Box::new(
                Rectangle {
                    height,
                    width,
                    radius: 50.0,
                    stroke: colors.outline,
                    color: colors.background,
                }.build(ctx, Rect::new(0.0, 0.0, window_size.x, window_size.y), true)?,
            ),
            Box::new(label)
        ])
    }
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
