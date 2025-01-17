//! Thermophysical properties of substances.

mod undefined_state;

use crate::error::FluidFromBinaryMixError;
use crate::io::{FluidParam, FluidTrivialParam};
use crate::native::AbstractState;
use crate::substance::*;
use crate::uom::si::f64::Ratio;
use crate::UndefinedState;
use std::collections::HashMap;
use std::marker::PhantomData;

/// Provider of thermophysical properties of substances.
///
/// It works with:
///
/// - pure or pseudo-pure substances _([`Pure`])_;
/// - incompressible pure substances _([`IncompPure`])_;
/// - refrigerants _([`Refrigerant`])_;
/// - predefined mixtures _([`PredefinedMix`])_;
/// - incompressible binary mixtures _([`BinaryMix`])_.
///
/// It implements the [typestate pattern](https://en.wikipedia.org/wiki/Typestate_analysis)
/// and has two generic type parameters:
///
/// - `T` — specified substance type _(see above)_;
/// - `S` — state type _([`UndefinedState`] or [`DefinedState`](crate::DefinedState))_.
///
/// Depending on `S`, the `Fluid` instance has different functionality.
#[derive(Debug)]
pub struct Fluid<T, S = UndefinedState>
where
    T: AsRef<str> + BackendName + Copy,
{
    /// Substance.
    pub substance: T,
    /// Fraction of binary mixture _([`Some`] for binary mixtures, [`None`] for others)_.
    pub fraction: Option<Ratio>,
    backend: AbstractState,
    trivial_params_cache: HashMap<FluidTrivialParam, f64>,
    params_cache: HashMap<FluidParam, f64>,
    state: PhantomData<S>,
}

impl From<Pure> for Fluid<Pure, UndefinedState> {
    /// Creates and returns a new [`Fluid`] instance
    /// with [`UndefinedState`] from [`Pure`].
    ///
    /// # Examples
    ///
    /// ```
    /// use rfluids::fluid::Fluid;
    /// use rfluids::substance::Pure;
    ///
    /// let water = Fluid::from(Pure::Water);
    /// ```
    fn from(value: Pure) -> Self {
        Self::new(value, None)
    }
}

impl From<IncompPure> for Fluid<IncompPure, UndefinedState> {
    /// Creates and returns a new [`Fluid`] instance
    /// with [`UndefinedState`] from [`IncompPure`].
    ///
    /// # Examples
    ///
    /// ```
    /// use rfluids::fluid::Fluid;
    /// use rfluids::substance::IncompPure;
    ///
    /// let incomp_water = Fluid::from(IncompPure::Water);
    /// ```
    fn from(value: IncompPure) -> Self {
        Self::new(value, None)
    }
}

impl From<Refrigerant> for Fluid<Refrigerant, UndefinedState> {
    /// Creates and returns a new [`Fluid`] instance
    /// with [`UndefinedState`] from [`Refrigerant`].
    ///
    /// # Examples
    ///
    /// ```
    /// use rfluids::fluid::Fluid;
    /// use rfluids::substance::Refrigerant;
    ///
    /// let r32 = Fluid::from(Refrigerant::R32);
    /// ```
    fn from(value: Refrigerant) -> Self {
        Self::new(value, None)
    }
}

impl From<PredefinedMix> for Fluid<PredefinedMix, UndefinedState> {
    /// Creates and returns a new [`Fluid`] instance
    /// with [`UndefinedState`] from [`PredefinedMix`].
    ///
    /// # Examples
    ///
    /// ```
    /// use rfluids::fluid::Fluid;
    /// use rfluids::substance::PredefinedMix;
    ///
    /// let natural_gas = Fluid::from(PredefinedMix::TypicalNaturalGas);
    /// ```
    fn from(value: PredefinedMix) -> Self {
        Self::new(value, None)
    }
}

impl TryFrom<(BinaryMix, Ratio)> for Fluid<BinaryMix, UndefinedState> {
    type Error = FluidFromBinaryMixError;

    /// Creates and returns a new [`Fluid`] instance
    /// with [`UndefinedState`] from [`BinaryMix`] and its fraction.
    ///
    /// # Errors
    ///
    /// For invalid fraction
    /// _(out of [[`BinaryMix::min_fraction`]; [`BinaryMix::max_fraction`]] range)_,
    /// an [`FluidFromBinaryMixError`] is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use rfluids::fluid::Fluid;
    /// use rfluids::substance::BinaryMix;
    /// use rfluids::uom::si::f64::Ratio;
    /// use rfluids::uom::si::ratio::percent;
    ///
    /// let propylene_glycol = Fluid::try_from((BinaryMix::MPG, Ratio::new::<percent>(40.0)));
    /// assert!(propylene_glycol.is_ok());
    /// ```
    fn try_from(value: (BinaryMix, Ratio)) -> Result<Self, Self::Error> {
        if !(value.0.min_fraction()..=value.0.max_fraction()).contains(&value.1) {
            return Err(FluidFromBinaryMixError::InvalidFraction {
                specified: value.1,
                min: value.0.min_fraction(),
                max: value.0.max_fraction(),
            });
        }
        let mut fluid = Self::new(value.0, Some(value.1));
        fluid.backend.set_fractions(&[value.1.value]).unwrap();
        Ok(fluid)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::uom::si::ratio::{part_per_billion, percent};
    use strum::IntoEnumIterator;

    #[test]
    fn from_each_pure_does_not_panic() {
        for substance in Pure::iter() {
            let _fluid = Fluid::from(substance);
        }
    }

    #[test]
    fn from_each_incomp_pure_does_not_panic() {
        for substance in IncompPure::iter() {
            let _fluid = Fluid::from(substance);
        }
    }

    #[test]
    fn from_each_refrigerant_does_not_panic() {
        for substance in Refrigerant::iter() {
            let _fluid = Fluid::from(substance);
        }
    }

    #[test]
    fn from_each_predefined_mix_does_not_panic() {
        for substance in PredefinedMix::iter() {
            let _fluid = Fluid::from(substance);
        }
    }

    #[test]
    fn try_from_binary_mix_with_valid_fraction_always_returns_ok() {
        for substance in BinaryMix::iter() {
            assert!(Fluid::try_from((substance, substance.min_fraction())).is_ok());
            assert!(Fluid::try_from((
                substance,
                0.5 * (substance.min_fraction() + substance.max_fraction())
            ))
            .is_ok());
            assert!(Fluid::try_from((substance, substance.max_fraction())).is_ok());
        }
    }

    #[test]
    fn try_from_binary_mix_with_invalid_fraction_always_returns_err() {
        let delta = Ratio::new::<part_per_billion>(1.0);
        for substance in BinaryMix::iter() {
            assert!(Fluid::try_from((substance, substance.min_fraction() - delta)).is_err());
            assert_eq!(
                Fluid::try_from((substance, substance.max_fraction() + delta))
                    .unwrap_err()
                    .to_string(),
                format!(
                    "Specified fraction ({:?} %) is out of possible range [{:.1}; {:.1}] %!",
                    (substance.max_fraction() + delta).get::<percent>(),
                    substance.min_fraction().get::<percent>(),
                    substance.max_fraction().get::<percent>()
                )
            );
        }
    }
}
