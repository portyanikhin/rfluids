# coolprop-sys

[<img alt="GitHub" src="https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="22">](https://github.com/portyanikhin/rfluids)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="22">](https://docs.rs/coolprop-sys)
[<img alt="crates.io" src="https://img.shields.io/crates/v/coolprop-sys?style=for-the-badge&logo=rust&labelColor=555555&color=fc8d62" height="22">](https://crates.io/crates/coolprop-sys)
[<img alt="CI" src="https://img.shields.io/github/actions/workflow/status/portyanikhin/rfluids/ci.yml?style=for-the-badge&logo=githubactions&logoColor=ffffff&label=ci&labelColor=555555" height="22">](https://github.com/portyanikhin/rfluids/actions/workflows/ci.yml)

Raw FFI bindings to [`CoolProp`](https://coolprop.org)

## Supported platforms

- `Linux AArch64`
- `Linux x86-64`
- `macOS AArch64`
- `macOS x86-64`
- `Windows AArch64`
- `Windows x86-64`

## MSRV

`coolprop-sys` requires `rustc` 1.85.0 or later.

## How to install

Add this to your `Cargo.toml`:

```toml
[dependencies]
coolprop-sys = "8"
```

Or via command line:

```shell
cargo add coolprop-sys
```

🎁 It comes with native `CoolProp` dynamic libraries for supported platforms. The library
required for your platform will be automatically copied to the target directory during build.

It also includes pre-generated FFI bindings, so `libclang` is not required for normal builds.

### Regenerating bindings

If you need to regenerate the FFI bindings (requires `libclang`), enable the
**`regen-bindings`** feature.

Add this to your `Cargo.toml`:

```toml
[dependencies]
coolprop-sys = { version = "8", features = ["regen-bindings"] }
```

Or via command line:

```shell
cargo add coolprop-sys --features regen-bindings
```

## Accessing the native library

Use the process-wide [`COOLPROP`](https://docs.rs/coolprop-sys/latest/coolprop_sys/static.COOLPROP.html) handle:

```rust
use coolprop_sys::COOLPROP;

let coolprop = COOLPROP.shared_access();
let critical_temperature = unsafe { coolprop.Props1SI(c"Water".as_ptr(), c"Tcrit".as_ptr()) };
assert!(critical_temperature.is_finite());
```

- Use [`shared_access()`](https://docs.rs/coolprop-sys/latest/coolprop_sys/struct.CoolPropLib.html#method.shared_access)
  only for native operations known to support concurrent execution.
- Use [`exclusive_access()`](https://docs.rs/coolprop-sys/latest/coolprop_sys/struct.CoolPropLib.html#method.exclusive_access)
  for configuration and debug changes, global error or warning handling, `REFPROP` operations,
  `VTPR` construction or reload, tabular backends, and operations whose concurrency guarantees
  are unknown. When in doubt, use exclusive access.

Some native functions report failure through a sentinel value and store details in the
process-global `errstring`. After such a failure with shared access, release the shared guard,
then acquire exclusive access. If the caller needs error details for that operation, read and
discard the stale `errstring` with
[`get_global_param_string`](https://docs.rs/coolprop-sys/latest/coolprop_sys/bindings/struct.CoolProp.html#method.get_global_param_string) (which clears it),
repeat the complete native call, and read the new `errstring` before releasing the exclusive
guard. If the caller does not need error details, clear the stale `errstring` before releasing
the exclusive guard; no retry is required.

When an exclusive native call may set a process-global error or warning, keep the same
exclusive guard from that call through retrieval of its `errstring` or `warnstring`.

Do not acquire another access guard while one is already held by the same thread. For this
synchronization boundary to be effective, all access to the bundled native library in a
process must go through [`COOLPROP`](https://docs.rs/coolprop-sys/latest/coolprop_sys/static.COOLPROP.html).
Constructing [`bindings::CoolProp`](https://docs.rs/coolprop-sys/latest/coolprop_sys/bindings/struct.CoolProp.html)
directly bypasses it and requires equivalent process-wide synchronization from the caller.

#### License

<sup>
This project is licensed under
<a href="https://github.com/portyanikhin/rfluids/blob/main/LICENSE">MIT License</a>
</sup>
