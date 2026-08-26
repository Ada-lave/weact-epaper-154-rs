use embedded_graphics_core::{Pixel, draw_target::DrawTarget, geometry::{OriginDimensions, Size}};
use embedded_hal::{delay::DelayNs, digital::{InputPin, OutputPin}, spi::SpiDevice};

use crate::{color::WBRYColor, driver::{DisplayDriver, ErrorOf, HEIGHT, WIDTH}};

impl<SPI, DC, BUSY, DELAY> OriginDimensions for DisplayDriver<SPI, DC, BUSY, DELAY>
where 
    SPI: SpiDevice,
    DC: OutputPin,
    BUSY: InputPin
{
    fn size(&self) -> Size {
        return Size { width: WIDTH as u32, height: HEIGHT as u32};
    }
}


impl<SPI, DC, BUSY, DELAY> DisplayDriver<SPI, DC, BUSY, DELAY>
where 
    SPI: SpiDevice,
    DC: OutputPin,
    BUSY: InputPin,
    DELAY: DelayNs
 
{
    pub fn set_pixel(&mut self, x: i32, y: i32, color: WBRYColor) {
        if x < 0 || y < 0 || x >= HEIGHT || y >= WIDTH {
            return;
        }

        let pixel = y * HEIGHT + x;
        let byte_index = pixel / 4;
        let pixel_in_byte = pixel % 4;
        let shift = 6 - pixel_in_byte * 2;

        let buffer = self.frame_buffer_mut();

        buffer[byte_index as usize] &=
            !(0b11 << shift);

        // write new
        buffer[byte_index as usize] |=
            (color as u8) << shift;
    }
}

impl<SPI, DC, BUSY, DELAY> DrawTarget for DisplayDriver<SPI, DC, BUSY, DELAY>
where 
    SPI: SpiDevice,
    DC: OutputPin,
    BUSY: InputPin,
    DELAY: DelayNs
 
{
    type Color = WBRYColor;
    type Error = ErrorOf<SPI, DC, BUSY>;


    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>
    {
        for Pixel(point, color) in pixels.into_iter() {
            self.set_pixel(point.x, point.y, color);
        }
        Ok(())
    }
}