use embedded_hal::{delay::DelayNs, digital::{InputPin, OutputPin}, spi::{Operation, SpiDevice}};


pub const HEIGHT: i32 = 200;
pub const WIDTH: i32 = 200;

const FRAME_BUFFER: i32 = (HEIGHT * WIDTH) / 4;

pub enum DriverError<SpiError, PinError, BusyPin, ResetPin> {
    Spi(SpiError),
    DcPin(PinError),
    BusyPin(BusyPin),
    ResetPin(ResetPin)
}

pub type ErrorOf<SPI, DC, BUSY, RESET> = DriverError<
    <SPI as embedded_hal::spi::ErrorType>::Error,
    <DC as embedded_hal::digital::ErrorType>::Error,
    <BUSY as embedded_hal::digital::ErrorType>::Error,
    <RESET as embedded_hal::digital::ErrorType>::Error
>;

pub struct DisplayDriver<SPI, DC, BUSY, DELAY, RESET> {
    frame_buffer: [u8; FRAME_BUFFER as usize],
    spi: SPI,
    dc: DC,
    busy: BUSY,
    delay: DELAY,
    reset: RESET
}

impl<SPI, DC, BUSY, DELAY, RESET> DisplayDriver<SPI, DC, BUSY, DELAY, RESET>
where 
    SPI: SpiDevice,
    DC: OutputPin,
    BUSY: InputPin,
    DELAY: DelayNs,
    RESET: OutputPin
{
    pub fn new(spi: SPI, dc: DC, busy: BUSY, delay: DELAY, reset: RESET) -> Self {
        return DisplayDriver { frame_buffer: [0x00; FRAME_BUFFER as usize], spi, dc, busy, delay, reset };
    }

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

    pub fn init(&mut self) -> Result<(), ErrorOf<SPI, DC, BUSY, RESET>> {
        self.send_command(0x4D)?;
        self.send_data(&[0x78])?;

        self.send_command(0x00)?;
        self.send_data(&[0x0F, 0x29])?;

        self.send_command(0x06)?;
        self.send_data(&[
            0x0D,
            0x12,
            0x30,
            0x20,
            0x19,
            0x2A,
            0x22,
        ])?;

        self.send_command(0x30)?;
        self.send_data(&[0x08])?;

        self.send_command(0x50)?;
        self.send_data(&[0x37])?;

        self.send_command(0x61)?;
        self.send_data(&[
            0x00, 0xC8,
            0x00, 0xC8
        ])?;

        self.send_command(0xE9)?;
        self.send_data(&[0x01])?;

        Ok(())
    }

    fn send_command(&mut self, cmd: u8) -> Result<(), ErrorOf<SPI, DC, BUSY, RESET>> {
        self.dc.set_low().map_err(DriverError::DcPin)?;

        self.spi.transaction(&mut [
            Operation::Write(&[cmd])
        ]).map_err(DriverError::Spi)?;

        Ok(())
    }

    fn send_data(&mut self, frame_buffer: &[u8]) -> Result<(), ErrorOf<SPI, DC, BUSY, RESET>> {
        self.dc.set_high().map_err(DriverError::DcPin)?;
        self.spi.transaction(&mut [
            Operation::Write(frame_buffer)
        ]).map_err(DriverError::Spi)?;
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

        self.spi.transaction(&mut [
            Operation::Write(&self.frame_buffer)
        ]).map_err(DriverError::Spi)?;

        Ok(())
    }
    
    pub fn flush(&mut self) -> Result<(), ErrorOf<SPI, DC, BUSY, RESET>> {
        self.send_command(0x10)?;
        self.send_frame_buffer()?;

        self.send_command(0x12)?;
        self.send_data(&[0x00])?;
        self.wait_until_idle()?;

        self.send_command(0x02)?;
        self.send_data(&[0x00])?;
        self.wait_until_idle()?;

        Ok(())
    }

    pub fn frame_buffer_mut(&mut self) -> &mut [u8] {
        &mut self.frame_buffer
    }
}

