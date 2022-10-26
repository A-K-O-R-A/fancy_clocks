use plotters::prelude::*;
use plotters::style::text_anchor::{HPos, Pos, VPos};

mod pattern;
use pattern::*;

/*
const COLOR: &RGBColor = &WHITE;
const STROKE_WIDTH: u32 = 3;
fn color() -> ShapeStyle {
    COLOR.stroke_width(STROKE_WIDTH)
}
*/

#[derive(Debug)]
pub struct Braille {
    patterns: [Pattern; 4],
}

impl super::Numeral for Braille {
    fn from_number(n: usize) -> Self {
        let mut patterns = [pattern::ZERO; 4];

        let digits = number_to_four_digits(n);
        patterns[0] = digits.0.into_pattern();
        patterns[1] = digits.1.into_pattern();
        patterns[2] = digits.2.into_pattern();
        patterns[3] = digits.3.into_pattern();

        Self { patterns }
    }

    fn draw(&self) -> Result<(), Box<dyn std::error::Error>> {
        let _scale = 50;

        let mut backend = SVGBackend::new("out/roman.svg", (200, 100));

        //Join patterns to single string
        let text = (&self.patterns)
            .into_iter()
            .map(|p| p.0.to_owned())
            .reduce(|a, b| a + &b)
            .expect("Couldn't convert pattern to string");

        println!("{text}");
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

fn number_to_four_digits(n: usize) -> (usize, usize, usize, usize) {
    let d4 = n % 10;
    let d3 = (n % 100) / 10;
    let d2 = (n % 1000) / 100;
    let d1 = (n % 10000) / 1000;

    (d1, d2, d3, d4)
}
