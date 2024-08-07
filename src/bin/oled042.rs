use display_interface::DisplayError;
use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Line, PrimitiveStyle},
    text::{Baseline, Text},
};
use esp_idf_svc::hal::i2c::{I2cConfig, I2cDriver};
use esp_idf_svc::hal::prelude::*;
use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};

pub mod consts {
    pub const PRIMARY_ADDRESS: u8 = 0x3C;
    pub const SECONDARY_ADDRESS: u8 = 0x3D;
}

pub mod cmds {
    pub const MEMORYMODE: u8 = 0x20; //  See datasheet
    pub const COLUMNADDR: u8 = 0x21; //  See datasheet
    pub const PAGEADDR: u8 = 0x22; //  See datasheet
    pub const SETCONTRAST: u8 = 0x81; //  See datasheet
    pub const CHARGEPUMP: u8 = 0x8D; //  See datasheet
    pub const SEGREMAP: u8 = 0xA0; //  See datasheet
    pub const DISPLAYALLON_RESUME: u8 = 0xA4; //  See datasheet
    pub const DISPLAYALLON: u8 = 0xA5; //  Not currently used
    pub const NORMALDISPLAY: u8 = 0xA6; //  See datasheet
    pub const INVERTDISPLAY: u8 = 0xA7; //  See datasheet
    pub const SETMULTIPLEX: u8 = 0xA8; //  See datasheet
    pub const DISPLAYOFF: u8 = 0xAE; //  See datasheet
    pub const DISPLAYON: u8 = 0xAF; //  See datasheet
    pub const COMSCANINC: u8 = 0xC0; //  Not currently used
    pub const COMSCANDEC: u8 = 0xC8; //  See datasheet
    pub const SETDISPLAYOFFSET: u8 = 0xD3; //  See datasheet
    pub const SETDISPLAYCLOCKDIV: u8 = 0xD5; //  See datasheet
    pub const SETPRECHARGE: u8 = 0xD9; //  See datasheet
    pub const SETCOMPINS: u8 = 0xDA; //  See datasheet
    pub const SETVCOMDETECT: u8 = 0xDB; //  See datasheet

    pub const SETLOWCOLUMN: u8 = 0x00; //  Not currently used
    pub const SETHIGHCOLUMN: u8 = 0x10; //  Not currently used
    pub const SETSTARTLINE: u8 = 0x40; //  See datasheet

    pub const RIGHT_HORIZONTAL_SCROLL: u8 = 0x26; //  Init rt scroll
    pub const LEFT_HORIZONTAL_SCROLL: u8 = 0x27; //  Init left scroll
    pub const VERTICAL_AND_RIGHT_HORIZONTAL_SCROLL: u8 = 0x29; //  Init diag scroll
    pub const VERTICAL_AND_LEFT_HORIZONTAL_SCROLL: u8 = 0x2A; //  Init diag scroll
    pub const DEACTIVATE_SCROLL: u8 = 0x2E; //  Stop scroll
    pub const ACTIVATE_SCROLL: u8 = 0x2F; //  Start scroll
    pub const SET_VERTICAL_SCROLL_AREA: u8 = 0xA3; //  Set scroll range
}

pub struct SSD1306<'a> {
    i2c: I2cDriver<'a>,
    addr: u8,
}

// SEGREMAP = 0
// SETCOMPINS = 0x02 ( sequential com pin config) not alternative
// column offset = 0
// width = 128
// height = 33
// pages = 5 (0 to 4)

pub const WIDTH: usize = 72;
pub const HEIGHT: usize = 40;

pub const OFFSETX: usize = 28;
pub const OFFSETY: usize = 24;

pub const PAGES: usize = HEIGHT / 8; // 8 rows per page

impl<'a> SSD1306<'a> {
    pub fn new(i2c: I2cDriver<'a>, addr: u8) -> Self {
        Self { i2c, addr }
    }

    pub fn init(&mut self) {
        use cmds::*;

        let init_seq1 = [
            DISPLAYOFF,         // 0xAE
            SETDISPLAYCLOCKDIV, // 0xD5
            0x80,               // the suggested ratio 0x80
            SETMULTIPLEX,       // 0xA8
            (HEIGHT as u8) - 1, // height
        ];

        self.send_commands(&init_seq1);

        const INIT2: &[u8] = &[
            SETDISPLAYOFFSET, // 0xD3
            0,                // no offset
            SETSTARTLINE | 0, // line #0
            CHARGEPUMP,       // 0x8D
        ];

        self.send_commands(INIT2);

        let external_vcc = false;
        if external_vcc {
            self.send_command(0x10);
        } else {
            self.send_command(0x14);
        }

        const INIT3: &[u8] = &[
            MEMORYMODE, // 0x20
            0x00,       // 0x0 act like ks0108
            SEGREMAP | 0x1,
            COMSCANDEC,
        ];

        self.send_commands(INIT3);

        self.send_commands(&[SETCOMPINS, 0x12]); // 128x64
        self.send_commands(&[SETCONTRAST, 0xCF]);

        self.send_command(SETPRECHARGE);
        if external_vcc {
            self.send_command(0x22);
        } else {
            self.send_command(0xF1);
        }

        const INIT5: &[u8] = &[
            SETVCOMDETECT, // 0xDB
            0x40,
            DISPLAYALLON_RESUME, // 0xA4
            NORMALDISPLAY,       // 0xA6
            DEACTIVATE_SCROLL,
            DISPLAYON, // Main screen turn on
        ];
        self.send_commands(INIT5);
    }

    #[inline]
    fn send_command(&mut self, c: u8) {
        self.i2c.write(self.addr, &[0x00, c], 5u32).unwrap();
    }
    #[inline]
    fn send_commands(&mut self, cmds: &[u8]) {
        for &c in cmds {
            self.send_command(c);
        }
    }
    #[inline]
    fn send_data(&mut self, d: u8) {
        self.i2c.write(self.addr, &[0x40, d], 5u32).unwrap();
    }

    pub fn display_fb(&mut self, fb: &[u8]) {
        self.send_commands(&[cmds::PAGEADDR, 0, 0xff]);
        self.send_commands(&[
            cmds::COLUMNADDR,
            (OFFSETX as u8),
            (OFFSETX as u8 + WIDTH as u8 - 1),
        ]); // width

        for page in 0..PAGES {
            // lsb
            for i in 0..WIDTH {
                self.send_data(fb[(page as usize) * WIDTH + i]);
                //self.send_data(0xaa);
            }
        }
    }
}

/// A framebuffer for use with embedded-graphics
/// Page-based addressing
pub struct Frambebuffer([u8; WIDTH * PAGES]);

impl Frambebuffer {
    pub fn new() -> Self {
        Self([0; WIDTH * PAGES])
    }

    pub fn data(&mut self) -> &mut [u8] {
        &mut self.0
    }

    pub fn set_pixel(&mut self, x: i16, y: i16, color: bool) {
        if x >= (WIDTH as i16) || y >= (HEIGHT as i16) || x < 0 || y < 0 {
            return;
        }

        let x = x as u8;
        let y = y as u8;
        let page = y / 8;
        let bit = y % 8;
        let mask = 1 << bit;
        let idx = (page as usize) * WIDTH + x as usize;
        if color {
            self.0[idx] |= mask;
        } else {
            self.0[idx] &= !mask;
        }
    }
}

impl OriginDimensions for Frambebuffer {
    fn size(&self) -> Size {
        Size::new(WIDTH as _, HEIGHT as _)
    }
}

impl DrawTarget for Frambebuffer {
    type Color = BinaryColor;

    type Error = core::convert::Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for Pixel(point, color) in pixels.into_iter() {
            self.set_pixel(point.x as i16, point.y as i16, color.is_on());
        }
        Ok(())
    }
}

pub struct Rand;

impl Rand {
    pub fn next(&self) -> u8 {
        static mut SEED: u8 = 0xaa;

        // use LFSR
        unsafe {
            let mut lfsr = SEED;
            let bit = (lfsr >> 0) ^ (lfsr >> 2) ^ (lfsr >> 3) ^ (lfsr >> 5);
            lfsr = (lfsr >> 1) | (bit << 7);
            SEED = lfsr;
            lfsr
        }
    }
}

pub struct World {
    points: [Point; 10],
}

impl World {
    pub fn new() -> Self {
        World {
            points: [
                Point::new(3, 10),
                Point::new(10, 1),
                Point::new(20, 5),
                Point::new(30, 2),
                Point::new(95, 10),
                Point::new(100, 2),
                Point::new(110, 10),
                Point::new(120, 20),
                Point::new(97, 30),
                Point::new(89, 32),
            ],
        }
    }

    pub fn tick(&mut self) {
        for p in self.points.iter_mut() {
            p.x -= 1;
            p.y += 1;
            if p.x >= (128 + 32) || p.y >= (64 + 10) || p.y == 0 || p.x == 0 {
                p.x = (Rand.next() % (128 + 32)) as _;
                p.y = 0;
            }
        }
    }

    pub fn draw(&self, fb: &mut Frambebuffer) {
        for p in self.points.iter() {
            let x = p.x as i16;
            let y = p.y as i16;
            fb.set_pixel(x - 1, y, true);
            fb.set_pixel(x, y - 1, true);
            fb.set_pixel(x + 1, y, true);
            fb.set_pixel(x, y + 1, true);

            for i in 2..10 {
                // draw tail
                fb.set_pixel(x + i, y - i, true);
            }
        }
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
    
    let mut screen = SSD1306::new(i2c, 0x3C);

    screen.init();

    let mut fb = Frambebuffer::new();
    fb.clear(BinaryColor::Off).unwrap();
    screen.display_fb(fb.data());

    fb.clear(BinaryColor::Off).unwrap();

    let i = 2;

    let rect = fb.bounding_box();
    rect.into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
        .draw(&mut fb);

    Line::new(Point::new(0, i), Point::new(69, i))
        .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
        .draw(&mut fb)
        .unwrap();

    screen.display_fb(fb.data());

    Ok(())
}
