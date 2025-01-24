

use ggez::Context;
use ggez::graphics::{
    TextFragment,
    Text,
    Rect,
    Color,
    DrawParam,
    Image,
};
use crate::GameResult;
use crate::ComponentBuilder;
use crate::structs::*;
use crate::palette;

#[derive(Clone, Debug)]
pub struct Icon(pub &'static str, pub f32);

impl ComponentBuilder for Icon {
    fn build_children(&mut self, ctx: &mut Context, parent_size: Vec2) -> GameResult<Vec<Child>> {
        let scale = (1.0 / 192.0) * px(ctx, self.1);
        Ok(vec![
            Box::new((
                Image::from_path(ctx, &format!("/{}.png", self.0)).unwrap(),
                DrawParam::default().scale(Vec2::new(scale, scale))
            ))
        ])
    }
}


// icon must be exported as 192x192
