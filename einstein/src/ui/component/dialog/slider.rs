use super::theme::DialogTheme;
use crate::ui::brick::*;
use crate::ui::common::Size;
use crate::ui::widget::common::*;
use crate::ui::widget::slider::Slider;

pub fn dialog_slider(theme: DialogTheme, size: Size, value: f32) -> Slider {
    Slider::new(
        size,
        value,
        move |size, slider_rect, highlighted, _resource_manager| {
            let background = theme.background(highlighted);
            let (color1, color2) = theme.colors3d();

            let scale = Brick::new(size.width, 4).border(Border::Beveled(color2, color1));

            let slider = Brick::new(slider_rect.width, slider_rect.height)
                .background(background)
                .border(Border::Beveled(color1, color2));

            Brick::new(size.width, size.height)
                .add(0, (size.height - 4) / 2, scale)
                .add(slider_rect.left as u32, slider_rect.top as u32, slider)
        },
    )
}
