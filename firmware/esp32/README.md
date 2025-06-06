# Table LEAD ESP32 Firmware

Table LEAD ESP32 Firmware
1. Установить ESP-IDF
```
git clone --recursive https://github.com/espressif/esp-idf.git
cd esp-idf
./install.sh
. ./export.sh
```
2. Установка rust toolchain для ESP32
```
rustup target add riscv32imc-unknown-none-elf
```
3. Клонировать и подготовить проект
```
git clone https://github.com/esp-rs/esp-idf-template.git my-esp-project
cd my-esp-project
```

4. Сборка проекта
```
cargo build
```

```
cargo espflash /dev/ttyUSB0 target/xtensa-esp32-none-elf/debug/my-esp-project
```
