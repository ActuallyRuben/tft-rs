pub mod display;
pub mod drivers;
pub enum Orientation {
    Portrait,
    Landscape,
    PortraitReverse,
    LandscapeReverse,
}

#[derive(Debug)]
pub enum Error {
    GPIO,
    SPI,
    InvalidBoundingBox,
    Delay,
}
