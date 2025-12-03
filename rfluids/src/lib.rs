//! [<img alt="GitHub" src="https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="22">](https://github.com/portyanikhin/rfluids)
//! [<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="22">](https://docs.rs/rfluids)
//! [<img alt="crates.io" src="https://img.shields.io/crates/v/rfluids?style=for-the-badge&logo=rust&labelColor=555555&color=fc8d62" height="22">](https://crates.io/crates/rfluids)
//! [<img alt="CI" src="https://img.shields.io/github/actions/workflow/status/portyanikhin/rfluids/ci.yml?style=for-the-badge&logo=githubactions&logoColor=ffffff&label=ci&labelColor=555555" height="22">](https://github.com/portyanikhin/rfluids/actions/workflows/ci.yml)
//! [<img alt="codecov" src="https://img.shields.io/codecov/c/github/portyanikhin/rfluids?style=for-the-badge&logo=codecov&label=codecov&labelColor=555555" height="22">](https://app.codecov.io/gh/portyanikhin/rfluids)
//!
//! 🦀 Rusty [CoolProp](https://coolprop.github.io/CoolProp/) wrapper.
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
//! 🎁 It comes with native `CoolProp` dynamic libraries for supported
//! platforms. The library required for your platform will be automatically
//! copied to the target directory during build.
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
//! assert_relative_eq!(
//!     water_vapor.specific_heat()?,
//!     2_079.937_085_633_241,
//!     max_relative = 1e-6
//! );
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
//! let mut propylene_glycol = Fluid::from(
//!     BinaryMixKind::MPG.with_fraction(0.6)?,
//! )
//! .in_state(FluidInput::pressure(100e3), FluidInput::temperature(253.15))?;
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
//! use std::collections::HashMap;
//!
//! use approx::assert_relative_eq;
//! use rfluids::prelude::*;
//!
//! let mut mix = Fluid::try_from(CustomMix::mass_based(HashMap::from([
//!     (Pure::Water, 0.6),
//!     (Pure::Ethanol, 0.4),
//! ]))?)?
//! .in_state(FluidInput::pressure(200e3), FluidInput::temperature(277.15))?;
//! assert_relative_eq!(
//!     mix.density()?,
//!     883.392_277_162_775_9,
//!     max_relative = 1e-6
//! );
//! # Ok::<(), rfluids::Error>(())
//! ```
//!
//! Wet-bulb temperature **\[K\]** of humid air
//! at _300 m_ above sea level, _30 °C_ and _50 %_ relative humidity:
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
//! implement the [`PartialEq`] trait. Equality is checked
//! by the thermodynamic state:
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
