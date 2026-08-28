use embedded_graphics_core::pixelcolor::PixelColor;

/// Four-color pixel type for the WeAct 1.54" e-paper display.
///
/// Each pixel uses two bits. The packed framebuffer stores four pixels per
/// byte in big-endian order (MSB first).
#[repr(u8)]
#[derive(Copy, Clone, Default, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum WBRYColor {
    /// Black (0b00).
    BLACK = 0b00,
    /// White (0b01).
    #[default]
    WHITE = 0b01,
    /// Yellow (0b10).
    YELLOW = 0b10,
    /// Red (0b11).
    RED = 0b11,
}

impl WBRYColor {
    /// Reconstructs a color from its raw two-bit value.
    ///
    /// Any value above `0b11` is clamped to `RED`.
    pub fn from_bits(bits: u8) -> Self {
        match bits & 0b11 {
            0b00 => WBRYColor::BLACK,
            0b01 => WBRYColor::WHITE,
            0b10 => WBRYColor::YELLOW,
            _ => WBRYColor::RED,
        }
    }
}

impl PixelColor for WBRYColor {
    type Raw = ();
}
