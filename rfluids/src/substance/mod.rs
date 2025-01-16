//! CoolProp substances.

#![allow(non_camel_case_types)]

pub use binary_mix::*;
pub use incomp_pure::*;
pub use predefined_mix::*;
pub use pure::*;
pub use refrigerant::*;
use uom::si::f64::Ratio;

mod binary_mix;
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
    /// use rfluids::substance;
    /// use rfluids::substance::BackendName;
    ///
    /// assert_eq!(substance::Pure::Water.backend_name(), "HEOS");
    /// assert_eq!(substance::IncompPure::Water.backend_name(), "INCOMP");
    /// assert_eq!(substance::BinaryMix::MPG.backend_name(), "INCOMP");
    /// assert_eq!(substance::PredefinedMix::TypicalNaturalGas.backend_name(), "HEOS");
    /// assert_eq!(substance::Refrigerant::R32.backend_name(), "HEOS");
    /// ```
    fn backend_name(&self) -> &'static str;
}

/// CoolProp substances.
///
/// Superset of:
///
/// - [`Pure`]
/// - [`IncompPure`]
/// - [`BinaryMix`]
/// - [`PredefinedMix`]
/// - [`Refrigerant`]
///
/// # Examples
///
/// [`Substance`] has the same behavior of converting to [`&str`](str), as its subsets:
///
/// ```
/// use rfluids::substance;
///
/// let water = substance::Pure::Water;
/// assert_eq!(
///     substance::Substance::from(water).as_ref(),
///     water.as_ref()
/// );
///
/// let incomp_water = substance::IncompPure::Water;
/// assert_eq!(
///     substance::Substance::from(incomp_water).as_ref(),
///     incomp_water.as_ref()
/// );
///
/// let propylene_glycol = substance::BinaryMix::MPG;
/// assert_eq!(
///     substance::Substance::from(propylene_glycol).as_ref(),
///     propylene_glycol.as_ref()
/// );
///
/// let natural_gas = substance::PredefinedMix::TypicalNaturalGas;
/// assert_eq!(
///     substance::Substance::from(natural_gas).as_ref(),
///     natural_gas.as_ref()
/// );
///
/// let r32 = substance::Refrigerant::R32;
/// assert_eq!(
///     substance::Substance::from(r32).as_ref(),
///     r32.as_ref()
/// );
/// ```
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Substance {
    Pure(Pure),
    IncompPure(IncompPure),
    BinaryMix(BinaryMix),
    PredefinedMix(PredefinedMix),
    Refrigerant(Refrigerant),
}

impl Substance {
    /// Minimum possible fraction of binary mixture
    /// _([`Some`] for binary mixtures, [`None`] for others)_.
    ///
    /// # Examples
    ///
    /// ```
    /// use rfluids::substance::{BinaryMix, Pure, Substance};
    /// use rfluids::uom::si::f64::Ratio;
    /// use rfluids::uom::si::ratio::percent;
    ///
    /// let propylene_glycol = Substance::BinaryMix(BinaryMix::MPG);
    /// assert_eq!(propylene_glycol.min_fraction(), Some(Ratio::new::<percent>(0.0)));
    /// let water = Substance::Pure(Pure::Water);
    /// assert_eq!(water.min_fraction(), None);
    /// ```
    pub fn min_fraction(&self) -> Option<Ratio> {
        match self {
            Substance::BinaryMix(binary_mix) => Some(binary_mix.min_fraction()),
            _ => None,
        }
    }

    /// Maximum possible fraction of binary mixture
    /// _([`Some`] for binary mixtures, [`None`] for others)_.
    ///
    /// # Examples
    ///
    /// ```
    /// use rfluids::substance::{BinaryMix, Pure, Substance};
    /// use rfluids::uom::si::f64::Ratio;
    /// use rfluids::uom::si::ratio::percent;
    ///
    /// let propylene_glycol = Substance::BinaryMix(BinaryMix::MPG);
    /// assert_eq!(propylene_glycol.max_fraction(), Some(Ratio::new::<percent>(60.0)));
    /// let water = Substance::Pure(Pure::Water);
    /// assert_eq!(water.max_fraction(), None);
    /// ```
    pub fn max_fraction(&self) -> Option<Ratio> {
        match self {
            Substance::BinaryMix(binary_mix) => Some(binary_mix.max_fraction()),
            _ => None,
        }
    }
}

impl BackendName for Substance {
    fn backend_name(&self) -> &'static str {
        match self {
            Substance::Pure(pure) => pure.backend_name(),
            Substance::IncompPure(incomp_pure) => incomp_pure.backend_name(),
            Substance::BinaryMix(binary_mix) => binary_mix.backend_name(),
            Substance::PredefinedMix(predefined_mix) => predefined_mix.backend_name(),
            Substance::Refrigerant(refrigerant) => refrigerant.backend_name(),
        }
    }
}

impl AsRef<str> for Substance {
    fn as_ref(&self) -> &str {
        match self {
            Substance::Pure(pure) => pure.as_ref(),
            Substance::IncompPure(incomp_pure) => incomp_pure.as_ref(),
            Substance::BinaryMix(binary_mix) => binary_mix.as_ref(),
            Substance::PredefinedMix(predefined_mix) => predefined_mix.as_ref(),
            Substance::Refrigerant(refrigerant) => refrigerant.as_ref(),
        }
    }
}

impl From<Pure> for Substance {
    fn from(value: Pure) -> Self {
        Substance::Pure(value)
    }
}

impl From<IncompPure> for Substance {
    fn from(value: IncompPure) -> Self {
        Substance::IncompPure(value)
    }
}

impl From<BinaryMix> for Substance {
    fn from(value: BinaryMix) -> Self {
        Substance::BinaryMix(value)
    }
}

impl From<PredefinedMix> for Substance {
    fn from(value: PredefinedMix) -> Self {
        Substance::PredefinedMix(value)
    }
}

impl From<Refrigerant> for Substance {
    fn from(value: Refrigerant) -> Self {
        Substance::Refrigerant(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;
    use strum::IntoEnumIterator;

    #[fixture]
    fn substances() -> Vec<Substance> {
        Pure::iter()
            .map(Substance::from)
            .chain(IncompPure::iter().map(Substance::from))
            .chain(BinaryMix::iter().map(Substance::from))
            .chain(PredefinedMix::iter().map(Substance::from))
            .chain(Refrigerant::iter().map(Substance::from))
            .collect()
    }

    #[rstest]
    fn min_max_fractions_are_transparent(substances: Vec<Substance>) {
        for s in substances {
            match s {
                Substance::BinaryMix(binary_mix) => {
                    assert_eq!(s.min_fraction(), Some(binary_mix.min_fraction()));
                    assert_eq!(s.max_fraction(), Some(binary_mix.max_fraction()));
                }
                _ => {
                    assert!(s.min_fraction().is_none());
                    assert!(s.max_fraction().is_none());
                }
            }
        }
    }

    #[rstest]
    fn backend_name_is_transparent(substances: Vec<Substance>) {
        for s in substances {
            match s {
                Substance::Pure(pure) => assert_eq!(s.backend_name(), pure.backend_name()),
                Substance::IncompPure(incomp_pure) => {
                    assert_eq!(s.backend_name(), incomp_pure.backend_name())
                }
                Substance::BinaryMix(binary_mix) => {
                    assert_eq!(s.backend_name(), binary_mix.backend_name())
                }
                Substance::PredefinedMix(predefined_mix) => {
                    assert_eq!(s.backend_name(), predefined_mix.backend_name())
                }
                Substance::Refrigerant(refrigerant) => {
                    assert_eq!(s.backend_name(), refrigerant.backend_name())
                }
            }
        }
    }

    #[rstest]
    fn as_ref_is_transparent(substances: Vec<Substance>) {
        for s in substances {
            match s {
                Substance::Pure(pure) => assert_eq!(s.as_ref(), pure.as_ref()),
                Substance::IncompPure(incomp_pure) => {
                    assert_eq!(s.as_ref(), incomp_pure.as_ref())
                }
                Substance::BinaryMix(binary_mix) => {
                    assert_eq!(s.as_ref(), binary_mix.as_ref())
                }
                Substance::PredefinedMix(predefined_mix) => {
                    assert_eq!(s.as_ref(), predefined_mix.as_ref())
                }
                Substance::Refrigerant(refrigerant) => {
                    assert_eq!(s.as_ref(), refrigerant.as_ref())
                }
            }
        }
    }
}
