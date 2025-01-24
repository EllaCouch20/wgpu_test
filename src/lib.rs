//pub mod components;
pub mod primitives;
pub mod structs;
pub mod traits;
//pub mod theme;

use traits::Drawable;

use ggez::{
    event,
    event::{EventHandler, EventLoop},
    graphics::{self, Canvas},
    Context, GameResult, GameError, ContextBuilder,
    conf::{WindowSetup, WindowMode}
};

use std::{path, env};

//use theme::*;

use traits::{ComponentBuilder};
use structs::{Rect, Vec2};

struct State(Box<dyn ComponentBuilder>);

impl State {
    fn new(root: impl ComponentBuilder + 'static) -> GameResult<State> {
        Ok(State(Box::new(root)))
    }
}

impl EventHandler<GameError> for State {
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        println!("-----------------------------------------DRAW-------------------------------");
        let screen_size = ctx.gfx.drawable_size();
        let screen_width = screen_size.0;
        let screen_height = screen_size.1;
        //let _fonts = load_fonts(ctx);

        let mut canvas = Canvas::from_frame(ctx, graphics::Color::BLACK);

        let offset = Vec2::new(0.0, 0.0);
        let window = Rect::new(offset.x, offset.y, screen_width, screen_height);

        //window.x,y offset from parent
        //window.w,h max size of element

        //window.x,y offset from top left of screen//Window offset
        //window.w,h screen size

        //offset.x,y offset from top left of screen//Actual element offset
        let page = self.0.build(ctx, window, false)?;
        println!("Page: {:#?}", page);
        page.draw(&mut canvas, window, offset);

        canvas.finish(ctx)?;
        Ok(())
    }

    fn update(&mut self, ctx: &mut Context) -> GameResult {self.0.update(ctx)}

//  fn mouse_button_down_event(
//      &mut self, ctx: &mut Context, b: MouseButton, x: f32, y: f32
//  ) -> GameResult {self.0.mouse_button_down_event(ctx, b, x, y)}

//  fn mouse_button_up_event(
//      &mut self, ctx: &mut Context, b: MouseButton, x: f32, y: f32
//  ) -> GameResult {self.0.mouse_button_up_event(ctx, b, x, y)}

//  fn mouse_motion_event(
//      &mut self, ctx: &mut Context, x: f32, y: f32, dx: f32, dy: f32
//  ) -> GameResult {self.0.mouse_motion_event(ctx, x, y, dx, dy)}

//  fn mouse_enter_or_leave(
//      &mut self, ctx: &mut Context, entered: bool
//  ) -> GameResult {self.0.mouse_enter_or_leave(ctx, entered)}

//  fn mouse_wheel_event(
//      &mut self, ctx: &mut Context, x: f32, y: f32
//  ) -> GameResult {self.0.mouse_wheel_event(ctx, x, y)}

//  fn key_down_event(
//      &mut self, ctx: &mut Context, input: KeyInput, repeated: bool
//  ) -> GameResult {self.0.key_down_event(ctx, input, repeated)}

//  fn key_up_event(
//      &mut self, ctx: &mut Context, input: KeyInput
//  ) -> GameResult {self.0.key_up_event(ctx, input)}

//  fn text_input_event(
//      &mut self, ctx: &mut Context, character: char
//  ) -> GameResult {self.0.text_input_event(ctx, character)}

//  fn touch_event(
//      &mut self, ctx: &mut Context, phase: TouchPhase, x: f64, y: f64
//  ) -> GameResult {self.0.touch_event(ctx, phase, x, y)}

//  fn gamepad_button_down_event(
//      &mut self, ctx: &mut Context, btn: Button, id: GamepadId
//  ) -> GameResult {self.0.gamepad_button_down_event(ctx, btn, id)}

//  fn gamepad_button_up_event(
//      &mut self, ctx: &mut Context, btn: Button, id: GamepadId
//  ) -> GameResult {self.0.gamepad_button_up_event(ctx, btn, id)}

//  fn gamepad_axis_event(
//      &mut self, ctx: &mut Context, axis: Axis, value: f32, id: GamepadId
//  ) -> GameResult {self.0.gamepad_axis_event(ctx, axis, value, id)}

//  fn focus_event(
//      &mut self, ctx: &mut Context, gained: bool
//  ) -> GameResult {self.0.focus_event(ctx, gained)}


//  fn resize_event(
//      &mut self, ctx: &mut Context, width: f32, height: f32
//  ) -> GameResult {self.0.resize_event(ctx, width, height)}

//  fn quit_event(&mut self, _ctx: &mut Context) -> GameResult<bool> {
//      //Shut down threads
//      Ok(true)
//  }

//  fn on_error(
//      &mut self, _ctx: &mut Context, _origin: ErrorOrigin, _e: E
//  ) -> bool {
//      //Trigger error page
//      true
//  }

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
            .window_setup(
                WindowSetup::default()
                    .title("RampDS")
                    .samples(ggez::conf::NumSamples::Four)
            )
            .window_mode(
                WindowMode::default()
                    .dimensions(1440.0, 780.0)
                    .resizable(true),
            )
            .add_resource_path(resource_dir)
            .build()?;
        Ok(Runtime{ctx, event_loop, state: State::new(root)?})
    }

    pub fn run(self) -> GameResult {
        event::run(self.ctx, self.event_loop, self.state)
    }
}
