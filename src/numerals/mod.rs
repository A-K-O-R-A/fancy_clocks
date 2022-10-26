//Expose only the different numerals
mod cistercian;
pub use cistercian::Cistercian;
mod roman;
pub use roman::Roman;

pub trait Numeral {
    fn from_number(n: usize) -> Self;
    fn draw(&self) -> Result<(), Box<dyn std::error::Error>>;
}
