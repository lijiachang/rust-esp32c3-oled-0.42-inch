use esp_idf_svc::hal::delay::FreeRtos;
use esp_idf_svc::hal::gpio::PinDriver;
use esp_idf_svc::hal::prelude::Peripherals;
use log::info;

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    info!("Hello, world!");

    let peripherals = Peripherals::take().unwrap();
    let mut led4 = PinDriver::output(peripherals.pins.gpio12).unwrap();
    let mut led5 = PinDriver::output(peripherals.pins.gpio13).unwrap();

    loop {
        info!("loop start...");
        led4.set_high().unwrap();
        led5.set_low().unwrap();

        FreeRtos::delay_ms(2000); // sleep 2s

        led4.set_low().unwrap();
        led5.set_high().unwrap();

        FreeRtos::delay_ms(2000); // sleep 2s

        info!("loop end.");
    }
}
