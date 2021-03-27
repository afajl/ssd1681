//! SSD1681 ePaper Display Driver
//!
//! Used in the [Adafruit 1.54" Tri-Color display](https://www.adafruit.com/product/4868)
//!
//! For a complete example see [the example](https://github.com/afajl/ssd1681/blob/master/examples/adafruit_1in54_tricolor.rs).
//!
//! This driver is losely modeled after the
//! [epd-waveshare](https://github.com/caemor/epd-waveshare) drivers but built for my needs.
//!
//!
//! ### Usage
//! This driver does not hide that you're working with one buffer for black/white and one for red. To
//! display something you:
//!
//! 1. first create a buffer (either b/w or red) and draw things onto it, preferably
//! with [`embedded_graphics`](https://github.com/jamwaffles/embedded-graphics).
//! 1. then send the frame to the display driver using [`driver::Ssd1681::update_bw_frame`] or
//!    [`driver::Ssd1681::update_red_frame`]
//! 1. then kick off a display update using [`driver::Ssd1681::update_bw_frame`] or
//!     [`driver::Ssd1681::update_red_frame`]
//!
//!
#![no_std]
#![deny(missing_docs)]
#![allow(clippy::pedantic)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::missing_errors_doc)]

pub mod color;
pub mod driver;

#[cfg(feature = "graphics")]
pub mod graphics;

mod cmd {
    pub const SW_RESET: u8 = 0x12;
    pub const DRIVER_CONTROL: u8 = 0x01;
    pub const DATA_ENTRY_MODE: u8 = 0x11;
    pub const SET_RAMXPOS: u8 = 0x44;
    pub const SET_RAMYPOS: u8 = 0x45;
    pub const BORDER_WAVEFORM_CONTROL: u8 = 0x3C;
    pub const TEMP_CONTROL: u8 = 0x18;

    // Update
    pub const SET_RAMX_COUNTER: u8 = 0x4E;
    pub const SET_RAMY_COUNTER: u8 = 0x4F;
    pub const WRITE_BW_DATA: u8 = 0x24;
    pub const WRITE_RED_DATA: u8 = 0x26;
    pub const UPDATE_DISPLAY_CTRL2: u8 = 0x22;
    pub const MASTER_ACTIVATE: u8 = 0x20;
}

mod flag {
    pub const DATA_ENTRY_INCRY_INCRX: u8 = 0b11;
    pub const INTERNAL_TEMP_SENSOR: u8 = 0x80;
    pub const BORDER_WAVEFORM_FOLLOW_LUT: u8 = 0b0100;
    pub const BORDER_WAVEFORM_LUT1: u8 = 0b0001;
    pub const DISPLAY_MODE_1: u8 = 0xF7;
}

/// Maximum display height this driver supports
pub const HEIGHT: u8 = 200;

/// Maximum display width this driver supports
pub const WIDTH: u8 = 200;

pub mod interface;

/// Useful exports
pub mod prelude {
    pub use crate::color::Color;
    pub use crate::driver::Ssd1681;

    #[cfg(feature = "graphics")]
    pub use crate::graphics::{Display, Display1in54, DisplayRotation};
}
