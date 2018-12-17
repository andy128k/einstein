use crate::ui::widget::widget::*;
use crate::ui::widget::container::Container;

pub struct GridBuilder<A> {
    container: Container<A>,
    cols: usize,
    rows: usize,
    children: Vec<(usize, usize, WidgetPtr<A>)>,
}

impl<A: Clone + 'static> GridBuilder<A> {
    pub fn new(container: Container<A>, cols: usize, rows: usize) -> Self {
        Self { container, cols, rows, children: Vec::new() }
    }

    pub fn add(mut self, col: usize, row: usize, widget: impl Widget<A> + 'static) -> Self {
        self.children.push((col, row, Box::new(widget)));
        self.cols = usize::max(self.cols, col + 1);
        self.rows = usize::max(self.rows, row + 1);
        self
    }

    pub fn build(mut self) -> Container<A> {
        let mut widths = vec![0; self.cols];
        let mut heights = vec![0; self.rows];
        for (col, row, child) in &self.children {
            let size = child.get_size();
            widths[*col] = u32::max(widths[*col], size.width);
            heights[*row] = u32::max(heights[*row], size.height);
        }
        let max_width: u32 = widths.iter().sum();
        let max_height: u32 = heights.iter().sum();
        let gap_x: u32 = if self.cols > 1 { (self.container.get_size().width - max_width) / ((self.cols - 1) as u32) } else { 0 };
        let gap_y: u32 = if self.rows > 1 { (self.container.get_size().height - max_height) / ((self.rows - 1) as u32) } else { 0 };
        let xs = widths.iter().scan(0, |state, w| {
            let x = *state;
            *state += w + gap_x;
            Some(x)
        }).collect::<Vec<u32>>();
        let ys = heights.iter().scan(0, |state, h| {
            let y = *state;
            *state += h + gap_y;
            Some(y)
        }).collect::<Vec<u32>>();

        for (col, row, child) in self.children.drain(0..) {
            let size = child.get_size();
            let x = xs[col] + (widths[col] - size.width) / 2;
            let y = ys[row] + (heights[row] - size.height) / 2;
            self.container.push(x, y, child);
        }

        self.container
    }
}
