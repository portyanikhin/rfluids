use super::Fluid;
use crate::error::FluidUpdateError;
use crate::io::FluidInput;

impl Fluid {
    /// Updates the state and returns a mutable reference to itself.
    ///
    /// # Args
    ///
    /// - input1 -- first input property.
    /// - input2 -- second input property.
    ///
    /// # Errors
    ///
    /// For invalid/unsupported inputs or invalid state,
    /// a [`FluidUpdateError`] is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use rfluids::error::FluidUpdateError;
    /// use rfluids::prelude::fluid::*;
    /// use rfluids::uom::si::pressure::atmosphere;
    /// use rfluids::uom::si::thermodynamic_temperature::degree_celsius;
    ///
    /// // After creation Fluid has Undefined state variant
    /// let mut water: Fluid<Undefined> = Fluid::from(Pure::Water);
    ///
    /// // First update will move the initial value and
    /// // perform conversion between Undefined and Defined state variants
    /// // (since Defined is the default state variant, it can be omitted)
    /// let mut water: Fluid = water.update(
    ///     FluidInput::pressure(Pressure::new::<atmosphere>(1.0)),
    ///     FluidInput::temperature(ThermodynamicTemperature::new::<degree_celsius>(20.0)),
    /// ).unwrap();
    ///
    /// // Secondary updates will work in place and
    /// // return mutable reference to the Fluid
    /// let result: Result<&mut Fluid, FluidUpdateError> = water.update(
    ///     FluidInput::pressure(Pressure::new::<atmosphere>(2.0)),
    ///     FluidInput::temperature(ThermodynamicTemperature::new::<degree_celsius>(40.0)),
    /// );
    /// assert!(result.is_ok());
    /// ```
    pub fn update(
        &mut self,
        input1: FluidInput,
        input2: FluidInput,
    ) -> Result<&mut Self, FluidUpdateError> {
        self.inner_update(input1, input2)?;
        Ok(self)
    }
}

impl Clone for Fluid {
    fn clone(&self) -> Self {
        let inputs: (FluidInput, FluidInput) = self.update_request.unwrap().into();
        let mut fluid = Fluid::try_from(self.substance.clone())
            .unwrap()
            .update(inputs.0, inputs.1)
            .unwrap();
        fluid.outputs = self.outputs.clone();
        fluid.trivial_outputs = self.trivial_outputs.clone();
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
    use crate::error::FluidUpdateError;
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
    fn sut(temperature: FluidInput, pressure: FluidInput) -> Fluid {
        Fluid::from(Pure::Water)
            .update(temperature, pressure)
            .unwrap()
    }

    #[rstest]
    fn substance_returns_entered_value(temperature: FluidInput, pressure: FluidInput) {
        let water = Pure::Water;
        let substance = Substance::from(water);
        let sut = Fluid::from(water).update(temperature, pressure).unwrap();
        assert_eq!(sut.substance(), &substance);
    }

    #[rstest]
    fn acentric_factor_returns_option(temperature: FluidInput, pressure: FluidInput) {
        let mut water = Fluid::from(Pure::Water)
            .update(temperature, pressure)
            .unwrap();
        assert!(water.acentric_factor().is_some());
        assert_relative_eq!(water.acentric_factor().unwrap(), 0.3442920843);
        let mut r444a = Fluid::from(PredefinedMix::R444A)
            .update(temperature, pressure)
            .unwrap();
        assert!(r444a.acentric_factor().is_none());
    }

    #[rstest]
    fn critical_density_returns_option(temperature: FluidInput, pressure: FluidInput) {
        let mut water = Fluid::from(Pure::Water)
            .update(temperature, pressure)
            .unwrap();
        assert!(water.critical_density().is_some());
        assert_relative_eq!(water.critical_density().unwrap().value, 322.0);
        let mut r444a = Fluid::from(PredefinedMix::R444A)
            .update(temperature, pressure)
            .unwrap();
        assert!(r444a.critical_density().is_none());
        let mut incomp_water = Fluid::from(IncompPure::Water)
            .update(temperature, pressure)
            .unwrap();
        assert!(incomp_water.critical_density().is_none());
    }

    #[rstest]
    fn critical_molar_density_returns_option(temperature: FluidInput, pressure: FluidInput) {
        let mut water = Fluid::from(Pure::Water)
            .update(temperature, pressure)
            .unwrap();
        assert!(water.critical_molar_density().is_some());
        assert_relative_eq!(
            water.critical_molar_density().unwrap().value,
            17873.72799560906
        );
        let mut r444a = Fluid::from(PredefinedMix::R444A)
            .update(temperature, pressure)
            .unwrap();
        assert!(r444a.critical_molar_density().is_none());
        let mut incomp_water = Fluid::from(IncompPure::Water)
            .update(temperature, pressure)
            .unwrap();
        assert!(incomp_water.critical_molar_density().is_none());
    }

    #[rstest]
    fn critical_pressure_returns_option(temperature: FluidInput, pressure: FluidInput) {
        let mut water = Fluid::from(Pure::Water)
            .update(temperature, pressure)
            .unwrap();
        assert!(water.critical_pressure().is_some());
        assert_relative_eq!(water.critical_pressure().unwrap().value, 22.064e6);
        let mut r444a = Fluid::from(PredefinedMix::R444A)
            .update(temperature, pressure)
            .unwrap();
        assert!(r444a.critical_pressure().is_none());
        let mut incomp_water = Fluid::from(IncompPure::Water)
            .update(temperature, pressure)
            .unwrap();
        assert!(incomp_water.critical_pressure().is_none());
    }

    #[rstest]
    fn critical_temperature_returns_option(temperature: FluidInput, pressure: FluidInput) {
        let mut water = Fluid::from(Pure::Water)
            .update(temperature, pressure)
            .unwrap();
        assert!(water.critical_temperature().is_some());
        assert_relative_eq!(water.critical_temperature().unwrap().value, 647.096);
        let mut r444a = Fluid::from(PredefinedMix::R444A)
            .update(temperature, pressure)
            .unwrap();
        assert!(r444a.critical_temperature().is_none());
        let mut incomp_water = Fluid::from(IncompPure::Water)
            .update(temperature, pressure)
            .unwrap();
        assert!(incomp_water.critical_temperature().is_none());
    }

    #[rstest]
    fn flammability_hazard_returns_option(temperature: FluidInput, pressure: FluidInput) {
        let mut water = Fluid::from(Pure::Water)
            .update(temperature, pressure)
            .unwrap();
        assert!(water.flammability_hazard().is_some());
        assert_relative_eq!(water.flammability_hazard().unwrap(), 0.0);
        let mut incomp_water = Fluid::from(IncompPure::Water)
            .update(temperature, pressure)
            .unwrap();
        assert!(incomp_water.flammability_hazard().is_none());
    }

    #[rstest]
    fn freezing_temperature_returns_option(temperature: FluidInput, pressure: FluidInput) {
        let mut water = Fluid::from(Pure::Water)
            .update(temperature, pressure)
            .unwrap();
        assert!(water.freezing_temperature().is_none());
        let mut propylene_glycol = Fluid::from(
            BinaryMix::try_from(BinaryMixKind::MPG, Ratio::new::<percent>(40.0)).unwrap(),
        )
        .update(temperature, pressure)
        .unwrap();
        assert!(propylene_glycol.freezing_temperature().is_some());
        assert_relative_eq!(
            propylene_glycol.freezing_temperature().unwrap().value,
            252.58175495305838
        );
    }

    #[rstest]
    fn gwp20_returns_option(temperature: FluidInput, pressure: FluidInput) {
        let mut water = Fluid::from(Pure::Water)
            .update(temperature, pressure)
            .unwrap();
        assert!(water.gwp20().is_none());
        let mut r32 = Fluid::from(Pure::R32)
            .update(temperature, pressure)
            .unwrap();
        assert!(r32.gwp20().is_some());
        assert_eq!(r32.gwp20().unwrap(), 2330.0);
    }

    #[rstest]
    fn update_valid_inputs_returns_ok(
        mut sut: Fluid,
        temperature: FluidInput,
        pressure: FluidInput,
    ) {
        assert!(sut.update(temperature, pressure).is_ok());
    }

    #[rstest]
    fn update_same_inputs_returns_err(mut sut: Fluid, pressure: FluidInput) {
        assert_eq!(
            sut.update(pressure, pressure).unwrap_err(),
            FluidUpdateError::InvalidInputPair(pressure.key(), pressure.key())
        );
    }

    #[rstest]
    fn update_invalid_inputs_returns_err(
        mut sut: Fluid,
        temperature: FluidInput,
        infinite_pressure: FluidInput,
    ) {
        assert_eq!(
            sut.update(temperature, infinite_pressure).unwrap_err(),
            FluidUpdateError::InvalidInputValue
        );
    }

    #[rstest]
    fn update_invalid_state_returns_err(
        mut sut: Fluid,
        temperature: FluidInput,
        negative_pressure: FluidInput,
    ) {
        assert!(matches!(
            sut.update(temperature, negative_pressure).unwrap_err(),
            FluidUpdateError::UpdateFailed(_)
        ));
    }

    #[rstest]
    fn clone_returns_new_instance(sut: Fluid) {
        let clone = sut.clone();
        assert_eq!(clone, sut);
        assert_eq!(clone.outputs, sut.outputs);
        assert_eq!(clone.trivial_outputs, sut.trivial_outputs);
    }
}
