use ramp_ds::Runtime;
use ramp_ds::primitives::*;
use ramp_ds::theme::*;
use ramp_ds::components::*;

use ggez::graphics::Image;

pub fn main() {
    Runtime::new(
      Column!(
            0.0,
            // TwoRectangle(0.2),
            // Container(Container(ExtRectangle(Color::BLUE), 50.0, 100.0), 100.0, 200.0),
            // ExtRectangle(Color::GREEN)
            CustomText::label("Label Example", 32.0),
            CustomText::primary("Primary Example", 32.0),
            CustomText::secondary("Secondary Example", 24.0),
            CustomText::heading("Heading Example", 48.0),
            TextInput("Text input..."),
            Icon("copy", 24.0),
            Icon("group", 24.0),
            Icon("message", 24.0),
            Button::primary(Size::Large, Width::Expand, None, "Continue"),
            Button::primary(Size::Medium, Width::Hug, None, "Continue"),
            Button::secondary(Size::Large, Width::Expand, None, "Continue"),
            Button::secondary(Size::Medium, Width::Hug, Some("exit"), "Continue"),
            Button::ghost(Size::Large, Width::Expand, None, "Continue"),
            Button::ghost(Size::Medium, Width::Hug, None, "Continue")
      )
    ).unwrap().run().unwrap()
}

            //CustomText::label("Label Example", 32.0)
            // CustomText::primary("Primary Example", 32.0),
            // CustomText::secondary("Secondary Example", 24.0),
            // CustomText::heading("Heading Example", 48.0),
            // Button(ButtonStyle::Primary, Size::Medium, "Continue"),
            // Button(ButtonStyle::Primary, Size::Large, "Continue"),
            // Button(ButtonStyle::Secondary, Size::Medium, "Continue"),
            // Button(ButtonStyle::Secondary, Size::Large, "Continue"),
            // Button(ButtonStyle::Ghost, Size::Medium, "Continue"),
            // Button(ButtonStyle::Ghost, Size::Large, "Continue")
