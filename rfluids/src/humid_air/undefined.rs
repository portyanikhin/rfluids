use super::{HumidAir, StateResult};
use crate::io::humid_air_input::HumidAirInput;
use crate::state_variant::Undefined;
use std::collections::HashMap;
use std::marker::PhantomData;

impl HumidAir<Undefined> {
    /// Creates and returns a new [`HumidAir`] instance
    /// with [`Undefined`] state variant.
    #[must_use]
    pub fn new() -> Self {
        HumidAir {
            update_request: None,
            outputs: HashMap::new(),
            state: PhantomData,
        }
    }

    /// Updates the thermodynamic state and returns itself with
    /// [`Defined`](crate::state_variant::Defined) state variant.
    ///
    /// # Args
    ///
    /// - input1 -- first input property.
    /// - input2 -- second input property.
    /// - input3 -- third input property.
    ///
    /// # Errors
    ///
    /// For invalid inputs,
    /// a [`HumidAirStateError`](crate::error::HumidAirStateError) is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use rfluids::humid_air::StateResult;
    /// use rfluids::prelude::humid_air::*;
    /// use uom::si::length::meter;
    /// use uom::si::pressure::atmosphere;
    /// use uom::si::ratio::percent;
    /// use uom::si::thermodynamic_temperature::degree_celsius;
    ///
    /// // After creation the `HumidAir` instance has `Undefined` state variant
    /// let mut humid_air: HumidAir<Undefined> = HumidAir::new();
    ///
    /// // Calling `in_state` method will move the initial value and
    /// // perform conversion between `Undefined` and `Defined` state variants
    /// // (since `Defined` is the default state variant, it can be omitted)
    /// let mut humid_air: HumidAir = humid_air.in_state(
    ///     humid_air_input::altitude!(0.0, meter)?,
    ///     humid_air_input::temperature!(20.0, degree_celsius),
    ///     humid_air_input::rel_humidity!(50.0, percent),
    /// )?;
    ///
    /// // The `HumidAir` instance now has `Defined` state variant
    /// // and it's thermodynamic state can be updated in place by calling `update` method
    /// // (which returns a mutable reference to the instance)
    /// let same_humid_air_in_new_state: StateResult<&mut HumidAir> = humid_air.update(
    ///     humid_air_input::pressure!(2.0, atmosphere),
    ///     humid_air_input::temperature!(40.0, degree_celsius),
    ///     humid_air_input::rel_humidity!(75.0, percent),
    /// );
    /// assert!(same_humid_air_in_new_state.is_ok());
    ///
    /// // Calling `in_state` method on `HumidAir<Defined>` will return
    /// // a new instance in the specified thermodynamic state
    /// let new_humid_air: StateResult<HumidAir> = humid_air.in_state(
    ///     humid_air_input::pressure!(4.0, atmosphere),
    ///     humid_air_input::temperature!(80.0, degree_celsius),
    ///     humid_air_input::rel_humidity!(100.0, percent),
    /// );
    /// assert!(new_humid_air.is_ok());
    /// # Ok::<(), rfluids::error::Error>(())
    /// ```
    ///
    /// # See also
    ///
    /// - [`HumidAir::update`](crate::humid_air::HumidAir::update)
    /// - [`HumidAir::in_state`](crate::humid_air::HumidAir::in_state)
    pub fn in_state(
        mut self,
        input1: HumidAirInput,
        input2: HumidAirInput,
        input3: HumidAirInput,
    ) -> StateResult<HumidAir> {
        self.inner_update(input1, input2, input3)?;
        Ok(HumidAir {
            update_request: self.update_request,
            outputs: self.outputs,
            state: PhantomData,
        })
    }
}

impl Clone for HumidAir<Undefined> {
    fn clone(&self) -> Self {
        Self::new()
    }
}

impl Default for HumidAir<Undefined> {
    fn default() -> Self {
        Self::new()
    }
}

impl PartialEq for HumidAir<Undefined> {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::HumidAirStateError;
    use crate::io::humid_air_input;
    use rstest::*;
    use uom::si::length::meter;
    use uom::si::pressure::atmosphere;
    use uom::si::ratio::percent;
    use uom::si::thermodynamic_temperature::degree_celsius;

    #[fixture]
    fn altitude(#[default(0.0)] value: f64) -> HumidAirInput {
        humid_air_input::altitude!(value, meter).unwrap()
    }

    #[fixture]
    fn pressure(#[default(1.0)] value: f64) -> HumidAirInput {
        humid_air_input::pressure!(value, atmosphere)
    }

    #[fixture]
    fn infinite_pressure(#[with(f64::INFINITY)] pressure: HumidAirInput) -> HumidAirInput {
        pressure
    }

    #[fixture]
    fn temperature(#[default(20.0)] value: f64) -> HumidAirInput {
        humid_air_input::temperature!(value, degree_celsius)
    }

    #[fixture]
    fn relative_humidity(#[default(50.0)] value: f64) -> HumidAirInput {
        humid_air_input::rel_humidity!(value, percent)
    }

    #[fixture]
    fn humid_air() -> HumidAir<Undefined> {
        HumidAir::new()
    }

    #[test]
    fn new_returns_humid_air_instance_with_undefined_state() {
        let humid_air = HumidAir::new();
        assert!(humid_air.update_request.is_none());
        assert!(humid_air.outputs.is_empty());
        assert_eq!(humid_air, HumidAir::default());
    }

    #[rstest]
    fn in_state_valid_inputs_returns_ok(
        humid_air: HumidAir<Undefined>,
        altitude: HumidAirInput,
        temperature: HumidAirInput,
        relative_humidity: HumidAirInput,
    ) {
        assert!(
            humid_air
                .in_state(altitude, temperature, relative_humidity)
                .is_ok()
        );
    }

    #[rstest]
    fn in_state_same_inputs_returns_err(
        humid_air: HumidAir<Undefined>,
        altitude: HumidAirInput,
        pressure: HumidAirInput,
        relative_humidity: HumidAirInput,
    ) {
        assert!(matches!(
            humid_air
                .in_state(altitude, pressure, relative_humidity)
                .unwrap_err(),
            HumidAirStateError::InvalidInputs(_, _, _)
        ));
    }

    #[rstest]
    fn in_state_invalid_inputs_returns_err(
        humid_air: HumidAir<Undefined>,
        infinite_pressure: HumidAirInput,
        temperature: HumidAirInput,
        relative_humidity: HumidAirInput,
    ) {
        assert!(matches!(
            humid_air
                .in_state(infinite_pressure, temperature, relative_humidity)
                .unwrap_err(),
            HumidAirStateError::InvalidInputValue
        ));
    }

    #[rstest]
    fn clone_returns_new_instance(humid_air: HumidAir<Undefined>) {
        let clone = humid_air.clone();
        assert_eq!(clone, humid_air);
    }
}
