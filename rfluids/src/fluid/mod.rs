//! Thermophysical properties of substances.

pub use backend_name::*;
pub use substance_variant::*;

mod backend_name;
mod defined;
mod invariant;
mod substance_variant;
mod undefined;

use crate::error::FluidFromCustomMixError;
use crate::io::{FluidParam, FluidTrivialParam, FluidUpdateRequest};
use crate::native::AbstractState;
use crate::state::{Defined, StateVariant, Undefined};
use crate::substance::*;
use std::collections::HashMap;
use std::fmt::Debug;
use std::marker::PhantomData;

/// Provider of thermophysical properties of substances.
///
/// It implements the [typestate pattern](https://en.wikipedia.org/wiki/Typestate_analysis)
/// and has two generic type parameters:
///
/// - `T` -- substance variant _([`Substance`] or [`CustomMix`])_.
/// - `S` -- state variant _([`Defined`] or [`Undefined`])_.
///
/// Depending on `T` and `S`, the `Fluid` instance has different functionality.
#[derive(Debug)]
pub struct Fluid<T: SubstanceVariant = Substance, S: StateVariant = Defined> {
    substance: T,
    backend: AbstractState,
    update_request: Option<FluidUpdateRequest>,
    trivial_outputs: HashMap<FluidTrivialParam, f64>,
    outputs: HashMap<FluidParam, f64>,
    state: PhantomData<S>,
}

impl From<Substance> for Fluid<Substance, Undefined> {
    /// Creates and returns a new [`Fluid`] instance
    /// with [`Undefined`] state variant from any [`Substance`].
    ///
    /// # Examples
    ///
    /// ```
    /// use rfluids::prelude::fluid::*;
    ///
    /// let water = Fluid::from(Substance::from(Pure::Water));
    /// ```
    fn from(value: Substance) -> Self {
        let mut backend = AbstractState::new(value.backend_name(), value).unwrap();
        if let Substance::BinaryMix(binary_mix) = value {
            backend.set_fractions(&[binary_mix.fraction.value]).unwrap();
        }
        Self {
            substance: value,
            backend,
            update_request: None,
            trivial_outputs: HashMap::new(),
            outputs: HashMap::new(),
            state: PhantomData,
        }
    }
}

impl From<Pure> for Fluid<Substance, Undefined> {
    /// Creates and returns a new [`Fluid`] instance
    /// with [`Undefined`] state variant from [`Pure`].
    ///
    /// # Examples
    ///
    /// ```
    /// use rfluids::prelude::fluid::*;
    ///
    /// let water = Fluid::from(Pure::Water);
    /// ```
    fn from(value: Pure) -> Self {
        Substance::from(value).into()
    }
}

impl From<IncompPure> for Fluid<Substance, Undefined> {
    /// Creates and returns a new [`Fluid`] instance
    /// with [`Undefined`] state variant from [`IncompPure`].
    ///
    /// # Examples
    ///
    /// ```
    /// use rfluids::prelude::fluid::*;
    ///
    /// let incomp_water = Fluid::from(IncompPure::Water);
    /// ```
    fn from(value: IncompPure) -> Self {
        Substance::from(value).into()
    }
}

impl From<Refrigerant> for Fluid<Substance, Undefined> {
    /// Creates and returns a new [`Fluid`] instance
    /// with [`Undefined`] state variant from [`Refrigerant`].
    ///
    /// # Examples
    ///
    /// ```
    /// use rfluids::prelude::fluid::*;
    ///
    /// let r32 = Fluid::from(Refrigerant::R32);
    /// ```
    fn from(value: Refrigerant) -> Self {
        Substance::from(value).into()
    }
}

impl From<PredefinedMix> for Fluid<Substance, Undefined> {
    /// Creates and returns a new [`Fluid`] instance
    /// with [`Undefined`] state variant from [`PredefinedMix`].
    ///
    /// # Examples
    ///
    /// ```
    /// use rfluids::prelude::fluid::*;
    ///
    /// let natural_gas = Fluid::from(PredefinedMix::TypicalNaturalGas);
    /// ```
    fn from(value: PredefinedMix) -> Self {
        Substance::from(value).into()
    }
}

impl From<BinaryMix> for Fluid<Substance, Undefined> {
    /// Creates and returns a new [`Fluid`] instance
    /// with [`Undefined`] state variant from [`BinaryMix`].
    ///
    /// # Examples
    ///
    /// ```
    /// use rfluids::prelude::fluid::*;
    /// use rfluids::uom::si::ratio::percent;
    ///
    /// let propylene_glycol = Fluid::from(
    ///     BinaryMix::try_from(BinaryMixKind::MPG, Ratio::new::<percent>(40.0)).unwrap(),
    /// );
    /// ```
    fn from(value: BinaryMix) -> Self {
        Substance::from(value).into()
    }
}

impl TryFrom<CustomMix> for Fluid<CustomMix, Undefined> {
    type Error = FluidFromCustomMixError;

    /// Creates and returns a new [`Fluid`] instance
    /// with [`Undefined`] state variant from [`CustomMix`].
    ///
    /// # Errors
    ///
    /// For unsupported custom mixtures, a [`FluidFromCustomMixError`] is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use rfluids::prelude::fluid::*;
    /// use rfluids::uom::si::ratio::percent;
    /// use std::collections::HashMap;
    ///
    /// let supported_mix = CustomMix::mass_based(HashMap::from([
    ///     (Pure::Water.into(), Ratio::new::<percent>(60.0)),
    ///     (Pure::Ethanol.into(), Ratio::new::<percent>(40.0)),
    /// ]))
    /// .unwrap();
    /// assert!(Fluid::try_from(supported_mix).is_ok());
    ///
    /// let unsupported_mix = CustomMix::mass_based(HashMap::from([
    ///     (Pure::Orthohydrogen.into(), Ratio::new::<percent>(60.0)),
    ///     (Refrigerant::R32.into(), Ratio::new::<percent>(40.0)),
    /// ]))
    /// .unwrap();
    /// assert!(Fluid::try_from(unsupported_mix).is_err());
    /// ```
    //noinspection RsUnwrap
    fn try_from(value: CustomMix) -> Result<Self, Self::Error> {
        let mix = value.to_mole_based();
        let (substances, fractions): (Vec<&str>, Vec<f64>) = mix
            .components()
            .iter()
            .map(|component| (component.0.as_ref(), component.1.value))
            .unzip();
        let mut backend = AbstractState::new(value.backend_name(), substances.join("&"))?;
        backend.set_fractions(&fractions).unwrap();
        Ok(Self {
            substance: value,
            backend,
            update_request: None,
            trivial_outputs: HashMap::new(),
            outputs: HashMap::new(),
            state: PhantomData,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use strum::IntoEnumIterator;
    use uom::si::f64::Ratio;
    use uom::si::ratio::percent;

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
    fn from_each_binary_mix_does_not_panic() {
        for kind in BinaryMixKind::iter() {
            let _fluid = Fluid::from(
                BinaryMix::try_from(kind, 0.5 * (kind.min_fraction() + kind.max_fraction()))
                    .unwrap(),
            );
        }
    }

    #[test]
    fn try_from_supported_custom_mix_returns_ok() {
        let supported_mix = CustomMix::mass_based(HashMap::from([
            (Refrigerant::R32.into(), Ratio::new::<percent>(70.0)),
            (Refrigerant::R134a.into(), Ratio::new::<percent>(30.0)),
        ]))
        .unwrap();
        assert!(Fluid::try_from(supported_mix).is_ok());
    }

    #[test]
    fn try_from_unsupported_custom_mix_returns_err() {
        let unsupported_mix = CustomMix::mole_based(HashMap::from([
            (Pure::Xenon.into(), Ratio::new::<percent>(10.0)),
            (Refrigerant::R22.into(), Ratio::new::<percent>(90.0)),
        ]))
        .unwrap();
        assert!(Fluid::try_from(unsupported_mix).is_err());
    }
}
