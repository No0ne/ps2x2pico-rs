# ps2x2pico
USB keyboard/mouse to PS/2 interface converter using a Raspberry Pi Pico

**Work in progress - until I know enought about Rust use this instead:**
* keyboard only: https://github.com/No0ne/ps2pico
* keyboard+mouse: https://github.com/No0ne/ps2x2pico

# Build
```
rustup target install thumbv6m-none-eabi
cargo install flip-link
cargo install elf2uf2-rs --locked
cargo build --release
elf2uf2-rs target/thumbv6m-none-eabi/release/ps2x2pico ps2x2pico.uf2
```

# Resources
* https://github.com/No0ne/ps2pico
