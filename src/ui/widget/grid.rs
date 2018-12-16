use crate::ui::context::Size;
use crate::ui::widget::widget::*;
use crate::ui::widget::container::Container;

fn child_pos(size: Size, child_size: Size, cols: usize, rows: usize, index: usize) -> (u32, u32) {
    let row = (index / cols) as u32;
    let col = (index % cols) as u32;
    let child_width = child_size.width;
    let child_height = child_size.height;
    let gap_x = if cols > 1 { (size.width - child_width * (cols as u32)) / ((cols - 1) as u32) } else { 0 };
    let gap_y = if rows > 1 { (size.height - child_height * (rows as u32)) / ((rows - 1) as u32) } else { 0 };
    (
        col * (child_width + gap_x),
        row * (child_height + gap_y),
    )
}

pub fn new_grid<A, Ch: Widget<A> + 'static>(size: Size, child_size: Size, cols: usize, rows: usize, children: Vec<Ch>) -> Container<A> {
    let mut container = Container::container(size, None);
    for (index, child) in children.into_iter().enumerate() {
        let (x, y) = child_pos(size, child_size, cols, rows, index);
        container.push(x, y, child);
    }
    container
}
