use plotters::prelude::*;

use super::lib;

#[derive(Debug)]
pub struct Cistercian {
    value: usize,
}

type Cords = (i32, i32);
trait Scaleable {
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
struct Line {
    from: Cords,
    to: Cords,
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

/*
DECEM | UNUM
      |
MILLE | CENTUM
*/
enum DigitType {
    UNUM,
    DECEM,
    CENTUM,
    MILLE,
}

impl DigitType {
    ///For drawing only
    const fn scaling(&self) -> Cords {
        match self {
            DigitType::UNUM => (1, 1),
            DigitType::DECEM => (-1, 1),
            DigitType::CENTUM => (1, -1),
            DigitType::MILLE => (-1, -1),
        }
    }
}

struct Digit {
    digit_type: DigitType,
    ///Digit value between 0 an 10
    value: u8,
}

impl Digit {
    fn draw(&self, backend: &mut SVGBackend, scale: i32, offset: Cords) {
        let scale = self.digit_type.scaling().scale(scale, scale);

        let mut val = self.value.clone();
        while val > 0 {
            match val {
                6..=9 => {
                    (backend.draw_line(
                        (0, 1).scale(scale.0, scale.1).offset(offset),
                        (0, 1).scale(scale.0, scale.1).offset(offset),
                        &RED,
                    ))
                }
                _ => (),
            };
        }
    }
}

impl lib::Numeral for Cistercian {
    fn from_number(n: usize) -> Self {
        Cistercian { value: n }
    }

    fn draw(&self) {
        println!("My value is {}", self.value);
    }
}
