use embedded_hal::{
    delay::DelayNs,
    digital::{InputPin, OutputPin},
    spi::{Operation, SpiDevice},
};

use crate::color::WBRYColor;

/// Display width in pixels.
pub const WIDTH: u32 = 200;

/// Display height in pixels.
pub const HEIGHT: u32 = 200;

const FRAME_BUFFER_SIZE: usize = ((WIDTH * HEIGHT) / 4) as usize;

/// Errors specific to the WeAct 154 display driver.
#[derive(Debug)]
pub enum DriverError<SpiError, PinError, BusyPin, ResetPin> {
    /// SPI bus error.
    Spi(SpiError),
    /// Data/Command pin error.
    DcPin(PinError),
    /// Busy pin error.
    BusyPin(BusyPin),
    /// Reset pin error.
    ResetPin(ResetPin),
    /// BUSY polling timed out.
    Timeout,
}

/// Convenience alias resolving the associated error types for a concrete pin/SPI set.
pub type ErrorOf<SPI, DC, BUSY, RESET> = DriverError<
    <SPI as embedded_hal::spi::ErrorType>::Error,
    <DC as embedded_hal::digital::ErrorType>::Error,
    <BUSY as embedded_hal::digital::ErrorType>::Error,
    <RESET as embedded_hal::digital::ErrorType>::Error,
>;

/// A driver for the WeAct 1.54-inch 200x200 four-color (B/W/R/Y) e-paper display.
///
/// # Startup sequence
///
/// ```text
/// new -> reset -> init -> power_on -> [draw] -> flush -> power_off
/// ```
///
/// # Hardware requirements
///
/// | Signal | Direction | Purpose |
/// |--------|-----------|---------|
/// | SPI    | Output    | Commands and framebuffer data |
/// | DC     | Output    | Selects command or data mode |
/// | BUSY   | Input     | Indicates that the controller is busy |
/// | RESET  | Output    | Resets the display controller |
/// | DELAY  | N/A       | Provides reset and polling delays |
pub struct WeAct154Display<SPI, DC, BUSY, DELAY, RESET> {
    frame_buffer: [u8; FRAME_BUFFER_SIZE],
    spi: SPI,
    dc: DC,
    busy: BUSY,
    delay: DELAY,
    reset: RESET,
}

impl<SPI, DC, BUSY, DELAY, RESET> WeAct154Display<SPI, DC, BUSY, DELAY, RESET>
where
    SPI: SpiDevice,
    DC: OutputPin,
    BUSY: InputPin,
    DELAY: DelayNs,
    RESET: OutputPin,
{
    /// Creates a new display driver instance.
    ///
    /// The framebuffer is initialised to white (`0x55`).
    pub fn new(spi: SPI, dc: DC, busy: BUSY, delay: DELAY, reset: RESET) -> Self {
        WeAct154Display {
            frame_buffer: [0x55; FRAME_BUFFER_SIZE],
            spi,
            dc,
            busy,
            delay,
            reset,
        }
    }

    /// Consumes the driver and returns the owned peripherals.
    pub fn release(self) -> (SPI, DC, BUSY, DELAY, RESET) {
        (self.spi, self.dc, self.busy, self.delay, self.reset)
    }

    /// Fills the entire framebuffer with the given color.
    pub fn clear(&mut self, color: WBRYColor) {
        let pattern: u8 = match color {
            WBRYColor::BLACK => 0x00,
            WBRYColor::WHITE => 0x55,
            WBRYColor::YELLOW => 0xAA,
            WBRYColor::RED => 0xFF,
        };
        self.frame_buffer.fill(pattern);
    }

    /// Returns a read-only reference to the framebuffer.
    pub fn frame_buffer(&self) -> &[u8] {
        &self.frame_buffer
    }

    /// Returns a mutable reference to the raw packed framebuffer.
    ///
    /// Each byte holds four pixels with two bits per pixel. Prefer [`clear`]
    /// or the `DrawTarget` implementation unless direct access to the packed
    /// pixel format is required.
    pub fn frame_buffer_mut(&mut self) -> &mut [u8] {
        &mut self.frame_buffer
    }

    fn send_command(&mut self, cmd: u8) -> Result<(), ErrorOf<SPI, DC, BUSY, RESET>> {
        self.dc.set_low().map_err(DriverError::DcPin)?;
        self.spi
            .transaction(&mut [Operation::Write(&[cmd])])
            .map_err(DriverError::Spi)?;
        Ok(())
    }

    fn send_data(&mut self, data: &[u8]) -> Result<(), ErrorOf<SPI, DC, BUSY, RESET>> {
        self.dc.set_high().map_err(DriverError::DcPin)?;
        self.spi
            .transaction(&mut [Operation::Write(data)])
            .map_err(DriverError::Spi)?;
        Ok(())
    }

    /// Performs a hardware reset sequence.
    pub fn reset(&mut self) -> Result<(), ErrorOf<SPI, DC, BUSY, RESET>> {
        self.reset.set_high().map_err(DriverError::ResetPin)?;
        self.delay.delay_ms(50);
        self.reset.set_low().map_err(DriverError::ResetPin)?;
        self.delay.delay_ms(5);
        self.reset.set_high().map_err(DriverError::ResetPin)?;
        self.delay.delay_ms(50);
        self.wait_until_idle()?;
        Ok(())
    }

    /// Writes the controller initialization sequence.
    ///
    /// Commands target the JD79660-style four-color controller used on
    /// WeAct 1.54" panels. Verify your panel's FPC markings before use.
    pub fn init(&mut self) -> Result<(), ErrorOf<SPI, DC, BUSY, RESET>> {
        // Booster soft start
        self.send_command(0x4D)?;
        self.send_data(&[0x78])?;

        // Panel setting: LUT selection + reserved bits
        self.send_command(0x00)?;
        self.send_data(&[0x0F, 0x29])?;

        // Write voltage register values
        self.send_command(0x06)?;
        self.send_data(&[0x0D, 0x12, 0x30, 0x20, 0x19, 0x2A, 0x22])?;

        // PLL control
        self.send_command(0x30)?;
        self.send_data(&[0x08])?;

        // VCOM and data interval setting
        self.send_command(0x50)?;
        self.send_data(&[0x37])?;

        // Resolution setting: 200x200
        self.send_command(0x61)?;
        self.send_data(&[0x00, 0xC8, 0x00, 0xC8])?;

        // Gate/source timing (panel-specific)
        self.send_command(0xE9)?;
        self.send_data(&[0x01])?;

        Ok(())
    }

    /// Powers on the display controller and waits until it is ready.
    pub fn power_on(&mut self) -> Result<(), ErrorOf<SPI, DC, BUSY, RESET>> {
        self.send_command(0x04)?;
        self.wait_until_idle()?;
        Ok(())
    }

    /// Powers off the display controller and waits until it is ready.
    pub fn power_off(&mut self) -> Result<(), ErrorOf<SPI, DC, BUSY, RESET>> {
        self.send_command(0x02)?;
        self.send_data(&[0x00])?;
        self.wait_until_idle()?;
        Ok(())
    }

    fn wait_until_idle(&mut self) -> Result<(), ErrorOf<SPI, DC, BUSY, RESET>> {
        while self.busy.is_low().map_err(DriverError::BusyPin)? {
            self.delay.delay_ms(1);
        }
        Ok(())
    }

    fn send_frame_buffer(&mut self) -> Result<(), ErrorOf<SPI, DC, BUSY, RESET>> {
        self.dc.set_high().map_err(DriverError::DcPin)?;
        self.spi
            .transaction(&mut [Operation::Write(&self.frame_buffer)])
            .map_err(DriverError::Spi)?;
        Ok(())
    }

    /// Sends the framebuffer to the display and triggers a full-screen refresh.
    pub fn flush(&mut self) -> Result<(), ErrorOf<SPI, DC, BUSY, RESET>> {
        // Command 0x10: DATA START TRANSMISSION — sends the full frame buffer
        self.send_command(0x10)?;
        self.send_frame_buffer()?;
        self.wait_until_idle()?;

        // Command 0x12: DISPLAY REFRESH — triggers the physical update
        self.send_command(0x12)?;
        self.send_data(&[0x00])?;
        self.wait_until_idle()?;

        Ok(())
    }
}
