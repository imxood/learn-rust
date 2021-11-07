// use esp32_hal::clock_control::{self, CPUSource, ClockControl};
// use esp32_hal::dport::Split;
// use esp32_hal::i2c;
// use esp32_hal::prelude::*;
// use esp32_hal::target::Peripherals;
// use esp32_hal::timer::Timer;
// use xtensa_lx::mutex::SpinLockMutex;

#[allow(unused_imports)]
use esp_idf_sys; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
// use esp32c3 as c3;

fn main() {
    println!("1,hello world!");
    // println!("2,hello world!");
    // println!("3,hello world!");
    // println!("4,hello world!");
    // println!("5,hello world!");
    // println!("6,hello world!");
    // println!("7,hello world!");
    // println!("8,hello world!");
    // std::thread::sleep(std::time::Duration::from_millis(100));
    // println!("9,hello world!");
    // let dp = Peripherals::take().unwrap();
    // let (mut dport, dport_clock_control) = dp.DPORT.split();

    // let mut clk_ctl = ClockControl::new(
    //     dp.RTCCNTL,
    //     dp.APB_CTRL,
    //     dport_clock_control,
    //     clock_control::XTAL_FREQUENCY_AUTO,
    // )
    // .unwrap();

    // clk_ctl
    //     .set_cpu_frequencies(
    //         CPUSource::PLL,
    //         80.MHz(),
    //         CPUSource::PLL,
    //         240.MHz(),
    //         CPUSource::PLL,
    //         80.MHz(),
    //     )
    //     .unwrap();

    // let (clkcntrl_config, mut watchdog) = clk_ctl.freeze().unwrap();

    // watchdog.disable();

    // // disable MST watchdogs
    // let (.., mut watchdog0) = Timer::new(dp.TIMG0, clkcntrl_config);
    // let (.., mut watchdog1) = Timer::new(dp.TIMG1, clkcntrl_config);
    // watchdog0.disable();
    // watchdog1.disable();

    // let pins = dp.GPIO.split();
    // let i2c0 = i2c::I2C::new(
    //     dp.I2C0,
    //     i2c::Pins {
    //         sda: pins.gpio4,
    //         scl: pins.gpio15,
    //     },
    //     400_000,
    //     &mut dport,
    // );
    // let i2c0 = SpinLockMutex::new(i2c0);

    // let s = 'a';
    // let mut ss = String::new();

    // std::thread::spawn(|| {
    //     println!("thread1!");
    //     std::thread::sleep(std::time::Duration::from_millis(1000));
    // });

    // loop {
    //     println!("thread0!");
    //     std::thread::sleep(std::time::Duration::from_millis(10));
    // }
}
