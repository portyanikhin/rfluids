//! Thermophysical properties of substances.

use crate::io::{FluidParam, FluidTrivialParam};
use crate::native::AbstractState;
use crate::substance::*;
use crate::UndefinedState;
use std::collections::HashMap;
use std::marker::PhantomData;

/// Provider of thermophysical properties of substances.
///
/// It works with [`Substance`] or any of its subsets:
///
/// - pure or pseudo-pure substances _([`Pure`])_;
/// - incompressible pure substances _([`IncompPure`])_;
/// - refrigerants _([`Refrigerant`])_;
/// - predefined mixtures _([`PredefinedMix`])_;
/// - incompressible binary mixtures _([`BinaryMix`])_.
///
/// It implements the [typestate pattern](https://en.wikipedia.org/wiki/Typestate_analysis)
/// and has one generic type parameter `S` _(state type, [`UndefinedState`] or
/// [`DefinedState`](crate::DefinedState))_.
///
/// Depending on `S`, the `Fluid` instance has different functionality.
#[derive(Debug)]
pub struct Fluid<S = UndefinedState> {
    /// Substance.
    pub substance: Substance,
    backend: AbstractState,
    trivial_outputs_cache: HashMap<FluidTrivialParam, f64>,
    outputs_cache: HashMap<FluidParam, f64>,
    state: PhantomData<S>,
}

impl<T: Into<Substance>> From<T> for Fluid<UndefinedState> {
    fn from(value: T) -> Self {
        let substance = value.into();
        Self {
            substance,
            backend: AbstractState::new(substance.backend_name(), substance).unwrap(),
            trivial_outputs_cache: HashMap::new(),
            outputs_cache: HashMap::new(),
            state: PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
    fn from_each_binary_mix_does_not_panic() {
        for kind in BinaryMixKind::iter() {
            let _fluid = Fluid::from(
                BinaryMix::try_new(kind, 0.5 * (kind.min_fraction() + kind.max_fraction()))
                    .unwrap(),
            );
        }
    }
}
