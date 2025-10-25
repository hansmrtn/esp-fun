#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

use defmt::info;
use embedded_graphics::{
    mono_font::{MonoTextStyleBuilder, ascii::FONT_6X10, ascii::FONT_10X20},
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Baseline, Text},
};
use esp_hal::clock::CpuClock;
use esp_hal::i2c::master::{Config, I2c};
use esp_hal::main;
use esp_println as _;
use ssd1306::{I2CDisplayInterface, Ssd1306, prelude::*};

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let p = esp_hal::init(config);

    let i2c = I2c::new(p.I2C0, Config::default())
        .unwrap()
        .with_sda(p.GPIO4)
        .with_scl(p.GPIO5);

    let interface = I2CDisplayInterface::new(i2c);

    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();

    display.init().unwrap();

    let text_style = MonoTextStyleBuilder::new()
        .font(&FONT_10X20)
        .text_color(BinaryColor::On)
        .build();

    loop {
        Text::with_baseline("Hello, Hans!", Point::new(0, 16), text_style, Baseline::Top)
            .draw(&mut display)
            .unwrap();
        display.flush().unwrap();
    }

}
