use sdl2_ttf::Font;
use ui::page_layout::{Page, PagesBuilder};
use error::*;

enum TextItem {
    Text(String),
    Image(&'static [u8]),
}

/*
void addLine(TextPage *page, std::wstring &line, int &curPosY, int &lineWidth)
{
    new Label(screen, &font, offsetX, offsetY + curPosY, 255,255,255, line, false);
}
*/

fn make_pages(text: &[TextItem], page_width: u16, page_height: u16, font: &Font) -> Result<Vec<Page>> {
    let mut pages = PagesBuilder::new(page_width, page_height);
    for text_item in text {
        match *text_item {
            TextItem::Text(ref content) => pages.add_text(content, font)?,
            TextItem::Image(ref image) => pages.add_image(image)?
        }
    }
    Ok(pages.build())
}
