use embedded_graphics_core::pixelcolor::PixelColor;

#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum WBRYColor {
    BLACK = 0b00,
    WHITE = 0b01,
    YELLOW = 0b10,
    RED = 0b11,
}

impl PixelColor for WBRYColor {
    type Raw = ();
}