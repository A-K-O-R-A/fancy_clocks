mod numerals;
use chrono::prelude::*;
use numerals::Numeral;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let n = get_time_as_number();
    println!("Current time {}", n);
    let num = numerals::Cistercian::from_number(n as usize);
    num.draw()
}

fn get_time_as_number() -> u32 {
    let local_time = Local::now();
    let hour = local_time.hour();
    let minute = local_time.minute();

    (hour * 100) + minute
}
