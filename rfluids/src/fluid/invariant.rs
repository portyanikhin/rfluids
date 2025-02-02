use super::requests::FluidUpdateRequest;
use super::Fluid;
use crate::error::FluidUpdateError;
use crate::io::{FluidInput, FluidTrivialParam};
use crate::state_variant::StateVariant;
use crate::substance::Substance;
use crate::uom::si::f64::MassDensity;
use crate::uom::si::mass_density::kilogram_per_cubic_meter;
use std::collections::hash_map::Entry;

impl<S: StateVariant> Fluid<S> {
    /// Specified substance.
    pub fn substance(&self) -> &Substance {
        &self.substance
    }

    /// Acentric factor
    /// _(key: [`AcentricFactor`](FluidTrivialParam::AcentricFactor), dimensionless)_.
    ///
    /// If it's not available for the specified substance, returns [`None`].
    ///
    /// # Examples
    ///
    /// ```
    /// use approx::assert_relative_eq;
    /// use rfluids::prelude::fluid::*;
    ///
    /// let mut water = Fluid::from(Pure::Water);
    /// assert!(water.acentric_factor().is_some());
    /// assert_relative_eq!(water.acentric_factor().unwrap(), 0.3442920843);
    ///
    /// let mut r444a = Fluid::from(PredefinedMix::R444A);
    /// assert!(r444a.acentric_factor().is_none());
    /// ```
    pub fn acentric_factor(&mut self) -> Option<f64> {
        self.trivial_output(FluidTrivialParam::AcentricFactor)
    }

    /// Critical point mass density
    /// _(key: [`DMassCritical`](FluidTrivialParam::DMassCritical), SI units: kg/m³)_.
    ///
    /// If it's not available for the specified substance, returns [`None`].
    ///
    /// # Examples
    ///
    /// ```
    /// use approx::assert_relative_eq;
    /// use rfluids::prelude::fluid::*;
    ///
    /// let mut water = Fluid::from(Pure::Water);
    /// assert!(water.critical_density().is_some());
    /// assert_relative_eq!(water.critical_density().unwrap().value, 322.0);
    ///
    /// let mut r444a = Fluid::from(PredefinedMix::R444A);
    /// assert!(r444a.critical_density().is_none());
    /// ```
    pub fn critical_density(&mut self) -> Option<MassDensity> {
        // Due to CoolProp freeze
        if let Substance::PredefinedMix(_) = self.substance {
            return None;
        }
        Some(MassDensity::new::<kilogram_per_cubic_meter>(
            self.trivial_output(FluidTrivialParam::DMassCritical)?,
        ))
    }

    pub(crate) fn trivial_output(&mut self, key: FluidTrivialParam) -> Option<f64> {
        match self.trivial_outputs.entry(key) {
            Entry::Occupied(entry) => *entry.get(),
            Entry::Vacant(entry) => *entry.insert(self.backend.keyed_output(key).ok()),
        }
    }

    pub(crate) fn inner_update(
        &mut self,
        input1: FluidInput,
        input2: FluidInput,
    ) -> Result<(), FluidUpdateError> {
        let request: FluidUpdateRequest = (input1, input2).try_into()?;
        self.backend.update(request.0, request.1, request.2)?;
        self.outputs.clear();
        self.outputs.insert(input1.key(), input1.si_value());
        self.outputs.insert(input2.key(), input2.si_value());
        self.update_request = Some(request);
        Ok(())
    }
}
