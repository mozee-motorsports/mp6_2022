#!/bin/sh
echo "Building Package"
cargo build --release
echo "Flashing Package"
cargo-flash --chip STM32F411RETx --release
