use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Baseline, Text},
};
use esp_idf_svc::hal::i2c::{I2cConfig, I2cDriver};
use esp_idf_svc::hal::prelude::*;

pub const WIDTH: usize = 128;
pub const HEIGHT: usize = 64;
pub const PAGES: usize = HEIGHT / 8;

pub struct SSD1306<'a> {
    i2c: I2cDriver<'a>,
    addr: u8,
    buffer: [u8; WIDTH * PAGES],
    x_offset: u8,
    y_offset: u8,
}

///此款屏幕与市面上常见的0.42寸的屏幕稍有差别，屏幕起点为12864的(30，14)
impl<'a> SSD1306<'a> {
    pub fn new(i2c: I2cDriver<'a>, addr: u8) -> Self {
        Self {
            i2c,
            addr,
            buffer: [0; WIDTH * PAGES],
            x_offset: 30,
            y_offset: 7,
        }
    }

    pub fn init(&mut self) {
        let init_commands = [
            0xAE,  //# display off
            0xD5,  //# set osc division
            0xF0,
            0xA8,  //# multiplex ratio
            0x27,  //# duty = 1/40
            0xD3,  //# set display offset
            0x0c,  //# commonly no shift, but this must shift 0x0c
            0x40,  //# Set Display Start Line
            0x8d,  //# set charge pump enable
            0x14,
            0x20,
            0x02,
            //#0x95,
            0xa1,
            0xc8,
            0xDA,  //# set COM pins
            0x12,
            0xAD,  //# Internal IREF Setting
            0x30,
            0x81,  //# contract control
            0xFF,  //# 128
            0xD9,  //# set pre-charge period
            0x22,
            0xDB,  //# set vcomh
            0x20,
            0xA4,  //# Set Entire Display On/Off
            0xA6,  //# normal / reverse
            //#0x0C,  # set lower column address
            //#0x11,  # set higher column address
            0xAF   //# display ON
        ];
        // let init_commands = [
        //     0xAE, 0xD5, 0xF0, 0xA8, 0x3F, 0xD3, 0x0C, 0x40, 0x8D, 0x14,
        //     0x20, 0x02, 0xA1, 0xC8, 0xDA, 0x12, 0xAD, 0x30, 0x81, 0xFF,
        //     0xD9, 0x22, 0xDB, 0x20, 0xA4, 0xA6, 0xAF
        // ];
        //         let init_commands = [
        //     0xAE,                   // 关闭显示
        //     0xD5, 0xF0,             // 设置显示时钟分频比/振荡器频率
        //     0xA8, 0x3F,             // 设置多路复用比
        //     0xD3, self.y_offset,    // 设置显示偏移 - 使用 self.y_offset
        //     0x40,                   // 设置显示开始行
        //     0x8D, 0x14,             // 启用电荷泵调节器
        //     0x20, 0x02,             // 设置内存寻址模式
        //     0xA1,                   // 设置段重映射
        //     0xC8,                   // 设置 COM 输出扫描方向
        //     0xDA, 0x12,             // 设置 COM 引脚硬件配置
        //     0xAD, 0x30,             // 设置内部 IREF
        //     0x81, 0xFF,             // 设置对比度控制
        //     0xD9, 0x22,             // 设置预充电周期
        //     0xDB, 0x20,             // 设置 VCOMH 取消选择级别
        //     0xA4,                   // 设置正常显示（非全亮）
        //     0xA6,                   // 设置正常显示（非反向）
        //     0xAF                    // 开启显示
        // ];

        for &cmd in &init_commands {
            self.send_command(cmd);
        }
    }

    fn send_command(&mut self, cmd: u8) {
        self.i2c.write(self.addr, &[0x00, cmd], 5u32).unwrap();
    }

    fn send_data(&mut self, data: u8) {
        self.i2c.write(self.addr, &[0x40, data], 5u32).unwrap();
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: bool) {
        if x >= WIDTH || y >= HEIGHT {
            return;
        }
        let page = y / 8;
        let bit = y % 8;
        let idx = page * WIDTH + x;
        if color {
            self.buffer[idx] |= 1 << bit;
        } else {
            self.buffer[idx] &= !(1 << bit);
        }
    }

    pub fn clear(&mut self) {
        self.buffer.fill(0);
    }

    pub fn show(&mut self) {
        for page in 0..8 {
            self.set_cursor(0, page as u8);
            for col in 0..WIDTH {
                self.send_data(self.buffer[WIDTH * page + col]);
            }
        }
    }

    fn set_cursor(&mut self, x: u8, y: u8) {
        self.send_command(0xB0 + y);
        self.send_command((((x + self.x_offset) & 0xF0) >> 4) | 0x10);
        self.send_command((x + self.x_offset) & 0x0F);
    }

    pub fn text(&mut self, text: &str, x: i32, y: i32) {
        let y = y + self.y_offset as i32;
        let style = MonoTextStyleBuilder::new()
            .font(&FONT_6X10)
            .text_color(BinaryColor::On)
            .build();

        Text::new(text, Point::new(x, y), style)
            .draw(self)
            .unwrap();
    }
}

impl<'a> DrawTarget for SSD1306<'a> {
    type Color = BinaryColor;
    type Error = core::convert::Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item=Pixel<Self::Color>>,
    {
        for Pixel(coord, color) in pixels.into_iter() {
            self.set_pixel(coord.x as usize, coord.y as usize, color.is_on());
        }
        Ok(())
    }
}

impl<'a> OriginDimensions for SSD1306<'a> {
    fn size(&self) -> Size {
        Size::new(WIDTH as u32, HEIGHT as u32)
    }
}

fn main() -> anyhow::Result<()> {
    esp_idf_svc::sys::link_patches();

    let peripherals = Peripherals::take().unwrap();
    let i2c = peripherals.i2c0;
    let sda = peripherals.pins.gpio5;
    let scl = peripherals.pins.gpio6;

    let config = I2cConfig::new().baudrate(400.kHz().into());
    let i2c: I2cDriver<'static> = I2cDriver::new(i2c, sda, scl, &config)?;

    let mut display = SSD1306::new(i2c, 0x3C);

    display.init();
    display.clear();

    display.text("Hello, Rust", 0, 0);
    display.text("author:", 0, 16);
    display.text("lijiachang", 0, 28);

    display.show();

    Ok(())
}