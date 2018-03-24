use failure::err_msg;
use sdl2::rwops::RWops;
use sdl2::ttf::{Font, Sdl2TtfContext};
use error::*;

static mut TITLE_FONT: Option<Font<'static, 'static>> = None;
static mut BUTTON_FONT: Option<Font<'static, 'static>> = None;
static mut TEXT_FONT: Option<Font<'static, 'static>> = None;
static mut MENU_FONT: Option<Font<'static, 'static>> = None;

const FONT_DUMP: &[u8] = include_bytes!("../../res/nova.ttf"); // /usr/share/fonts/truetype/liberation/LiberationSans-Regular.ttf

fn load_font(context: &'static Sdl2TtfContext, point_size: u16) -> Result<Font<'static, 'static>> {
    let ops = RWops::from_bytes(FONT_DUMP).map_err(err_msg)?;
    let font = context.load_font_from_rwops(ops, point_size).map_err(err_msg)?;
    Ok(font)
}

pub fn init_fonts(context: &'static Sdl2TtfContext) -> Result<()> {
    unsafe {
        TITLE_FONT = Some(load_font(context, 26)?);
        BUTTON_FONT = Some(load_font(context, 14)?);
        TEXT_FONT = Some(load_font(context, 16)?);
        MENU_FONT = Some(load_font(context, 20)?);
    }
    Ok(())
}

pub fn title_font() -> Result<&'static Font<'static, 'static>> {
    unsafe { TITLE_FONT.as_ref().ok_or_else(|| err_msg("Title font is not loaded.")) }
}

pub fn button_font() -> Result<&'static Font<'static, 'static>> {
    unsafe { BUTTON_FONT.as_ref().ok_or_else(|| err_msg("Button font is not loaded.")) }
}

pub fn text_font() -> Result<&'static Font<'static, 'static>> {
    unsafe { TEXT_FONT.as_ref().ok_or_else(|| err_msg("Text font is not loaded.")) }
}

pub fn menu_font() -> Result<&'static Font<'static, 'static>> {
    unsafe { MENU_FONT.as_ref().ok_or_else(|| err_msg("Menu font is not loaded.")) }
}
