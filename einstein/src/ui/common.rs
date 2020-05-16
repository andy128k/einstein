#[derive(Clone, Copy, Default)]
pub struct Rect {
    pub left: i32,
    pub top: i32,
    pub width: u32,
    pub height: u32,
}

impl Rect {
    pub const fn new(left: i32, top: i32, width: u32, height: u32) -> Self {
        Self {
            left,
            top,
            width,
            height,
        }
    }

    pub fn contains_point(&self, p: (i32, i32)) -> bool {
        p.0 >= self.left
            && p.0 < self.left + self.width as i32
            && p.1 >= self.top
            && p.1 < self.top + self.height as i32
    }

    pub fn intersection(&self, other: &Self) -> Option<Rect> {
        let left = i32::max(self.left, other.left);
        let right = i32::min(
            self.left + self.width as i32,
            other.left + other.width as i32,
        );
        let top = i32::max(self.top, other.top);
        let bottom = i32::min(
            self.top + self.height as i32,
            other.top + other.height as i32,
        );
        if left < right && top < bottom {
            Some(Rect::new(
                left,
                top,
                (right - left) as u32,
                (bottom - top) as u32,
            ))
        } else {
            None
        }
    }
}

#[derive(Clone, Copy)]
pub struct Size {
    pub width: u32,
    pub height: u32,
}

impl Size {
    pub const EMPTY: Size = Size {
        width: 0,
        height: 0,
    };

    pub const fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    pub fn to_rect(&self) -> Rect {
        Rect::new(0, 0, self.width, self.height)
    }

    pub fn contains(&self, x: i32, y: i32) -> bool {
        x >= 0 && (x as u32) < self.width && y >= 0 && (y as u32) < self.height
    }
}

#[derive(Clone, Copy)]
pub enum HorizontalAlign {
    Left,
    Center,
    Right,
}

#[derive(Clone, Copy)]
pub enum VerticalAlign {
    Top,
    Middle,
    Bottom,
}
