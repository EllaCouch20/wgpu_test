use ggez::graphics::{Rect, Image};
use ggez::{Context};
use crate::primitives::*;
use crate::Vec2;
use crate::ComponentBuilder;
use crate::structs::{BuildResult, px};
use crate::theme::*;

#[derive(Debug, Clone)]
pub struct Button(pub ButtonStyle, pub Size, pub &'static str);

impl ComponentBuilder for Button {
    fn build(&mut self, ctx: &mut Context, size: Vec2) -> BuildResult {
        let colors = palette().button.colors_from(self.0, ButtonState::Default);

        let (text_size, height) = match self.1 {
            Size::Medium => (32.0, px(ctx, 32.0)),
            Size::Large => (48.0, px(ctx, 48.0))
        };

        let label = CustomText(self.2, text_size).build(ctx, size)?;
        let label_size = label.size(ctx);

        let width = match self.1 {
            Size::Medium => label_size.x + 48.0,
            Size::Large => size.x
        };

        Component![
            (
                Rectangle {
                    height,
                    width,
                    radius: 50.0,
                    stroke: colors.outline,
                    color: colors.background,
                }.build(ctx, size)?,
                Rect::new(0.0, 0.0, size.x, size.y)
            ),
            (
                label,
                Rect::new((width-label_size.x) / 2., (height-label_size.y) / 2., size.x, size.y)
            ),
            // (
            //     Image::from_path(ctx, "/profile_picture.png")?,
            //     Rect::new(0.0, 0.0, 50.0, 50.0)
            // )
        ]
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
