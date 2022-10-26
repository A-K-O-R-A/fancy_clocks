use plotters::prelude::*;
use plotters::style::text_anchor::{HPos, Pos, VPos};

mod digit;
use digit::*;

/*
const COLOR: &RGBColor = &WHITE;
const STROKE_WIDTH: u32 = 3;
fn color() -> ShapeStyle {
    COLOR.stroke_width(STROKE_WIDTH)
}
*/

#[derive(Debug)]
pub struct Roman {
    digits: Vec<Digit>,
}

impl super::Numeral for Roman {
    fn from_number(n: usize) -> Self {
        let mut digits: Vec<Digit> = Vec::new();

        //Convert number to invalid roman notation (exp. 1412 => MCCCCXII)
        let mut val = n.clone();
        while val > 0 {
            let dig = match val {
                1000..=usize::MAX => M,
                500..=999 => D,
                100..=499 => C,
                50..=99 => L,
                10..=49 => X,
                5..=9 => V,
                1..=4 => I,
                _ => panic!("NO"),
            };

            val -= dig.val();
            digits.push(dig);
        }

        //Combine more than 3 of the same digits to 2 (exp. CCCC => CD, IIII => IV)
        let mut i = 0;
        while i < digits.len() - 3 {
            let d = digits[i];
            if d != digits[i + 1] {
                i += 1;
                continue;
            }
            if d != digits[i + 2] {
                i += 2;
                continue;
            }
            if d != digits[i + 3] {
                i += 3;
                continue;
            }
            //Next 3 digits are the same as the one at position 3

            let next_d = d.next();

            //Remove the same digits
            digits.remove(i);
            digits.remove(i);
            digits.remove(i);
            digits.remove(i);

            //Insert next bigger digit
            digits.insert(i, next_d);
            //Insert old digit before
            digits.insert(i, d);

            i += 2;
            digits_to_text(&digits);
        }

        Self { digits }
    }

    fn draw(&self) -> Result<(), Box<dyn std::error::Error>> {
        let _scale = 50;

        let mut backend = SVGBackend::new("out/roman.svg", (200, 100));
        let text = digits_to_text(&self.digits);
        let pos = Pos::new(HPos::Center, VPos::Center);
        backend.draw_text(
            &text,
            &TextStyle::from(("sans-serif", 30).into_font())
                .color(&WHITE)
                .pos(pos),
            (100, 50),
        )?;

        Ok(())
    }
}

fn digits_to_text(digits: &Vec<Digit>) -> String {
    digits
        .into_iter()
        .map(|d| d.str().to_owned())
        .reduce(|a, b| a + &b)
        .expect("LOL")
}
