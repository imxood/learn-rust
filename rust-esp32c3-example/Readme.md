## 熟悉一遍 esp-idf 的 c 开发环境

    参考： https://docs.espressif.com/projects/esp-idf/zh_CN/latest/esp32c3/get-started/index.html

    mkdir -p ~/.espressif
    git clone --recursive https://github.com/espressif/esp-idf.git ~/.espressif/esp-idf-master

    cd ~/.espressif/esp-idf-master
    ./install.sh esp32c3

    添加环境变量：
        alias get-idf=". ./export.sh"
        get-idf

    测试：
        cp -r $IDF_PATH/examples/get-started/hello_world .

        cd hello_world

        idf.py set-target esp32c3
        idf.py menuconfig
        idf.py build
        idf.py -p /dev/ttyACM0 flash -b 921600
        idf.py monitor

            note: Ctrl+] 退出 monitor 程序

## rust 开发

    cargo install cargo-generate
    cargo install ldproxy
    cargo install cargo-espflash
    cargo install cargo-espmonitor

    生成项目：
        cargo generate --vcs none --git https://github.com/ivmarkov/esp-idf-template cargo

    进入项目目录后：

        修改.cargo/config.toml， 添加:
            [env]
            ESP_IDF_GLOBAL_INSTALL = { value = "y" }
        
        编译：
            cargo build
        
        烧写：
            cargo espflash /dev/ttyACM0 --speed 921600
        
        串口：
            cargo espmonitor --chip esp32c3 /dev/ttyACM0


    sdkconfig文件是esp编译工具产生的文件, 一起生成的还有sdkconfig.h, 是配置开关的头文件
    sdkconfig.defaults文件是 如果没有sdkconfig文件, 那么就根据这个默认配置文件生成 sdkconfig


    查看所有的段信息：
        
        riscv32-esp-elf-size -A /tmp/arduino-sketch-FEBE44FEEF24853163ABB2D9E833C415/HTTPClientEnterprise.ino.elf


    烧写:

        python "/home/maxu/.arduino15/packages/esp32/tools/esptool_py/3.1.0/esptool.py" --chip esp32c3 --port "/dev/ttyACM0" --baud 921600  --before default_reset --after hard_reset write_flash -z --flash_mode dio --flash_freq 80m --flash_size 4MB 0xe000 "/home/maxu/.arduino15/packages/esp32/hardware/esp32/2.0.1-RC1/tools/partitions/boot_app0.bin" 0x0 "/tmp/arduino-sketch-FEBE44FEEF24853163ABB2D9E833C415/HTTPClientEnterprise.ino.bootloader.bin" 0x10000 "/tmp/arduino-sketch-FEBE44FEEF24853163ABB2D9E833C415/HTTPClientEnterprise.ino.bin" 0x8000 "/tmp/arduino-sketch-FEBE44FEEF24853163ABB2D9E833C415/HTTPClientEnterprise.ino.partitions.bin"


    工具

        rustup component add llvm-tools-preview
        cargo install cargo-binutils

        查看段信息：
            cargo size -- -A

        //把elf转为bin
        //    cargo objcopy --release -- -O binary app.bin

        把elf转为bin
            cargo espflash save-image app.bin


        python "/home/maxu/.arduino15/packages/esp32/tools/esptool_py/3.1.0/esptool.py" --chip esp32c3 elf2image -o app.bin target/riscv32imc-esp-espidf/debug/rust-esp32c3-example

        python "/home/maxu/.arduino15/packages/esp32/tools/esptool_py/3.1.0/esptool.py" --chip esp32c3 --port "/dev/ttyACM0" --baud 921600  --before default_reset --after hard_reset write_flash -z --flash_mode dio --flash_freq 80m --flash_size 4MB 0x10000 app.bin



        生成 外设访问
            cargo install svd2rust
            cargo install form

            cargo add esp32c3