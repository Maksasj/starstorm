# Starstorm

Starstorm - space shooter, bullet hell game written in Rust using Bevy game engine.

**Entry for Mini Jame Gam #18**

Graphically highly inspired by [Downwell](https://store.steampowered.com/app/360740/Downwell/) and gameplay probably Touhou Project, aka game from which this popular song called [【東方】Bad Apple!! ＰＶ【影絵】](https://youtu.be/FtutLA63Cp8) came out 😊.

> And yet again, starstorm is a not a new javascript framework !

### Links:
1. Web version available at [starsstorm.com](https://starsstorm.com/) <br>
2. Game page on Itch.io [maksasj.itch.io/starstorm](https://maksasj.itch.io/starstorm) <br>
3. Source code avaiable at [github.com/Maksasj/starstorm](https://github.com/Maksasj/starstorm)

Cool looking widgets 
<img src="https://img.shields.io/github/license/Maksasj/starstorm" alt="license">
<img src="https://img.shields.io/github/v/release/Maksasj/starstorm" alt="version">
<img src="https://img.shields.io/github/actions/workflow/status/Maksasj/starstorm/rust_release.yml?label=build" alt="rust_build">
<img src="https://img.shields.io/github/actions/workflow/status/Maksasj/starstorm/web_release.yml?label=web build" alt="web_build">

## Building
### Requirements
Initially project have been build with these versions
1. cargo 1.70.0
2. rustc 1.70.0
3. wasm-bindgen 0.2.86 (required only for web build)

### Windows build
There is two main building options first one is building `exe` file, and second one is a `web` version.
As for `exe` version, you simply can do default or use preexisting `build.bat` and `make.bat` script.
```bash
cargo build --release 
```
Cargo will automatically, download and build dependencies, such as bevy.

### Web build
As for the web version, you will need to have `wasm-bindgen` cli utility(see this guide [link](https://rustwasm.github.io/wasm-bindgen/reference/cli.html)). After installing `wasm-bindgen`, you can try to run 
```bash
cargo build --release --target wasm32-unknown-unknown
```
this command will compile game into a wasm file. Then you can follow this guide [link](https://bevy-cheatbook.github.io/platforms/wasm.html). For development simplicity there is also `web-build` script, that compiles game into a wasm file, and runs `wasm-bindgen` automatically.