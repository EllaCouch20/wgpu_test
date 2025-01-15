use ggez::Context;
use ggez::graphics::{
    TextFragment,
    Text,
    Rect,
    Color,
};
use crate::Component;
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
    fn build(&mut self, _ctx: &mut Context, size: Vec2) -> BuildResult {
        Component![(
            Text::new(
                TextFragment {
                    text: self.0.to_string(),
                    scale: Some(self.1.into()),
                    font: Some(self.2.into()),
                    color: Some(self.3)
                }
            ),
            Rect::new(0.0, 0.0, size.x, size.y),
            None
        )]
    }
}