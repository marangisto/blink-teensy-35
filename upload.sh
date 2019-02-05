:
# generate binary and upload to MCU
ELF=blink-teensy-35
cargo size --bin $ELF -- -A
~/arduino-1.6.13/hardware/tools/arm/bin/arm-none-eabi-objcopy -O ihex -R .eeprom ./target/thumbv7em-none-eabihf/debug/$ELF /tmp/$ELF.hex
~/arduino-1.6.13/hardware/tools/arm/bin/arm-none-eabi-objdump -d ./target/thumbv7em-none-eabihf/debug/$ELF|less

