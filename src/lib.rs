//! Driver for the WeAct 1.54-inch 200x200 four-color (black, white, red,
//! yellow) e-paper display.
//!
//! Built on [`embedded-hal`] 1.0 SPI, GPIO, and delay traits. The
//! [`Display`](driver::WeAct154Display) type implements
//! [`DrawTarget`](embedded_graphics_core::draw_target::DrawTarget) so it
//! integrates directly with [`embedded-graphics`].
//!
//! # Quick start
//!
//! ```text
//! new -> reset -> init -> power_on -> [draw] -> flush -> power_off
//! ```
//!
//! [`embedded-hal`]: https://docs.rs/embedded-hal/1.0
//! [`embedded-graphics`]: https://docs.rs/embedded-graphics

#![no_std]
#![warn(missing_docs)]

/// Four-color pixel type (`WBRYColor`) and conversion helpers.
pub mod color;

/// `embedded-graphics` integration (`DrawTarget`, `OriginDimensions`).
pub mod display;

/// Low-level SPI driver, hardware constants, and error types.
pub mod driver;
