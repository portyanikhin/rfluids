//! `CoolProp` substances.

#![allow(missing_docs, non_camel_case_types)]

pub use backend_name::*;
pub use binary_mix::*;
pub use custom_mix::*;
pub use incomp_pure::*;
pub use predefined_mix::*;
pub use pure::*;

mod backend_name;
mod binary_mix;
mod custom_mix;
mod incomp_pure;
mod predefined_mix;
mod pure;

/// `CoolProp` substance.
///
/// Superset of:
///
/// - [`Pure`]
/// - [`IncompPure`]
/// - [`PredefinedMix`]
/// - [`BinaryMix`]
/// - [`CustomMix`]
#[derive(Debug, Clone, PartialEq)]
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
