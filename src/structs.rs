use ggez::graphics::Canvas;
use ggez::{Context};
use ggez::{graphics, glam, mint};
use crate::traits::Drawable;

pub fn min(x: f32, y: f32) -> f32 {
    std::cmp::min(x as i32, y as i32) as f32
}

pub fn max(x: f32, y: f32) -> f32 {
    std::cmp::max(x as i32, y as i32) as f32
}

pub fn px(ctx: &mut Context, a: f32) -> f32 {
    let scale_factor = ctx.gfx.window().scale_factor(); // DPI scale factor
    a * scale_factor as f32
}

pub type Child = Box<dyn Drawable>;

#[derive(Clone, Debug)]
pub struct Component(pub Vec<Child>, pub Rect, pub bool);//(Offset, size), STFu

impl Drawable for Component {
    fn draw(&self, canvas: &mut Canvas, window: Rect, offset: Vec2) {
        let window = Rect::new(
            max(window.x, window.x+self.1.x), max(window.y, window.y+self.1.y),//New window offset
            min(window.w, self.1.w), min(window.h, self.1.h)//New window size
        );

        for child in &self.0 {
            child.draw(canvas, window, offset+self.1.position());
        }
    }

    //Size of an element is Max Size+Offset of its children limited to the Max size
    fn size(&self, ctx: &Context) -> Vec2 {
        if !self.2 {return self.1.size();}

        let size = self.0.iter().fold(Vec2::new(0.0, 0.0), |old_size, c| {
            let size = c.size(ctx);
            let offset = c.offset(ctx);
            Vec2::new(max(old_size.x, offset.x+size.x), max(old_size.y, offset.y+size.y))
        });
        Vec2::new(min(size.x, self.1.w), min(size.y, self.1.h))
    }

    fn offset(&self, _ctx: &Context) -> Vec2 {self.1.position()}
}

//  #[macro_export]
//  macro_rules! Component {
//      [$(($i:expr, $x:expr $(, $y:expr)?)),*] => {{
//          Ok(crate::structs::Component(vec![$(($i, $x $(, $y)?).into()),*]))
//      }}
//  }

#[derive(Copy, Clone, Debug)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Vec2{x, y}
    }
}

impl std::ops::Add for Vec2 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self{
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl std::ops::AddAssign for Vec2 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl From<Vec2> for glam::Vec2 {//maybe needs to be into
    fn from(vec: Vec2) -> glam::Vec2 {
        glam::Vec2::new(vec.x, vec.y)
    }
}

impl From<Vec2> for mint::Point2<f32> {//maybe needs to be into
    fn from(vec: Vec2) -> mint::Point2<f32> {
        mint::Point2::<f32>::from_slice(&[vec.x, vec.y])
    }
}

impl From<Vec2> for mint::Vector2<f32> {//maybe needs to be into
    fn from(vec: Vec2) -> mint::Vector2<f32> {
        mint::Vector2::<f32>::from_slice(&[vec.x, vec.y])
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32
}

impl Rect {
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Rect{x, y, w, h}
    }

    pub fn position(&self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }

    pub fn size(&self) -> Vec2 {
        Vec2::new(self.w, self.h)
    }
}

impl From<Rect> for graphics::Rect {//maybe needs to be into
    fn from(rect: Rect) -> graphics::Rect {
        graphics::Rect::new(rect.x, rect.y, rect.w, rect.h)
    }
}
