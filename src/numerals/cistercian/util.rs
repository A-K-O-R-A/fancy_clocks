pub type Cords = (i32, i32);
pub trait Scaleable {
    fn scale(&self, x_scale: i32, y_scale: i32) -> Self;
    fn offset(&self, off: Cords) -> Self;
}
impl Scaleable for Cords {
    fn scale(&self, x_scale: i32, y_scale: i32) -> Cords {
        (self.0 * x_scale, self.1 * y_scale)
    }

    fn offset(&self, off: Cords) -> Cords {
        (self.0 + off.0, self.1 + off.1)
    }
}
pub struct Line {
    pub from: Cords,
    pub to: Cords,
}
impl Line {
    pub fn from_to(from: Cords, to: Cords) -> Self {
        Self { from, to }
    }
}
impl Scaleable for Line {
    fn scale(&self, x_scale: i32, y_scale: i32) -> Line {
        Line {
            from: self.from.scale(x_scale, y_scale),
            to: self.to.scale(x_scale, y_scale),
        }
    }

    fn offset(&self, off: Cords) -> Line {
        Line {
            from: self.from.offset(off),
            to: self.to.offset(off),
        }
    }
}
