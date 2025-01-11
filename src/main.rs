use ramp_ds::Runtime;
use ramp_ds::components::*;

pub fn main() {
    Runtime::new(
        Stack(vec![Box::new(ABox(0.2, 0.3)), Box::new(ABox(0.5, 0.1))])
    ).unwrap().run().unwrap()
}
