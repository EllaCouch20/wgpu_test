use ggez::Context;
use ggez::graphics::FontData;

pub fn load_fonts(ctx: &mut Context) -> Result<(), Box<dyn std::error::Error>> {
    ctx.gfx.add_font(
        "Label",
        FontData::from_path(ctx, "/outfit_bold.ttf")?,
    );
    ctx.gfx.add_font(
        "Heading",
        FontData::from_path(ctx, "/outfit_bold.ttf")?,
    );
    ctx.gfx.add_font(
        "Text",
        FontData::from_path(ctx, "/outfit_regular.ttf")?,
    );

    Ok(())
}