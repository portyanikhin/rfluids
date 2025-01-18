//! Thermophysical properties of substances.

use crate::error::CoolPropError;
use crate::io::{FluidParam, FluidTrivialParam};
use crate::native::AbstractState;
use crate::substance::*;
use crate::{Remember, UndefinedState};
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::hash::Hash;
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
    trivial_outputs: HashMap<FluidTrivialParam, f64>,
    outputs: HashMap<FluidParam, f64>,
    state: PhantomData<S>,
}

impl<T: Into<Substance>> From<T> for Fluid<UndefinedState> {
    fn from(value: T) -> Self {
        let substance = value.into();
        Self {
            substance,
            backend: AbstractState::new(substance.backend_name(), substance).unwrap(),
            trivial_outputs: HashMap::new(),
            outputs: HashMap::new(),
            state: PhantomData,
        }
    }
}

impl<K> Remember<&AbstractState, K> for HashMap<K, f64>
where
    K: Into<u8> + Copy + Eq + Hash,
{
    type Error = CoolPropError;

    fn remember(&mut self, src: &AbstractState, key: K) -> Result<f64, CoolPropError> {
        Ok(match self.entry(key) {
            Entry::Occupied(entry) => *entry.get(),
            Entry::Vacant(entry) => *entry.insert(src.keyed_output(key)?),
        })
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
