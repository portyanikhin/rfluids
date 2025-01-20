//! CoolProp substances.

#![allow(missing_docs, non_camel_case_types)]

pub use binary_mix::*;
pub use custom_mix::*;
pub use incomp_pure::*;
pub use predefined_mix::*;
pub use pure::*;
pub use refrigerant::*;

mod binary_mix;
mod custom_mix;
mod incomp_pure;
mod predefined_mix;
mod pure;
mod refrigerant;

/// CoolProp backend name.
pub trait BackendName {
    /// Returns CoolProp backend name.
    ///
    /// # Examples
    ///
    /// ```
    /// use rfluids::substance::*;
    /// use rfluids::uom::si::f64::Ratio;
    /// use rfluids::uom::si::ratio::percent;
    /// use std::collections::HashMap;
    ///
    /// assert_eq!(Pure::Water.backend_name(), "HEOS");
    /// assert_eq!(IncompPure::Water.backend_name(), "INCOMP");
    /// assert_eq!(Refrigerant::R32.backend_name(), "HEOS");
    /// assert_eq!(PredefinedMix::TypicalNaturalGas.backend_name(), "HEOS");
    /// assert_eq!(BinaryMixKind::MPG.backend_name(), "INCOMP");
    /// assert_eq!(
    ///     CustomMix::try_from(HashMap::from([
    ///         (Pure::Water.into(), Ratio::new::<percent>(60.0)),
    ///         (Pure::Ethanol.into(), Ratio::new::<percent>(40.0)),
    ///     ]))
    ///     .unwrap()
    ///     .backend_name(),
    ///     "HEOS"
    /// );
    /// ```
    fn backend_name(&self) -> &'static str;
}

/// CoolProp substance.
///
/// Superset of:
///
///  - [`Pure`]
///  - [`IncompPure`]
///  - [`Refrigerant`]
///  - [`PredefinedMix`]
///  - [`BinaryMix`]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Substance {
    /// Pure or pseudo-pure substance.
    Pure(Pure),

    /// Incompressible pure substance.
    IncompPure(IncompPure),

    /// Refrigerant.
    Refrigerant(Refrigerant),

    /// Predefined mixture.
    PredefinedMix(PredefinedMix),

    /// Incompressible binary mixture _(mass-based or volume-based)_.
    BinaryMix(BinaryMix),
}

impl BackendName for Substance {
    fn backend_name(&self) -> &'static str {
        match self {
            Substance::Pure(pure) => pure.backend_name(),
            Substance::IncompPure(incomp_pure) => incomp_pure.backend_name(),
            Substance::Refrigerant(refrigerant) => refrigerant.backend_name(),
            Substance::PredefinedMix(predefined_mix) => predefined_mix.backend_name(),
            Substance::BinaryMix(binary_mix) => binary_mix.kind.backend_name(),
        }
    }
}

impl AsRef<str> for Substance {
    fn as_ref(&self) -> &str {
        match self {
            Substance::Pure(pure) => pure.as_ref(),
            Substance::IncompPure(incomp_pure) => incomp_pure.as_ref(),
            Substance::Refrigerant(refrigerant) => refrigerant.as_ref(),
            Substance::PredefinedMix(predefined_mix) => predefined_mix.as_ref(),
            Substance::BinaryMix(binary_mix) => binary_mix.kind.as_ref(),
        }
    }
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

impl From<Refrigerant> for Substance {
    fn from(value: Refrigerant) -> Self {
        Self::Refrigerant(value)
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

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;
    use strum::IntoEnumIterator;

    #[fixture]
    fn all_substances() -> Vec<Substance> {
        Pure::iter()
            .map(Substance::from)
            .chain(IncompPure::iter().map(Substance::from))
            .chain(Refrigerant::iter().map(Substance::from))
            .chain(PredefinedMix::iter().map(Substance::from))
            .chain(BinaryMixKind::iter().map(|kind| {
                Substance::from(
                    BinaryMix::try_from(kind, 0.5 * (kind.min_fraction() + kind.max_fraction()))
                        .unwrap(),
                )
            }))
            .collect()
    }

    #[rstest]
    fn substance_is_transparent(all_substances: Vec<Substance>) {
        for substance in all_substances {
            match substance {
                Substance::Pure(pure) => {
                    assert_eq!(substance.backend_name(), pure.backend_name());
                    assert_eq!(substance.as_ref(), pure.as_ref());
                }
                Substance::IncompPure(incomp_pure) => {
                    assert_eq!(substance.backend_name(), incomp_pure.backend_name());
                    assert_eq!(substance.as_ref(), incomp_pure.as_ref());
                }
                Substance::Refrigerant(refrigerant) => {
                    assert_eq!(substance.backend_name(), refrigerant.backend_name());
                    assert_eq!(substance.as_ref(), refrigerant.as_ref());
                }
                Substance::PredefinedMix(predefined_mix) => {
                    assert_eq!(substance.backend_name(), predefined_mix.backend_name());
                    assert_eq!(substance.as_ref(), predefined_mix.as_ref());
                }
                Substance::BinaryMix(binary_mix) => {
                    assert_eq!(substance.backend_name(), binary_mix.kind.backend_name());
                    assert_eq!(substance.as_ref(), binary_mix.kind.as_ref());
                }
            }
        }
    }
}
