use embedded_graphics_core::{
    Pixel,
    draw_target::DrawTarget,
    geometry::{OriginDimensions, Size},
};
use embedded_hal::{
    delay::DelayNs,
    digital::{InputPin, OutputPin},
    spi::SpiDevice,
};

use crate::{
    color::WBRYColor,
    driver::{ErrorOf, HEIGHT, WIDTH, WeAct154Display},
};

impl<SPI, DC, BUSY, DELAY, RESET> OriginDimensions for WeAct154Display<SPI, DC, BUSY, DELAY, RESET>
where
    SPI: SpiDevice,
    DC: OutputPin,
    BUSY: InputPin,
{
    fn size(&self) -> Size {
        Size::new(WIDTH, HEIGHT)
    }
}

impl<SPI, DC, BUSY, DELAY, RESET> WeAct154Display<SPI, DC, BUSY, DELAY, RESET>
where
    SPI: SpiDevice,
    DC: OutputPin,
    BUSY: InputPin,
    DELAY: DelayNs,
    RESET: OutputPin,
{
    /// Sets a single pixel in the framebuffer.
    ///
    /// Coordinates outside the display bounds are silently ignored.
    pub fn set_pixel(&mut self, x: i32, y: i32, color: WBRYColor) {
        if x < 0 || y < 0 || x >= WIDTH as i32 || y >= HEIGHT as i32 {
            return;
        }

        let pixel = y as u32 * WIDTH + x as u32;
        let byte_index = pixel / 4;
        let pixel_in_byte = pixel % 4;
        let shift = 6 - pixel_in_byte * 2;

        let buffer = self.frame_buffer_mut();
        buffer[byte_index as usize] &= !(0b11 << shift);
        buffer[byte_index as usize] |= (color as u8) << shift;
    }

    /// Reads the color of a pixel from the framebuffer.
    ///
    /// Returns `None` if coordinates are out of bounds.
    pub fn get_pixel(&self, x: i32, y: i32) -> Option<WBRYColor> {
        if x < 0 || y < 0 || x >= WIDTH as i32 || y >= HEIGHT as i32 {
            return None;
        }

        let pixel = y as u32 * WIDTH + x as u32;
        let byte_index = pixel / 4;
        let pixel_in_byte = pixel % 4;
        let shift = 6 - pixel_in_byte * 2;

        let byte = self.frame_buffer()[byte_index as usize];
        let bits = (byte >> shift) & 0b11;
        Some(WBRYColor::from_bits(bits))
    }
}

impl<SPI, DC, BUSY, DELAY, RESET> DrawTarget for WeAct154Display<SPI, DC, BUSY, DELAY, RESET>
where
    SPI: SpiDevice,
    DC: OutputPin,
    BUSY: InputPin,
    DELAY: DelayNs,
    RESET: OutputPin,
{
    type Color = WBRYColor;
    type Error = ErrorOf<SPI, DC, BUSY, RESET>;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for Pixel(point, color) in pixels.into_iter() {
            self.set_pixel(point.x, point.y, color);
        }
        Ok(())
    }
}
