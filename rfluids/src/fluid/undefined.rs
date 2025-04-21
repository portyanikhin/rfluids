use super::{Fluid, StateResult};
use crate::{io::FluidInput, state_variant::Undefined};
use std::marker::PhantomData;

impl Fluid<Undefined> {
    /// Updates the thermodynamic state and returns itself with
    /// [`Defined`](crate::state_variant::Defined) state variant.
    ///
    /// # Args
    ///
    /// - `input1` -- first input property.
    /// - `input2` -- second input property.
    ///
    /// # Errors
    ///
    /// For invalid/unsupported inputs or invalid state,
    /// a [`FluidStateError`](crate::error::FluidStateError) is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use rfluids::fluid::StateResult;
    /// use rfluids::prelude::*;
    ///
    /// // After creation the `Fluid` instance has `Undefined` state variant
    /// let mut water: Fluid<Undefined> = Fluid::from(Pure::Water);
    ///
    /// // Calling `in_state` method will move the initial value and
    /// // perform conversion between `Undefined` and `Defined` state variants
    /// // (since `Defined` is the default state variant, it can be omitted)
    /// let mut water: Fluid = water.in_state(
    ///     FluidInput::pressure(101_325.0),
    ///     FluidInput::temperature(293.15),
    /// )?;
    ///
    /// // The `Fluid` instance now has `Defined` state variant
    /// // and it's thermodynamic state can be updated in place by calling `update` method
    /// // (which returns a mutable reference to the instance)
    /// let same_water_in_new_state: StateResult<&mut Fluid> = water.update(
    ///     FluidInput::pressure(202_650.0),
    ///     FluidInput::temperature(313.15),
    /// );
    /// assert!(same_water_in_new_state.is_ok());
    ///
    /// // Calling `in_state` method on `Fluid<Defined>` will return
    /// // a new instance in the specified thermodynamic state
    /// let new_water: StateResult<Fluid> = water.in_state(
    ///     FluidInput::pressure(405_300.0),
    ///     FluidInput::temperature(353.15),
    /// );
    /// assert!(new_water.is_ok());
    /// # Ok::<(), rfluids::error::Error>(())
    /// ```
    ///
    /// # See also
    ///
    /// - [`Fluid::update`](crate::fluid::Fluid::update)
    /// - [`Fluid::in_state`](crate::fluid::Fluid::in_state)
    pub fn in_state(mut self, input1: FluidInput, input2: FluidInput) -> StateResult<Fluid> {
        self.inner_update(input1, input2)?;
        Ok(Fluid {
            substance: self.substance,
            backend: self.backend,
            update_request: self.update_request,
            outputs: self.outputs,
            trivial_outputs: self.trivial_outputs,
            state: PhantomData,
        })
    }
}

impl Clone for Fluid<Undefined> {
    fn clone(&self) -> Self {
        let mut fluid = Fluid::try_from(self.substance.clone()).unwrap();
        fluid.trivial_outputs.clone_from(&self.trivial_outputs);
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
    use crate::{error::FluidStateError, substance::*, test::test_output};
    use rstest::*;

    #[fixture]
    fn temperature(#[default(293.15)] value: f64) -> FluidInput {
        FluidInput::temperature(value)
    }

    #[fixture]
    fn pressure(#[default(101_325.0)] value: f64) -> FluidInput {
        FluidInput::pressure(value)
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
    fn pg() -> Fluid<Undefined> {
        Fluid::from(BinaryMixKind::MPG.with_fraction(0.4).unwrap())
    }

    #[test]
    fn substance_returns_entered_value() {
        let water = Pure::Water;
        let substance = Substance::from(water);
        let sut = Fluid::from(water);
        assert_eq!(sut.substance(), &substance);
    }

    test_output!(Fluid<Undefined>: acentric_factor, water: 0.344_292_084_3, r444a: Err);
    test_output!(Fluid<Undefined>: critical_density, water: 322.0, r444a: Err, incomp_water: Err);
    test_output!(Fluid<Undefined>: critical_molar_density, water: 17_873.727_995_609_06, r444a: Err, incomp_water: Err);
    test_output!(Fluid<Undefined>: critical_pressure, water: 22.064e6, r444a: Err, incomp_water: Err);
    test_output!(Fluid<Undefined>: critical_temperature, water: 647.096, r444a: Err, incomp_water: Err);
    test_output!(Fluid<Undefined>: flammability_hazard, water: 0.0, incomp_water: Err);
    test_output!(Fluid<Undefined>: freezing_temperature, pg: 252.581_754_953_058_38, water: Err);
    test_output!(Fluid<Undefined>: gwp20, r32: 2330.0, water: Err);
    test_output!(Fluid<Undefined>: gwp100, r32: 675.0, water: Err);
    test_output!(Fluid<Undefined>: gwp500, r32: 205.0, water: Err);
    test_output!(Fluid<Undefined>: health_hazard, water: 0.0, incomp_water: Err);
    test_output!(Fluid<Undefined>: max_pressure, water: 1e9, incomp_water: Err);
    test_output!(Fluid<Undefined>: max_temperature, water: 2e3, always_ok);
    test_output!(Fluid<Undefined>: min_pressure, water: 611.654_800_896_868_4, incomp_water: Err);
    test_output!(Fluid<Undefined>: min_temperature, water: 273.16, always_ok);
    test_output!(Fluid<Undefined>: molar_mass, water: 0.018_015_268, incomp_water: Err);
    test_output!(Fluid<Undefined>: odp, r22: 0.05, water: Err, incomp_water: Err);
    test_output!(Fluid<Undefined>: physical_hazard, water: 0.0, incomp_water: Err);
    test_output!(Fluid<Undefined>: reducing_density, water: 322.0, incomp_water: Err);
    test_output!(Fluid<Undefined>: reducing_molar_density, water: 17_873.727_995_609_06, incomp_water: Err);
    test_output!(Fluid<Undefined>: reducing_pressure, water: 22.064e6, incomp_water: Err);
    test_output!(Fluid<Undefined>: reducing_temperature, water: 647.096, incomp_water: Err);
    test_output!(Fluid<Undefined>: triple_pressure, water: 611.654_800_896_868_4, incomp_water: Err);
    test_output!(Fluid<Undefined>: triple_temperature, water: 273.16, incomp_water: Err);

    #[rstest]
    fn in_state_valid_inputs_returns_ok(
        water: Fluid<Undefined>,
        temperature: FluidInput,
        pressure: FluidInput,
    ) {
        assert!(water.in_state(temperature, pressure).is_ok());
    }

    #[rstest]
    fn in_state_same_inputs_returns_err(water: Fluid<Undefined>, pressure: FluidInput) {
        assert_eq!(
            water.in_state(pressure, pressure).unwrap_err(),
            FluidStateError::InvalidInputPair(pressure.key, pressure.key)
        );
    }

    #[rstest]
    fn in_state_invalid_inputs_returns_err(
        water: Fluid<Undefined>,
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
        water: Fluid<Undefined>,
        temperature: FluidInput,
        negative_pressure: FluidInput,
    ) {
        assert!(matches!(
            water.in_state(temperature, negative_pressure).unwrap_err(),
            FluidStateError::UpdateFailed(_)
        ));
    }

    #[rstest]
    fn clone_returns_new_instance(water: Fluid<Undefined>) {
        let clone = water.clone();
        assert_eq!(clone, water);
        assert_eq!(clone.trivial_outputs, water.trivial_outputs);
    }
}
