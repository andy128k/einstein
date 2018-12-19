#[derive(Clone, Copy, Default)]
pub struct Rect(pub i32, pub i32, pub u32, pub u32);

impl Rect {
    pub fn new(x: i32, y: i32, w: u32, h: u32) -> Self {
        Rect(x, y, w, h)
    }

    pub fn left(&self) -> i32 { self.0 }
    pub fn top(&self) -> i32 { self.1 }
    pub fn width(&self) -> u32 { self.2 }
    pub fn height(&self) -> u32 { self.3 }

    pub fn contains_point(&self, p: (i32, i32)) -> bool {
        p.0 >= self.0 && p.0 < self.0 + self.2 as i32 &&
        p.1 >= self.1 && p.1 < self.1 + self.3 as i32
    }

    pub fn intersection(&self, other: &Self) -> Option<Rect> {
        let left   = i32::max(self.0, other.0);
        let right  = i32::min(self.0 + self.2 as i32, other.0 + other.2 as i32);
        let top    = i32::max(self.1, other.1);
        let bottom = i32::min(self.1 + self.3 as i32, other.1 + other.3 as i32);
        if left < right && top < bottom {
            Some(Rect(left, top, (right - left) as u32, (bottom - top) as u32))
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
    pub const EMPTY: Size = Size { width: 0, height: 0 };

    pub const fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    pub fn to_rect(&self) -> Rect {
        Rect::new(0, 0, self.width, self.height)
    }

    pub fn contains(&self, x: i32, y: i32) -> bool {
        x >= 0 && (x as u32) < self.width &&
        y >= 0 && (y as u32) < self.height
    }
}

#[derive(Clone, Copy)]
pub enum HorizontalAlign {
    Left,
    Center,
    Right
}

#[derive(Clone, Copy)]
pub enum VerticalAlign {
    Top,
    Middle,
    Bottom
}
