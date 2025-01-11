use ggez::graphics::{Rect, Canvas, DrawParam};
use ggez::{Context, GameResult};

pub trait Component {
    fn spawn(&self, ctx: &mut Context, bound: Rect) -> GameResult<Vec<(Box<dyn Drawable>, Rect)>>;
    fn update(&mut self, _ctx: &mut Context) -> GameResult {Ok(())}
    //TODO: fill out rest of events defaulting to no behavior
}

pub trait Drawable {
    fn draw(&self, canvas: &mut Canvas, param: DrawParam);
    fn dimensions(&self, gfx: &Context) -> Option<Rect>;
}

impl<T: ggez::graphics::Drawable> Drawable for T {
    fn draw(&self, canvas: &mut Canvas, param: DrawParam) {
        ggez::graphics::Drawable::draw(self, canvas, param)
    }
    fn dimensions(&self, gfx: &Context) -> Option<Rect> {
        ggez::graphics::Drawable::dimensions(self, gfx)
    }
}

pub trait Pages: Default {}
