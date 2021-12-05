
    cargo generate --vcs none --git https://github.com/ivmarkov/esp-idf-template cmake

    cd <ProjectName>

    idf.py set-target esp32c3
    
    idf.py build

    idf.py -p /dev/ttyACM0 flash

    idf.py -p /dev/ttyACM0 monitor