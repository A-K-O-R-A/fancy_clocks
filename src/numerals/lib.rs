pub trait Numeral {
    fn from_number(n: usize) -> Self;
    fn draw(&self);
}
