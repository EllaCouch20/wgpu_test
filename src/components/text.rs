use ggez::Context;
use ggez::graphics::{
    TextFragment,
    Text,
    Rect,
    Color,
    DrawParam,
};
use crate::GameResult;
use crate::ComponentBuilder;
use crate::structs::*;
use crate::palette;

#[derive(Clone, Debug)]
pub struct CustomText(pub &'static str, pub f32, pub &'static str, pub Color);

impl CustomText {
    pub fn heading(t: &'static str, s: f32) -> Self {
        Self(t, s, "Heading", palette().text.heading)
    } 
    pub fn label(t: &'static str, s: f32) -> Self {
        Self(t, s, "Label", palette().text.heading)
    } 
    pub fn primary(t: &'static str, s: f32) -> Self {
        Self(t, s, "Text", palette().text.primary)
    } 
    pub fn secondary(t: &'static str, s: f32) -> Self {
        Self(t, s, "Text", palette().text.secondary)
    }
}

impl ComponentBuilder for CustomText {
    fn build_children(&mut self, ctx: &mut Context, window_size: Vec2) -> GameResult<Vec<Child>> {
        Ok(vec![
            Box::new((
                Text::new(
                    TextFragment {
                        text: self.0.to_string(),
                        scale: Some(self.1.into()),
                        font: Some(self.2.into()),
                        color: Some(self.3)
                    }
                ),
                DrawParam::default().scale(Vec2::new(1.0, 1.0))//Offset in DrawParam is ignored
            )),
        ])
    }
}
