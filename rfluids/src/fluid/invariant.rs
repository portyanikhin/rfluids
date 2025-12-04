use super::{
    Fluid, FluidOutputError, OutputResult, StateResult,
    common::{cached_output, guard},
    request::FluidUpdateRequest,
};
use crate::{
    io::{FluidInput, FluidTrivialParam},
    ops::mul,
    state_variant::StateVariant,
    substance::{BackendName, Substance},
};

impl<S: StateVariant> Fluid<S> {
    /// Specified substance.
    #[must_use]
    pub fn substance(&self) -> &Substance {
        &self.substance
    }

    /// `CoolProp` backend name.
    #[must_use]
    pub fn backend_name(&self) -> &str {
        self.custom_backend_name.as_deref().unwrap_or_else(|| self.substance.backend_name())
    }

    /// Acentric factor **\[dimensionless\]**.
    ///
    /// # Errors
    ///
    /// Returns a [`FluidOutputError`] if the property is not available for the specified substance.
    pub fn acentric_factor(&mut self) -> OutputResult<f64> {
        self.trivial_output(FluidTrivialParam::AcentricFactor)
    }

    /// Critical point mass density **\[kg/m³\]**.
    ///
    /// # Errors
    ///
    /// Returns a [`FluidOutputError`] if the property is not available for the specified substance.
    pub fn critical_density(&mut self) -> OutputResult<f64> {
        let key = FluidTrivialParam::DMassCritical;
        // Due to CoolProp freeze
        if let Substance::PredefinedMix(_) = self.substance {
            return Err(FluidOutputError::UnavailableTrivialOutput(key));
        }
        self.positive_trivial_output(key)
    }

    /// Critical point molar density **\[mol/m³\]**.
    ///
    /// # Errors
    ///
    /// Returns a [`FluidOutputError`] if the property is not available for the specified substance.
    pub fn critical_molar_density(&mut self) -> OutputResult<f64> {
        let key = FluidTrivialParam::DMolarCritical;
        // Due to CoolProp freeze
        if let Substance::PredefinedMix(_) = self.substance {
            return Err(FluidOutputError::UnavailableTrivialOutput(key));
        }
        self.positive_trivial_output(key)
    }

    /// Critical point pressure **\[Pa\]**.
    ///
    /// # Errors
    ///
    /// Returns a [`FluidOutputError`] if the property is not available for the specified substance.
    pub fn critical_pressure(&mut self) -> OutputResult<f64> {
        let key = FluidTrivialParam::PCritical;
        // Due to CoolProp freeze
        if let Substance::PredefinedMix(_) = self.substance {
            return Err(FluidOutputError::UnavailableTrivialOutput(key));
        }
        self.positive_trivial_output(key)
    }

    /// Critical point temperature **\[K\]**.
    ///
    /// # Errors
    ///
    /// Returns a [`FluidOutputError`] if the property is not available for the specified substance.
    pub fn critical_temperature(&mut self) -> OutputResult<f64> {
        let key = FluidTrivialParam::TCritical;
        // Due to CoolProp freeze
        if let Substance::PredefinedMix(_) = self.substance {
            return Err(FluidOutputError::UnavailableTrivialOutput(key));
        }
        self.positive_trivial_output(key)
    }

    /// Flammability hazard index **\[dimensionless\]**.
    ///
    /// # Errors
    ///
    /// Returns a [`FluidOutputError`] if the property is not available for the specified substance.
    pub fn flammability_hazard(&mut self) -> OutputResult<f64> {
        self.non_negative_trivial_output(FluidTrivialParam::FH)
    }

    /// Freezing temperature for incompressible mixtures **\[K\]**.
    ///
    /// # Errors
    ///
    /// Returns a [`FluidOutputError`] if the property is not available for the specified substance.
    pub fn freezing_temperature(&mut self) -> OutputResult<f64> {
        self.positive_trivial_output(FluidTrivialParam::TFreeze)
    }

    /// 20-year global warming potential **\[dimensionless\]**.
    ///
    /// # Errors
    ///
    /// Returns a [`FluidOutputError`] if the property is not available for the specified substance.
    pub fn gwp20(&mut self) -> OutputResult<f64> {
        self.non_negative_trivial_output(FluidTrivialParam::GWP20)
    }

    /// 100-year global warming potential **\[dimensionless\]**.
    ///
    /// # Errors
    ///
    /// Returns a [`FluidOutputError`] if the property is not available for the specified substance.
    pub fn gwp100(&mut self) -> OutputResult<f64> {
        self.non_negative_trivial_output(FluidTrivialParam::GWP100)
    }

    /// 500-year global warming potential **\[dimensionless\]**.
    ///
    /// # Errors
    ///
    /// Returns a [`FluidOutputError`] if the property is not available for the specified substance.
    pub fn gwp500(&mut self) -> OutputResult<f64> {
        self.non_negative_trivial_output(FluidTrivialParam::GWP500)
    }

    /// Health hazard index **\[dimensionless\]**.
    ///
    /// # Errors
    ///
    /// Returns a [`FluidOutputError`] if the property is not available for the specified substance.
    pub fn health_hazard(&mut self) -> OutputResult<f64> {
        self.non_negative_trivial_output(FluidTrivialParam::HH)
    }

    /// Maximum pressure **\[Pa\]**.
    ///
    /// # Errors
    ///
    /// Returns a [`FluidOutputError`] if the property is not available for the specified substance.
    pub fn max_pressure(&mut self) -> OutputResult<f64> {
        self.positive_trivial_output(FluidTrivialParam::PMax)
    }

    /// Maximum temperature **\[K\]**.
    pub fn max_temperature(&mut self) -> f64 {
        self.positive_trivial_output(FluidTrivialParam::TMax).unwrap()
    }

    /// Minimum pressure **\[Pa\]**.
    ///
    /// # Errors
    ///
    /// Returns a [`FluidOutputError`] if the property is not available for the specified substance.
    pub fn min_pressure(&mut self) -> OutputResult<f64> {
        self.positive_trivial_output(FluidTrivialParam::PMin)
    }

    /// Minimum temperature **\[K\]**.
    pub fn min_temperature(&mut self) -> f64 {
        self.positive_trivial_output(FluidTrivialParam::TMin).unwrap()
    }

    /// Molar mass **\[kg/mol\]**.
    ///
    /// # Errors
    ///
    /// Returns a [`FluidOutputError`] if the property is not available for the specified substance.
    pub fn molar_mass(&mut self) -> OutputResult<f64> {
        self.positive_trivial_output(FluidTrivialParam::MolarMass)
    }

    /// Ozone depletion potential **\[dimensionless\]**.
    ///
    /// # Errors
    ///
    /// Returns a [`FluidOutputError`] if the property is not available for the specified substance.
    pub fn odp(&mut self) -> OutputResult<f64> {
        self.non_negative_trivial_output(FluidTrivialParam::ODP)
    }

    /// Physical hazard index **\[dimensionless\]**.
    ///
    /// # Errors
    ///
    /// Returns a [`FluidOutputError`] if the property is not available for the specified substance.
    pub fn physical_hazard(&mut self) -> OutputResult<f64> {
        self.non_negative_trivial_output(FluidTrivialParam::PH)
    }

    /// Reducing point mass density **\[kg/m³\]**.
    ///
    /// # Errors
    ///
    /// Returns a [`FluidOutputError`] if the property is not available for the specified substance.
    pub fn reducing_density(&mut self) -> OutputResult<f64> {
        mul(self.reducing_molar_density(), self.molar_mass())
    }

    /// Reducing point molar density **\[mol/m³\]**.
    ///
    /// # Errors
    ///
    /// Returns a [`FluidOutputError`] if the property is not available for the specified substance.
    pub fn reducing_molar_density(&mut self) -> OutputResult<f64> {
        self.positive_trivial_output(FluidTrivialParam::DMolarReducing)
    }

    /// Reducing point pressure **\[Pa\]**.
    ///
    /// # Errors
    ///
    /// Returns a [`FluidOutputError`] if the property is not available for the specified substance.
    pub fn reducing_pressure(&mut self) -> OutputResult<f64> {
        self.positive_trivial_output(FluidTrivialParam::PReducing)
    }

    /// Reducing point temperature **\[K\]**.
    ///
    /// # Errors
    ///
    /// Returns a [`FluidOutputError`] if the property is not available for the specified substance.
    pub fn reducing_temperature(&mut self) -> OutputResult<f64> {
        self.positive_trivial_output(FluidTrivialParam::TReducing)
    }

    /// Triple point pressure **\[Pa\]**.
    ///
    /// # Errors
    ///
    /// Returns a [`FluidOutputError`] if the property is not available for the specified substance.
    pub fn triple_pressure(&mut self) -> OutputResult<f64> {
        self.positive_trivial_output(FluidTrivialParam::PTriple)
    }

    /// Triple point temperature **\[K\]**.
    ///
    /// # Errors
    ///
    /// Returns a [`FluidOutputError`] if the property is not available for the specified substance.
    pub fn triple_temperature(&mut self) -> OutputResult<f64> {
        self.positive_trivial_output(FluidTrivialParam::TTriple)
    }

    pub(crate) fn inner_update(
        &mut self,
        input1: FluidInput,
        input2: FluidInput,
    ) -> StateResult<()> {
        let request: FluidUpdateRequest = (input1, input2).try_into()?;
        self.backend.update(request.input_pair, request.value1, request.value2)?;
        self.outputs.clear();
        self.outputs.insert(input1.key, Ok(input1.value));
        self.outputs.insert(input2.key, Ok(input2.value));
        self.update_request = Some(request);
        Ok(())
    }

    fn positive_trivial_output(&mut self, key: FluidTrivialParam) -> OutputResult<f64> {
        self.trivial_output(key).and_then(|value| guard(key.into(), value, |x| x > 0.0))
    }

    fn non_negative_trivial_output(&mut self, key: FluidTrivialParam) -> OutputResult<f64> {
        self.trivial_output(key).and_then(|value| guard(key.into(), value, |x| x >= 0.0))
    }

    fn trivial_output(&mut self, key: FluidTrivialParam) -> OutputResult<f64> {
        cached_output(&mut self.trivial_outputs, &mut self.backend, key, |_| {
            FluidOutputError::UnavailableTrivialOutput(key)
        })
        .and_then(|value| guard(key.into(), value, f64::is_finite))
    }
}
