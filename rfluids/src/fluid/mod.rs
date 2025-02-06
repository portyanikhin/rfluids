//! Thermophysical and transport properties of substances.
//!
//! This module provides a comprehensive interface for calculating
//! thermophysical and transport properties of various substances,
//! including pure fluids and different mixtures (i.e., [`Substance`] or any of its subset).
//!
//! # Types
//!
//! The core type for managing thermophysical and transport properties of substances is [`Fluid`].
//! It encompasses various state management features and property calculations.
//! The [`Fluid`] struct is generic over the state variant ([`Defined`] or [`Undefined`]).
//! Depending on the state variant, the [`Fluid`] instance has different functionality
//! (e.g., with [`Undefined`] state variant it has only trivial properties available,
//! but with [`Defined`] state variant it supports a full set of property calculations).
//! The [`Fluid`] struct can be created from any [`Pure`], [`IncompPure`], [`PredefinedMix`]
//! or [`BinaryMix`] using the [`From`]/[`Into`] traits; and from any [`Substance`]
//! or [`CustomMix`] using the [`TryFrom`]/[`TryInto`] traits. This is due to the fact that
//! [`CustomMix`] potentially can be unsupported by the `CoolProp`.

mod defined;
mod invariant;
mod requests;
mod undefined;

use crate::error::{FluidFromCustomMixError, FluidOutputError, FluidStateError};
use crate::io::{FluidParam, FluidTrivialParam};
use crate::native::AbstractState;
use crate::state_variant::{Defined, StateVariant, Undefined};
use crate::substance::{BinaryMix, CustomMix, IncompPure, PredefinedMix, Pure, Substance};
use requests::{FluidCreateRequest, FluidUpdateRequest};
use std::collections::HashMap;
use std::fmt::Debug;
use std::marker::PhantomData;

/// Result type for operations that could fail while updating fluid state.
pub type StateResult<T> = Result<T, FluidStateError>;

/// Result type for operations that could fail while retrieving fluid properties.
pub type OutputResult<T> = Result<T, FluidOutputError>;

/// Provider of thermophysical and transport properties of substances.
///
/// It implements the [typestate pattern](https://en.wikipedia.org/wiki/Typestate_analysis)
/// and has one generic type parameter: `S` -- state variant _([`Defined`] or [`Undefined`])_.
///
/// Depending on `S`, the `Fluid` instance has different functionality
/// (e.g., with [`Undefined`] state variant it has only trivial properties available,
/// but with [`Defined`] state variant it supports a full set of property calculations).
#[derive(Debug)]
pub struct Fluid<S: StateVariant = Defined> {
    substance: Substance,
    backend: AbstractState,
    update_request: Option<FluidUpdateRequest>,
    outputs: HashMap<FluidParam, Result<f64, FluidOutputError>>,
    trivial_outputs: HashMap<FluidTrivialParam, Option<f64>>,
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
    ///     ]))?,
    /// );
    /// assert!(Fluid::try_from(supported_mix).is_ok());
    ///
    /// let unsupported_mix = Substance::from(
    ///     CustomMix::mass_based(HashMap::from([
    ///         (Pure::Orthohydrogen, Ratio::new::<percent>(60.0)),
    ///         (Pure::R32, Ratio::new::<percent>(40.0)),
    ///     ]))?,
    /// );
    /// assert!(Fluid::try_from(unsupported_mix).is_err());
    /// # Ok::<(), rfluids::error::Error>(())
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
            outputs: HashMap::new(),
            trivial_outputs: HashMap::new(),
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
    ///     BinaryMix::with_fraction(BinaryMixKind::MPG, Ratio::new::<percent>(40.0))?,
    /// );
    /// # Ok::<(), rfluids::error::Error>(())
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
    /// ]))?;
    /// assert!(Fluid::try_from(supported_mix).is_ok());
    ///
    /// let unsupported_mix = CustomMix::mass_based(HashMap::from([
    ///     (Pure::Orthohydrogen, Ratio::new::<percent>(60.0)),
    ///     (Pure::R32, Ratio::new::<percent>(40.0)),
    /// ]))?;
    /// assert!(Fluid::try_from(unsupported_mix).is_err());
    /// # Ok::<(), rfluids::error::Error>(())
    /// ```
    fn try_from(value: CustomMix) -> Result<Self, Self::Error> {
        Substance::from(value).try_into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::substance::*;
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
                BinaryMix::with_fraction(kind, 0.5 * (kind.min_fraction() + kind.max_fraction()))
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
