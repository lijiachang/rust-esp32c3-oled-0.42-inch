use std::fmt::Error;
use std::net::Ipv4Addr;
use embedded_graphics::mono_font::MonoTextStyle;
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::hal::delay::FreeRtos;
use esp_idf_svc::hal::gpio::PinDriver;
use esp_idf_svc::hal::prelude::Peripherals;
use esp_idf_svc::nvs::EspDefaultNvsPartition;
use esp_idf_svc::ping::EspPing;
use esp_idf_svc::wifi::{BlockingWifi, ClientConfiguration, Configuration, EspWifi};
use embedded_svc::http::client::Client;
use embedded_svc::utils::io::try_read_full;
use esp_idf_svc::http::client::EspHttpConnection;
use log::info;

use esp_idf_svc::http::client::Configuration as HttpConfig;

use embedded_graphics::{
    mono_font::{ascii::{FONT_10X20,FONT_4X6}, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Baseline, Text},
  };
use ssd1306::command::Command;
use ssd1306::{mode::BufferedGraphicsMode, prelude::*, I2CDisplayInterface, Ssd1306};
use esp_idf_svc::hal::i2c::{I2cConfig, I2cDriver};
use esp_idf_svc::hal::prelude::FromValueType;  // support 100_u32.kHz
use display_interface::DisplayError;


#[derive(Debug, Copy, Clone)]
pub struct DisplaySize72x40;
///此款屏幕与市面上常见的0.42寸的屏幕稍有差别，屏幕起点为12864的(30，14)
impl DisplaySize for DisplaySize72x40 {
    const WIDTH: u8 = 128;  // 使用128宽度的缓冲区
    const HEIGHT: u8 = 64;  // 使用64高度的缓冲区

    const OFFSETX: u8 = 58;  // 将此从28改为 28+30
    const OFFSETY: u8 = 14;  // 将此从0改为 0+14
    type Buffer = [u8; Self::WIDTH as usize * Self::HEIGHT as usize / 8];

    fn configure(&self, iface: &mut impl WriteOnlyDataCommand) -> Result<(), DisplayError> {
        Command::ComPinConfig(true, false).send(iface).expect("dis: ");
        Command::InternalIref(true, true).send(iface)
    }
}

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();
    esp_idf_sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    info!("Hello, world!");

    let peripherals = Peripherals::take().unwrap();
    let sysloop = EspSystemEventLoop::take().unwrap();
    let nvs = EspDefaultNvsPartition::take().unwrap();

    // display
    let i2c = peripherals.i2c0;
    let sda = peripherals.pins.gpio5;
    let scl = peripherals.pins.gpio6;

    let config = I2cConfig::new().baudrate(100_u32.kHz().into());
    let i2c = I2cDriver::new(i2c, sda, scl, &config).expect("i2c: ");

    // let interface = I2CDisplayInterface::new(i2c);
    let interface = I2CDisplayInterface::new_custom_address(i2c, 0x3C);

    let mut display = Ssd1306::new(interface, DisplaySize72x40, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();

    display.init().unwrap();

    display.clear_buffer();
    display.flush().unwrap();
    display.set_display_on(true).unwrap();

    let text_style = MonoTextStyleBuilder::new()
    .font(&FONT_4X6)
    .text_color(BinaryColor::On)
    .build();


    Text::with_baseline(
        "Hello, World!",
        Point::new(0, 0),
        text_style,
        Baseline::Top,
    )
    .draw(&mut display)
    .unwrap();

    Text::with_baseline(
        "Hello, World2!",
        Point::new(0, 10),
        text_style,
        Baseline::Top,
    )
    .draw(&mut display)
    .unwrap();

    display.flush().unwrap();

    println!("--------display---------");


}
