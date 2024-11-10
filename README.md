# tft-rs
Generic interface library for several SPI controlled displays.
Implements the `embedded_graphics_core::draw_target::DrawTarget` interface for these displays.
Heavily inspired by the [TFT_eSPI](https://github.com/Bodmer/TFT_eSPI) library.

Currently, this library only supports the ST7789 type of display, and is very much a work in progress.
The API will change in future versions.