use ramp_ds::Runtime;
use ramp_ds::primitives::*;

pub fn main() {
    Runtime::new(
        Stack(vec![Box::new(Rectangle(0.2, 0.3)), Box::new(Rectangle(0.5, 0.1))])
    ).unwrap().run().unwrap()
}
