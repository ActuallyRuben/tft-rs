use crate::drivers::DisplayDriver;
use crate::Error;
use embedded_graphics_core::draw_target::DrawTarget;
use embedded_graphics_core::geometry::Dimensions;
use embedded_graphics_core::prelude::*;
use embedded_graphics_core::primitives::Rectangle;
use embedded_graphics_core::Pixel;
use embedded_hal::delay::DelayNs;
use embedded_hal::digital::OutputPin;
use embedded_hal::spi::SpiDevice;

pub struct Display<DI: DisplayInterface, BL, DRIVER> {
    di: DI,
    bl: Option<BL>,
    driver: DRIVER,
}

impl<SPI, DC, RST, DELAY, BL, DRIVER> Display<DisplayInterfaceImpl<SPI, DC, RST, DELAY>, BL, DRIVER>
where
    BL: OutputPin,
    DRIVER: DisplayDriver,
    SPI: SpiDevice,
    DC: OutputPin,
    RST: OutputPin,
    DELAY: DelayNs,
{
    pub fn new(
        spi: SPI,
        dc: DC,
        rst: RST,
        bl: Option<BL>,
        delay: DELAY,
        driver: DRIVER,
    ) -> Result<Self, Error> {
        let di = DisplayInterfaceImpl {
            spi,
            dc,
            rst,
            delay,
        };
        let mut device = Self { di, bl, driver };
        device.driver.init(&mut device.di)?;
        if let Some(bl) = &mut device.bl {
            bl.set_high().map_err(|_| Error::GPIO)?;
        }
        Ok(device)
    }
}

pub struct DisplayInterfaceImpl<SPI: SpiDevice, DC: OutputPin, RST: OutputPin, DELAY: DelayNs> {
    spi: SPI,
    dc: DC,
    rst: RST,
    delay: DELAY,
}

pub trait DisplayInterface {
    fn spi(&mut self) -> &mut impl SpiDevice;
    fn dc(&mut self) -> &mut impl OutputPin;
    fn reset(&mut self) -> &mut impl OutputPin;
    fn delay(&mut self) -> &mut impl DelayNs;
}

impl<SPI: SpiDevice, DC: OutputPin, RST: OutputPin, DELAY: DelayNs> DisplayInterface
    for DisplayInterfaceImpl<SPI, DC, RST, DELAY>
{
    fn spi(&mut self) -> &mut impl SpiDevice {
        &mut self.spi
    }

    fn dc(&mut self) -> &mut impl OutputPin {
        &mut self.dc
    }

    fn reset(&mut self) -> &mut impl OutputPin {
        &mut self.rst
    }

    fn delay(&mut self) -> &mut impl DelayNs {
        &mut self.delay
    }
}

impl<DI: DisplayInterface, BL, DRIVER: DisplayDriver> Dimensions for Display<DI, BL, DRIVER> {
    fn bounding_box(&self) -> Rectangle {
        Rectangle::new(Point::new(0, 0), self.driver.bounding_box().size)
    }
}

impl<DI: DisplayInterface, BL, DRIVER: DisplayDriver> DrawTarget for Display<DI, BL, DRIVER> {
    type Color = DRIVER::Color;
    type Error = Error;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for Pixel(p, c) in pixels {
            if !(self.bounding_box().contains(p)) {
                continue;
            }
            self.driver
                .set_draw_area(&mut self.di, &Rectangle::new(p, Size::new(2, 2)))?;
            self.driver.write_color_data(&mut self.di, [c])?
        }
        Ok(())
    }

    fn fill_contiguous<I>(&mut self, area: &Rectangle, colors: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Self::Color>,
    {
        let bb = self.bounding_box();
        let area = self.bounding_box().intersection(area);
        self.driver.set_draw_area(&mut self.di, &area)?;
        let iter = area
            .points()
            .zip(colors)
            .filter(|(p, _)| bb.contains(*p))
            .map(|x| x.1);
        self.driver.write_color_data(&mut self.di, iter)
    }
}
