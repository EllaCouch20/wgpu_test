use ggez::graphics::{Image, DrawParam};
use ggez::{Context};
use crate::primitives::*;
use crate::traits::Drawable;
use crate::Vec2;
use crate::ComponentBuilder;
use crate::structs::{Child, Rect, px};
use crate::theme::*;
use super::CustomText;
use super::Icon;
use crate::GameResult;
use crate::traits;

#[derive(Debug, Clone)]
pub struct Button{
    pub style: ButtonStyle, 
    pub size: Size, 
    pub width: Width, 
    pub icon: Option<&'static str>, 
    pub label: &'static str
}

impl Button {
    pub fn primary(size: Size, width: Width, icon: Option<&'static str>, label: &'static str) -> Self {
        Self { style: ButtonStyle::Primary, size, width, icon, label }
    }
    pub fn secondary(size: Size, width: Width, icon: Option<&'static str>, label: &'static str) -> Self {
        Self { style: ButtonStyle::Secondary, size, width, icon, label }
    }
    pub fn ghost(size: Size, width: Width, icon: Option<&'static str>, label: &'static str) -> Self {
        Self { style: ButtonStyle::Ghost, size, width, icon, label }
    }
}

impl ComponentBuilder for Button {
    fn build_children(&mut self, ctx: &mut Context, parent_size: Vec2) -> GameResult<Vec<Child>> {
        let colors = palette().button.colors_from(self.style, ButtonState::Default);

        let (text_size, height, icon_size) = match self.size {
            Size::Medium => (32.0, px(ctx, 32.0), px(ctx, 16.0)),
            Size::Large => (48.0, px(ctx, 48.0), px(ctx, 24.0))
        };

        let mut label = CustomText::label(self.label, text_size)
            .build(ctx, Rect::new(0.0, 0.0, parent_size.x, parent_size.y), true)?;

        let label_size = label.size(ctx);

        let width = match self.width {
            Width::Hug => label_size.x + 48.0,
            Width::Expand => parent_size.x
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
                }.build(ctx, Rect::new(0.0, 0.0, parent_size.x, parent_size.y), true)?,
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
pub enum Width {
    Expand,
    Hug,
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
