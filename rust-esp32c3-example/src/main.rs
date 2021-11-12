#![allow(unused_imports)]
#![allow(unused)]

use std::time::Duration;

use embedded_font::FontTextStyleBuilder;
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
use hal::delay::{Ets, FreeRtos};
use hal::gpio::InputPin;
use hal::gpio::OutputPin;
use hal::prelude::*;
use hal::{gpio, spi};
use rusttype::Font;
use st7735_lcd::Orientation;

fn main() {
    esp_idf_sys::link_patches();
    println!("hello world!");

    // std::thread::spawn(|| {
    //     let mut delay = FreeRtos {};
    //     loop {
    //         println!("hello world!");
    //         delay.delay_ms(1000 as u32);
    //     }
    //     // std::thread::sleep(Duration::from_secs(1));

    //     // println!("hello world!");
    //     // std::thread::sleep(Duration::from_secs(1));

    //     // println!("hello world!");
    //     // std::thread::sleep(Duration::from_secs(1));
    // });

    // Bind the log crate to the ESP Logging facilities
    // esp_idf_svc::log::EspLogger::initialize_default();

    // 获取 外设 对象
    let peripherals = Peripherals::take().unwrap();

    // 获取 引脚集合
    let pins = peripherals.pins;

    // 1.8寸 ST7735 LCD屏 所需引脚

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
    )
    .unwrap();

    // 创建一个 延时对象
    let mut delay = Delay::new();

    let mut display = st7735_lcd::ST7735::new(
        master,
        dc.into_output().unwrap(),
        rst.into_output().unwrap(),
        true,
        false,
        160,
        128,
    );

    display.init(&mut delay).unwrap();
    display.set_orientation(&Orientation::Landscape).unwrap();

    // // paint start position
    // display.set_offset(0, 25);

    let style = FontTextStyleBuilder::new(
        Font::try_from_bytes(include_bytes!("assets/Roboto-Regular.ttf")).unwrap(),
    )
    .font_size(16)
    .text_color(Rgb565::WHITE)
    .build();

    Text::new("Hello World!", Point::new(15, 30), style)
        .draw(&mut display)
        .unwrap();

    // let output_settings = OutputSettingsBuilder::new().scale(2).build();
    // Window::new("Fonts", &output_settings).show_static(&display);

    // draw ferris
    // let image_raw: ImageRawLE<Rgb565> = ImageRaw::new(include_bytes!("assets/ferris.raw"), 86);
    // let image: Image<_> = Image::new(&image_raw, Point::new(34, 8));

    // loop {
    //     display.clear(Rgb565::BLACK.into());
    //     image.draw(&mut display).unwrap();
    //     std::thread::sleep(Duration::from_millis(1000));

    //     display.clear(Rgb565::WHITE.into());
    //     image.draw(&mut display).unwrap();
    //     std::thread::sleep(Duration::from_millis(1000));
    //     println!("ok!");
    // }
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
