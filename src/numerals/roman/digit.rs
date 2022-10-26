///Digit representing a character suich as X with a value like 10;
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Digit(&'static str, usize);
pub const I: Digit = Digit("I", 1);
pub const V: Digit = Digit("V", 5);
pub const X: Digit = Digit("X", 10);
pub const L: Digit = Digit("L", 50);
pub const C: Digit = Digit("C", 100);
pub const D: Digit = Digit("D", 500);
pub const M: Digit = Digit("M", 1000);
///For readability only
impl Digit {
    pub fn str(&self) -> &str {
        &self.0
    }

    pub fn val(&self) -> usize {
        self.1
    }

    ///Return next bigger digit
    pub fn next(&self) -> Digit {
        match self {
            &I => V,
            &V => X,
            &X => L,
            &L => C,
            &C => D,
            &D => M,
            &Digit(&_, _) => todo!(),
        }
    }
}
