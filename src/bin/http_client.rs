use std::net::Ipv4Addr;
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

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    info!("Hello, world!");

    let peripherals = Peripherals::take().unwrap();
    let sysloop = EspSystemEventLoop::take().unwrap();
    let nvs = EspDefaultNvsPartition::take().unwrap();

    let mut wifi = BlockingWifi::wrap(
        EspWifi::new(peripherals.modem, sysloop.clone(), Some(nvs)).unwrap(),
        sysloop,
    ).unwrap();

    info!("配置WiFi");
    wifi.set_configuration(
        &Configuration::Client(ClientConfiguration {
            ssid: "HIWIFI".try_into().unwrap(),  // todo 改为环境变量
            bssid: None,
            auth_method: Default::default(),
            password: "lichang.".try_into().unwrap(),
            channel: None,
            scan_method: Default::default(),
            pmf_cfg: Default::default(),
        },
        )
    ).expect("set_configuration: panic");

    info!("启动WiFi");
    wifi.start().unwrap();

    info!("连接WiFi");
    wifi.connect().unwrap();

    info!("等待底层网络接口启动");
    wifi.wait_netif_up().unwrap();

    info!(
        "获取到IP地址为:{:?}",
        wifi.wifi().sta_netif().get_ip_info()
    );


    let mut clinet = Client::wrap(EspHttpConnection::new(&Default::default()).unwrap());

    // GET
    let url = "http://httpbin.org/get";
    let mut resp = clinet.get(url).unwrap().submit().unwrap();
    info!("响应状态：{}", resp.status());

    let (_headers, mut body) = resp.split();
    let mut buf = [0_u8; 1024];
    let br = try_read_full(&mut body, &mut buf).unwrap();
    let body = std::str::from_utf8(&buf[0..br]).unwrap();
    info!("响应内容：{body}");

    // POST
    let url = "http://httpbin.org/post";
    let mut resp = clinet.post(url, &[]).unwrap().submit().unwrap();
    info!("响应状态：{}", resp.status());

    let (_headers, mut body) = resp.split();
    let mut buf = [0_u8; 1024];
    let br = try_read_full(&mut body, &mut buf).unwrap();
    let body = std::str::from_utf8(&buf[0..br]).unwrap();
    info!("响应内容：{body}");

}
