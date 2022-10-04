#!/bin/sh
echo "Building Package"
cargo build --release
echo "Flashing Package"
cargo-flash --chip SMT32F411RETx --release
