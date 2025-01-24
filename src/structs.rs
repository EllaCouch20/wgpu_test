use ggez::graphics::{Canvas, DrawParam};
use ggez::{GameResult, Context};
use ggez::{graphics, glam, mint};
use crate::traits::{Drawable};
use crate::traits;
use either::Either;

//pub use ggez::glam::Vec2;

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

#[derive(Clone)]
pub struct Child(Either<(Box<dyn Drawable>, DrawParam), Component>);//MaxSize, STF

impl Child {
    pub fn new_drawable(ctx: &Context, drawable: impl Drawable + 'static, param: DrawParam) -> Self {
        let drawable = Box::new(drawable);
        Child(Either::Left((drawable, param)))
    }

    pub fn new_component(component: Component) -> Self {
        Child(Either::Right(component))
    }

    pub fn size(&self, ctx: &Context) -> Vec2 {
        match &self.0 {
            Either::Left((drawable, _param)) => drawable.size(ctx),
            Either::Right(component) => component.size(ctx),
        }
    }

    pub fn offset(&self, ctx: &Context) -> Vec2 {
        match &self.0 {
            Either::Left((drawable, _param)) => drawable.offset(ctx),
            Either::Right(component) => component.offset(ctx),
        }
    }

    pub fn draw(&self, ctx: &Context, canvas: &mut Canvas, window: Rect, mut offset: Vec2) {
        match &self.0 {
            Either::Left((drawable, param)) => drawable.draw(canvas, window, offset, param.clone()),
            Either::Right(component) => component.draw(ctx, canvas, window, offset)
        }
    }
}

impl std::fmt::Debug for Child {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_tuple("Child");
        match &self.0 {
            Either::Left((drawable, _param)) => fmt.field(&"(Drawable, DrawParam)"),
            Either::Right(component) => fmt.field(component)
        };
        fmt.finish()
    }
}

pub struct ComponentParam {
    pub offset: Vec2,
    pub shrink_to_fit: bool
}

#[derive(Clone, Debug)]
pub struct Component(pub Vec<Child>, pub Vec2, pub Vec2, pub bool);//Offset, size, STF

impl Component {
    //Size of an element is Max Size+Offset of its children limited to their Max size
    pub fn size(&self, ctx: &Context) -> Vec2 {
        if !self.3 {return self.2;}

        let size = self.0.iter().fold(Vec2::new(0.0, 0.0), |old_size, c| {
            let size = c.size(ctx);
            let offset = c.offset(ctx);
            Vec2::new(max(old_size.x, offset.x+size.x), max(old_size.y, offset.y+size.y))
        });
        Vec2::new(min(size.x, self.2.x), min(size.y, self.2.y))
    }

    pub fn offset(&self, ctx: &Context) -> Vec2 {self.1}

    pub fn draw(&self, ctx: &Context, canvas: &mut Canvas, window: Rect, offset: Vec2) {
        let window = Rect::new(
            max(window.x, window.x+self.1.x), max(window.y, window.y+self.1.y),//New window offset
            min(window.w, self.2.x), min(window.h, self.2.y)//New window size
        );

        for child in &self.0 {
            child.draw(ctx, canvas, window, offset+self.1);
        }
    }
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
