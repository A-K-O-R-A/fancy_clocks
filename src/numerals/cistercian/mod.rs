use plotters::{prelude::*, style::full_palette::PINK};

//Import util stuff
mod util;
use util::*;

const COLOR: &RGBColor = &WHITE;
const STROKE_WIDTH: u32 = 3;
fn color() -> ShapeStyle {
    COLOR.stroke_width(STROKE_WIDTH)
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
    ///For flipping the lines according to their position
    /// (1,1) is top right
    const fn scaling(&self) -> Cords {
        match self {
            DigitType::UNUM => (1, 1),    //Top right
            DigitType::DECEM => (-1, 1),  //Top left
            DigitType::CENTUM => (1, -1), //Bottom right
            DigitType::MILLE => (-1, -1), //Bottom left
        }
    }
}

#[derive(Debug)]
struct Digit {
    digit_type: DigitType,
    ///Digit value between 0 and 9
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
                &color(),
            )?;
            return Ok(());
        }

        //Draw line with biggest possible value until the combined value of the drawn lines equals val
        while val > 0 {
            let line = match val {
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
                _ => panic!("{} is not represantable by this system", val),
            };
            backend.draw_line(
                line.from.scale(scale.0, scale.1).offset(offset),
                line.to.scale(scale.0, scale.1).offset(offset),
                &color(),
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

impl super::Numeral for Cistercian {
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

        backend.draw_line((100, 50), (100, 250), &color())?;

        self.unum.draw(&mut backend, scale, (100, 50))?;
        self.decem.draw(&mut backend, scale, (100, 50))?;
        self.centum.draw(&mut backend, scale, (100, 250))?;
        self.mille.draw(&mut backend, scale, (100, 250))?;

        Ok(())
    }
}
