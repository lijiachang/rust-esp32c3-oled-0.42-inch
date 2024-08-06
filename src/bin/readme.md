# build
cargo b --bin wifi --release
cargo b --bin http_client --release
cargo b --bin https_client --release

# flash to esp32-c3  and monitor
espflash flash -p /dev/ttyACM0  target/riscv32imc-esp-espidf/release/wifi --monitor
espflash flash -p /dev/ttyACM0  target/riscv32imc-esp-espidf/release/http_client --monitor
espflash flash -p /dev/ttyACM0  target/riscv32imc-esp-espidf/release/https_client --monitor


```

I (30) boot: ESP-IDF v5.1.2-342-gbcf1645e44 2nd stage bootloader
I (30) boot: compile time Dec 12 2023 10:50:58
I (31) boot: chip revision: v0.4
I (34) boot.esp32c3: SPI Speed      : 40MHz
I (39) boot.esp32c3: SPI Mode       : DIO
I (44) boot.esp32c3: SPI Flash Size : 4MB
I (49) boot: Enabling RNG early entropy source...
I (54) boot: Partition Table:
I (58) boot: ## Label            Usage          Type ST Offset   Length
I (65) boot:  0 nvs              WiFi data        01 02 00009000 00006000
I (72) boot:  1 phy_init         RF data          01 01 0000f000 00001000
I (80) boot:  2 factory          factory app      00 00 00010000 003f0000
I (87) boot: End of partition table
I (92) esp_image: segment 0: paddr=00010020 vaddr=3c0c0020 size=28558h (165208) map
I (137) esp_image: segment 1: paddr=00038580 vaddr=3fc90600 size=029c4h ( 10692) load
I (140) esp_image: segment 2: paddr=0003af4c vaddr=40380000 size=050cch ( 20684) load
I (148) esp_image: segment 3: paddr=00040020 vaddr=42000020 size=b925ch (758364) map
I (319) esp_image: segment 4: paddr=000f9284 vaddr=403850cc size=0b4d8h ( 46296) load
I (336) boot: Loaded app from partition at offset 0x10000
I (336) boot: Disabling RNG early entropy source...
I (347) cpu_start: Unicore app
I (348) cpu_start: Pro cpu up.
I (356) cpu_start: Pro cpu start user code
I (356) cpu_start: cpu freq: 160000000 Hz
I (357) cpu_start: Application information:
I (359) cpu_start: Project name:     libespidf
I (365) cpu_start: App version:      1
I (369) cpu_start: Compile time:     Jul 14 2024 21:23:56
I (375) cpu_start: ELF file SHA256:  0000000000000000...
I (381) cpu_start: ESP-IDF:          v5.1.4
I (386) cpu_start: Min chip rev:     v0.3
I (391) cpu_start: Max chip rev:     v1.99 
I (396) cpu_start: Chip rev:         v0.4
I (400) heap_init: Initializing. RAM available for dynamic allocation:
I (408) heap_init: At 3FC97100 len 00028F00 (163 KiB): DRAM
I (414) heap_init: At 3FCC0000 len 0001C710 (113 KiB): DRAM/RETENTION
I (421) heap_init: At 3FCDC710 len 00002950 (10 KiB): DRAM/RETENTION/STACK
I (428) heap_init: At 50000010 len 00001FD8 (7 KiB): RTCRAM
I (436) spi_flash: detected chip: generic
I (439) spi_flash: flash io: dio
W (444) timer_group: legacy driver is deprecated, please migrate to `driver/gptimer.h`
I (452) sleep: Configure to isolate all GPIO pins in sleep state
I (459) sleep: Enable automatic switching of GPIO sleep configuration
I (466) app_start: Starting scheduler on CPU0
I (471) main_task: Started on CPU0
I (471) main_task: Calling app_main()
I (471) wifi: Hello, world!
I (491) pp: pp rom version: 9387209
I (491) net80211: net80211 rom version: 9387209
I (501) wifi:wifi driver task: 3fc9faec, prio:23, stack:6656, core=0
I (501) wifi:wifi firmware version: 3ce09e5
I (501) wifi:wifi certification version: v7.0
I (501) wifi:config NVS flash: enabled
I (501) wifi:config nano formating: disabled
I (511) wifi:Init data frame dynamic rx buffer num: 32
I (511) wifi:Init static rx mgmt buffer num: 10
I (521) wifi:Init management short buffer num: 32
I (521) wifi:Init dynamic tx buffer num: 32
I (531) wifi:Init static tx FG buffer num: 2
I (531) wifi:Init static rx buffer size: 1600
I (531) wifi:Init static rx buffer num: 10
I (541) wifi:Init dynamic rx buffer num: 32
I (541) wifi_init: rx ba win: 6
I (541) wifi_init: tcpip mbox: 32
I (551) wifi_init: udp mbox: 6
I (551) wifi_init: tcp mbox: 6
I (561) wifi_init: tcp tx win: 5760
I (561) wifi_init: tcp rx win: 5760
I (561) wifi_init: tcp mss: 1440
I (571) wifi_init: WiFi IRAM OP enabled
I (571) wifi_init: WiFi RX IRAM OP enabled
I (581) wifi: 配置WiFi
I (581) wifi: 启动WiFi
I (581) phy_init: phy_version 1170,f4aea9b,Apr 30 2024,10:49:24
I (631) wifi:mode : sta (54:32:04:73:8b:44)
I (631) wifi:enable tsf
I (631) wifi: 连接WiFi
I (3051) wifi:new:<5,0>, old:<1,0>, ap:<255,255>, sta:<5,0>, prof:1
I (3051) wifi:state: init -> auth (b0)
I (3081) wifi:state: auth -> assoc (0)
I (3141) wifi:state: assoc -> run (10)
I (3151) wifi:connected with HIWIFI, aid = 13, channel 5, BW20, bssid = 64:64:4a:9a:2b:c5
I (3151) wifi:security: WPA2-PSK, phy: bgn, rssi: -21
I (3161) wifi:pm start, type: 1

I (3161) wifi:dp: 1, bi: 102400, li: 3, scale listen interval from 307200 us to 307200 us
I (3171) wifi:set rx beacon pti, rx_bcn_pti: 0, bcn_timeout: 25000, mt_pti: 0, mt_time: 10000
I (3181) wifi: 等待底层网络接口启动
I (3191) wifi:AP's beacon interval = 102400 us, DTIM period = 1
I (3201) wifi:<ba-add>idx:0 (ifx:0, 64:64:4a:9a:2b:c5), tid:6, ssn:2, winSize:64
I (4681) esp_netif_handlers: sta ip: 192.168.31.93, mask: 255.255.255.0, gw: 192.168.31.1
I (4681) wifi: 获取到IP地址为:Ok(IpInfo { ip: 192.168.31.93, subnet: Subnet { gateway: 192.168.31.1, mask: Mask(24) }, dns: Some(192.168.31.1), secondary_dns: Some(0.0.0.0) })
I (4691) wifi: 开始ping...
I (4701) esp_idf_svc::ping: About to run a summary ping 192.168.31.254 with configuration Configuration { count: 5, interval: 1s, timeout: 1s, data_size: 56, tos: 0 }
I (4711) esp_idf_svc::ping: Ping session established, got handle 0x3fca8b40
I (4721) esp_idf_svc::ping: Ping session started
I (4721) esp_idf_svc::ping: Waiting for the ping session to complete
I (4731) wifi:<ba-add>idx:1 (ifx:0, 64:64:4a:9a:2b:c5), tid:0, ssn:0, winSize:64
I (4741) esp_idf_svc::ping: Ping success callback invoked
I (4741) esp_idf_svc::ping: From 254.31.168.192 icmp_seq=1 ttl=64 time=18ms bytes=56
I (5861) esp_idf_svc::ping: Ping success callback invoked
I (5861) esp_idf_svc::ping: From 254.31.168.192 icmp_seq=2 ttl=64 time=141ms bytes=56
I (6721) esp_idf_svc::ping: Ping success callback invoked
I (6721) esp_idf_svc::ping: From 254.31.168.192 icmp_seq=3 ttl=64 time=4ms bytes=56
I (7721) esp_idf_svc::ping: Ping success callback invoked
I (7721) esp_idf_svc::ping: From 254.31.168.192 icmp_seq=4 ttl=64 time=3ms bytes=56
I (8731) esp_idf_svc::ping: Ping success callback invoked
I (8731) esp_idf_svc::ping: From 254.31.168.192 icmp_seq=5 ttl=64 time=8ms bytes=56
I (9721) esp_idf_svc::ping: Ping end callback invoked
I (9721) esp_idf_svc::ping: 5 packets transmitted, 5 received, time 174ms
I (9721) esp_idf_svc::ping: Ping session stopped
I (9721) esp_idf_svc::ping: Ping session 0x3fca8b40 removed
I (9731) wifi: ping 192.168.31.254
I (10731) esp_idf_svc::ping: About to run a summary ping 192.168.31.254 with configuration Configuration { count: 5, interval: 1s, timeout: 1s, data_size: 56, tos: 0 }
I (10731) esp_idf_svc::ping: Ping session established, got handle 0x3fca8b40
I (10741) esp_idf_svc::ping: Ping session started
I (10741) esp_idf_svc::ping: Ping success callback invoked
I (10751) esp_idf_svc::ping: From 254.31.168.192 icmp_seq=1 ttl=64 time=2ms bytes=56
I (10761) esp_idf_svc::ping: Waiting for the ping session to complete
I (11741) esp_idf_svc::ping: Ping success callback invoked
I (11741) esp_idf_svc::ping: From 254.31.168.192 icmp_seq=2 ttl=64 time=3ms bytes=56
I (12741) esp_idf_svc::ping: Ping success callback invoked
I (12741) esp_idf_svc::ping: From 254.31.168.192 icmp_seq=3 ttl=64 time=2ms bytes=56
I (13751) esp_idf_svc::ping: Ping success callback invoked
I (13751) esp_idf_svc::ping: From 254.31.168.192 icmp_seq=4 ttl=64 time=11ms bytes=56
I (14741) esp_idf_svc::ping: Ping success callback invoked
I (14741) esp_idf_svc::ping: From 254.31.168.192 icmp_seq=5 ttl=64 time=6ms bytes=56
I (15741) esp_idf_svc::ping: Ping end callback invoked
I (15741) esp_idf_svc::ping: 5 packets transmitted, 5 received, time 24ms
I (15741) esp_idf_svc::ping: Ping session stopped
I (15741) esp_idf_svc::ping: Ping session 0x3fca8b40 removed
I (15751) wifi: ping 192.168.31.254
I (16751) esp_idf_svc::ping: About to run a summary ping 192.168.31.254 with configuration Configuration { count: 5, interval: 1s, timeout: 1s, data_size: 56, tos: 0 }
I (16751) esp_idf_svc::ping: Ping session established, got handle 0x3fca8b40
I (16761) esp_idf_svc::ping: Ping session started
I (16761) esp_idf_svc::ping: Ping success callback invoked
I (16771) esp_idf_svc::ping: From 254.31.168.192 icmp_seq=1 ttl=64 time=2ms bytes=56
I (16781) esp_idf_svc::ping: Waiting for the ping session to complete
I (17761) esp_idf_svc::ping: Ping success callback invoked
I (17761) esp_idf_svc::ping: From 254.31.168.192 icmp_seq=2 ttl=64 time=2ms bytes=56
I (18771) esp_idf_svc::ping: Ping success callback invoked
I (18771) esp_idf_svc::ping: From 254.31.168.192 icmp_seq=3 ttl=64 time=11ms bytes=56
I (19771) esp_idf_svc::ping: Ping success callback invoked
I (19771) esp_idf_svc::ping: From 254.31.168.192 icmp_seq=4 ttl=64 time=8ms bytes=56
I (20761) esp_idf_svc::ping: Ping success callback invoked

```


```
I (5810) http_client: 响应状态：200
I (5810) http_client: 响应内容：{
  "args": {}, 
  "headers": {
    "Content-Length": "0", 
    "Host": "httpbin.org", 
    "User-Agent": "ESP32 HTTP Client/1.0", 
    "X-Amzn-Trace-Id": "Root=1-669bbf0c-45d84dbf7d877aa00ae8ebb4"
  }, 
  "origin": "91.103.120.186", 
  "url": "http://httpbin.org/get"
}

I (7350) http_client: 响应状态：200
I (7350) http_client: 响应内容：{
  "args": {}, 
  "data": "", 
  "files": {}, 
  "form": {}, 
  "headers": {
    "Host": "httpbin.org", 
    "Transfer-Encoding": "chunked", 
    "User-Agent": "ESP32 HTTP Client/1.0", 
    "X-Amzn-Trace-Id": "Root=1-669bbf0e-6c34910a1cfff02907c78df5"
  }, 
  "json": null, 
  "origin": "91.103.120.186", 
  "url": "http://httpbin.org/post"
}


```


```text
I (4985) esp-x509-crt-bundle: Certificate validated
I (5635) https_client: 响应状态：200
I (5635) https_client: 响应内容：{"symbol":"BTCUSDT","price":"66616.02000000"}

```