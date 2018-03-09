use failure::err_msg;
use sdl2::rwops::RWops;
use sdl2_ttf::Font;
use sdl2_ttf::Sdl2TtfContext;
use error::*;

static mut TITLE_FONT: Option<Font<'static>> = None;
static mut BUTTON_FONT: Option<Font<'static>> = None;
static mut TEXT_FONT: Option<Font<'static>> = None;

const FONT_DUMP: &[u8] = include_bytes!("../../res/nova.ttf");

fn load_font(context: &Sdl2TtfContext, point_size: u16) -> Result<Font<'static>> {
    let ops = RWops::from_bytes(FONT_DUMP).map_err(err_msg)?;
    let font = context.load_font_from_rwops(ops, point_size).map_err(err_msg)?;
    Ok(font)
}

pub fn init_fonts(context: &Sdl2TtfContext) -> Result<()> {
    unsafe {
        TITLE_FONT = Some(load_font(context, 26)?);
        BUTTON_FONT = Some(load_font(context, 14)?);
        TEXT_FONT = Some(load_font(context, 16)?);
    }
    Ok(())
}

pub fn title_font() -> Result<&'static Font<'static>> {
    unsafe { TITLE_FONT.as_ref().ok_or_else(|| err_msg("Title font is not loaded.")) }
}

pub fn button_font() -> Result<&'static Font<'static>> {
    unsafe { BUTTON_FONT.as_ref().ok_or_else(|| err_msg("Button font is not loaded.")) }
}

pub fn text_font() -> Result<&'static Font<'static>> {
    unsafe { TEXT_FONT.as_ref().ok_or_else(|| err_msg("Text font is not loaded.")) }
}
