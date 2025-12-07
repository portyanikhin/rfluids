//! `CoolProp` substances.
//!
//! Provides types and functionality for working with `CoolProp` substances.
//!
//! This module defines various substance types that can be used with `CoolProp`,
//! including pure substances and different types of mixtures.
//!
//! # Types
//!
//! The main types provided by this module are:
//!
//! - [`Substance`] -- an enum representing different types of substances:
//!
//!     - [`Pure`] -- pure or pseudo-pure substances
//!     - [`IncompPure`] -- incompressible pure substances
//!     - [`PredefinedMix`] -- predefined mixtures
//!     - [`BinaryMix`] -- binary mixtures with specified fraction _(mass-based or volume-based)_:
//!
//!         - [`BinaryMixKind`] -- binary mixture kinds
//!
//!     - [`CustomMix`] -- custom mixtures
//!
//! Each substance type can be converted into the [`Substance`] enum
//! using the standard [`From`]/[`Into`] traits.

#![allow(missing_docs, non_camel_case_types)]

mod binary_mix;
mod custom_mix;
mod incomp_pure;
mod predefined_mix;
mod pure;

pub use binary_mix::*;
pub use custom_mix::*;
pub use incomp_pure::*;
pub use predefined_mix::*;
pub use pure::*;

/// `CoolProp` substance.
///
/// This enum represents different types of substances that can be used in `CoolProp`.
/// Each variant corresponds to a specific type of substance:
///
/// - [`Pure`] -- represents a pure or pseudo-pure substance
/// - [`IncompPure`] -- represents an incompressible pure substance
/// - [`PredefinedMix`] -- represents a predefined mixture
/// - [`BinaryMix`] -- represents an incompressible binary mixture _(mass-based or volume-based)_
/// - [`CustomMix`] -- represents a custom mixture
///
/// `Substance` enum provides [`From`] implementations for each of its variants,
/// allowing for easy conversion from specific substance types to the `Substance` enum.
#[derive(Clone, Debug, PartialEq)]
pub enum Substance {
    /// Pure or pseudo-pure substance.
    Pure(Pure),
    /// Incompressible pure substance.
    IncompPure(IncompPure),
    /// Predefined mixture.
    PredefinedMix(PredefinedMix),
    /// Incompressible binary mixture _(mass-based or volume-based)_.
    BinaryMix(BinaryMix),
    /// Custom mixture.
    CustomMix(CustomMix),
}

impl From<Pure> for Substance {
    fn from(value: Pure) -> Self {
        Self::Pure(value)
    }
}

impl From<IncompPure> for Substance {
    fn from(value: IncompPure) -> Self {
        Self::IncompPure(value)
    }
}

impl From<PredefinedMix> for Substance {
    fn from(value: PredefinedMix) -> Self {
        Self::PredefinedMix(value)
    }
}

impl From<BinaryMix> for Substance {
    fn from(value: BinaryMix) -> Self {
        Self::BinaryMix(value)
    }
}

impl From<CustomMix> for Substance {
    fn from(value: CustomMix) -> Self {
        Self::CustomMix(value)
    }
}
