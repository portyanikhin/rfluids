//! [<img alt="GitHub" src="https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="22">](https://github.com/portyanikhin/rfluids)
//! [<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="22">](https://docs.rs/coolprop-sys)
//! [<img alt="crates.io" src="https://img.shields.io/crates/v/coolprop-sys?style=for-the-badge&logo=rust&labelColor=555555&color=fc8d62" height="22">](https://crates.io/crates/coolprop-sys)
//! [<img alt="CI" src="https://img.shields.io/github/actions/workflow/status/portyanikhin/rfluids/ci.yml?style=for-the-badge&logo=githubactions&logoColor=ffffff&label=ci&labelColor=555555" height="22">](https://github.com/portyanikhin/rfluids/actions/workflows/ci.yml)
//!
//! Raw FFI bindings to [`CoolProp`](https://coolprop.org)
//!
//! ## Supported platforms
//!
//! - `Linux AArch64`
//! - `Linux x86-64`
//! - `macOS AArch64`
//! - `macOS x86-64`
//! - `Windows AArch64`
//! - `Windows x86-64`
//!
//! ## MSRV
//!
//! `coolprop-sys` requires `rustc` 1.85.0 or later.
//!
//! ## How to install
//!
//! Add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! coolprop-sys = "8"
//! ```
//!
//! Or via command line:
//!
//! ```shell
//! cargo add coolprop-sys
//! ```
//!
//! 🎁 It comes with native `CoolProp` dynamic libraries for supported platforms. The library
//! required for your platform will be automatically copied to the target directory during build.
//!
//! It also includes pre-generated FFI bindings, so `libclang` is not required for normal builds.
//!
//! ### Regenerating bindings
//!
//! If you need to regenerate the FFI bindings (requires `libclang`), enable the
//! **`regen-bindings`** feature.
//!
//! Add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! coolprop-sys = { version = "8", features = ["regen-bindings"] }
//! ```
//!
//! Or via command line:
//!
//! ```shell
//! cargo add coolprop-sys --features regen-bindings
//! ```
//!
//! ### Static linking
//!
//! By default, `coolprop-sys` links `CoolProp` dynamically: the native library is copied next to
//! your binary and loaded at runtime. Enable the **`static-link`** feature to statically link
//! `CoolProp` into your binary instead, so no native library needs to be shipped or found at
//! runtime.
//!
//! Add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! coolprop-sys = { version = "8", features = ["static-link"] }
//! ```
//!
//! Or via command line:
//!
//! ```shell
//! cargo add coolprop-sys --features static-link
//! ```
//!
//! ## Accessing the native library
//!
//! Use the process-wide [`COOLPROP`] handle:
//!
//! ```rust
//! use coolprop_sys::COOLPROP;
//!
//! let coolprop = COOLPROP.shared_access();
//! let critical_temperature = unsafe { coolprop.Props1SI(c"Water".as_ptr(), c"Tcrit".as_ptr()) };
//! assert!(critical_temperature.is_finite());
//! ```
//!
//! - Use [`shared_access()`](CoolPropLib::shared_access) only for native operations known to
//!   support concurrent execution.
//! - Use [`exclusive_access()`](CoolPropLib::exclusive_access) for configuration and debug changes,
//!   global error or warning handling, `REFPROP` operations, `VTPR` construction or reload, tabular
//!   backends, and operations whose concurrency guarantees are unknown. When in doubt, use
//!   exclusive access.
//!
//! Some native functions report failure through a sentinel value and store details in the
//! process-global `errstring`. After such a failure with shared access, release the shared guard,
//! then acquire exclusive access. If the caller needs error details for that operation, read and
//! discard the stale `errstring` with
//! [`get_global_param_string`](bindings::CoolProp::get_global_param_string) (which clears it),
//! repeat the complete native call, and read the new `errstring` before releasing the exclusive
//! guard. If the caller does not need error details, clear the stale `errstring` before releasing
//! the exclusive guard; no retry is required.
//!
//! When an exclusive native call may set a process-global error or warning, keep the same
//! exclusive guard from that call through retrieval of its `errstring` or `warnstring`.
//!
//! Do not acquire another access guard while one is already held by the same thread. For this
//! synchronization boundary to be effective, all access to the bundled native library in a
//! process must go through [`COOLPROP`]. Constructing [`bindings::CoolProp`] directly bypasses it
//! and requires equivalent process-wide synchronization from the caller.
//!
//! #### License
//!
//! <sup>
//! This project is licensed under
//! <a href="https://github.com/portyanikhin/rfluids/blob/main/LICENSE">MIT License</a>
//! </sup>

use std::{
    ops::Deref,
    sync::{LazyLock, RwLock, RwLockReadGuard, RwLockWriteGuard},
};

pub mod bindings;

/// `CoolProp` dynamic library absolute path.
#[cfg(all(target_os = "linux", target_arch = "aarch64", not(feature = "static-link")))]
pub const COOLPROP_PATH: &str = coolprop_sys_linux_aarch64::COOLPROP_PATH;
#[cfg(all(target_os = "linux", target_arch = "x86_64", not(feature = "static-link")))]
pub const COOLPROP_PATH: &str = coolprop_sys_linux_x86_64::COOLPROP_PATH;
#[cfg(all(target_os = "macos", target_arch = "aarch64", not(feature = "static-link")))]
pub const COOLPROP_PATH: &str = coolprop_sys_macos_aarch64::COOLPROP_PATH;
#[cfg(all(target_os = "macos", target_arch = "x86_64", not(feature = "static-link")))]
pub const COOLPROP_PATH: &str = coolprop_sys_macos_x86_64::COOLPROP_PATH;
#[cfg(all(target_os = "windows", target_arch = "aarch64", not(feature = "static-link")))]
pub const COOLPROP_PATH: &str = coolprop_sys_windows_aarch64::COOLPROP_PATH;
#[cfg(all(target_os = "windows", target_arch = "x86_64", not(feature = "static-link")))]
pub const COOLPROP_PATH: &str = coolprop_sys_windows_x86_64::COOLPROP_PATH;

#[cfg(all(target_os = "linux", target_arch = "aarch64", feature = "static-link"))]
extern crate coolprop_sys_linux_aarch64;
#[cfg(all(target_os = "linux", target_arch = "x86_64", feature = "static-link"))]
extern crate coolprop_sys_linux_x86_64;
#[cfg(all(target_os = "macos", target_arch = "aarch64", feature = "static-link"))]
extern crate coolprop_sys_macos_aarch64;
#[cfg(all(target_os = "macos", target_arch = "x86_64", feature = "static-link"))]
extern crate coolprop_sys_macos_x86_64;
#[cfg(all(target_os = "windows", target_arch = "aarch64", feature = "static-link"))]
extern crate coolprop_sys_windows_aarch64;
#[cfg(all(target_os = "windows", target_arch = "x86_64", feature = "static-link"))]
extern crate coolprop_sys_windows_x86_64;

/// Process-wide synchronization boundary for the loaded `CoolProp` dynamic library.
///
/// Use [`CoolPropLib::shared_access`] only for native operations known to support concurrent
/// execution. Use [`CoolPropLib::exclusive_access`] for configuration and debug changes, global
/// error or warning handling, `REFPROP` operations, `VTPR` construction or reload, tabular
/// backends, and operations whose concurrency guarantees are unknown. When in doubt, use
/// exclusive access.
///
/// Do not acquire a second access guard while another guard is held by the same thread. Drop the
/// current guard before changing access modes.
///
/// For this synchronization boundary to be effective, all access to the bundled native library in
/// a process must go through [`COOLPROP`]. Constructing [`bindings::CoolProp`] directly bypasses
/// this boundary and requires equivalent process-wide synchronization from the caller.
pub struct CoolPropLib(RwLock<bindings::CoolProp>);

impl CoolPropLib {
    /// Acquires shared access to the native library.
    ///
    /// A shared guard does not make an arbitrary native function or backend reentrant. Use it only
    /// for operations explicitly known to support concurrent execution, such as calculations on
    /// independent states backed by `HEOS`, `INCOMP`, `IF97`, `SRK`, `PR`, `PCSAFT`, or an
    /// already-constructed `VTPR` state.
    ///
    /// Some native functions report failure through a sentinel value and store details in the
    /// process-global `errstring`. Do not release shared access and then treat that string as the
    /// error from the failed call: another failure may replace it first. To retrieve attributable
    /// error details:
    ///
    /// 1. Release the shared guard.
    /// 2. Acquire exclusive access.
    /// 3. Read and discard any stale `errstring` with
    ///    [`get_global_param_string`](bindings::CoolProp::get_global_param_string), which clears
    ///    the stored message.
    /// 4. Repeat the complete native call.
    /// 5. Read `errstring` with
    ///    [`get_global_param_string`](bindings::CoolProp::get_global_param_string) before releasing
    ///    the same exclusive guard.
    ///
    /// If error details are not needed, perform only steps 1–3; no retry is required.
    ///
    /// Lock poisoning is recovered transparently; access does not panic solely because a previous
    /// guard holder panicked.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use coolprop_sys::COOLPROP;
    ///
    /// let coolprop = COOLPROP.shared_access();
    /// let critical_temperature = unsafe { coolprop.Props1SI(c"Water".as_ptr(), c"Tcrit".as_ptr()) };
    /// assert!(critical_temperature.is_finite());
    /// ```
    pub fn shared_access(&self) -> SharedAccess<'_> {
        SharedAccess(self.0.read().unwrap_or_else(|err| err.into_inner()))
    }

    /// Acquires exclusive access to the native library.
    ///
    /// Use this for configuration changes, pending-error retrieval, `REFPROP` calls, `VTPR` state
    /// construction, tabular backends, and other calls that touch mutable process-global state.
    /// When a native call may set a process-global error or warning, keep the same exclusive guard
    /// from that call through retrieval of its `errstring` or `warnstring`.
    ///
    /// Lock poisoning is recovered transparently; access does not panic solely because a previous
    /// guard holder panicked.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use coolprop_sys::COOLPROP;
    ///
    /// let coolprop = COOLPROP.exclusive_access();
    /// unsafe {
    ///     coolprop.set_debug_level(0);
    /// }
    /// ```
    pub fn exclusive_access(&self) -> ExclusiveAccess<'_> {
        ExclusiveAccess(self.0.write().unwrap_or_else(|err| err.into_inner()))
    }
}

/// Shared access to native operations known to support concurrent execution.
#[must_use]
pub struct SharedAccess<'a>(RwLockReadGuard<'a, bindings::CoolProp>);

impl Deref for SharedAccess<'_> {
    type Target = bindings::CoolProp;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Exclusive access to native `CoolProp` calls that must not overlap other calls.
///
/// This type intentionally does not implement [`DerefMut`](std::ops::DerefMut): exclusive access
/// is an execution mode, not permission to replace or mutate the loaded function table.
#[must_use]
pub struct ExclusiveAccess<'a>(RwLockWriteGuard<'a, bindings::CoolProp>);

impl Deref for ExclusiveAccess<'_> {
    type Target = bindings::CoolProp;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Global instance of the `CoolProp` library, statically or dynamically linked depending on
/// whether the **`static-link`** feature is enabled.
///
/// The library is initialized lazily. Before the handle is published, an internal probe
/// initializes native process-global configuration. This is automatic; callers do not need to
/// perform a special first native call.
///
/// # Panics
///
/// Panics on initialization if the `CoolProp` dynamic library cannot be loaded (not applicable
/// when statically linked) or if its initialization probe does not produce a finite value.
///
/// # Safety
///
/// Methods exposed by [`bindings::CoolProp`] remain unsafe. Callers must uphold each function's
/// pointer and lifetime requirements and select the access mode required by the native operation.
/// Loading and the initialization probe occur once, but synchronization is effective only for
/// calls made through this handle.
///
/// # See Also
///
/// - [`CoolPropLib.h` Reference](https://coolprop.org/_static/doxygen/html/_cool_prop_2_cool_prop_lib_8h.html)
pub static COOLPROP: LazyLock<CoolPropLib> =
    LazyLock::new(|| CoolPropLib(RwLock::new(load_coolprop())));

#[cfg(feature = "static-link")]
fn load_coolprop() -> bindings::CoolProp {
    let coolprop = bindings::CoolProp::new().expect("Failed to initialize CoolProp bindings");
    probe(&coolprop);
    coolprop
}

#[cfg(not(feature = "static-link"))]
fn load_coolprop() -> bindings::CoolProp {
    let coolprop = unsafe { bindings::CoolProp::new(COOLPROP_PATH) }
        .expect("CoolProp dynamic library should load from `COOLPROP_PATH`");
    probe(&coolprop);
    coolprop
}

/// Runs the one-time initialization probe shared by both link modes, catching a broken link
/// (bad ABI, missing symbol, ...) at startup instead of at some arbitrary later call site.
fn probe(coolprop: &bindings::CoolProp) {
    let value = unsafe { coolprop.Props1SI(c"Water".as_ptr(), c"Tcrit".as_ptr()) };
    assert!(
        value.is_finite(),
        "CoolProp initialization probe `Props1SI(\"Water\", \"Tcrit\")` should return a finite value"
    );
}

#[cfg(test)]
mod tests {
    use std::{sync::TryLockError, thread};

    use static_assertions::assert_not_impl_any;

    use super::*;

    assert_not_impl_any!(ExclusiveAccess<'static>: std::ops::DerefMut);

    fn test_lib() -> CoolPropLib {
        LazyLock::force(&COOLPROP);
        CoolPropLib(RwLock::new(load_coolprop()))
    }

    fn shared_access_is_available(lib: &CoolPropLib) -> bool {
        match lib.0.try_read() {
            Ok(_) | Err(TryLockError::Poisoned(_)) => true,
            Err(TryLockError::WouldBlock) => false,
        }
    }

    fn exclusive_access_is_available(lib: &CoolPropLib) -> bool {
        match lib.0.try_write() {
            Ok(_) | Err(TryLockError::Poisoned(_)) => true,
            Err(TryLockError::WouldBlock) => false,
        }
    }

    #[test]
    fn access_types_deref_to_coolprop() {
        // Given
        let lib = test_lib();

        // When
        let shared = lib.shared_access();
        let shared_target = std::ptr::from_ref::<bindings::CoolProp>(&shared);
        drop(shared);
        let exclusive = lib.exclusive_access();
        let exclusive_target = std::ptr::from_ref::<bindings::CoolProp>(&exclusive);

        // Then
        assert_eq!(shared_target, exclusive_target);
    }

    #[test]
    fn poisoned_lock_is_recovered() {
        // Given
        let lib = test_lib();

        // When
        let panic_result = thread::scope(|scope| {
            scope
                .spawn(|| {
                    let _access = lib.exclusive_access();
                    panic!("poison the test lock");
                })
                .join()
        });
        let shared = lib.shared_access();
        let shared_level = unsafe { shared.get_debug_level() };
        drop(shared);
        let exclusive = lib.exclusive_access();
        let exclusive_level = unsafe { exclusive.get_debug_level() };

        // Then
        assert!(panic_result.is_err());
        assert!((0..=10).contains(&shared_level));
        assert!((0..=10).contains(&exclusive_level));
    }

    #[test]
    fn shared_access_allows_another_reader_and_blocks_a_writer() {
        // Given
        let lib = test_lib();
        let _shared = lib.shared_access();

        // When
        let another_reader_is_available = shared_access_is_available(&lib);
        let writer_is_available = exclusive_access_is_available(&lib);

        // Then
        assert!(another_reader_is_available);
        assert!(!writer_is_available);
    }

    #[test]
    fn exclusive_access_blocks_other_access() {
        // Given
        let lib = test_lib();
        let _exclusive = lib.exclusive_access();

        // When
        let reader_is_available = shared_access_is_available(&lib);
        let writer_is_available = exclusive_access_is_available(&lib);

        // Then
        assert!(!reader_is_available);
        assert!(!writer_is_available);
    }

    #[test]
    fn unlocked_lib_allows_exclusive_access() {
        // Given
        let lib = test_lib();

        // When
        let writer_is_available = exclusive_access_is_available(&lib);

        // Then
        assert!(writer_is_available);
    }
}
