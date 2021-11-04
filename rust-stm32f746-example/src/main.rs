//! Blinks an LED

#![deny(unsafe_code)]
// #![deny(warnings)]
#![no_main]
#![no_std]

extern crate panic_halt;
use cortex_m_rt::entry;
use embedded_graphics::{
    image::{Image, ImageRaw},
    pixelcolor::BinaryColor,
    prelude::*,
};
use hal::{
    i2c::{self, BlockingI2c},
    pac,
    prelude::*,
};
use rtt_target::{rprintln, rtt_init_print};
use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};
use stm32f7xx_hal as hal;

#[entry]
fn main() -> ! {
    rtt_init_print!();

    rprintln!("1");

    let dp = pac::Peripherals::take().unwrap();

    let mut rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.sysclk(216_000_000.Hz()).freeze();

    let gpioi = dp.GPIOI.split();
    let mut led = gpioi.pi1.into_push_pull_output();

    // dp.FLASH.constrain();
    let gpiob = dp.GPIOB.split();

    let scl = gpiob.pb8.into_alternate_af4();
    let sda = gpiob.pb9.into_alternate_af4();

    let i2c1 = BlockingI2c::i2c1(
        dp.I2C1,
        (scl, sda),
        i2c::Mode::Fast {
            frequency: 400_000.Hz(),
        },
        clocks,
        &mut rcc.apb1,
        5000000,
    );

    let interface = I2CDisplayInterface::new(i2c1);

    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();

    rprintln!("2");
    display.init().unwrap();

    rprintln!("3");

    let raw: ImageRaw<BinaryColor> = ImageRaw::new(include_bytes!("./rust.raw"), 64);

    let im = Image::new(&raw, Point::new(32, 0));

    im.draw(&mut display).unwrap();

    display.flush().unwrap();

    loop {
        for _ in 0..5_000 {
            led.set_high().unwrap();
        }
        for _ in 0..5_000 {
            led.set_low().unwrap();
        }
        rprintln!("Hello, world!");
    }
}
