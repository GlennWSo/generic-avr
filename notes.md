# to flash it:
sudo avrdude -patmega328p -carduino -P /dev/ttyACM0  -b 115200 -D -Uflash:w:target/avr-atmega328p/release/generic-avr.elf:e
