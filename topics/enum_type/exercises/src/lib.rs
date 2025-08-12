#[derive(Debug, PartialEq)]
#[repr(u8)]
pub enum Color {
    Red,
    Green,
    Blue,
    Rgba(u8, u8, u8, f32),
}
