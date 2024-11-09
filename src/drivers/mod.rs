use crate::display::DisplayInterface;
use crate::Error;
use embedded_graphics_core::pixelcolor::PixelColor;
use embedded_graphics_core::primitives::Rectangle;

pub mod st7789;

pub trait DisplayDriver {
    type Color: PixelColor;
    fn bounding_box(&self) -> Rectangle;
    fn set_draw_area(
        &mut self,
        di: &mut impl DisplayInterface,
        area: &Rectangle,
    ) -> Result<(), Error>;
    fn write_color_data(
        &mut self,
        di: &mut impl DisplayInterface,
        data: impl IntoIterator<Item = Self::Color>,
    ) -> Result<(), Error>;
    fn init(&mut self, di: &mut impl DisplayInterface) -> Result<(), Error>;
}
