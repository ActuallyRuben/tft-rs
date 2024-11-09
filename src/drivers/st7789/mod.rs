use crate::display::DisplayInterface;
use crate::drivers::st7789::command::Command;
use crate::drivers::DisplayDriver;
use crate::{Error, Orientation};
use embedded_graphics_core::pixelcolor::raw::ToBytes;
use embedded_graphics_core::pixelcolor::Rgb565;
use embedded_graphics_core::primitives::Rectangle;
use embedded_hal::delay::DelayNs;
use embedded_hal::digital::OutputPin;
use embedded_hal::spi::SpiDevice;

pub mod command;

pub struct ST7789(pub Orientation, pub Rectangle);

impl ST7789 {
    fn command(&self, di: &mut impl DisplayInterface, command: Command) -> Result<(), Error> {
        di.dc().set_low().map_err(|_| Error::GPIO)?;
        di.spi().write(&[command as u8]).map_err(|_| Error::SPI)?;
        di.dc().set_high().map_err(|_| Error::GPIO)?;
        Ok(())
    }

    fn data(&self, di: &mut impl DisplayInterface, data: &[u8]) -> Result<(), Error> {
        di.spi().write(data).map_err(|_| Error::SPI)?;
        Ok(())
    }

    fn reset(&self, di: &mut impl DisplayInterface) -> Result<(), Error> {
        di.reset().set_low().map_err(|_| Error::GPIO)?;
        di.delay().delay_us(20);
        di.reset().set_high().map_err(|_| Error::GPIO)?;
        di.delay().delay_ms(120);

        Ok(())
    }
}

impl DisplayDriver for ST7789 {
    type Color = Rgb565;
    fn bounding_box(&self) -> Rectangle {
        self.1
    }

    fn set_draw_area(
        &mut self,
        di: &mut impl DisplayInterface,
        area: &Rectangle,
    ) -> Result<(), Error> {
        let mut rect = *area;
        rect.top_left += self.1.top_left;
        let [xs_hi, xs_lo] = (rect.top_left.x as u16).to_be_bytes();
        let [xe_hi, xe_lo] =
            (rect.bottom_right().ok_or(Error::InvalidBoundingBox)?.x as u16).to_be_bytes();
        let [ys_hi, ys_lo] = (rect.top_left.y as u16).to_be_bytes();
        let [ye_hi, ye_lo] =
            (rect.bottom_right().ok_or(Error::InvalidBoundingBox)?.y as u16).to_be_bytes();

        self.command(di, Command::CASET)?;
        self.data(di, &[xs_hi, xs_lo, xe_hi, xe_lo])?;
        self.command(di, Command::RASET)?;
        self.data(di, &[ys_hi, ys_lo, ye_hi, ye_lo])?;
        self.command(di, Command::RAMWR)?;
        Ok(())
    }

    fn write_color_data(
        &mut self,
        di: &mut impl DisplayInterface,
        data: impl IntoIterator<Item = Self::Color>,
    ) -> Result<(), Error> {
        let buffer: Vec<u8> = data.into_iter().flat_map(|x| x.to_be_bytes()).collect();
        di.spi().write(&buffer).map_err(|_| Error::SPI)
    }

    fn init(&mut self, di: &mut impl DisplayInterface) -> Result<(), Error> {
        use Command::*;

        di.dc().set_high().map_err(|_| Error::GPIO)?;

        self.reset(di)?;
        self.command(di, SLPOUT)?;
        di.delay().delay_ms(120);
        self.command(di, NORON)?;
        self.command(di, MADCTL)?;
        self.data(di, &[0b00001000])?;
        self.command(di, UNKNOWN)?;
        self.data(di, &[0x0A, 0x82])?;
        self.command(di, COLMOD)?;
        self.data(di, &[0x55])?;
        di.delay().delay_ms(10);
        self.command(di, PORCTRL)?;
        self.data(di, &[0x0c, 0x0c, 0x00, 0x33, 0x33])?;
        self.command(di, GCTRL)?;
        self.data(di, &[0x35])?;
        self.command(di, VCOMS)?;
        self.data(di, &[0x28])?;
        self.command(di, LCMCTRL)?;
        self.data(di, &[0x9C])?;
        self.command(di, VDVVRHEN)?;
        self.data(di, &[0x01, 0xFF])?;
        self.command(di, VRHS)?;
        self.data(di, &[0x10])?;
        self.command(di, VDVS)?;
        self.data(di, &[0x20])?;
        self.command(di, FRCTRL2)?;
        self.data(di, &[0x0f])?;
        self.command(di, PWCTRL1)?;
        self.data(di, &[0xA4, 0xA1])?;
        self.command(di, PVGAMCTRL)?;
        self.data(
            di,
            &[
                0xd0, 0x00, 0x02, 0x07, 0x0a, 0x28, 0x32, 0x44, 0x42, 0x06, 0x0e, 0x12, 0x14, 0x17,
            ],
        )?;
        self.command(di, NVGAMCTRL)?;
        self.data(
            di,
            &[
                0xd0, 0x00, 0x02, 0x07, 0x0a, 0x28, 0x31, 0x54, 0x47, 0x0e, 0x1c, 0x17, 0x1b, 0x1e,
            ],
        )?;
        self.command(di, CASET)?;
        self.data(di, &[0x00, 0x00, 0x00, 0xE5])?;
        self.command(di, RASET)?;
        self.data(di, &[0x00, 0x00, 0x01, 0x3F])?;
        di.delay().delay_ms(120);
        self.command(di, DISPON)?;
        self.command(di, RAMWR)?;
        Ok(())
    }
}
