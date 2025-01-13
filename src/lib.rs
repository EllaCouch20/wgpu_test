use ggez::{
    event,
    glam::Vec2,
    event::{EventHandler, EventLoop},
    graphics::{self, Rect, Canvas},
    Context, GameResult, GameError, ContextBuilder
};

pub mod primitives;
pub mod structs;
pub mod traits;

use traits::{ComponentBuilder};

struct State(Box<dyn ComponentBuilder>);

impl State {
    fn new(root: impl ComponentBuilder + 'static) -> GameResult<State> {
        Ok(State(Box::new(root)))
    }
}

impl EventHandler<GameError> for State {
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let screen_size = ctx.gfx.drawable_size();
        let screen_width = screen_size.0;
        let screen_height = screen_size.1;

        let mut canvas = Canvas::from_frame(ctx, graphics::Color::from([0.1, 0.2, 0.3, 1.0]));
        let bound = Rect::new(10.0, 10.0, screen_width-20.0, screen_height-20.0);
        self.0.build(ctx, Vec2::new(screen_width-20.0, screen_height-20.0))?.draw(&mut canvas, bound);
        canvas.finish(ctx)?;
        Ok(())
    }

    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.0.update(ctx)
    }

    //TODO: Fill out rest of events
}

pub struct Runtime {
    ctx: Context,
    event_loop: EventLoop<()>,
    state: State
}

impl Runtime {
    pub fn new(root: impl ComponentBuilder + 'static) -> GameResult<Self> {
        let cb = ContextBuilder::new("super_simple", "ggez");
        let (ctx, event_loop) = cb.build()?;
        Ok(Runtime{ctx, event_loop, state: State::new(root)?})
    }

    pub fn run(self) -> GameResult {
        event::run(self.ctx, self.event_loop, self.state)
    }
}
