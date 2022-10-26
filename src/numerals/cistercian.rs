use plotters::{prelude::*, style::full_palette::PINK};

const COLOR: &RGBColor = &PINK;

use super::lib;

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
impl Line {
    fn from_to(from: Cords, to: Cords) -> Self {
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

/*
DECEM | UNUM
      |
MILLE | CENTUM
*/
#[derive(Debug)]
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

#[derive(Debug)]
struct Digit {
    digit_type: DigitType,
    ///Digit value between 0 an 10
    value: u8,
}

/*
(0) - - - -(1)
 |
 |
 |
 |
(1)

*/
impl Digit {
    fn draw(
        &self,
        backend: &mut SVGBackend,
        scale: i32,
        offset: Cords,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let scale = self.digit_type.scaling().scale(scale, scale);

        let mut val = self.value.clone();

        //For some reason the number 3 isn't represented as a 2 and 1 combined
        if val == 3 {
            backend.draw_line(
                (0, 0).scale(scale.0, scale.1).offset(offset),
                (1, 1).scale(scale.0, scale.1).offset(offset),
                COLOR,
            )?;
            return Ok(());
        }

        while val > 0 {
            let l = match val {
                6..=9 => {
                    val -= 6;
                    Line::from_to((1, 0), (1, 1))
                }
                4..=5 => {
                    val -= 4;
                    Line::from_to((0, 1), (1, 0))
                }
                /*
                3 => {
                    val -= 3;
                    Line::from_to((0, 0), (1, 1))
                }
                 */
                2..=3 => {
                    val -= 2;
                    Line::from_to((0, 1), (1, 1))
                }
                1 => {
                    val -= 1;
                    Line::from_to((0, 0), (1, 0))
                }
                _ => todo!("This shouldn't happen"),
            };
            backend.draw_line(
                l.from.scale(scale.0, scale.1).offset(offset),
                l.to.scale(scale.0, scale.1).offset(offset),
                COLOR,
            )?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct Cistercian {
    unum: Digit,
    decem: Digit,
    centum: Digit,
    mille: Digit,
}

impl lib::Numeral for Cistercian {
    fn from_number(n: usize) -> Self {
        let d4 = n % 10;
        let d3 = (n % 100) / 10;
        let d2 = (n % 1000) / 100;
        let d1 = (n % 10000) / 1000;
        Cistercian {
            unum: Digit {
                digit_type: DigitType::UNUM,
                value: d4 as u8,
            },
            decem: Digit {
                digit_type: DigitType::DECEM,
                value: d3 as u8,
            },
            centum: Digit {
                digit_type: DigitType::CENTUM,
                value: d2 as u8,
            },
            mille: Digit {
                digit_type: DigitType::MILLE,
                value: d1 as u8,
            },
        }
    }

    fn draw(&self) -> Result<(), Box<dyn std::error::Error>> {
        let scale = 50;

        let mut backend = SVGBackend::new("out/cistercian.svg", (200, 300));

        backend.draw_line((100, 50), (100, 250), &PINK)?;

        self.unum.draw(&mut backend, scale, (100, 50))?;
        self.decem.draw(&mut backend, scale, (100, 50))?;
        self.centum.draw(&mut backend, scale, (100, 250))?;
        self.mille.draw(&mut backend, scale, (100, 250))?;

        Ok(())
    }
}
