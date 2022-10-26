mod numerals;
use chrono::prelude::*;
use numerals::Numeral;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let n = get_time_as_number() as usize;
    println!("Current time {}", n);

    let cistercian = numerals::Cistercian::from_number(n);
    cistercian.draw()?;

    let roman = numerals::Roman::from_number(n);
    roman.draw()?;

    let braille = numerals::Braille::from_number(n);
    braille.draw()?;

    Ok(())
}

fn get_time_as_number() -> u32 {
    let local_time = Local::now();
    let hour = local_time.hour();
    let minute = local_time.minute();

    (hour * 100) + minute
}
