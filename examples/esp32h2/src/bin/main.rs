#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]

use embedded_graphics::Drawable;
use embedded_graphics::geometry::Point;
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::mono_font::ascii::FONT_10X20;
use embedded_graphics::text::{Alignment, Text};
use embedded_hal_bus::spi::{ExclusiveDevice, NoDelay};
use esp_backtrace as _;
use esp_hal::clock::CpuClock;
use esp_hal::delay::Delay;
use esp_hal::gpio::{Input, InputConfig, Level, Output, OutputConfig};
use esp_hal::spi::master::Config;
use esp_hal::{main, spi};
use esp_println as _;
use weact_epaper_154_rs::color::WBRYColor;
use weact_epaper_154_rs::driver::WeAct154Display;

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[allow(
    clippy::large_stack_frames,
    reason = "it's not unusual to allocate larger buffers etc. in main"
)]
#[main]
fn main() -> ! {
    // generator version: 1.3.0
    // generator parameters: --chip esp32h2 -o unstable-hal -o nightly-2026-05-04-aarch64-apple-darwin -o vscode -o esp-backtrace -o defmt

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    let display_cs = Output::new(peripherals.GPIO5, Level::Low, OutputConfig::default());
    let display_reset = Output::new(peripherals.GPIO13, Level::High, OutputConfig::default());
    let dispay_dc = Output::new(peripherals.GPIO12, Level::Low, OutputConfig::default());
    let display_busy = Input::new(peripherals.GPIO14, InputConfig::default());

    let spi_bus = spi::master::Spi::new(peripherals.SPI2, Config::default())
        .unwrap()
        .with_sck(peripherals.GPIO10)
        .with_mosi(peripherals.GPIO11);

    let display_spi = ExclusiveDevice::new(spi_bus, display_cs, NoDelay).unwrap();

    let delay = Delay::new();

    let mut we_act_display =
        WeAct154Display::new(display_spi, dispay_dc, display_busy, delay, display_reset);

    we_act_display.reset().unwrap();
    we_act_display.init().unwrap();
    we_act_display.power_on().unwrap();

    let text_style_red = MonoTextStyle::new(&FONT_10X20, WBRYColor::RED);
    let text_style_yellow = MonoTextStyle::new(&FONT_10X20, WBRYColor::YELLOW);

    Text::with_alignment(
        "Driver Works",
        Point::new(100, 100),
        text_style_red,
        Alignment::Center,
    )
    .draw(&mut we_act_display)
    .unwrap();

    Text::with_alignment(
        "Press like ;)",
        Point::new(100, 124),
        text_style_yellow,
        Alignment::Center,
    )
    .draw(&mut we_act_display)
    .unwrap();

    we_act_display.flush().unwrap();
    we_act_display.power_off().unwrap();

    // Один refresh
    loop {}

    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/esp-hal-v1.1.0/examples
}
