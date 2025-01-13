use ramp_ds::Runtime;
use ramp_ds::theme::*;
use ramp_ds::primitives::*;
use ramp_ds::components::*;

pub fn main() {
    Runtime::new(
        Column!(
            24.0,
            CustomText::label("Label Example", 32.0),
            CustomText::primary("Primary Example", 32.0),
            CustomText::secondary("Secondary Example", 24.0),
            CustomText::heading("Heading Example", 48.0),
            Button(ButtonStyle::Primary, Size::Medium, "Continue"),
            Button(ButtonStyle::Primary, Size::Large, "Continue"),
            Button(ButtonStyle::Secondary, Size::Medium, "Continue"),
            Button(ButtonStyle::Secondary, Size::Large, "Continue"),
            Button(ButtonStyle::Ghost, Size::Medium, "Continue"),
            Button(ButtonStyle::Ghost, Size::Large, "Continue")
        )
    ).unwrap().run().unwrap()
}
