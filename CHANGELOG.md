# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.2] - 2026-08-28

### Added

- Add `clear`, `get_pixel`, immutable `frame_buffer`, and `release` APIs.
- Add `Default` and raw two-bit conversion support for `WBRYColor`.
- Add crate-level and public API documentation.

### Changed

- Use unsigned types for display dimensions and framebuffer indexes.

### Fixed

- Correct coordinate bounds checks and framebuffer indexing.

## [0.1.1] - 2026-08-27

### Added

- Add an experimental `no_std` driver for the 1.54-inch, 200 x 200,
  four-color WeAct e-paper display.
- Add a 10,000-byte packed framebuffer with black, white, red, and yellow
  pixel support.
- Add an `embedded-graphics-core` `DrawTarget` implementation.
- Add hardware reset, display initialization, power-on, power-off, and
  full-screen refresh operations.
- Add separate error variants for SPI, DC, BUSY, and RESET failures.
- Add an ESP32-H2 hardware example.

### Fixed

- Correct the two-bit pixel packing within framebuffer bytes.
- Correct the encoded values used for the four supported colors.
- Make `flush` send a display refresh command after transferring framebuffer
  data.
- Wait for the display to become ready after transmitting data.

### Changed

- Rename the display type to `WeAct154Display`.
- Return operation results so hardware communication errors can be handled by
  callers.
- Require callers to power off the display explicitly instead of powering it
  off as part of `flush`.

[Unreleased]: https://github.com/Ada-lave/weact-epaper-154-rs/compare/v0.1.2...HEAD
[0.1.2]: https://github.com/Ada-lave/weact-epaper-154-rs/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/Ada-lave/weact-epaper-154-rs/releases/tag/v0.1.1
