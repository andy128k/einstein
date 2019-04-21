use sdl2::pixels::Color;
use sdl2::render::{Canvas, Texture, TextureQuery};
use sdl2::video::Window;
use sdl2::ttf::Font;
use crate::ui::widget::common::*;
use crate::ui::common::{Rect, HorizontalAlign, VerticalAlign};
use crate::resources::manager::ResourceManager;

pub struct Text {
    text: String,
    font_size: FontSize,
    color: Color,
    horizontal_align: HorizontalAlign,
    vertical_align: VerticalAlign,
    shadow: bool,
}

impl Text {
    pub fn new(text: impl ToString) -> Self {
        Self {
            text: text.to_string(),
            font_size: FontSize::TEXT,
            color: Color::RGB(0, 0, 0),
            horizontal_align: HorizontalAlign::Center,
            vertical_align: VerticalAlign::Middle,
            shadow: false,
        }
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn font_size(mut self, font_size: FontSize) -> Self {
        self.font_size = font_size;
        self
    }

    pub fn shadow(mut self) -> Self {
        self.shadow = true;
        self
    }

    pub fn halign(mut self, horizontal_align: HorizontalAlign) -> Self {
        self.horizontal_align = horizontal_align;
        self
    }

    pub fn valign(mut self, vertical_align: VerticalAlign) -> Self {
        self.vertical_align = vertical_align;
        self
    }
}

struct Child {
    left: u32,
    top: u32,
    brick: Brick,
}

pub struct Brick {
    width: u32,
    height: u32,
    background: Option<Background>,
    text: Option<Text>,
    border: Option<Border>,
    children: Vec<Child>,
}

impl Brick {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            text: None,
            background: None,
            border: None,
            children: vec![],
        }
    }

    pub fn text(mut self, text: Text) -> Self {
        self.text = Some(text);
        self
    }

    pub fn background(mut self, background: Background) -> Self {
        self.background = Some(background);
        self
    }

    pub fn border(mut self, border: Border) -> Self {
        self.border = Some(border);
        self
    }

    pub fn add(mut self, left: u32, top: u32, child: Self) -> Self {
        self.children.push(Child { left, top, brick: child });
        self
    }

    pub fn push(&mut self, left: u32, top: u32, child: Self) {
        self.children.push(Child { left, top, brick: child });
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>, left: u32, top: u32, resource_manager: &dyn ResourceManager) -> Result<(), ::failure::Error> {
        let rect = Rect::new(left as i32, top as i32, self.width, self.height);
        match self.background {
            Some(Background::Color(color)) => {
                canvas.set_draw_color(color);
                canvas.fill_rect(Some(rect_to_rect2(rect))).map_err(::failure::err_msg)?;
            },
            Some(Background::Image(resource, s_rect)) => {
                let image = resource_manager.image(&resource);
                let src_rect = s_rect.unwrap_or_else(|| {
                    let q = image.query();
                    Rect::new(0, 0, q.width, q.height)
                });
                sprite(canvas, rect, &*image, src_rect)?;
            },
            None => {},
        }
        if let Some(ref text) = self.text {
            let font = resource_manager.font(text.font_size.0);
            render_text(canvas, rect, &text.text, &font, text.color, text.shadow, text.horizontal_align, text.vertical_align)?;
        }
        match self.border {
            Some(Border::Beveled(color1, color2)) => bevel(canvas, rect, color1, color2)?,
            Some(Border::Etched(color1, color2)) => etched(canvas, rect, color1, color2)?,
            None => {},
        }
        for child in &self.children {
            let r = Rect::new((left + child.left) as i32, (top + child.top) as i32, child.brick.width, child.brick.height);
            let child_rect = r.intersection(&rect).unwrap_or_default();
            canvas.set_clip_rect(Some(rect_to_rect2(child_rect)));
            child.brick.draw(canvas, left + child.left, top + child.top, resource_manager)?;
            canvas.set_clip_rect(None);
        }
        Ok(())
    }
}

fn rect_to_rect2(rect: Rect) -> ::sdl2::rect::Rect {
    ::sdl2::rect::Rect::new(rect.0, rect.1, rect.2, rect.3)
}

fn sprite(canvas: &mut Canvas<Window>, rect: Rect, src_image: &Texture, src_rect: Rect) -> Result<(), ::failure::Error> {
    let tile_width = src_rect.width();
    let tile_height = src_rect.height();

    let cw = (rect.width() + tile_width - 1) / tile_width;
    let ch = (rect.height() + tile_height - 1) / tile_height;

    for j in 0..ch {
        for i in 0..cw {
            let dst = Rect::new(rect.left() + ((i * tile_width) as i32), rect.top() + ((j * tile_height) as i32), tile_width, tile_height);
            let clip = dst.intersection(&rect).unwrap();
            canvas.set_clip_rect(Some(rect_to_rect2(clip)));
            canvas.copy(&src_image, Some(rect_to_rect2(src_rect)), Some(rect_to_rect2(dst))).map_err(::failure::err_msg)?;
            canvas.set_clip_rect(None);
        }
    }

    Ok(())
}

fn render_text(canvas: &mut Canvas<Window>, rect: Rect,
    text: &str,
    font: &Font, color: Color, shadow: bool,
    horizontal_align: HorizontalAlign, vertical_align: VerticalAlign) -> Result<(), ::failure::Error>
{
    if text.is_empty() {
        return Ok(());
    }

    canvas.set_clip_rect(Some(rect_to_rect2(rect)));

    let (w, h) = font.size_of(text)?;

    let x = match horizontal_align {
        HorizontalAlign::Left => rect.left(),
        HorizontalAlign::Center => rect.left() + (rect.width().saturating_sub(w) as i32) / 2,
        HorizontalAlign::Right => rect.left() + (rect.width().saturating_sub(w) as i32)
    };

    let y = match vertical_align {
        VerticalAlign::Top => rect.top(),
        VerticalAlign::Middle => rect.top() + (rect.height().saturating_sub(h) as i32) / 2,
        VerticalAlign::Bottom => rect.top() + (rect.height().saturating_sub(h) as i32)
    };

    let texture_creator = canvas.texture_creator();
    if shadow {
        let shadow_surface = font.render(text).blended(Color::RGBA(0, 0, 0, 0))?;
        let shadow_texture = texture_creator.create_texture_from_surface(shadow_surface)?;
        let TextureQuery { width, height, .. } = shadow_texture.query();
        canvas.copy(&shadow_texture, None, rect_to_rect2(Rect::new(x + 1, y + 1, width, height))).map_err(::failure::err_msg)?;
    }
    {
        let text_surface = font.render(text).blended(color)?;
        let text_texture = texture_creator.create_texture_from_surface(text_surface)?;
        let TextureQuery { width, height, .. } = text_texture.query();
        canvas.copy(&text_texture, None, rect_to_rect2(Rect::new(x, y, width, height))).map_err(::failure::err_msg)?;
    }

    canvas.set_clip_rect(None);

    Ok(())
}

fn bevel(canvas: &mut Canvas<Window>, rect: Rect, top_left: Color, bottom_right: Color) -> Result<(), ::failure::Error> {
    let left = rect.left();
    let top = rect.top();
    let width = rect.width();
    let height = rect.height();
    let right = left + (width as i32) - 1;
    let bottom = top + (height as i32) - 1;

    canvas.set_draw_color(top_left);
    canvas.draw_line((left, top), (left,  bottom)).map_err(::failure::err_msg)?;
    canvas.draw_line((left, top), (right, top)).map_err(::failure::err_msg)?;
    canvas.set_draw_color(bottom_right);
    canvas.draw_line((right, top + 1), (right, bottom)).map_err(::failure::err_msg)?;
    canvas.draw_line((left + 1, bottom), (right, bottom)).map_err(::failure::err_msg)?;

    Ok(())
}

fn etched(canvas: &mut Canvas<Window>, rect: Rect, top_left: Color, bottom_right: Color) -> Result<(), ::failure::Error> {
    let inner_rect = Rect::new(rect.left() + 1, rect.top() + 1, rect.width() - 2, rect.height() - 2);
    bevel(canvas, inner_rect, top_left, bottom_right)?;
    bevel(canvas, rect, bottom_right, top_left)?;
    Ok(())
}
