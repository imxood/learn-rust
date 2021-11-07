#![allow(unused_imports)]
#![allow(unused)]

use std::time::Duration;

use anyhow::Result;
use embedded_graphics::{
    draw_target::DrawTarget,
    image::{Image, ImageRaw, ImageRawLE},
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    pixelcolor::{BinaryColor, Rgb565},
    prelude::Primitive,
    prelude::*,
    prelude::{Dimensions, Point},
    primitives::{
        Circle, PrimitiveStyle, PrimitiveStyleBuilder, Rectangle, StrokeAlignment, Triangle,
    },
    text::{Alignment, Text},
    Drawable,
};
use embedded_hal::blocking::delay::DelayMs;
use embedded_svc::anyerror::AnyError;
use esp_idf_hal as hal;
use esp_idf_sys;
use hal::delay::Ets;
use hal::gpio::InputPin;
use hal::gpio::OutputPin;
use hal::prelude::*;
use hal::{gpio, spi};
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

    // // 1.8寸 ST7735 LCD屏 所需引脚
    // let rst = pins.gpio5; // 复位
    // let dc = pins.gpio9; // 数据/命令

    // let sclk = pins.gpio6;
    // let sdo = pins.gpio7;
    // let cs = pins.gpio10;

    // let backlight = pins.gpio3;
    let rst = pins.gpio9;
    let dc = pins.gpio8;

    let sclk = pins.gpio4;
    let sdo = pins.gpio6;
    let cs = pins.gpio10;

    // spi config
    let config = <spi::config::Config as Default>::default()
        .baudrate(26.MHz().into())
        .bit_order(spi::config::BitOrder::MSBFirst);

    // 创建 spi master 对象
    let master = spi::Master::<spi::SPI2, _, _, _, _>::new(
        peripherals.spi2,
        spi::Pins {
            sclk,
            sdo,
            cs: Some(cs),
            sdi: Option::<gpio::Gpio21<gpio::Unknown>>::None,
        },
        config,
    )?;

    // 创建一个 延时对象
    let mut delay = Delay::new();

    let mut display = st7735_lcd::ST7735::new(
        master,
        dc.into_output()?,
        rst.into_output()?,
        true,
        false,
        160,
        128,
    );

    display.init(&mut delay).unwrap();
    display.set_orientation(&Orientation::Landscape).unwrap();
    // display.clear(Rgb565::BLACK.into());
    display.set_offset(0, 25);

    // draw ferris
    let image_raw: ImageRawLE<Rgb565> = ImageRaw::new(include_bytes!("assets/ferris.raw"), 86);
    let image: Image<_> = Image::new(&image_raw, Point::new(34, 8));

    loop {
        display.clear(Rgb565::BLACK.into());
        image.draw(&mut display).unwrap();
        std::thread::sleep(Duration::from_millis(1000));

        display.clear(Rgb565::WHITE.into());
        image.draw(&mut display).unwrap();
        std::thread::sleep(Duration::from_millis(1000));
        println!("ok!");
    }

    Ok(())
}

pub struct Delay {
    ets: Ets,
}

impl Delay {
    pub fn new() -> Self {
        Self { ets: Ets {} }
    }
}

impl DelayMs<u8> for Delay {
    fn delay_ms(&mut self, ms: u8) {
        self.ets.delay_ms(ms as u32);
    }
}
