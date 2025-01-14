use ramp_ds::Runtime;
use ramp_ds::primitives::*;
//use ramp_ds::components::*;

pub fn main() {
    Runtime::new(
      //  ExtRectangle(500.0, 100.0)
        Column!(
            20.0,
            Container(ExtRectangle(300.0, 800.0), 290.0, 20.0),
            Container(Scrollable(ExtRectangle(300.0, 800.0), 10.0), 290.0, 20.0),
            ExtRectangle(100.0, 200.0),
            ExtRectangle(500.0, 100.0)
        )
    ).unwrap().run().unwrap()
}
