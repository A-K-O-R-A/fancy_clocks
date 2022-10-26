///A pattern repressents one braille sign
/// Example
/// x o
/// x o
/// o o
/// Would result in Pattern("⠃")
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Pattern(pub &'static str);

const X: bool = true;
const O: bool = false;

///Source: https://nfb.org/images/nfb/images/braillereadingpals/braille_numbers_graphic.jpg

pub const NUMBER_SIGN: Pattern = Pattern("⠼");
pub const ZERO: Pattern = Pattern("⠚");
pub const ONE: Pattern = Pattern("⠁");
pub const TWO: Pattern = Pattern("⠃");
pub const THREE: Pattern = Pattern("⠉");
pub const FOUR: Pattern = Pattern("⠙");
pub const FIVE: Pattern = Pattern("⠑");
pub const SIX: Pattern = Pattern("⠋");
pub const SEVEN: Pattern = Pattern("⠛");
pub const EIGHT: Pattern = Pattern("⠓");
pub const NINE: Pattern = Pattern("⠊");

pub trait IntoPattern {
    fn into_pattern(&self) -> Pattern;
}

impl IntoPattern for usize {
    fn into_pattern(&self) -> Pattern {
        match self {
            0 => ZERO,
            1 => ONE,
            2 => TWO,
            3 => THREE,
            4 => FOUR,
            5 => FIVE,
            6 => SIX,
            7 => SEVEN,
            8 => EIGHT,
            9 => NINE,
            _ => panic!("Can't convert {} to braille pattern", self),
        }
    }
}
