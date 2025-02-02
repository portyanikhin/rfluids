use super::Fluid;
use crate::error::FluidUpdateError;
use crate::io::FluidInput;
use crate::state_variant::Undefined;
use std::collections::HashMap;
use std::marker::PhantomData;

impl Fluid<Undefined> {
    /// Updates the state and returns itself with
    /// [`Defined`](crate::state_variant::Defined) state variant.
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
        mut self,
        input1: FluidInput,
        input2: FluidInput,
    ) -> Result<Fluid, FluidUpdateError> {
        self.inner_update(input1, input2)?;
        Ok(Fluid {
            substance: self.substance,
            backend: self.backend,
            update_request: self.update_request,
            outputs: HashMap::new(),
            trivial_outputs: self.trivial_outputs,
            state: PhantomData,
        })
    }
}

impl Clone for Fluid<Undefined> {
    fn clone(&self) -> Self {
        let mut fluid = Fluid::try_from(self.substance.clone()).unwrap();
        fluid.trivial_outputs = self.trivial_outputs.clone();
        fluid
    }
}

impl PartialEq for Fluid<Undefined> {
    fn eq(&self, other: &Self) -> bool {
        self.substance == other.substance
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
    fn water() -> Fluid<Undefined> {
        Fluid::from(Pure::Water)
    }

    #[fixture]
    fn r22() -> Fluid<Undefined> {
        Fluid::from(Pure::R22)
    }

    #[fixture]
    fn r32() -> Fluid<Undefined> {
        Fluid::from(Pure::R32)
    }

    #[fixture]
    fn incomp_water() -> Fluid<Undefined> {
        Fluid::from(IncompPure::Water)
    }

    #[fixture]
    fn r444a() -> Fluid<Undefined> {
        Fluid::from(PredefinedMix::R444A)
    }

    #[fixture]
    fn propylene_glycol() -> Fluid<Undefined> {
        Fluid::from(BinaryMix::try_from(BinaryMixKind::MPG, Ratio::new::<percent>(40.0)).unwrap())
    }

    #[test]
    fn substance_returns_entered_value() {
        let water = Pure::Water;
        let substance = Substance::from(water);
        let sut = Fluid::from(water);
        assert_eq!(sut.substance(), &substance);
    }

    #[rstest]
    fn acentric_factor_returns_option(mut water: Fluid<Undefined>, mut r444a: Fluid<Undefined>) {
        assert!(water.acentric_factor().is_some());
        assert_relative_eq!(water.acentric_factor().unwrap(), 0.3442920843);
        assert!(r444a.acentric_factor().is_none());
    }

    #[rstest]
    fn critical_density_returns_option(
        mut water: Fluid<Undefined>,
        mut r444a: Fluid<Undefined>,
        mut incomp_water: Fluid<Undefined>,
    ) {
        assert!(water.critical_density().is_some());
        assert_relative_eq!(water.critical_density().unwrap().value, 322.0);
        assert!(r444a.critical_density().is_none());
        assert!(incomp_water.critical_density().is_none());
    }

    #[rstest]
    fn critical_molar_density_returns_option(
        mut water: Fluid<Undefined>,
        mut r444a: Fluid<Undefined>,
        mut incomp_water: Fluid<Undefined>,
    ) {
        assert!(water.critical_molar_density().is_some());
        assert_relative_eq!(
            water.critical_molar_density().unwrap().value,
            17873.72799560906
        );
        assert!(r444a.critical_molar_density().is_none());
        assert!(incomp_water.critical_molar_density().is_none());
    }

    #[rstest]
    fn critical_pressure_returns_option(
        mut water: Fluid<Undefined>,
        mut r444a: Fluid<Undefined>,
        mut incomp_water: Fluid<Undefined>,
    ) {
        assert!(water.critical_pressure().is_some());
        assert_relative_eq!(water.critical_pressure().unwrap().value, 22.064e6);
        assert!(r444a.critical_pressure().is_none());
        assert!(incomp_water.critical_pressure().is_none());
    }

    #[rstest]
    fn critical_temperature_returns_option(
        mut water: Fluid<Undefined>,
        mut r444a: Fluid<Undefined>,
        mut incomp_water: Fluid<Undefined>,
    ) {
        assert!(water.critical_temperature().is_some());
        assert_relative_eq!(water.critical_temperature().unwrap().value, 647.096);
        assert!(r444a.critical_temperature().is_none());
        assert!(incomp_water.critical_temperature().is_none());
    }

    #[rstest]
    fn flammability_hazard_returns_option(
        mut water: Fluid<Undefined>,
        mut incomp_water: Fluid<Undefined>,
    ) {
        assert!(water.flammability_hazard().is_some());
        assert_eq!(water.flammability_hazard().unwrap(), 0.0);
        assert!(incomp_water.flammability_hazard().is_none());
    }

    #[rstest]
    fn freezing_temperature_returns_option(
        mut water: Fluid<Undefined>,
        mut propylene_glycol: Fluid<Undefined>,
    ) {
        assert!(water.freezing_temperature().is_none());
        assert!(propylene_glycol.freezing_temperature().is_some());
        assert_relative_eq!(
            propylene_glycol.freezing_temperature().unwrap().value,
            252.58175495305838
        );
    }

    #[rstest]
    fn gwp20_returns_option(mut water: Fluid<Undefined>, mut r32: Fluid<Undefined>) {
        assert!(water.gwp20().is_none());
        assert!(r32.gwp20().is_some());
        assert_eq!(r32.gwp20().unwrap(), 2330.0);
    }

    #[rstest]
    fn gwp100_returns_option(mut water: Fluid<Undefined>, mut r32: Fluid<Undefined>) {
        assert!(water.gwp100().is_none());
        assert!(r32.gwp100().is_some());
        assert_eq!(r32.gwp100().unwrap(), 675.0);
    }

    #[rstest]
    fn gwp500_returns_option(mut water: Fluid<Undefined>, mut r32: Fluid<Undefined>) {
        assert!(water.gwp500().is_none());
        assert!(r32.gwp500().is_some());
        assert_eq!(r32.gwp500().unwrap(), 205.0);
    }

    #[rstest]
    fn health_hazard_returns_option(
        mut water: Fluid<Undefined>,
        mut incomp_water: Fluid<Undefined>,
    ) {
        assert!(water.health_hazard().is_some());
        assert_eq!(water.health_hazard().unwrap(), 0.0);
        assert!(incomp_water.health_hazard().is_none());
    }

    #[rstest]
    fn max_pressure_returns_option(
        mut water: Fluid<Undefined>,
        mut incomp_water: Fluid<Undefined>,
    ) {
        assert!(water.max_pressure().is_some());
        assert_eq!(water.max_pressure().unwrap().value, 1e9);
        assert!(incomp_water.max_pressure().is_none());
    }

    #[rstest]
    fn max_temperature_returns_expected_value(mut water: Fluid<Undefined>) {
        assert_eq!(water.max_temperature().value, 2e3);
    }

    #[rstest]
    fn min_pressure_returns_option(
        mut water: Fluid<Undefined>,
        mut incomp_water: Fluid<Undefined>,
    ) {
        assert!(water.min_pressure().is_some());
        assert_relative_eq!(water.min_pressure().unwrap().value, 611.6548008968684);
        assert!(incomp_water.min_pressure().is_none());
    }

    #[rstest]
    fn min_temperature_returns_expected_value(mut water: Fluid<Undefined>) {
        assert_eq!(water.min_temperature().value, 273.16);
    }

    #[rstest]
    fn molar_mass_returns_option(mut water: Fluid<Undefined>, mut incomp_water: Fluid<Undefined>) {
        assert!(water.molar_mass().is_some());
        assert_relative_eq!(water.molar_mass().unwrap().value, 0.018015268);
        assert!(incomp_water.molar_mass().is_none());
    }

    #[rstest]
    fn opd_returns_option(
        mut water: Fluid<Undefined>,
        mut r32: Fluid<Undefined>,
        mut r22: Fluid<Undefined>,
    ) {
        assert!(water.odp().is_none());
        assert!(r32.odp().is_none());
        assert!(r22.odp().is_some());
        assert_eq!(r22.odp().unwrap(), 0.05);
    }

    #[rstest]
    fn physical_hazard_returns_option(
        mut water: Fluid<Undefined>,
        mut incomp_water: Fluid<Undefined>,
    ) {
        assert!(water.physical_hazard().is_some());
        assert_eq!(water.physical_hazard().unwrap(), 0.0);
        assert!(incomp_water.physical_hazard().is_none());
    }

    #[rstest]
    fn reducing_density_returns_option(
        mut water: Fluid<Undefined>,
        mut incomp_water: Fluid<Undefined>,
    ) {
        assert!(water.reducing_density().is_some());
        assert_relative_eq!(water.reducing_density().unwrap().value, 322.0);
        assert!(incomp_water.reducing_density().is_none());
    }

    #[rstest]
    fn reducing_molar_density_returns_option(
        mut water: Fluid<Undefined>,
        mut incomp_water: Fluid<Undefined>,
    ) {
        assert!(water.reducing_molar_density().is_some());
        assert_relative_eq!(
            water.reducing_molar_density().unwrap().value,
            17873.72799560906
        );
        assert!(incomp_water.reducing_molar_density().is_none());
    }

    #[rstest]
    fn update_valid_inputs_returns_ok(
        water: Fluid<Undefined>,
        temperature: FluidInput,
        pressure: FluidInput,
    ) {
        assert!(water.update(temperature, pressure).is_ok());
    }

    #[rstest]
    fn update_same_inputs_returns_err(water: Fluid<Undefined>, pressure: FluidInput) {
        assert_eq!(
            water.update(pressure, pressure).unwrap_err(),
            FluidUpdateError::InvalidInputPair(pressure.key(), pressure.key())
        );
    }

    #[rstest]
    fn update_invalid_inputs_returns_err(
        water: Fluid<Undefined>,
        temperature: FluidInput,
        infinite_pressure: FluidInput,
    ) {
        assert_eq!(
            water.update(temperature, infinite_pressure).unwrap_err(),
            FluidUpdateError::InvalidInputValue
        );
    }

    #[rstest]
    fn update_invalid_state_returns_err(
        water: Fluid<Undefined>,
        temperature: FluidInput,
        negative_pressure: FluidInput,
    ) {
        assert!(matches!(
            water.update(temperature, negative_pressure).unwrap_err(),
            FluidUpdateError::UpdateFailed(_)
        ));
    }

    #[rstest]
    fn clone_returns_new_instance(water: Fluid<Undefined>) {
        let clone = water.clone();
        assert_eq!(clone, water);
        assert_eq!(clone.trivial_outputs, water.trivial_outputs);
    }
}
