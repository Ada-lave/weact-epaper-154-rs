use embedded_hal::{delay::DelayNs, digital::{InputPin, OutputPin}, spi::{Operation, SpiDevice}};


pub const HEIGHT: i32 = 200;
pub const WIDTH: i32 = 200;

const FRAME_BUFFER: i32 = (HEIGHT * WIDTH) / 4;

pub enum DriverError<SpiError, PinError, BusyPin> {
    Spi(SpiError),
    DcPin(PinError),
    BusyPin(BusyPin)
}

pub type ErrorOf<SPI, DC, BUSY> = DriverError<
    <SPI as embedded_hal::spi::ErrorType>::Error,
    <DC as embedded_hal::digital::ErrorType>::Error,
    <BUSY as embedded_hal::digital::ErrorType>::Error
>;

pub struct DisplayDriver<SPI, DC, BUSY, DELAY> {
    frame_buffer: [u8; FRAME_BUFFER as usize],
    spi: SPI,
    dc: DC,
    busy: BUSY,
    delay: DELAY
}

impl<SPI, DC, BUSY, DELAY> DisplayDriver<SPI, DC, BUSY, DELAY>
where 
    SPI: SpiDevice,
    DC: OutputPin,
    BUSY: InputPin,
    DELAY: DelayNs
{
    pub fn new(spi: SPI, dc: DC, busy: BUSY, delay: DELAY) -> Self {
        return DisplayDriver { frame_buffer: [0; FRAME_BUFFER as usize], spi, dc, busy, delay };
    }

    fn send_command(&mut self, cmd: u8) -> Result<(), ErrorOf<SPI, DC, BUSY>> {
        self.dc.set_low().map_err(DriverError::DcPin)?;

        self.spi.transaction(&mut [
            Operation::Write(&[cmd])
        ]).map_err(DriverError::Spi)?;

        Ok(())
    }

    fn send_data(&mut self, frame_buffer: &[u8]) -> Result<(), ErrorOf<SPI, DC, BUSY>> {
        self.dc.set_high().map_err(DriverError::DcPin)?;
        self.spi.transaction(&mut [
            Operation::Write(frame_buffer)
        ]).map_err(DriverError::Spi)?;
        Ok(())
    }

    fn wait_until_idle(&mut self) -> Result<(), ErrorOf<SPI, DC, BUSY>> {
        while self.busy.is_low().map_err(DriverError::BusyPin)? {
            self.delay.delay_ms(1);
        }
        Ok(())
    }

    pub fn power_on(&mut self) -> Result<(), ErrorOf<SPI, DC, BUSY>> {
        self.send_command(0x06)?;
        self.wait_until_idle()?;
        Ok(())
    }

    fn send_frame_buffer(&mut self) -> Result<(), ErrorOf<SPI, DC, BUSY>> {
        self.dc.set_high().map_err(DriverError::DcPin)?;

        self.spi.transaction(&mut [
            Operation::Write(&self.frame_buffer)
        ]).map_err(DriverError::Spi)?;

        Ok(())
    }
    
    pub fn flush(&mut self) -> Result<(), ErrorOf<SPI, DC, BUSY>> {
        self.send_command(0x10)?;
        self.wait_until_idle()?;
        self.send_frame_buffer()?;
        Ok(())
    }

    pub fn frame_buffer_mut(&mut self) -> &mut [u8] {
        &mut self.frame_buffer
    }
}

