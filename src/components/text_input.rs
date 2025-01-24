use ggez::graphics::{DrawParam};
use ggez::{Context};
use crate::primitives::*;
use crate::traits::Drawable;
use crate::Vec2;
use crate::ComponentBuilder;
use crate::structs::{Child, Rect, px};
use crate::theme::*;
use crate::components::*;
use super::CustomText;
use crate::GameResult;

#[derive(Debug, Clone)]
pub struct TextInput(pub &'static str);

impl ComponentBuilder for TextInput {
    fn build_children(&mut self, ctx: &mut Context, parent_size: Vec2) -> GameResult<Vec<Child>> {
        let colors = palette().button.colors_from(ButtonStyle::Secondary, ButtonState::Default);

        Ok(vec![
            Box::new(
                Rectangle {
                    height: px(ctx, 48.0),
                    width: parent_size.x,
                    radius: 24.0,
                    stroke: colors.outline,
                    color: colors.background,
                }.build(ctx, Rect::new(0.0, 0.0, parent_size.x, parent_size.y), true)?,
            )
        ])
    }
}
