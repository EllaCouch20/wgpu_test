use ramp_ds::Runtime;
use ramp_ds::primitives::*;

pub fn main() {
    Runtime::new(
        Column!(
            0.0,
            Container(ExtRectangle(200.0, 300.0), 0.4, 0.2),
            ExtRectangle(500.0, 100.0)
        )
    ).unwrap().run().unwrap()
}
