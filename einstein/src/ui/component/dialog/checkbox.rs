use sdl2::pixels::Color;
use crate::ui::brick::*;
use crate::ui::widget::common::*;
use crate::ui::widget::checkbox::Checkbox;
use super::theme::DialogTheme;

pub fn dialog_checkbox(theme: DialogTheme, checked: bool) -> Checkbox {
    Checkbox::new(checked, move |size, highlighted, checked, _resource_manager| {
        let background = theme.background(highlighted);
        let (color1, color2) = theme.colors3d();
        let border = Border::Etched(color1, color2);

        let mut brick = Brick::new(size.width, size.height)
            .background(background)
            .border(border);
        if checked {
            brick = brick.text(Text::new("X").color(Color::RGB(255, 255, 255)).shadow());
        }
        brick
    })
}
