//! 🦀 Rusty [CoolProp](https://coolprop.github.io/CoolProp/) wrapper.
//!
//! ## Supported platforms
//!
//! - `Windows x86-64`
//! - `Windows AArch64`
//! - `Linux x86-64`
//! - `macOS x86-64`
//! - `macOS AArch64`
//!
//! ## How to install
//!
//! Run the following command in your project directory:
//!
//! ```shell
//! cargo add rfluids
//! ```
//!
//! **NB.** It comes with native `CoolProp` dynamic libraries for supported platforms.
//! The library required for your platform will be automatically
//! copied to the target directory during build.
//!
//! ## Unit safety
//!
//! This library uses the [uom](https://crates.io/crates/uom) crate (units of measurement)
//! for type-safe zero-cost dimensional analysis. This allows you to avoid
//! errors associated with incorrect quantity dimensions,
//! and will help you save a lot of time on their search and elimination.
//! In addition, you will be able to convert all values to many other dimensions easily.
//!
//! ## Examples
//!
//! Specific heat of saturated water vapor at _1 atm_:
//!
//! ```
//! use approx::assert_relative_eq;
//! use rfluids::prelude::*;
//! use uom::si::pressure::atmosphere;
//! use uom::si::ratio::percent;
//! use uom::si::specific_heat_capacity::calorie_per_gram_kelvin;
//!
//! let mut water_vapor = Fluid::from(Pure::Water).in_state(
//!    fluid_input::pressure!(1.0, atmosphere),
//!    fluid_input::quality!(100.0, percent),
//! )?;
//! // in SI units (J/kg/K)
//! assert_relative_eq!(
//!     water_vapor.specific_heat()?.value,
//!     2079.937085633241,
//!     max_relative = 1e-6
//! );
//! // in cal/g/K
//! assert_relative_eq!(
//!     water_vapor.specific_heat()?.get::<calorie_per_gram_kelvin>(),
//!     0.49711689427180716,
//!     max_relative = 1e-6
//! );
//! # Ok::<(), rfluids::error::Error>(())
//! ```
//!
//! Dynamic viscosity of propylene glycol aqueous solution
//! with _60 %_ mass fraction at _100 kPa_ and _-20 °C_:
//!
//! ```
//! use approx::assert_relative_eq;
//! use rfluids::prelude::*;
//! use uom::si::dynamic_viscosity::poise;
//! use uom::si::pressure::kilopascal;
//! use uom::si::ratio::percent;
//! use uom::si::thermodynamic_temperature::degree_celsius;
//!
//! let mut propylene_glycol = Fluid::from(
//!     BinaryMix::with_fraction(BinaryMixKind::MPG, Ratio::new::<percent>(60.0))?,
//! ).in_state(
//!     fluid_input::pressure!(100.0, kilopascal),
//!     fluid_input::temperature!(-20.0, degree_celsius),
//! )?;
//! // in SI units (Pa·s)
//! assert_relative_eq!(
//!     propylene_glycol.dynamic_viscosity()?.value,
//!     0.13907391053938878,
//!     max_relative = 1e-6
//! );
//! // in P (poise)
//! assert_relative_eq!(
//!     propylene_glycol.dynamic_viscosity()?.get::<poise>(),
//!     1.3907391053938878,
//!     max_relative = 1e-6
//! );
//! # Ok::<(), rfluids::error::Error>(())
//! ```
//!
//! Density of ethanol aqueous solution (with ethanol _40 %_ mass fraction)
//! at _200 kPa_ and _4 °C_:
//!
//! ```
//! use approx::assert_relative_eq;
//! use rfluids::prelude::*;
//! use std::collections::HashMap;
//! use uom::si::mass_density::gram_per_cubic_centimeter;
//! use uom::si::pressure::kilopascal;
//! use uom::si::ratio::percent;
//! use uom::si::thermodynamic_temperature::degree_celsius;
//!
//! let mut mix = Fluid::try_from(
//!     CustomMix::mass_based(HashMap::from([
//!         (Pure::Water, Ratio::new::<percent>(60.0)),
//!         (Pure::Ethanol, Ratio::new::<percent>(40.0)),
//!     ]))?
//! )?.in_state(
//!     fluid_input::pressure!(200.0, kilopascal),
//!     fluid_input::temperature!(4.0, degree_celsius),
//! )?;
//! // in SI units (kg/m³)
//! assert_relative_eq!(
//!     mix.density()?.value,
//!     883.3922771627759,
//!     max_relative = 1e-6
//! );
//! // in g/cm³
//! assert_relative_eq!(
//!     mix.density()?.get::<gram_per_cubic_centimeter>(),
//!     0.8833922771627759,
//!     max_relative = 1e-6
//! );
//! # Ok::<(), rfluids::error::Error>(())
//! ```
//!
//! Wet-bulb temperature of humid air
//! at _300 m_ above sea level, _30 °C_ and _50 %_ relative humidity:
//!
//! ```
//! use approx::assert_relative_eq;
//! use rfluids::prelude::*;
//! use uom::si::length::meter;
//! use uom::si::ratio::percent;
//! use uom::si::thermodynamic_temperature::{degree_celsius, degree_fahrenheit};
//!
//! let mut humid_air = HumidAir::new().in_state(
//!     humid_air_input::altitude!(300.0, meter)?,
//!     humid_air_input::temperature!(30.0, degree_celsius),
//!     humid_air_input::rel_humidity!(50.0, percent),
//! )?;
//! // in SI units (K)
//! assert_relative_eq!(
//!     humid_air.wet_bulb_temperature()?.value,
//!     295.06756903347457,
//!     max_relative = 1e-6
//! );
//! // in °C
//! assert_relative_eq!(
//!     humid_air.wet_bulb_temperature()?.get::<degree_celsius>(),
//!     21.917569033474592,
//!     max_relative = 1e-6
//! );
//! // in °F
//! assert_relative_eq!(
//!     humid_air.wet_bulb_temperature()?.get::<degree_fahrenheit>(),
//!     71.45162426025416,
//!     max_relative = 1e-6
//! );
//! # Ok::<(), rfluids::error::Error>(())
//! ```
//!
//! [`Fluid`](crate::fluid::Fluid) and [`HumidAir`](crate::humid_air::HumidAir)
//! implement the [`PartialEq`] trait. Equality is checked by the thermodynamic state:
//!
//! ```
//! use rfluids::prelude::*;
//! use uom::si::length::meter;
//! use uom::si::pressure::atmosphere;
//! use uom::si::ratio::percent;
//! use uom::si::thermodynamic_temperature::degree_celsius;
//!
//! let mut humid_air = HumidAir::new().in_state(
//!     humid_air_input::altitude!(0.0, meter)?,
//!     humid_air_input::temperature!(20.0, degree_celsius),
//!     humid_air_input::rel_humidity!(50.0, percent),
//! )?;
//! let mut another_humid_air = HumidAir::new().in_state(
//!     humid_air_input::pressure!(1.0, atmosphere),
//!     humid_air_input::temperature!(20.0, degree_celsius),
//!     humid_air_input::rel_humidity!(50.0, percent),
//! )?;
//! assert_eq!(humid_air, another_humid_air);
//!
//! another_humid_air.update(
//!     humid_air_input::pressure!(2.0, atmosphere),
//!     humid_air_input::temperature!(20.0, degree_celsius),
//!     humid_air_input::rel_humidity!(50.0, percent),
//! )?;
//! assert_ne!(humid_air, another_humid_air);
//! # Ok::<(), rfluids::error::Error>(())
//! ```
//!
//! ## License
//!
//! This project is licensed under [MIT License](https://github.com/portyanikhin/rfluids/blob/main/LICENSE).

#![warn(clippy::all, clippy::pedantic, missing_docs)]
#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss,
    clippy::float_cmp,
    clippy::too_many_lines,
    clippy::missing_panics_doc
)]

pub extern crate uom;

pub mod error;
pub mod fluid;
pub mod humid_air;
pub mod io;
pub mod native;
mod ops;
pub mod prelude;
pub mod state_variant;
pub mod substance;
#[cfg(test)]
mod test;
pub mod uom_ext;
