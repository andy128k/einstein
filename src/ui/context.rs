#[derive(Clone, Copy, Default)]
pub struct Rect(pub i32, pub i32, pub u32, pub u32);

impl Rect {
    pub fn new(x: i32, y: i32, w: u32, h: u32) -> Self {
        Rect(x, y, w, h)
    }

    pub fn new0(w: u32, h: u32) -> Self {
        Rect(0, 0, w, h)
    }

    pub fn left(&self) -> i32 { self.0 }
    pub fn top(&self) -> i32 { self.1 }
    pub fn width(&self) -> u32 { self.2 }
    pub fn height(&self) -> u32 { self.3 }

    pub fn contains_point(&self, p: (i32, i32)) -> bool {
        p.0 >= self.0 && p.0 < self.0 + self.2 as i32 &&
        p.1 >= self.1 && p.1 < self.1 + self.3 as i32
    }

    pub fn offset(&self, dx: i32, dy: i32) -> Self {
        Rect(self.0 + dx, self.1 + dy, self.2, self.3)
    }

    pub fn relative(&self) -> Self {
        Rect(0, 0, self.2, self.3)
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

pub fn rect_to_rect2(rect: Rect) -> ::sdl2::rect::Rect {
    ::sdl2::rect::Rect::new(rect.0, rect.1, rect.2, rect.3)
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
