- You are cross-compiling for a target which doesn’t have std prepackaged. Consider one of the following:
	- Adding a pre-compiled version of std with rustup target add
	- Building std from source with cargo build -Z build-std
	- Using #![no_std] at the crate root, so you won’t need std in the first place.
- You are developing the compiler itself and haven’t built libstd from source. You can usually build it with x.py build library/std. More information about x.py is available in the rustc-dev-guide.



## Safety
Programming safety is defined as a combination of the following: 
- 


Steps for installing

1. Rust 
	- Windows: install rust-init.exe from website
	- WSL: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
	- rustup update
2. Configure Cargo
	- cargo install cargo-flash 
	- rustup target add thumbv7em-none-eabihf
3. .cargo/config 

```toml:
[target.thumbv7em-none-eabihf]
# use linker script Tlink.x script
rustflags = ["-C", "link-arg=-Tlink.x"]
```
This flag 

[build]
# always compile to stm32f4
target = "thumbv7em-none-eabihf"

[env]
DEFMT_LOG = "info"
```

4. Cargo.toml
```toml
[package]
name = "stm_rust_test"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
[profile.release]
opt-level = 'z'
lto = true

[dependencies]
embedded-hal = "^0.2.4"
nb = "1"
cortex-m = "0.7"
cortex-m-rt = "0.7"
panic-halt = "^0.2.0"

[dependencies.stm32f4xx-hal]
version = "0.13.2"
features = ["stm32f411"]
```









