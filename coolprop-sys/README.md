# coolprop-sys

[<img alt="GitHub" src="https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="22">](https://github.com/portyanikhin/rfluids)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="22">](https://docs.rs/coolprop-sys)
[<img alt="crates.io" src="https://img.shields.io/crates/v/coolprop-sys?style=for-the-badge&logo=rust&labelColor=555555&color=fc8d62" height="22">](https://crates.io/crates/coolprop-sys)
[<img alt="CI" src="https://img.shields.io/github/actions/workflow/status/portyanikhin/rfluids/ci.yml?style=for-the-badge&logo=githubactions&logoColor=ffffff&label=ci&labelColor=555555" height="22">](https://github.com/portyanikhin/rfluids/actions/workflows/ci.yml)

Raw FFI bindings to [`CoolProp`](https://coolprop.org)

## Supported platforms

- `Linux x86-64`
- `macOS AArch64`
- `macOS x86-64`
- `Windows AArch64`
- `Windows x86-64`

## MSRV

`coolprop-sys` requires `rustc` 1.85.0 or later.

## How to install

Run the following command in your project directory:

```shell
cargo add coolprop-sys
```

🎁 It comes with native `CoolProp` dynamic libraries for supported platforms.
The library required for your platform will be automatically copied
to the target directory during build.

It also includes pre-generated FFI bindings, so `libclang` is not required for normal builds.

### Regenerating bindings

If you need to regenerate the FFI bindings (requires `libclang`), enable the `regen-bindings` feature:

```shell
cargo add coolprop-sys --features regen-bindings
```

#### License

<sup>
This project is licensed under
<a href="https://github.com/portyanikhin/rfluids/blob/main/LICENSE">MIT License</a>.
</sup>
