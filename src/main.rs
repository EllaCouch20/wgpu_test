use ramp_ds::Runtime;
use ramp_ds::components::*;

pub fn main() {
    Runtime::new(
        Column(vec![Box::new(ABox(200.0, 300.0)), Box::new(ABox(500.0, 100.0))], 32.0)
    ).unwrap().run().unwrap()
}
