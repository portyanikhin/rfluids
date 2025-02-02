use super::requests::FluidUpdateRequest;
use super::Fluid;
use crate::error::FluidUpdateError;
use crate::io::{FluidInput, FluidTrivialParam};
use crate::state_variant::StateVariant;
use crate::substance::Substance;
use crate::uom::si::f64::{MassDensity, MolarConcentration, Pressure, ThermodynamicTemperature};
use crate::uom::si::mass_density::kilogram_per_cubic_meter;
use crate::uom::si::molar_concentration::mole_per_cubic_meter;
use crate::uom::si::pressure::pascal;
use crate::uom::si::thermodynamic_temperature::kelvin;
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

    /// Critical point molar density
    /// _(key: [`DMolarCritical`](FluidTrivialParam::DMolarCritical), SI units: mol/m³)_.
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
    /// assert_relative_eq!(
    ///     water.critical_molar_density().unwrap().value,
    ///     17873.72799560906
    /// );
    ///
    /// let mut r444a = Fluid::from(PredefinedMix::R444A);
    /// assert!(r444a.critical_molar_density().is_none());
    /// ```
    pub fn critical_molar_density(&mut self) -> Option<MolarConcentration> {
        // Due to CoolProp freeze
        if let Substance::PredefinedMix(_) = self.substance {
            return None;
        }
        Some(MolarConcentration::new::<mole_per_cubic_meter>(
            self.trivial_output(FluidTrivialParam::DMolarCritical)?,
        ))
    }

    /// Critical point pressure
    /// _(key: [`PCritical`](FluidTrivialParam::PCritical), SI units: Pa)_.
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
    /// assert_relative_eq!(water.critical_pressure().unwrap().value, 22.064e6);
    ///
    /// let mut r444a = Fluid::from(PredefinedMix::R444A);
    /// assert!(r444a.critical_pressure().is_none());
    /// ```
    pub fn critical_pressure(&mut self) -> Option<Pressure> {
        // Due to CoolProp freeze
        if let Substance::PredefinedMix(_) = self.substance {
            return None;
        }
        Some(Pressure::new::<pascal>(
            self.trivial_output(FluidTrivialParam::PCritical)?,
        ))
    }

    /// Critical point temperature
    /// _(key: [`TCritical`](FluidTrivialParam::TCritical), SI units: K)_.
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
    /// assert_relative_eq!(water.critical_temperature().unwrap().value, 647.096);
    ///
    /// let mut r444a = Fluid::from(PredefinedMix::R444A);
    /// assert!(r444a.critical_temperature().is_none());
    /// ```
    pub fn critical_temperature(&mut self) -> Option<ThermodynamicTemperature> {
        // Due to CoolProp freeze
        if let Substance::PredefinedMix(_) = self.substance {
            return None;
        }
        Some(ThermodynamicTemperature::new::<kelvin>(
            self.trivial_output(FluidTrivialParam::TCritical)?,
        ))
    }

    /// Flammability hazard index
    /// _(key: [`FH`](FluidTrivialParam::FH), dimensionless)_.
    ///
    /// If it's not available for the specified substance, returns [`None`].
    ///
    /// # Examples
    ///
    /// ```
    /// use rfluids::prelude::fluid::*;
    /// use rfluids::uom::si::ratio::percent;
    ///
    /// let mut acetone = Fluid::from(Pure::Acetone);
    /// assert_eq!(acetone.flammability_hazard().unwrap(), 3.0);
    ///
    /// let mut water = Fluid::from(Pure::Water);
    /// assert_eq!(water.flammability_hazard().unwrap(), 0.0);
    ///
    /// let mut propylene_glycol = Fluid::from(
    ///     BinaryMix::try_from(BinaryMixKind::MPG, Ratio::new::<percent>(40.0)).unwrap(),
    /// );
    /// assert!(propylene_glycol.flammability_hazard().is_none());
    /// ```
    pub fn flammability_hazard(&mut self) -> Option<f64> {
        self.trivial_output(FluidTrivialParam::FH)
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
