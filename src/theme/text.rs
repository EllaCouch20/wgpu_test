use ggez::Context;
use ggez::graphics::{
    FontData,
    TextFragment,
    Text,
    Rect,
    Color,
};
use crate::Component;
use crate::ComponentBuilder;
use crate::structs::*;

pub fn load_fonts(ctx: &mut Context) -> Result<(), Box<dyn std::error::Error>> {
    ctx.gfx.add_font(
        "Label",
        FontData::from_path(ctx, "/outfit_bold.ttf")?,
    );
    ctx.gfx.add_font(
        "Heading",
        FontData::from_path(ctx, "/outfit_bold.ttf")?,
    );
    ctx.gfx.add_font(
        "Text",
        FontData::from_path(ctx, "/outfit_regular.ttf")?,
    );

    Ok(())
}

#[derive(Clone, Debug)]
pub struct CustomText(pub &'static str, pub f32);

impl ComponentBuilder for CustomText {
    fn build(&mut self, ctx: &mut Context, size: Vec2) -> BuildResult {
        Component![(
            Text::new(
                TextFragment {
                    text: self.0.to_string(),
                    scale: Some(self.1.into()),
                    font: Some("Label".into()),
                    color: Some(Color::WHITE)
                }
            ),
            Rect::new(0.0, 0.0, size.x, size.y)
        )]
    }
}