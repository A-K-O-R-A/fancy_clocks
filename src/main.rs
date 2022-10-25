mod numerals;
use numerals::lib::Numeral;

fn main() {
    let a = numerals::Cistercian::from_number(10);
    println!("Hello, world! {:?}", a);
    a.draw();
}
