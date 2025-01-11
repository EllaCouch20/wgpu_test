use ramp_ds::Runtime;
use ramp_ds::primitives::*;

pub fn main() {
    Runtime::new(
        Column(vec![
            Box::new(Rectangle(0.2, 0.3)),
            Box::new(Rectangle(0.5, 0.1)),
            Box::new(Container::new(Scrollable::new(ExtRectangle(50.0, 200.0)), 100.0, 150.0))
        ], 32.0)
    ).unwrap().run().unwrap()
}
