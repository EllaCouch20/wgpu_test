#![allow(clippy::unnecessary_wraps)]

use ggez::{
    event,
    glam::*,
    graphics::{self, Color, Rect},
    Context, GameResult,
};

struct MainState {
    circle: graphics::Mesh,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let circle = graphics::Mesh::new_rounded_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            Rect::new(0.0, 0.0, 500.0, 500.0),
            40.0,
            Color::WHITE,
        )?;

        Ok(MainState { circle })
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([0.1, 0.2, 0.3, 1.0]));

        // Get the screen dimensions
        let screen_size = ctx.gfx.drawable_size();
        let screen_width = screen_size.0;
        let screen_height = screen_size.1;

        // Center the rectangle
        let rect_center_x = screen_width / 2.0 - 250.0; // 250 is half the rectangle's width
        let rect_center_y = screen_height / 2.0 - 250.0; // 250 is half the rectangle's height

        canvas.draw(&self.circle, Vec2::new(rect_center_x, rect_center_y));

        canvas.finish(ctx)?;

        Ok(())
    }
}

pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("super_simple", "ggez");
    let (mut ctx, event_loop) = cb.build()?;
    let state = MainState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}

fn is_in_bounds(rect: Rect, mouse_coords: Mous) -> bool {

}