use ramp_ds::Runtime;
use ramp_ds::primitives::*;
use ramp_ds::components::*;

pub fn main() {
    Runtime::new(
        Column!(
            0.0,
            Button(ButtonStyle::Primary, Size::Large),
            Container(ExtRectangle(200.0, 300.0), 0.4, 0.2),
            ExtRectangle(500.0, 100.0)
        )
    ).unwrap().run().unwrap()
}
