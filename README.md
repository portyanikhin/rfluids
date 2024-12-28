# fluids-rs

![Platform](https://img.shields.io/badge/platform-win--x64_%7C_lin--x64_%7C_mac--x64_%7C_mac--arm64-lightgrey)
[![License](https://img.shields.io/github/license/portyanikhin/fluids-rs)](https://github.com/portyanikhin/fluids-rs/blob/main/LICENSE)

Simple, full-featured, lightweight, cross-platform
[CoolProp](https://coolprop.github.io/CoolProp/) wrapper for Rust.

**[Documentation](https://docs.rs/fluids-rs/)**

## Supported platforms

- Windows x64
- Linux x64
- macOS x64
- macOS ARM64

## Usage

Add the following to your `Cargo.toml`:

```toml
[dependencies]
fluids-rs = "0.1.0-alpha"
```

**NB.** It comes with native CoolProp dynamic libraries for supported platforms.
The library required for your platform will be automatically
copied to the target directory during build.

## License

This project is licensed under [MIT License](https://github.com/portyanikhin/fluids-rs/blob/main/LICENSE).
