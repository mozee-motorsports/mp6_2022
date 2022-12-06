- [x] install rust with c++ build tools
- [x] `rustup install target thumbv7em-none-eabihf`
- [x] `cargo install cargo-flash`
- [x] Install st-link
- [x] install vscode
- [x] install rust-analyzer
- [ ] pull repository

# How to install rust and all other things

### 1. Install Rust
First, we have to install the Rust language server. This requires some Visual Studio build tools to build properly on Windows. 
Download and run the installer from [here](https://www.rust-lang.org/tools/install), and if prompted, follow the steps in the installer to install the Visual Studio components. 

You should at the very least restart all shells at this point, and it's probably not a bad idea to restart your computer.


### 2. Install Visual Studio Code
Visual Studio code has the best infrastructure for our use case. I'd reccomend either of the following versions 
- [Microsoft Visual Studio Code](https://code.visualstudio.com/): best supported version of VSCode, full extension library.
- [VSCodium](https://github.com/VSCodium/vscodium): de-Microsoft'd fork. VSCode is open source but Microsoft's version sends telemetry. 

### 3. Install rust-analyzer
Once we have Visual Studio Code installed, we need to install `rust-analyzer` to support the language server in the editor. 
Click on "Extensions" on the left-side bar and search for "rust-analyzer". 
Install and restart VSCode. 

### 4. Install Build Target and toolchain
Rust has a whole bunch of build targets packaged with the compiler by default, but they seem to have forgot ours. 
In your terminal of choice (VSCode's integrated terminal will work just fine), run `rustup install target thumv7em-none-eabihf`. 
Next, run `cargo install cargo-flash`.
Finally, to connect to STM boards, we need to install the ST-Link drivers:
- [build from scratch](https://github.com/stlink-org/stlink)
- [installer](https://www.st.com/en/development-tools/stsw-link009.html)

### 5. Pull GitHub Repository
Finally, we have to pull down the code repository, which can be found [here](https://github.com/Will-Dale-19/FormulaHybrid_MP6) or by calling:
```sh
git clone https://github.com/Will-Dale-19/FormulaHybrid_MP6
```

