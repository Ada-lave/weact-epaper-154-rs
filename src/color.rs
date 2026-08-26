use embedded_graphics_core::pixelcolor::PixelColor;

#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum WBRYColor {
    WHITE = 0b00,
    BLACK = 0b01,
    RED = 0b10,
    YELLOW = 0b11
}

impl PixelColor for WBRYColor {
    type Raw = ();
}