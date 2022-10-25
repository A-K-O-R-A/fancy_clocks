mod numerals;
use numerals::lib::Numeral;

fn main() {
    let n = 9933;
    let a = numerals::Cistercian::from_number(n);
    println!("Hello, world of {n}");
    a.draw();
}
