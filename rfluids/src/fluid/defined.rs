// cSpell:disable

use super::{
    Fluid, FluidOutputError, OutputResult, StateResult,
    common::{cached_output, guard},
};
use crate::{
    io::{FluidInput, FluidParam, Phase},
    ops::div,
};

impl Fluid {
    /// Ideal gas Helmholtz energy contribution **\[dimensionless\]**.
    ///
    /// # Errors
    ///
    /// Returns a [`FluidOutputError`] if the property is not available or calculation fails.
    pub fn alpha0(&mut self) -> OutputResult<f64> {
        self.output(FluidParam::Alpha0)
    }

    /// Residual Helmholtz energy contribution **\[dimensionless\]**.
    ///
    /// # Errors
    ///
    /// Returns a [`FluidOutputError`] if the property is not available or calculation fails.
    pub fn alphar(&mut self) -> OutputResult<f64> {
        self.output(FluidParam::AlphaR)
    }

    /// Second virial coefficient **\[dimensionless\]**.
    ///
    /// # Errors
    ///
    /// Returns a [`FluidOutputError`] if the property is not available or calculation fails.
    pub fn bvirial(&mut self) -> OutputResult<f64> {
        self.output(FluidParam::BVirial)
    }

    /// Compressibility factor **\[dimensionless\]**.
    ///
    /// # Errors
    ///
    /// Returns a [`FluidOutputError`] if the property is not available or calculation fails.
    pub fn compressibility(&mut self) -> OutputResult<f64> {
        self.positive_output(FluidParam::Z)
    }

    /// Thermal conductivity **\[W/m/K\]**.
    ///
    /// # Errors
    ///
    /// Returns a [`FluidOutputError`] if the property is not available or calculation fails.
    pub fn conductivity(&mut self) -> OutputResult<f64> {
        self.positive_output(FluidParam::Conductivity)
    }

    /// Third virial coefficient **\[dimensionless\]**.
    ///
    /// # Errors
    ///
    /// Returns a [`FluidOutputError`] if the property is not available or calculation fails.
    pub fn cvirial(&mut self) -> OutputResult<f64> {
        self.output(FluidParam::CVirial)
    }

    /// Derivative of ideal gas Helmholtz energy contribution
    /// with [`delta`](crate::fluid::Fluid::delta) **\[dimensionless\]**.
    ///
    /// # Errors
    ///
    /// Returns a [`FluidOutputError`] if the property is not available or calculation fails.
    pub fn dalpha0_ddelta_const_tau(&mut self) -> OutputResult<f64> {
        self.output(FluidParam::DAlpha0DDeltaConstTau)
    }

    /// Second derivative of ideal gas Helmholtz energy contribution
    /// with [`delta`](crate::fluid::Fluid::delta) **\[dimensionless\]**.
    ///
    /// # Errors
    ///
    /// Returns a [`FluidOutputError`] if the property is not available or calculation fails.
    pub fn d2alpha0_ddelta2_const_tau(&mut self) -> OutputResult<f64> {
        self.output(FluidParam::D2Alpha0DDelta2ConstTau)
    }

    /// Third derivative of ideal gas Helmholtz energy contribution
    /// with [`delta`](crate::fluid::Fluid::delta) **\[dimensionless\]**.
    ///
    /// # Errors
    ///
    /// Returns a [`FluidOutputError`] if the property is not available or calculation fails.
    pub fn d3alpha0_ddelta3_const_tau(&mut self) -> OutputResult<f64> {
        self.output(FluidParam::D3Alpha0DDelta3ConstTau)
    }

    /// Derivative of ideal gas Helmholtz energy contribution
    /// with [`tau`](crate::fluid::Fluid::tau) **\[dimensionless\]**.
    ///
    /// # Errors
    ///
    /// Returns a [`FluidOutputError`] if the property is not available or calculation fails.
    pub fn dalpha0_dtau_const_delta(&mut self) -> OutputResult<f64> {
        self.output(FluidParam::DAlpha0DTauConstDelta)
    }

    /// Derivative of residual Helmholtz energy contribution
    /// with [`delta`](crate::fluid::Fluid::delta) **\[dimensionless\]**.
    ///
    /// # Errors
    ///
    /// Returns a [`FluidOutputError`] if the property is not available or calculation fails.
    pub fn dalphar_ddelta_const_tau(&mut self) -> OutputResult<f64> {
        self.output(FluidParam::DAlphaRDDeltaConstTau)
    }

    /// Derivative of residual Helmholtz energy contribution
    /// with [`tau`](crate::fluid::Fluid::tau) **\[dimensionless\]**.
    ///
    /// # Errors
    ///
    /// Returns a [`FluidOutputError`] if the property is not available or calculation fails.
    pub fn dalphar_dtau_const_delta(&mut self) -> OutputResult<f64> {
        self.output(FluidParam::DAlphaRDTauConstDelta)
    }

    /// Derivative of second virial coefficient
    /// with [`temperature`](crate::fluid::Fluid::temperature) **\[dimensionless\]**.
    ///
    /// # Errors
    ///
    /// Returns a [`FluidOutputError`] if the property is not available or calculation fails.
    pub fn dbvirial_dt(&mut self) -> OutputResult<f64> {
        self.output(FluidParam::DBVirialDT)
    }

    /// Derivative of third virial coefficient with
    /// [`temperature`](crate::fluid::Fluid::temperature) **\[dimensionless\]**.
    ///
    /// # Errors
    ///
    /// Returns a [`FluidOutputError`] if the property is not available or calculation fails.
    pub fn dcvirial_dt(&mut self) -> OutputResult<f64> {
        self.output(FluidParam::DCVirialDT)
    }

    /// Reduced density = [`density`](crate::fluid::Fluid::density) /
    /// [`critical_density`](crate::fluid::Fluid::critical_density) **\[dimensionless\]**.
    ///
    /// # Errors
    ///
    /// Returns a [`FluidOutputError`] if the property is not available or calculation fails.
    pub fn delta(&mut self) -> OutputResult<f64> {
        div(self.density(), self.critical_density())
    }

    /// Mass density **\[kg/m³\]**.
    ///
    /// # Errors
    ///
    /// Returns a [`FluidOutputError`] if the property is not available or calculation fails.
    pub fn density(&mut self) -> OutputResult<f64> {
        self.positive_output(FluidParam::DMass)
    }

    /// Dynamic viscosity **\[Pa·s\]**.
    ///
    /// # Errors
    ///
    /// Returns a [`FluidOutputError`] if the property is not available or calculation fails.
    pub fn dynamic_viscosity(&mut self) -> OutputResult<f64> {
        self.positive_output(FluidParam::DynamicViscosity)
    }

    /// Mass specific enthalpy **\[J/kg\]**.
    ///
    /// # Errors
    ///
    /// Returns a [`FluidOutputError`] if the property is not available or calculation fails.
    pub fn enthalpy(&mut self) -> OutputResult<f64> {
        self.output(FluidParam::HMass)
    }

    /// Mass specific entropy **\[J/kg/K\]**.
    ///
    /// # Errors
    ///
    /// Returns a [`FluidOutputError`] if the property is not available or calculation fails.
    pub fn entropy(&mut self) -> OutputResult<f64> {
        self.output(FluidParam::SMass)
    }

    /// Fundamental derivative of gas dynamics **\[dimensionless\]**.
    ///
    /// # Errors
    ///
    /// Returns a [`FluidOutputError`] if the property is not available or calculation fails.
    pub fn fundamental_derivative_of_gas_dynamics(&mut self) -> OutputResult<f64> {
        self.output(FluidParam::FundamentalDerivativeOfGasDynamics)
    }

    /// Mass specific Gibbs energy **\[J/kg\]**.
    ///
    /// # Errors
    ///
    /// Returns a [`FluidOutputError`] if the property is not available or calculation fails.
    pub fn gibbs_energy(&mut self) -> OutputResult<f64> {
        self.output(FluidParam::GMass)
    }

    /// Mass specific Helmholtz energy **\[J/kg\]**.
    ///
    /// # Errors
    ///
    /// Returns a [`FluidOutputError`] if the property is not available or calculation fails.
    pub fn helmholtz_energy(&mut self) -> OutputResult<f64> {
        self.output(FluidParam::HelmholtzMass)
    }

    /// Ideal gas molar specific heat at constant pressure
    /// **\[J/mol/K\]**.
    ///
    /// # Errors
    ///
    /// Returns a [`FluidOutputError`] if the property is not available or calculation fails.
    pub fn ideal_gas_molar_specific_heat(&mut self) -> OutputResult<f64> {
        self.positive_output(FluidParam::Cp0Molar)
    }

    /// Ideal gas mass specific heat at constant pressure
    /// **\[J/kg/K\]**.
    ///
    /// # Errors
    ///
    /// Returns a [`FluidOutputError`] if the property is not available or calculation fails.
    pub fn ideal_gas_specific_heat(&mut self) -> OutputResult<f64> {
        self.positive_output(FluidParam::Cp0Mass)
    }

    /// Mass specific internal energy **\[J/kg\]**.
    ///
    /// # Errors
    ///
    /// Returns a [`FluidOutputError`] if the property is not available or calculation fails.
    pub fn internal_energy(&mut self) -> OutputResult<f64> {
        self.output(FluidParam::UMass)
    }

    /// Isentropic expansion coefficient **\[dimensionless\]**.
    ///
    /// # Errors
    ///
    /// Returns a [`FluidOutputError`] if the property is not available or calculation fails.
    pub fn isentropic_expansion_coefficient(&mut self) -> OutputResult<f64> {
        self.output(FluidParam::IsentropicExpansionCoefficient)
    }

    /// Isobaric expansion coefficient **\[1/K\]**.
    ///
    /// # Errors
    ///
    /// Returns a [`FluidOutputError`] if the property is not available or calculation fails.
    pub fn isobaric_expansion_coefficient(&mut self) -> OutputResult<f64> {
        self.output(FluidParam::IsobaricExpansionCoefficient)
    }

    /// Isothermal compressibility **\[1/Pa\]**.
    ///
    /// # Errors
    ///
    /// Returns a [`FluidOutputError`] if the property is not available or calculation fails.
    pub fn isothermal_compressibility(&mut self) -> OutputResult<f64> {
        self.output(FluidParam::IsothermalCompressibility)
    }

    /// Kinematic viscosity =
    /// [`dynamic_viscosity`](crate::fluid::Fluid::dynamic_viscosity) /
    /// [`density`](crate::fluid::Fluid::density) **\[m²/s\]**.
    ///
    /// # Errors
    ///
    /// Returns a [`FluidOutputError`] if the property is not available or calculation fails.
    pub fn kinematic_viscosity(&mut self) -> OutputResult<f64> {
        div(self.dynamic_viscosity(), self.density())
    }

    /// Molar density **\[mol/m³\]**.
    ///
    /// # Errors
    ///
    /// Returns a [`FluidOutputError`] if the property is not available or calculation fails.
    pub fn molar_density(&mut self) -> OutputResult<f64> {
        self.positive_output(FluidParam::DMolar)
    }

    /// Molar specific enthalpy **\[J/mol\]**.
    ///
    /// # Errors
    ///
    /// Returns a [`FluidOutputError`] if the property is not available or calculation fails.
    pub fn molar_enthalpy(&mut self) -> OutputResult<f64> {
        self.output(FluidParam::HMolar)
    }

    /// Molar specific entropy **\[J/mol/K\]**.
    ///
    /// # Errors
    ///
    /// Returns a [`FluidOutputError`] if the property is not available or calculation fails.
    pub fn molar_entropy(&mut self) -> OutputResult<f64> {
        self.output(FluidParam::SMolar)
    }

    /// Molar specific Gibbs energy **\[J/mol\]**.
    ///
    /// # Errors
    ///
    /// Returns a [`FluidOutputError`] if the property is not available or calculation fails.
    pub fn molar_gibbs_energy(&mut self) -> OutputResult<f64> {
        self.output(FluidParam::GMolar)
    }

    /// Molar specific Helmholtz energy **\[J/mol\]**.
    ///
    /// # Errors
    ///
    /// Returns a [`FluidOutputError`] if the property is not available or calculation fails.
    pub fn molar_helmholtz_energy(&mut self) -> OutputResult<f64> {
        self.output(FluidParam::HelmholtzMolar)
    }

    /// Molar specific internal energy **\[J/mol\]**.
    ///
    /// # Errors
    ///
    /// Returns a [`FluidOutputError`] if the property is not available or calculation fails.
    pub fn molar_internal_energy(&mut self) -> OutputResult<f64> {
        self.output(FluidParam::UMolar)
    }

    /// Molar specific heat at constant pressure **\[J/mol/K\]**.
    ///
    /// # Errors
    ///
    /// Returns a [`FluidOutputError`] if the property is not available or calculation fails.
    pub fn molar_specific_heat(&mut self) -> OutputResult<f64> {
        self.positive_output(FluidParam::CpMolar)
    }

    /// Molar specific heat at constant volume **\[J/mol/K\]**.
    ///
    /// # Errors
    ///
    /// Returns a [`FluidOutputError`] if the property is not available or calculation fails.
    pub fn molar_specific_heat_const_volume(&mut self) -> OutputResult<f64> {
        self.positive_output(FluidParam::CvMolar)
    }

    /// Phase state.
    pub fn phase(&mut self) -> Phase {
        Phase::try_from(
            self.non_negative_output(FluidParam::Phase)
                .unwrap_or(f64::from(u8::from(Phase::Unknown))),
        )
        .unwrap()
    }

    /// Phase identification parameter **\[dimensionless\]**.
    ///
    /// # Errors
    ///
    /// Returns a [`FluidOutputError`] if the property is not available or calculation fails.
    pub fn phase_id_param(&mut self) -> OutputResult<f64> {
        self.output(FluidParam::PIP)
    }

    /// Prandtl number **\[dimensionless\]**.
    ///
    /// # Errors
    ///
    /// Returns a [`FluidOutputError`] if the property is not available or calculation fails.
    pub fn prandtl(&mut self) -> OutputResult<f64> {
        self.positive_output(FluidParam::Prandtl)
    }

    /// Pressure **\[Pa\]**.
    ///
    /// # Errors
    ///
    /// Returns a [`FluidOutputError`] if the property is not available or calculation fails.
    pub fn pressure(&mut self) -> OutputResult<f64> {
        self.positive_output(FluidParam::P)
    }

    /// Vapor quality **\[dimensionless, from 0 to 1\]**.
    ///
    /// # Errors
    ///
    /// Returns a [`FluidOutputError`] if the property is not available or calculation fails.
    pub fn quality(&mut self) -> OutputResult<f64> {
        let key = FluidParam::Q;
        self.output(key).and_then(|value| guard(key.into(), value, |x| (0.0..=1.0).contains(&x)))
    }

    /// Residual molar specific enthalpy **\[J/mol\]**.
    ///
    /// # Errors
    ///
    /// Returns a [`FluidOutputError`] if the property is not available or calculation fails.
    pub fn residual_molar_enthalpy(&mut self) -> OutputResult<f64> {
        self.output(FluidParam::HMolarResidual)
    }

    /// Residual molar specific entropy **\[J/mol/K\]**.
    ///
    /// # Errors
    ///
    /// Returns a [`FluidOutputError`] if the property is not available or calculation fails.
    pub fn residual_molar_entropy(&mut self) -> OutputResult<f64> {
        self.output(FluidParam::SMolarResidual)
    }

    /// Residual molar specific Gibbs energy **\[J/mol\]**.
    ///
    /// # Errors
    ///
    /// Returns a [`FluidOutputError`] if the property is not available or calculation fails.
    pub fn residual_molar_gibbs_energy(&mut self) -> OutputResult<f64> {
        self.output(FluidParam::GMolarResidual)
    }

    /// Sound speed **\[m/s\]**.
    ///
    /// # Errors
    ///
    /// Returns a [`FluidOutputError`] if the property is not available or calculation fails.
    pub fn sound_speed(&mut self) -> OutputResult<f64> {
        self.positive_output(FluidParam::SoundSpeed)
    }

    /// Mass specific heat at constant pressure **\[J/kg/K\]**.
    ///
    /// # Errors
    ///
    /// Returns a [`FluidOutputError`] if the property is not available or calculation fails.
    pub fn specific_heat(&mut self) -> OutputResult<f64> {
        self.positive_output(FluidParam::CpMass)
    }

    /// Mass specific heat at constant volume **\[J/kg/K\]**.
    ///
    /// # Errors
    ///
    /// Returns a [`FluidOutputError`] if the property is not available or calculation fails.
    pub fn specific_heat_const_volume(&mut self) -> OutputResult<f64> {
        self.positive_output(FluidParam::CvMass)
    }

    /// Specific volume = `1` / [`density`](crate::fluid::Fluid::density) **\[m³/kg\]**.
    ///
    /// # Errors
    ///
    /// Returns a [`FluidOutputError`] if the property is not available or calculation fails.
    pub fn specific_volume(&mut self) -> OutputResult<f64> {
        self.density().map(|value| 1.0 / value)
    }

    /// Surface tension **\[N/m\]**.
    ///
    /// # Errors
    ///
    /// Returns a [`FluidOutputError`] if the property is not available or calculation fails.
    pub fn surface_tension(&mut self) -> OutputResult<f64> {
        self.positive_output(FluidParam::SurfaceTension)
    }

    /// Reciprocal reduced temperature =
    /// [`critical_temperature`](crate::fluid::Fluid::critical_temperature) /
    /// [`temperature`](crate::fluid::Fluid::temperature)
    /// **\[dimensionless\]**.
    ///
    /// # Errors
    ///
    /// Returns a [`FluidOutputError`] if the property is not available or calculation fails.
    pub fn tau(&mut self) -> OutputResult<f64> {
        // Due to CoolProp bug
        div(self.critical_temperature(), self.temperature())
    }

    /// Temperature **\[K\]**.
    ///
    /// # Errors
    ///
    /// Returns a [`FluidOutputError`] if the property is not available or calculation fails.
    pub fn temperature(&mut self) -> OutputResult<f64> {
        self.positive_output(FluidParam::T)
    }

    /// Updates the thermodynamic state in place and returns a mutable reference to itself.
    ///
    /// # Arguments
    ///
    /// - `input1` -- first input property
    /// - `input2` -- second input property
    ///
    /// # Errors
    ///
    /// Returns a [`FluidStateError`](crate::fluid::FluidStateError)
    /// for invalid/unsupported inputs or invalid state.
    ///
    /// # Examples
    ///
    /// ```
    /// use rfluids::{fluid::StateResult, prelude::*};
    ///
    /// // After creation, the `Fluid` instance has `Undefined` state variant
    /// let mut water: Fluid<Undefined> = Fluid::from(Pure::Water);
    ///
    /// // Calling `in_state` will move the initial value and
    /// // perform conversion between `Undefined` and `Defined` state variants
    /// // (since `Defined` is the default state variant, it can be omitted)
    /// let mut water: Fluid =
    ///     water.in_state(FluidInput::pressure(101_325.0), FluidInput::temperature(293.15))?;
    ///
    /// // The `Fluid` instance now has `Defined` state variant
    /// // and its thermodynamic state can be updated in place by calling `update`
    /// // (which returns a mutable reference to the instance)
    /// let same_water_in_new_state: StateResult<&mut Fluid> =
    ///     water.update(FluidInput::pressure(202_650.0), FluidInput::temperature(313.15));
    /// assert!(same_water_in_new_state.is_ok());
    ///
    /// // Calling `in_state` on `Fluid<Defined>` will return
    /// // a new instance in the specified thermodynamic state
    /// let new_water: StateResult<Fluid> =
    ///     water.in_state(FluidInput::pressure(405_300.0), FluidInput::temperature(353.15));
    /// assert!(new_water.is_ok());
    /// # Ok::<(), rfluids::Error>(())
    /// ```
    ///
    /// # See Also
    ///
    /// - [`Fluid::in_state`](crate::fluid::Fluid::in_state)
    pub fn update(&mut self, input1: FluidInput, input2: FluidInput) -> StateResult<&mut Self> {
        self.inner_update(input1, input2)?;
        Ok(self)
    }

    /// Returns a new instance in the specified thermodynamic state.
    ///
    /// # Arguments
    ///
    /// - `input1` -- first input property
    /// - `input2` -- second input property
    ///
    /// # Errors
    ///
    /// Returns a [`FluidStateError`](crate::fluid::FluidStateError)
    /// for invalid/unsupported inputs or invalid state.
    ///
    /// # Examples
    ///
    /// ```
    /// use rfluids::{fluid::StateResult, prelude::*};
    ///
    /// // After creation, the `Fluid` instance has `Undefined` state variant
    /// let mut water: Fluid<Undefined> = Fluid::from(Pure::Water);
    ///
    /// // Calling `in_state` will move the initial value and
    /// // perform conversion between `Undefined` and `Defined` state variants
    /// // (since `Defined` is the default state variant, it can be omitted)
    /// let mut water: Fluid =
    ///     water.in_state(FluidInput::pressure(101_325.0), FluidInput::temperature(293.15))?;
    ///
    /// // The `Fluid` instance now has `Defined` state variant
    /// // and its thermodynamic state can be updated in place by calling `update`
    /// // (which returns a mutable reference to the instance)
    /// let same_water_in_new_state: StateResult<&mut Fluid> =
    ///     water.update(FluidInput::pressure(202_650.0), FluidInput::temperature(313.15));
    /// assert!(same_water_in_new_state.is_ok());
    ///
    /// // Calling `in_state` on `Fluid<Defined>` will return
    /// // a new instance in the specified thermodynamic state
    /// let new_water: StateResult<Fluid> =
    ///     water.in_state(FluidInput::pressure(405_300.0), FluidInput::temperature(353.15));
    /// assert!(new_water.is_ok());
    /// # Ok::<(), rfluids::Error>(())
    /// ```
    ///
    /// # See Also
    ///
    /// - [`Fluid::update`](crate::fluid::Fluid::update)
    pub fn in_state(&self, input1: FluidInput, input2: FluidInput) -> StateResult<Self> {
        let mut fluid = Fluid::builder()
            .substance(self.substance.clone())
            .maybe_with_backend(self.custom_backend_name.clone())
            .build()
            .unwrap()
            .in_state(input1, input2)?;
        fluid.trivial_outputs.clone_from(&self.trivial_outputs);
        Ok(fluid)
    }

    fn positive_output(&mut self, key: FluidParam) -> OutputResult<f64> {
        self.output(key).and_then(|value| guard(key.into(), value, |x| x > 0.0))
    }

    fn non_negative_output(&mut self, key: FluidParam) -> OutputResult<f64> {
        self.output(key).and_then(|value| guard(key.into(), value, |x| x >= 0.0))
    }

    fn output(&mut self, key: FluidParam) -> OutputResult<f64> {
        cached_output(&mut self.outputs, &mut self.backend, key, |e| {
            FluidOutputError::CalculationFailed(key, e)
        })
        .and_then(|value| guard(key.into(), value, f64::is_finite))
    }
}

impl Clone for Fluid {
    fn clone(&self) -> Self {
        let inputs: (FluidInput, FluidInput) = self.update_request.unwrap().into();
        let mut fluid = Fluid::builder()
            .substance(self.substance.clone())
            .maybe_with_backend(self.custom_backend_name.clone())
            .build()
            .unwrap()
            .in_state(inputs.0, inputs.1)
            .unwrap();
        fluid.outputs.clone_from(&self.outputs);
        fluid.trivial_outputs.clone_from(&self.trivial_outputs);
        fluid
    }
}

impl PartialEq for Fluid {
    fn eq(&self, other: &Self) -> bool {
        self.substance == other.substance
            && self.backend_name() == other.backend_name()
            && self.update_request == other.update_request
    }
}

#[cfg(test)]
mod tests {
    use rstest::*;

    use super::*;
    use crate::{
        Undefined,
        fluid::FluidStateError,
        substance::*,
        test::{SutFactory, assert_relative_eq, test_output},
    };

    struct Context {
        pressure: FluidInput,
        temperature: FluidInput,
        water: Pure,
        incomp_water: IncompPure,
        r22: Pure,
        r32: Pure,
        r444a: PredefinedMix,
        pg: BinaryMix,
    }

    impl<P: Into<Fluid<Undefined>>> SutFactory<P> for Context {
        type Sut = Fluid;

        fn sut(&self, payload: P) -> Self::Sut {
            payload.into().in_state(self.temperature, self.pressure).unwrap()
        }
    }

    #[fixture]
    fn ctx() -> Context {
        Context {
            pressure: FluidInput::pressure(101_325.0),
            temperature: FluidInput::temperature(293.15),
            water: Pure::Water,
            incomp_water: IncompPure::Water,
            r22: Pure::R22,
            r32: Pure::R32,
            r444a: PredefinedMix::R444A,
            pg: BinaryMixKind::MPG.with_fraction(0.4).unwrap(),
        }
    }

    test_output!(acentric_factor, water: 0.344_292_084_3, r444a: Err);
    test_output!(critical_density, water: 322.0, r444a: Err, incomp_water: Err);
    test_output!(critical_molar_density, water: 17_873.727_995_609_06, r444a: Err, incomp_water: Err);
    test_output!(critical_pressure, water: 22.064e6, r444a: Err, incomp_water: Err);
    test_output!(critical_temperature, water: 647.096, r444a: Err, incomp_water: Err);
    test_output!(flammability_hazard, water: 0.0, incomp_water: Err);
    test_output!(freezing_temperature, pg: 252.581_754_953_058_38, water: Err);
    test_output!(gwp20, r32: 2330.0, water: Err);
    test_output!(gwp100, r32: 675.0, water: Err);
    test_output!(gwp500, r32: 205.0, water: Err);
    test_output!(health_hazard, water: 0.0, incomp_water: Err);
    test_output!(max_pressure, water: 1e9, incomp_water: Err);
    test_output!(max_temperature, water: 2e3, always_ok);
    test_output!(min_pressure, water: 611.654_800_896_868_4, incomp_water: Err);
    test_output!(min_temperature, water: 273.16, always_ok);
    test_output!(molar_mass, water: 0.018_015_268, incomp_water: Err);
    test_output!(odp, r22: 0.05, water: Err, incomp_water: Err);
    test_output!(physical_hazard, water: 0.0, incomp_water: Err);
    test_output!(reducing_density, water: 322.0, incomp_water: Err);
    test_output!(reducing_molar_density, water: 17_873.727_995_609_06, incomp_water: Err);
    test_output!(reducing_pressure, water: 22.064e6, incomp_water: Err);
    test_output!(reducing_temperature, water: 647.096, incomp_water: Err);
    test_output!(triple_pressure, water: 611.654_800_896_868_4, incomp_water: Err);
    test_output!(triple_temperature, water: 273.16, incomp_water: Err);
    test_output!(alpha0, water: 9.942_698_150_834_108, pg: Err);
    test_output!(alphar, water: -9.964_888_981_266_709, pg: Err);
    test_output!(bvirial, water: -0.001_357_832_070_614_953_6, pg: Err);
    test_output!(compressibility, water: 0.000_750_269_594_463_781_6, pg: Err);
    test_output!(conductivity, water: 0.598_012_355_523_438);
    test_output!(cvirial, water: -5.326_204_726_542_83e-6, pg: Err);
    test_output!(dalpha0_ddelta_const_tau, water: 0.322_578_334_415_914_1, pg: Err);
    test_output!(d2alpha0_ddelta2_const_tau, water: -0.104_056_781_834_545_31, pg: Err);
    test_output!(d3alpha0_ddelta3_const_tau, water: 0.067_132_926_737_735_53, pg: Err);
    test_output!(dalpha0_dtau_const_delta, water: 8.047_537_051_126_078, pg: Err);
    test_output!(dalphar_ddelta_const_tau, water: -0.322_336_313_699_769_06, pg: Err);
    test_output!(dalphar_dtau_const_delta, water: -7.766_583_487_846_833, pg: Err);
    test_output!(dbvirial_dt, water: 2.496_398_488_386_135_7e-5, pg: Err);
    test_output!(dcvirial_dt, water: 1.860_361_534_926_561e-7, pg: Err);
    test_output!(delta, water: 3.100_022_206_422_137, pg: Err);
    test_output!(density, water: 998.207_150_467_928_4);
    test_output!(dynamic_viscosity, water: 0.001_001_596_143_120_594_6);
    test_output!(enthalpy, water: 84_007.300_850_662_8);
    test_output!(entropy, water: 296.462_836_225_179_9);
    test_output!(fundamental_derivative_of_gas_dynamics, water: 3.515_654_313_772_814_5, pg: Err);
    test_output!(gibbs_energy, water: -2_900.779_588_748_779, pg: Err);
    test_output!(helmholtz_energy, water: -3_002.286_575_534_692, pg: Err);
    test_output!(ideal_gas_molar_specific_heat, water: 33.565_699_649_260_64, pg: Err);
    test_output!(ideal_gas_specific_heat, water: 1_863.180_700_351_537, pg: Err);
    test_output!(internal_energy, water: 83_905.793_863_876_88);
    test_output!(isentropic_expansion_coefficient, water: 21_647.280_169_592_654, pg: Err);
    test_output!(isobaric_expansion_coefficient, water: 2.068_062_073_013_346_5e-4, pg: Err);
    test_output!(isothermal_compressibility, water: 4.589_128_995_632_698_5e-10, pg: Err);
    test_output!(kinematic_viscosity, water: 1.003_395_079_519_393_9e-6);
    test_output!(molar_density, water: 55_408.953_697_937_126, pg: Err);
    test_output!(molar_enthalpy, water: 1_513.414_038_781_318_4, pg: Err);
    test_output!(molar_entropy, water: 5.340_857_446_636_725, pg: Err);
    test_output!(molar_gibbs_energy, water: -52.258_321_700_239_044, pg: Err);
    test_output!(molar_helmholtz_energy, water: -54.086_997_271_059_72, pg: Err);
    test_output!(molar_internal_energy, water: 1_511.585_363_210_497_7, pg: Err);
    test_output!(molar_specific_heat, water: 75.376_798_730_939_36, pg: Err);
    test_output!(molar_specific_heat_const_volume, water: 74.883_730_724_235_63, pg: Err);
    test_output!(phase_id_param, water: -14.551_541_389_431_09, pg: Err);
    test_output!(prandtl, water: 7.007_763_685_676_371);
    test_output!(pressure, water: 101_325.000_030_278_93);
    test_output!(residual_molar_enthalpy, water: -44_221.274_182_385_6, pg: Err);
    test_output!(residual_molar_entropy, water: -59.688_703_328_769_79, pg: Err);
    test_output!(residual_molar_gibbs_energy, water: -26_723.530_801_556_73, pg: Err);
    test_output!(sound_speed, water: 1_482.346_174_847_555_6, pg: Err);
    test_output!(specific_heat, water: 4_184.050_924_523_541);
    test_output!(specific_heat_const_volume, water: 4_156.681_472_861_554_5);
    test_output!(specific_volume, water: 1.001_796_069_614_639_7e-3);
    test_output!(tau, water: 2.207_388_708_852_123_6);
    test_output!(temperature, water: 293.15);

    #[rstest]
    fn substance(ctx: Context) {
        // Given
        let Context { water, .. } = ctx;
        let substance = Substance::from(water);
        let sut = ctx.sut(water);

        // When
        let res = sut.substance();

        // Then
        assert_eq!(*res, substance);
    }

    #[rstest]
    fn phase_water(ctx: Context) {
        // Given
        let Context { water, .. } = ctx;
        let mut sut = ctx.sut(water);

        // When
        let res = sut.phase();

        // Then
        assert_eq!(res, Phase::Liquid);
    }

    #[rstest]
    fn phase_water_vapor(ctx: Context) {
        // Given
        let Context { pressure, water, .. } = ctx;
        let mut sut = ctx.sut(water).in_state(pressure, FluidInput::temperature(423.15)).unwrap();

        // When
        let res = sut.phase();

        // Then
        assert_eq!(res, Phase::Gas);
    }

    #[rstest]
    fn phase_pg(ctx: Context) {
        // Given
        let Context { pg, .. } = ctx;
        let mut sut = ctx.sut(pg);

        // When
        let res = sut.phase();

        // Then
        assert_eq!(res, Phase::Unknown);
    }

    #[rstest]
    fn quality_water(ctx: Context) {
        // Given
        let Context { water, .. } = ctx;
        let mut sut = ctx.sut(water);

        // When
        let res = sut.quality();

        // Then
        assert!(res.is_err());
    }

    #[rstest]
    fn quality_pg(ctx: Context) {
        // Given
        let Context { pg, .. } = ctx;
        let mut sut = ctx.sut(pg);

        // When
        let res = sut.quality();

        // Then
        assert!(res.is_err());
    }

    #[rstest]
    fn quality_saturated_water_vapor(ctx: Context) {
        // Given
        let Context { pressure, water, .. } = ctx;
        let mut sut = ctx.sut(water).in_state(pressure, FluidInput::quality(1.0)).unwrap();

        // When
        let res = sut.quality();

        // Then
        assert!(res.is_ok());
        assert_relative_eq!(res.unwrap(), 1.0);
    }

    #[rstest]
    fn surface_tension_water(ctx: Context) {
        // Given
        let Context { water, .. } = ctx;
        let mut sut = ctx.sut(water);

        // When
        let res = sut.surface_tension();

        // Then
        assert!(res.is_err());
    }

    #[rstest]
    fn surface_tension_pg(ctx: Context) {
        // Given
        let Context { pg, .. } = ctx;
        let mut sut = ctx.sut(pg);

        // When
        let res = sut.surface_tension();

        // Then
        assert!(res.is_err());
    }

    #[rstest]
    fn surface_tension_saturated_water_vapor(ctx: Context) {
        // Given
        let Context { pressure, water, .. } = ctx;
        let mut sut = ctx.sut(water).in_state(pressure, FluidInput::quality(1.0)).unwrap();

        // When
        let res = sut.surface_tension();

        // Then
        assert!(res.is_ok());
        assert_relative_eq!(res.unwrap(), 0.058_925_588_400_728_54);
    }

    #[rstest]
    fn update_valid_inputs(ctx: Context) {
        // Given
        let Context { pressure, temperature, water, .. } = ctx;
        let mut sut = ctx.sut(water);

        // When
        let res = sut.update(pressure, temperature);

        // Then
        assert!(res.is_ok());
    }

    #[rstest]
    fn update_same_inputs(ctx: Context) {
        // Given
        let Context { pressure, water, .. } = ctx;
        let mut sut = ctx.sut(water);

        // When
        let res = sut.update(pressure, pressure);

        // Then
        assert_eq!(res, Err(FluidStateError::InvalidInputPair(pressure.key, pressure.key)));
    }

    #[rstest]
    fn update_invalid_inputs(ctx: Context) {
        // Given
        let Context { temperature, water, .. } = ctx;
        let infinite_pressure = FluidInput::pressure(f64::INFINITY);
        let mut sut = ctx.sut(water);

        // When
        let res = sut.update(infinite_pressure, temperature);

        // Then
        assert_eq!(res, Err(FluidStateError::InvalidInputValue));
    }

    #[rstest]
    fn update_invalid_state(ctx: Context) {
        // Given
        let Context { temperature, water, .. } = ctx;
        let negative_pressure = FluidInput::pressure(-1.0);
        let mut sut = ctx.sut(water);

        // When
        let res = sut.update(negative_pressure, temperature);

        // Then
        assert!(matches!(res, Err(FluidStateError::UpdateFailed(_))));
    }

    #[rstest]
    fn in_state_valid_inputs(ctx: Context) {
        // Given
        let Context { pressure, temperature, water, .. } = ctx;
        let sut = ctx.sut(water);

        // When
        let res = sut.in_state(pressure, temperature);

        // Then
        assert!(res.is_ok());
    }

    #[rstest]
    fn in_state_same_inputs(ctx: Context) {
        // Given
        let Context { pressure, water, .. } = ctx;
        let sut = ctx.sut(water);

        // When
        let res = sut.in_state(pressure, pressure);

        // Then
        assert_eq!(res, Err(FluidStateError::InvalidInputPair(pressure.key, pressure.key)));
    }

    #[rstest]
    fn in_state_invalid_inputs(ctx: Context) {
        // Given
        let Context { temperature, water, .. } = ctx;
        let infinite_pressure = FluidInput::pressure(f64::INFINITY);
        let sut = ctx.sut(water);

        // When
        let res = sut.in_state(infinite_pressure, temperature);

        // Then
        assert_eq!(res, Err(FluidStateError::InvalidInputValue));
    }

    #[rstest]
    fn in_state_invalid_state(ctx: Context) {
        // Given
        let Context { temperature, water, .. } = ctx;
        let negative_pressure = FluidInput::pressure(-1.0);
        let sut = ctx.sut(water);

        // When
        let res = sut.in_state(negative_pressure, temperature);

        // Then
        assert!(matches!(res, Err(FluidStateError::UpdateFailed(_))));
    }

    #[rstest]
    fn clone(ctx: Context) {
        // Given
        let Context { water, .. } = ctx;
        let sut = ctx.sut(water);

        // When
        let clone = sut.clone();

        // Then
        assert_eq!(clone, sut);
        assert_eq!(clone.outputs, sut.outputs);
        assert_eq!(clone.trivial_outputs, sut.trivial_outputs);
    }
}
