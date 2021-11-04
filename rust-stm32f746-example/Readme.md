## rust stm32 学习笔记

参考： https://bacelarhenrique.me/2021/02/21/rust-and-stm32-a-quick-start-guide.html


### Cargo binary utils

        sudo apt install build-essential
        cargo install cargo-binutils
        rustup component add llvm-tools-preview

        rustup target add thumbv7em-none-eabihf

### Cargo flash

    sudo apt install libusb-1.0-0-dev

    cargo install cargo-flash

    cargo flash --chip stm32f746ng --release --log debug

## 工具

    cargo install probe-run， 可以直接使用 cargo run， 烧写并执行

    rtt-target， log打印

    embedded_graphics

    ssd1306, oled i2c驱动


    cargo size

    cargo readobj -- --file-headers
