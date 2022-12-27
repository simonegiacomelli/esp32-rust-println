#!/bin/bash
. ~/export-esp.sh && \
cargo build && \
  espflash --speed 921600 /dev/cu.usbserial-0001 target/xtensa-esp32-espidf/debug/esp-rs-gen1