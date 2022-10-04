# Blink example for STM32-F411RE

1. Install Rust 
	- Windows: install rust-init.exe from website
	- WSL: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
2. Update toolchain: `rustup update`
3. Install cargo-flash
	- `cargo install cargo-flash `
4. Install compile target
	- `rustup target add thumbv7em-none-eabihf`
5. Flash board
	- Plug in board
	- `cargo-flash --chip STM32F411RETx --release`
	- Alternatively, `./build_unix` for Linux/WSL or `./build` for Windows
