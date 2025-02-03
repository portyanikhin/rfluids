use super::Fluid;
use crate::error::FluidStateError;
use crate::io::FluidInput;

impl Fluid {
    /// Updates the thermodynamic state and returns a mutable reference to itself.
    ///
    /// # Args
    ///
    /// - input1 -- first input property.
    /// - input2 -- second input property.
    ///
    /// # Errors
    ///
    /// For invalid/unsupported inputs or invalid state,
    /// a [`FluidStateError`] is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use rfluids::error::FluidStateError;
    /// use rfluids::prelude::fluid::*;
    /// use rfluids::uom::si::pressure::atmosphere;
    /// use rfluids::uom::si::thermodynamic_temperature::degree_celsius;
    ///
    /// // After creation the `Fluid` instance has `Undefined` state variant
    /// let mut water: Fluid<Undefined> = Fluid::from(Pure::Water);
    ///
    /// // Calling `in_state` method will move the initial value and
    /// // perform conversion between `Undefined` and `Defined` state variants
    /// // (since `Defined` is the default state variant, it can be omitted)
    /// let mut water: Fluid = water.in_state(
    ///     FluidInput::pressure(Pressure::new::<atmosphere>(1.0)),
    ///     FluidInput::temperature(ThermodynamicTemperature::new::<degree_celsius>(20.0)),
    /// ).unwrap();
    ///
    /// // The `Fluid` instance now has `Defined` state variant
    /// // and it's thermodynamic state can be updated in place by calling `update` method
    /// // (which returns a mutable reference to the instance)
    /// let same_water_in_new_state: Result<&mut Fluid, FluidStateError> = water.update(
    ///     FluidInput::pressure(Pressure::new::<atmosphere>(2.0)),
    ///     FluidInput::temperature(ThermodynamicTemperature::new::<degree_celsius>(40.0)),
    /// );
    /// assert!(same_water_in_new_state.is_ok());
    ///
    /// // Calling `in_state` method on `Fluid<Defined>` will return
    /// // a new instance in the specified thermodynamic state
    /// let new_water: Result<Fluid, FluidStateError> = water.in_state(
    ///     FluidInput::pressure(Pressure::new::<atmosphere>(4.0)),
    ///     FluidInput::temperature(ThermodynamicTemperature::new::<degree_celsius>(80.0)),
    /// );
    /// assert!(new_water.is_ok());
    /// ```
    ///
    /// # See also
    ///
    /// - [`Fluid::in_state`](crate::fluid::Fluid::in_state)
    pub fn update(
        &mut self,
        input1: FluidInput,
        input2: FluidInput,
    ) -> Result<&mut Self, FluidStateError> {
        self.inner_update(input1, input2)?;
        Ok(self)
    }

    /// Returns a new instance in the specified thermodynamic state.
    ///
    /// # Args
    ///
    /// - input1 -- first input property.
    /// - input2 -- second input property.
    ///
    /// # Errors
    ///
    /// For invalid/unsupported inputs or invalid state,
    /// a [`FluidStateError`] is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use rfluids::error::FluidStateError;
    /// use rfluids::prelude::fluid::*;
    /// use rfluids::uom::si::pressure::atmosphere;
    /// use rfluids::uom::si::thermodynamic_temperature::degree_celsius;
    ///
    /// // After creation the `Fluid` instance has `Undefined` state variant
    /// let mut water: Fluid<Undefined> = Fluid::from(Pure::Water);
    ///
    /// // Calling `in_state` method will move the initial value and
    /// // perform conversion between `Undefined` and `Defined` state variants
    /// // (since `Defined` is the default state variant, it can be omitted)
    /// let mut water: Fluid = water.in_state(
    ///     FluidInput::pressure(Pressure::new::<atmosphere>(1.0)),
    ///     FluidInput::temperature(ThermodynamicTemperature::new::<degree_celsius>(20.0)),
    /// ).unwrap();
    ///
    /// // The `Fluid` instance now has `Defined` state variant
    /// // and it's thermodynamic state can be updated in place by calling `update` method
    /// // (which returns a mutable reference to the instance)
    /// let same_water_in_new_state: Result<&mut Fluid, FluidStateError> = water.update(
    ///     FluidInput::pressure(Pressure::new::<atmosphere>(2.0)),
    ///     FluidInput::temperature(ThermodynamicTemperature::new::<degree_celsius>(40.0)),
    /// );
    /// assert!(same_water_in_new_state.is_ok());
    ///
    /// // Calling `in_state` method on `Fluid<Defined>` will return
    /// // a new instance in the specified thermodynamic state
    /// let new_water: Result<Fluid, FluidStateError> = water.in_state(
    ///     FluidInput::pressure(Pressure::new::<atmosphere>(4.0)),
    ///     FluidInput::temperature(ThermodynamicTemperature::new::<degree_celsius>(80.0)),
    /// );
    /// assert!(new_water.is_ok());
    /// ```
    ///
    /// # See also
    ///
    /// - [`Fluid::update`](crate::fluid::Fluid::update)
    pub fn in_state(
        &self,
        input1: FluidInput,
        input2: FluidInput,
    ) -> Result<Self, FluidStateError> {
        let mut fluid = Fluid::try_from(self.substance.clone())
            .unwrap()
            .in_state(input1, input2)?;
        fluid.trivial_outputs.clone_from(&self.trivial_outputs);
        Ok(fluid)
    }
}

impl Clone for Fluid {
    fn clone(&self) -> Self {
        let inputs: (FluidInput, FluidInput) = self.update_request.unwrap().into();
        let mut fluid = Fluid::try_from(self.substance.clone())
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
        self.substance == other.substance && self.update_request == other.update_request
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::FluidStateError;
    use crate::substance::*;
    use crate::uom::si::f64::{Pressure, Ratio, ThermodynamicTemperature};
    use crate::uom::si::pressure::atmosphere;
    use crate::uom::si::ratio::percent;
    use crate::uom::si::thermodynamic_temperature::degree_celsius;
    use approx::assert_relative_eq;
    use rstest::*;

    #[fixture]
    fn temperature(#[default(20.0)] value: f64) -> FluidInput {
        FluidInput::temperature(ThermodynamicTemperature::new::<degree_celsius>(value))
    }

    #[fixture]
    fn pressure(#[default(1.0)] value: f64) -> FluidInput {
        FluidInput::pressure(Pressure::new::<atmosphere>(value))
    }

    #[fixture]
    fn infinite_pressure(#[with(f64::INFINITY)] pressure: FluidInput) -> FluidInput {
        pressure
    }

    #[fixture]
    fn negative_pressure(#[with(-1.0)] pressure: FluidInput) -> FluidInput {
        pressure
    }

    #[fixture]
    fn water(temperature: FluidInput, pressure: FluidInput) -> Fluid {
        Fluid::from(Pure::Water)
            .in_state(temperature, pressure)
            .unwrap()
    }

    #[fixture]
    fn r22(temperature: FluidInput, pressure: FluidInput) -> Fluid {
        Fluid::from(Pure::R22)
            .in_state(temperature, pressure)
            .unwrap()
    }

    #[fixture]
    fn r32(temperature: FluidInput, pressure: FluidInput) -> Fluid {
        Fluid::from(Pure::R32)
            .in_state(temperature, pressure)
            .unwrap()
    }

    #[fixture]
    fn incomp_water(temperature: FluidInput, pressure: FluidInput) -> Fluid {
        Fluid::from(IncompPure::Water)
            .in_state(temperature, pressure)
            .unwrap()
    }

    #[fixture]
    fn r444a(temperature: FluidInput, pressure: FluidInput) -> Fluid {
        Fluid::from(PredefinedMix::R444A)
            .in_state(temperature, pressure)
            .unwrap()
    }

    #[fixture]
    fn propylene_glycol(temperature: FluidInput, pressure: FluidInput) -> Fluid {
        Fluid::from(BinaryMix::try_from(BinaryMixKind::MPG, Ratio::new::<percent>(40.0)).unwrap())
            .in_state(temperature, pressure)
            .unwrap()
    }

    #[rstest]
    fn substance_returns_entered_value(temperature: FluidInput, pressure: FluidInput) {
        let water = Pure::Water;
        let substance = Substance::from(water);
        let sut = Fluid::from(water).in_state(temperature, pressure).unwrap();
        assert_eq!(sut.substance(), &substance);
    }

    #[rstest]
    fn acentric_factor_returns_option(mut water: Fluid, mut r444a: Fluid) {
        assert!(water.acentric_factor().is_some());
        assert_relative_eq!(water.acentric_factor().unwrap(), 0.344_292_084_3);
        assert!(r444a.acentric_factor().is_none());
    }

    #[rstest]
    fn critical_density_returns_option(
        mut water: Fluid,
        mut r444a: Fluid,
        mut incomp_water: Fluid,
    ) {
        assert!(water.critical_density().is_some());
        assert_relative_eq!(water.critical_density().unwrap().value, 322.0);
        assert!(r444a.critical_density().is_none());
        assert!(incomp_water.critical_density().is_none());
    }

    #[rstest]
    fn critical_molar_density_returns_option(
        mut water: Fluid,
        mut r444a: Fluid,
        mut incomp_water: Fluid,
    ) {
        assert!(water.critical_molar_density().is_some());
        assert_relative_eq!(
            water.critical_molar_density().unwrap().value,
            17_873.727_995_609_06
        );
        assert!(r444a.critical_molar_density().is_none());
        assert!(incomp_water.critical_molar_density().is_none());
    }

    #[rstest]
    fn critical_pressure_returns_option(
        mut water: Fluid,
        mut r444a: Fluid,
        mut incomp_water: Fluid,
    ) {
        assert!(water.critical_pressure().is_some());
        assert_relative_eq!(water.critical_pressure().unwrap().value, 22.064e6);
        assert!(r444a.critical_pressure().is_none());
        assert!(incomp_water.critical_pressure().is_none());
    }

    #[rstest]
    fn critical_temperature_returns_option(
        mut water: Fluid,
        mut r444a: Fluid,
        mut incomp_water: Fluid,
    ) {
        assert!(water.critical_temperature().is_some());
        assert_relative_eq!(water.critical_temperature().unwrap().value, 647.096);
        assert!(r444a.critical_temperature().is_none());
        assert!(incomp_water.critical_temperature().is_none());
    }

    #[rstest]
    fn flammability_hazard_returns_option(mut water: Fluid, mut incomp_water: Fluid) {
        assert!(water.flammability_hazard().is_some());
        assert_eq!(water.flammability_hazard().unwrap(), 0.0);
        assert!(incomp_water.flammability_hazard().is_none());
    }

    #[rstest]
    fn freezing_temperature_returns_option(mut water: Fluid, mut propylene_glycol: Fluid) {
        assert!(water.freezing_temperature().is_none());
        assert!(propylene_glycol.freezing_temperature().is_some());
        assert_relative_eq!(
            propylene_glycol.freezing_temperature().unwrap().value,
            252.581_754_953_058_38
        );
    }

    #[rstest]
    fn gwp20_returns_option(mut water: Fluid, mut r32: Fluid) {
        assert!(water.gwp20().is_none());
        assert!(r32.gwp20().is_some());
        assert_eq!(r32.gwp20().unwrap(), 2330.0);
    }

    #[rstest]
    fn gwp100_returns_option(mut water: Fluid, mut r32: Fluid) {
        assert!(water.gwp100().is_none());
        assert!(r32.gwp100().is_some());
        assert_eq!(r32.gwp100().unwrap(), 675.0);
    }

    #[rstest]
    fn gwp500_returns_option(mut water: Fluid, mut r32: Fluid) {
        assert!(water.gwp500().is_none());
        assert!(r32.gwp500().is_some());
        assert_eq!(r32.gwp500().unwrap(), 205.0);
    }

    #[rstest]
    fn health_hazard_returns_option(mut water: Fluid, mut incomp_water: Fluid) {
        assert!(water.health_hazard().is_some());
        assert_eq!(water.health_hazard().unwrap(), 0.0);
        assert!(incomp_water.health_hazard().is_none());
    }

    #[rstest]
    fn max_pressure_returns_option(mut water: Fluid, mut incomp_water: Fluid) {
        assert!(water.max_pressure().is_some());
        assert_eq!(water.max_pressure().unwrap().value, 1e9);
        assert!(incomp_water.max_pressure().is_none());
    }

    #[rstest]
    fn max_temperature_returns_expected_value(mut water: Fluid) {
        assert_eq!(water.max_temperature().value, 2e3);
    }

    #[rstest]
    fn min_pressure_returns_option(mut water: Fluid, mut incomp_water: Fluid) {
        assert!(water.min_pressure().is_some());
        assert_relative_eq!(water.min_pressure().unwrap().value, 611.654_800_896_868_4);
        assert!(incomp_water.min_pressure().is_none());
    }

    #[rstest]
    fn min_temperature_returns_expected_value(mut water: Fluid) {
        assert_eq!(water.min_temperature().value, 273.16);
    }

    #[rstest]
    fn molar_mass_returns_option(mut water: Fluid, mut incomp_water: Fluid) {
        assert!(water.molar_mass().is_some());
        assert_relative_eq!(water.molar_mass().unwrap().value, 0.018_015_268);
        assert!(incomp_water.molar_mass().is_none());
    }

    #[rstest]
    fn opd_returns_option(mut water: Fluid, mut r32: Fluid, mut r22: Fluid) {
        assert!(water.odp().is_none());
        assert!(r32.odp().is_none());
        assert!(r22.odp().is_some());
        assert_eq!(r22.odp().unwrap(), 0.05);
    }

    #[rstest]
    fn physical_hazard_returns_option(mut water: Fluid, mut incomp_water: Fluid) {
        assert!(water.physical_hazard().is_some());
        assert_eq!(water.physical_hazard().unwrap(), 0.0);
        assert!(incomp_water.physical_hazard().is_none());
    }

    #[rstest]
    fn reducing_density_returns_option(mut water: Fluid, mut incomp_water: Fluid) {
        assert!(water.reducing_density().is_some());
        assert_relative_eq!(water.reducing_density().unwrap().value, 322.0);
        assert!(incomp_water.reducing_density().is_none());
    }

    #[rstest]
    fn reducing_molar_density_returns_option(mut water: Fluid, mut incomp_water: Fluid) {
        assert!(water.reducing_molar_density().is_some());
        assert_relative_eq!(
            water.reducing_molar_density().unwrap().value,
            17_873.727_995_609_06
        );
        assert!(incomp_water.reducing_molar_density().is_none());
    }

    #[rstest]
    fn reducing_pressure_returns_option(mut water: Fluid, mut incomp_water: Fluid) {
        assert!(water.reducing_pressure().is_some());
        assert_relative_eq!(water.reducing_pressure().unwrap().value, 22.064e6);
        assert!(incomp_water.reducing_pressure().is_none());
    }

    #[rstest]
    fn reducing_temperature_returns_option(mut water: Fluid, mut incomp_water: Fluid) {
        assert!(water.reducing_temperature().is_some());
        assert_relative_eq!(water.reducing_temperature().unwrap().value, 647.096);
        assert!(incomp_water.reducing_temperature().is_none());
    }

    #[rstest]
    fn triple_pressure_returns_option(mut water: Fluid, mut incomp_water: Fluid) {
        assert!(water.triple_pressure().is_some());
        assert_relative_eq!(
            water.triple_pressure().unwrap().value,
            611.654_800_896_868_4
        );
        assert!(incomp_water.triple_pressure().is_none());
    }

    #[rstest]
    fn triple_temperature_returns_option(mut water: Fluid, mut incomp_water: Fluid) {
        assert!(water.triple_temperature().is_some());
        assert_relative_eq!(water.triple_temperature().unwrap().value, 273.16);
        assert!(incomp_water.triple_temperature().is_none());
    }

    #[rstest]
    fn update_valid_inputs_returns_ok(
        mut water: Fluid,
        temperature: FluidInput,
        pressure: FluidInput,
    ) {
        assert!(water.update(temperature, pressure).is_ok());
    }

    #[rstest]
    fn update_same_inputs_returns_err(mut water: Fluid, pressure: FluidInput) {
        assert_eq!(
            water.update(pressure, pressure).unwrap_err(),
            FluidStateError::InvalidInputPair(pressure.key(), pressure.key())
        );
    }

    #[rstest]
    fn update_invalid_inputs_returns_err(
        mut water: Fluid,
        temperature: FluidInput,
        infinite_pressure: FluidInput,
    ) {
        assert_eq!(
            water.update(temperature, infinite_pressure).unwrap_err(),
            FluidStateError::InvalidInputValue
        );
    }

    #[rstest]
    fn update_invalid_state_returns_err(
        mut water: Fluid,
        temperature: FluidInput,
        negative_pressure: FluidInput,
    ) {
        assert!(matches!(
            water.update(temperature, negative_pressure).unwrap_err(),
            FluidStateError::UpdateFailed(_)
        ));
    }

    #[rstest]
    fn in_state_valid_inputs_returns_ok(
        water: Fluid,
        temperature: FluidInput,
        pressure: FluidInput,
    ) {
        assert!(water.in_state(temperature, pressure).is_ok());
    }

    #[rstest]
    fn in_state_same_inputs_returns_err(water: Fluid, pressure: FluidInput) {
        assert_eq!(
            water.in_state(pressure, pressure).unwrap_err(),
            FluidStateError::InvalidInputPair(pressure.key(), pressure.key())
        );
    }

    #[rstest]
    fn in_state_invalid_inputs_returns_err(
        water: Fluid,
        temperature: FluidInput,
        infinite_pressure: FluidInput,
    ) {
        assert_eq!(
            water.in_state(temperature, infinite_pressure).unwrap_err(),
            FluidStateError::InvalidInputValue
        );
    }

    #[rstest]
    fn in_state_invalid_state_returns_err(
        water: Fluid,
        temperature: FluidInput,
        negative_pressure: FluidInput,
    ) {
        assert!(matches!(
            water.in_state(temperature, negative_pressure).unwrap_err(),
            FluidStateError::UpdateFailed(_)
        ));
    }

    #[rstest]
    fn clone_returns_new_instance(water: Fluid) {
        let clone = water.clone();
        assert_eq!(clone, water);
        assert_eq!(clone.outputs, water.outputs);
        assert_eq!(clone.trivial_outputs, water.trivial_outputs);
    }
}
