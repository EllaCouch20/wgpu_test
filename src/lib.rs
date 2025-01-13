use ggez::{
    event,
    glam::Vec2,
    event::{EventHandler, EventLoop},
    graphics::{self, Rect, Canvas},
    Context, GameResult, GameError, ContextBuilder,
    conf::{WindowSetup, WindowMode}
};

use std::{path, env};

pub mod components;
pub mod primitives;
pub mod structs;
pub mod traits;
pub mod theme;

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

        ctx.gfx.add_font(
            "Label",
            graphics::FontData::from_path(ctx, "/outfit_bold.ttf")?,
        );
        ctx.gfx.add_font(
            "Heading",
            graphics::FontData::from_path(ctx, "/outfit_bold.ttf")?,
        );
        ctx.gfx.add_font(
            "Text",
            graphics::FontData::from_path(ctx, "/outfit_regular.ttf")?,
        );

        let mut canvas = Canvas::from_frame(ctx, graphics::Color::BLACK);
        
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
        let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
            let mut path = path::PathBuf::from(manifest_dir);
            path.push("assets");
            path
        } else {
            path::PathBuf::from("./assets")
        };
    
        let (ctx, event_loop) = ContextBuilder::new("super_simple", "ggez")
            .window_setup(WindowSetup::default().title("RampDS"))
            .window_mode(
                WindowMode::default()
                    .dimensions(1440.0, 780.0)
                    .resizable(true),
            )
            .add_resource_path(resource_dir)
            .build()?;

        // let cb = ContextBuilder::new("super_simple", "ggez");
        // let (ctx, event_loop) = cb.build()?;
        Ok(Runtime{ctx, event_loop, state: State::new(root)?})
    }

    pub fn run(self) -> GameResult {
        event::run(self.ctx, self.event_loop, self.state)
    }
}
