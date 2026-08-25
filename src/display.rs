use crate::color::Color;

const HEIGHT: usize = 200;
const WIDTH: usize = 200;

const FRAME_BUFFER: usize = (HEIGHT * WIDTH) / 4;

pub struct Display {
    data: [u8; FRAME_BUFFER]
}

impl Display {
    pub fn set_pixel(mut self, x: usize, y: usize, color: Color) {
        if x >= HEIGHT || y >= WIDTH {
            return;
        }

        let pixel = y * HEIGHT + x;
        let byte_index = pixel / 4;
        let pixel_in_byte = byte_index % 4;
        let shift = 6 - pixel_in_byte * 2;

        self.data[byte_index] &=
            !(0b11 << shift);

        // записать новые
        self.data[byte_index] |=
            (color as u8) << shift;
    }
}