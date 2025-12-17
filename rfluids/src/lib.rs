//! [<img alt="GitHub" src="https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="22">](https://github.com/portyanikhin/rfluids)
//! [<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="22">](https://docs.rs/rfluids)
//! [<img alt="crates.io" src="https://img.shields.io/crates/v/rfluids?style=for-the-badge&logo=rust&labelColor=555555&color=fc8d62" height="22">](https://crates.io/crates/rfluids)
//! [<img alt="CI" src="https://img.shields.io/github/actions/workflow/status/portyanikhin/rfluids/ci.yml?style=for-the-badge&logo=githubactions&logoColor=ffffff&label=ci&labelColor=555555" height="22">](https://github.com/portyanikhin/rfluids/actions/workflows/ci.yml)
//! [<img alt="codecov" src="https://img.shields.io/codecov/c/github/portyanikhin/rfluids?style=for-the-badge&logo=codecov&label=codecov&labelColor=555555" height="22">](https://app.codecov.io/gh/portyanikhin/rfluids)
//!
//! 🦀 Rusty [`CoolProp`](https://coolprop.org) wrapper
//!
//! ## Overview
//!
//! `rfluids` provides a safe, idiomatic Rust interface to [`CoolProp`](https://coolprop.org) ---
//! a comprehensive C++ library for thermophysical property calculations.
//! Built with type safety and ergonomics in mind, it enables precise fluid property
//! calculations for engineering and scientific applications.
//!
//! ### Key Features
//!
//! - **Rich substance library**: pure fluids, incompressible fluids, predefined mixtures, binary
//!   mixtures, and custom mass- or volume-based mixtures
//! - **Comprehensive property calculations**: thermodynamic (e.g., density, enthalpy, entropy,
//!   etc.) and transport properties (e.g., viscosity, thermal conductivity, etc.)
//! - **Psychrometric calculations**: based on **ASHRAE RP-1485** standard
//! - **Flexible backends**: choose between `HEOS`, `IF97`, and other `CoolProp` backends, which
//!   determine the underlying equation of state or calculation method used for thermophysical
//!   property calculations
//! - **Type-safe API**: leverages Rust's type system with the [Typestate Pattern](https://en.wikipedia.org/wiki/Typestate_analysis)
//!   to prevent invalid operations at compile time
//! - **Zero configuration**: includes pre-compiled `CoolProp` dynamic libraries for all supported
//!   platforms
//!
//! ### Modules
//!
//! - [`fluid`](crate::fluid) -- thermophysical properties of substances (pure fluids and mixtures)
//! - [`humid_air`](crate::humid_air) -- thermophysical properties of _**real**_ humid air
//! - [`substance`](crate::substance) -- types representing `CoolProp` substances
//! - [`io`](crate::io) -- input/output parameter types for fluid and humid air calculations
//! - [`native`](crate::native) -- low-level and high-level `CoolProp` API bindings
//! - [`prelude`](crate::prelude) -- convenient re-exports of commonly used types and traits
//!
//! ## Supported platforms
//!
//! - `Linux x86-64`
//! - `macOS AArch64`
//! - `macOS x86-64`
//! - `Windows AArch64`
//! - `Windows x86-64`
//!
//! ## MSRV
//!
//! `rfluids` requires `rustc` 1.85.0 or later.
//!
//! ## How to install
//!
//! Run the following command in your project directory:
//!
//! ```shell
//! cargo add rfluids
//! ```
//!
//! 🎁 It comes with native `CoolProp` dynamic libraries for supported platforms. The library
//! required for your platform will be automatically copied to the target directory during build.
//!
//! It also includes pre-generated FFI bindings, so `libclang` is not required for normal builds.
//!
//! ### Regenerating bindings
//!
//! If you need to regenerate the FFI bindings (requires `libclang`), enable the `regen-bindings`
//! feature:
//!
//! ```shell
//! cargo add rfluids --features regen-bindings
//! ```
//!
//! ## Examples
//!
//! | ℹ️ All calculations are performed in SI units |
//! | :-------------------------------------------: |
//!
//! Specific heat **\[J/kg/K\]** of saturated water vapor at _1 atm_:
//!
//! ```
//! use approx::assert_relative_eq;
//! use rfluids::prelude::*;
//!
//! let mut water_vapor = Fluid::from(Pure::Water)
//!     .in_state(FluidInput::pressure(101_325.0), FluidInput::quality(1.0))?;
//! assert_relative_eq!(water_vapor.specific_heat()?, 2_079.937_085_633_241, max_relative = 1e-6);
//! # Ok::<(), rfluids::Error>(())
//! ```
//!
//! Dynamic viscosity **\[Pa·s\]** of propylene glycol aqueous solution
//! with _60 %_ mass fraction at _100 kPa_ and _-20 °C_:
//!
//! ```
//! use approx::assert_relative_eq;
//! use rfluids::prelude::*;
//!
//! let mut propylene_glycol = Fluid::from(BinaryMixKind::MPG.with_fraction(0.6)?)
//!     .in_state(FluidInput::pressure(100e3), FluidInput::temperature(253.15))?;
//! assert_relative_eq!(
//!     propylene_glycol.dynamic_viscosity()?,
//!     0.139_073_910_539_388_78,
//!     max_relative = 1e-6
//! );
//! # Ok::<(), rfluids::Error>(())
//! ```
//!
//! Density **\[kg/m³\]** of ethanol aqueous solution
//! (with ethanol _40 %_ mass fraction) at _200 kPa_ and _4 °C_:
//!
//! ```
//! use approx::assert_relative_eq;
//! use rfluids::prelude::*;
//!
//! let mut mix =
//!     Fluid::try_from(CustomMix::mass_based([(Pure::Water, 0.6), (Pure::Ethanol, 0.4)])?)?
//!         .in_state(FluidInput::pressure(200e3), FluidInput::temperature(277.15))?;
//! assert_relative_eq!(mix.density()?, 883.392_277_162_775_9, max_relative = 1e-6);
//! # Ok::<(), rfluids::Error>(())
//! ```
//!
//! Wet-bulb temperature **\[K\]** of humid air at _300 m_ above sea level,
//! _30 °C_ and _50 %_ relative humidity:
//!
//! ```
//! use approx::assert_relative_eq;
//! use rfluids::prelude::*;
//!
//! let mut humid_air = HumidAir::new().in_state(
//!     HumidAirInput::altitude(300.0)?,
//!     HumidAirInput::temperature(303.15),
//!     HumidAirInput::rel_humidity(0.5),
//! )?;
//! assert_relative_eq!(
//!     humid_air.wet_bulb_temperature()?,
//!     295.067_569_033_474_57,
//!     max_relative = 1e-6
//! );
//! # Ok::<(), rfluids::Error>(())
//! ```
//!
//! [`Fluid`](crate::fluid::Fluid) and [`HumidAir`](crate::humid_air::HumidAir)
//! implement the [`PartialEq`] trait. Equality is checked by the thermodynamic state:
//!
//! ```
//! use rfluids::prelude::*;
//!
//! let mut humid_air = HumidAir::new().in_state(
//!     HumidAirInput::altitude(0.0)?,
//!     HumidAirInput::temperature(293.15),
//!     HumidAirInput::rel_humidity(0.5),
//! )?;
//! let mut another_humid_air = HumidAir::new().in_state(
//!     HumidAirInput::pressure(101_325.0),
//!     HumidAirInput::temperature(293.15),
//!     HumidAirInput::rel_humidity(0.5),
//! )?;
//! assert_eq!(humid_air, another_humid_air);
//!
//! another_humid_air.update(
//!     HumidAirInput::pressure(101_325.0),
//!     HumidAirInput::temperature(303.15),
//!     HumidAirInput::rel_humidity(0.5),
//! )?;
//! assert_ne!(humid_air, another_humid_air);
//! # Ok::<(), rfluids::Error>(())
//! ```
//!
//! You can also specify a `CoolProp` backend for [`Fluid`](crate::fluid::Fluid)
//! instead of the default one using [`Fluid::builder`](crate::fluid::Fluid::builder):
//!
//! ```
//! use rfluids::prelude::*;
//!
//! let mut water = Fluid::from(Pure::Water)
//!     .in_state(FluidInput::pressure(101_325.0), FluidInput::temperature(293.15))?;
//! let mut if97_water = Fluid::builder()
//!     .substance(Pure::Water)
//!     .with_backend(BaseBackend::If97)
//!     .build()?
//!     .in_state(FluidInput::pressure(101_325.0), FluidInput::temperature(293.15))?;
//!
//! // Fluids with different backends are never equal
//! assert_ne!(water, if97_water);
//!
//! // Different backends may yield slightly different results for the same property
//! assert!((water.specific_heat()? - if97_water.specific_heat()?).abs() > 1e-6);
//! # Ok::<(), rfluids::Error>(())
//! ```
//!
//! #### License
//!
//! <sup>
//! This project is licensed under
//! <a href="https://github.com/portyanikhin/rfluids/blob/main/LICENSE">MIT License</a>.
//! </sup>

#![warn(clippy::all, clippy::pedantic, missing_docs)]
#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss,
    clippy::float_cmp,
    clippy::too_many_lines,
    clippy::missing_panics_doc
)]

mod error;
pub mod fluid;
pub mod humid_air;
pub mod io;
pub mod native;
mod ops;
pub mod prelude;
mod state_variant;
pub mod substance;
#[cfg(test)]
mod test;

pub use error::*;
pub use state_variant::*;
