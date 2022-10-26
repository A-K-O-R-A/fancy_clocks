mod cistercian;
pub use cistercian::Cistercian;

pub trait Numeral {
    fn from_number(n: usize) -> Self;
    fn draw(&self) -> Result<(), Box<dyn std::error::Error>>;
}
