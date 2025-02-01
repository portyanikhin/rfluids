//! Thermophysical properties of substances.

mod defined;
mod invariant;
mod requests;
mod undefined;

use requests::{FluidCreateRequest, FluidUpdateRequest};

use crate::error::FluidFromCustomMixError;
use crate::io::{FluidParam, FluidTrivialParam};
use crate::native::AbstractState;
use crate::state_variant::{Defined, StateVariant, Undefined};
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
pub struct Fluid<S: StateVariant = Defined> {
    substance: Substance,
    backend: AbstractState,
    update_request: Option<FluidUpdateRequest>,
    trivial_outputs: HashMap<FluidTrivialParam, f64>,
    outputs: HashMap<FluidParam, f64>,
    state: PhantomData<S>,
}

impl TryFrom<Substance> for Fluid<Undefined> {
    type Error = FluidFromCustomMixError;

    /// Creates and returns a new [`Fluid`] instance
    /// with [`Undefined`] state variant from any [`Substance`].
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
    /// let water = Substance::from(Pure::Water);
    /// assert!(Fluid::try_from(water).is_ok());
    ///
    /// let supported_mix = Substance::from(
    ///     CustomMix::mass_based(HashMap::from([
    ///         (Pure::Water, Ratio::new::<percent>(60.0)),
    ///         (Pure::Ethanol, Ratio::new::<percent>(40.0)),
    ///     ]))
    ///     .unwrap(),
    /// );
    /// assert!(Fluid::try_from(supported_mix).is_ok());
    ///
    /// let unsupported_mix = Substance::from(
    ///     CustomMix::mass_based(HashMap::from([
    ///         (Pure::Orthohydrogen, Ratio::new::<percent>(60.0)),
    ///         (Pure::R32, Ratio::new::<percent>(40.0)),
    ///     ]))
    ///     .unwrap(),
    /// );
    /// assert!(Fluid::try_from(unsupported_mix).is_err());
    /// ```
    fn try_from(value: Substance) -> Result<Self, Self::Error> {
        let request = FluidCreateRequest::from(&value);
        let mut backend = AbstractState::new(request.backend_name, request.name)?;
        if let Some(fractions) = request.fractions {
            backend.set_fractions(&fractions).unwrap();
        }
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

impl From<Pure> for Fluid<Undefined> {
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
        Substance::from(value).try_into().unwrap()
    }
}

impl From<IncompPure> for Fluid<Undefined> {
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
        Substance::from(value).try_into().unwrap()
    }
}

impl From<PredefinedMix> for Fluid<Undefined> {
    /// Creates and returns a new [`Fluid`] instance
    /// with [`Undefined`] state variant from [`PredefinedMix`].
    ///
    /// # Examples
    ///
    /// ```
    /// use rfluids::prelude::fluid::*;
    ///
    /// let r444a = Fluid::from(PredefinedMix::R444A);
    /// ```
    fn from(value: PredefinedMix) -> Self {
        Substance::from(value).try_into().unwrap()
    }
}

impl From<BinaryMix> for Fluid<Undefined> {
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
        Substance::from(value).try_into().unwrap()
    }
}

impl TryFrom<CustomMix> for Fluid<Undefined> {
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
    ///     (Pure::Water, Ratio::new::<percent>(60.0)),
    ///     (Pure::Ethanol, Ratio::new::<percent>(40.0)),
    /// ]))
    /// .unwrap();
    /// assert!(Fluid::try_from(supported_mix).is_ok());
    ///
    /// let unsupported_mix = CustomMix::mass_based(HashMap::from([
    ///     (Pure::Orthohydrogen, Ratio::new::<percent>(60.0)),
    ///     (Pure::R32, Ratio::new::<percent>(40.0)),
    /// ]))
    /// .unwrap();
    /// assert!(Fluid::try_from(unsupported_mix).is_err());
    /// ```
    fn try_from(value: CustomMix) -> Result<Self, Self::Error> {
        Substance::from(value).try_into()
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
            (Pure::R32, Ratio::new::<percent>(70.0)),
            (Pure::R134a, Ratio::new::<percent>(30.0)),
        ]))
        .unwrap();
        assert!(Fluid::try_from(supported_mix).is_ok());
    }

    #[test]
    fn try_from_unsupported_custom_mix_returns_err() {
        let unsupported_mix = CustomMix::mole_based(HashMap::from([
            (Pure::Xenon, Ratio::new::<percent>(10.0)),
            (Pure::R22, Ratio::new::<percent>(90.0)),
        ]))
        .unwrap();
        assert!(Fluid::try_from(unsupported_mix).is_err());
    }
}
