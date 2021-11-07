#![allow(unused_imports)]
#![allow(unused)]

use embedded_graphics::draw_target::DrawTarget;
use anyhow::Result;
use embedded_graphics::image::Image;
use embedded_graphics::Drawable;
use embedded_graphics::image::ImageRaw;
use embedded_graphics::image::ImageRawLE;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::Point;
use esp_idf_hal as hal;
use esp_idf_sys;
use hal::delay::Ets;
use hal::gpio::InputPin;
use hal::gpio::OutputPin;
use hal::prelude::*;
use hal::{spi, gpio};
use embedded_hal::blocking::delay::{DelayMs};
use st7735_lcd::Orientation;

fn main() -> Result<()> {
    esp_idf_sys::link_patches();
    println!("hello world!");


    // Bind the log crate to the ESP Logging facilities
    // esp_idf_svc::log::EspLogger::initialize_default();

    // 获取 外设 对象
    let peripherals = Peripherals::take().unwrap();

    // 获取 引脚集合
    let pins = peripherals.pins;

    // 1.8寸 ST7735 LCD屏 所需引脚
    let rst = pins.gpio5; // 复位
    let dc = pins.gpio9; // 数据/命令
    let backlight = pins.gpio8; // 背光
    
    let sclk = pins.gpio6;
    let sdo = pins.gpio7;
    let cs = pins.gpio10;

    // spi config
    let config = <spi::config::Config as Default>::default()
        .baudrate(26.MHz().into())
        .bit_order(spi::config::BitOrder::MSBFirst);

    // 创建 spi master 对象
    let master = spi::Master::<spi::SPI2, _, _, _, _>::new(peripherals.spi2, spi::Pins {
        sclk, sdo, cs: Some(cs), sdi: Option::<gpio::Gpio21<gpio::Unknown>>::None
    }, config)?;

    let mut disp = st7735_lcd::ST7735::new(master, dc.into_output()?, rst.into_output()?, true, false, 160, 128);

    let mut delay = Delay::new();

    disp.init(&mut delay).unwrap();
    disp.set_orientation(&Orientation::Landscape).unwrap();
    disp.clear(Rgb565::new(0x00, 0x00, 0x00));
    disp.set_offset(0, 25);

    // draw ferris
    let image_raw: ImageRawLE<Rgb565> = ImageRaw::new(include_bytes!("assets/ferris.raw"), 86);
    let image: Image<_> = Image::new(&image_raw, Point::new(0, 0));
    image.draw(&mut disp).unwrap();
    
    // loop { continue; }


    Ok(())
}


pub struct Delay {
    ets: Ets,
}

impl Delay {
    pub fn new() -> Self {
        Self {
            ets: Ets {}
        }
    }
}

impl DelayMs<u8> for Delay {
    fn delay_ms(&mut self, ms: u8) {
        self.ets.delay_ms(ms as u32);
    }
}
