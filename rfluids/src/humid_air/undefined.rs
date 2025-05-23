use super::{HumidAir, StateResult};
use crate::{io::HumidAirInput, state_variant::Undefined};
use std::{collections::HashMap, marker::PhantomData};

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
    /// - `input1` -- first input property.
    /// - `input2` -- second input property.
    /// - `input3` -- third input property.
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
    /// use rfluids::prelude::*;
    ///
    /// // After creation the `HumidAir` instance has `Undefined` state variant
    /// let mut humid_air: HumidAir<Undefined> = HumidAir::new();
    ///
    /// // Calling `in_state` method will move the initial value and
    /// // perform conversion between `Undefined` and `Defined` state variants
    /// // (since `Defined` is the default state variant, it can be omitted)
    /// let mut humid_air: HumidAir = humid_air.in_state(
    ///     HumidAirInput::altitude(0.0)?,
    ///     HumidAirInput::temperature(293.15),
    ///     HumidAirInput::rel_humidity(0.5),
    /// )?;
    ///
    /// // The `HumidAir` instance now has `Defined` state variant
    /// // and it's thermodynamic state can be updated in place by calling `update` method
    /// // (which returns a mutable reference to the instance)
    /// let same_humid_air_in_new_state: StateResult<&mut HumidAir> = humid_air.update(
    ///     HumidAirInput::pressure(202_650.0),
    ///     HumidAirInput::temperature(313.15),
    ///     HumidAirInput::rel_humidity(0.75),
    /// );
    /// assert!(same_humid_air_in_new_state.is_ok());
    ///
    /// // Calling `in_state` method on `HumidAir<Defined>` will return
    /// // a new instance in the specified thermodynamic state
    /// let new_humid_air: StateResult<HumidAir> = humid_air.in_state(
    ///     HumidAirInput::pressure(405_300.0),
    ///     HumidAirInput::temperature(353.15),
    ///     HumidAirInput::rel_humidity(1.0),
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
    use crate::{error::HumidAirStateError, io::HumidAirInput};
    use rstest::*;

    #[fixture]
    fn altitude(#[default(0.0)] value: f64) -> HumidAirInput {
        HumidAirInput::altitude(value).unwrap()
    }

    #[fixture]
    fn pressure(#[default(101_325.0)] value: f64) -> HumidAirInput {
        HumidAirInput::pressure(value)
    }

    #[fixture]
    fn infinite_pressure(#[with(f64::INFINITY)] pressure: HumidAirInput) -> HumidAirInput {
        pressure
    }

    #[fixture]
    fn temperature(#[default(293.15)] value: f64) -> HumidAirInput {
        HumidAirInput::temperature(value)
    }

    #[fixture]
    fn rel_humidity(#[default(0.5)] value: f64) -> HumidAirInput {
        HumidAirInput::rel_humidity(value)
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
        rel_humidity: HumidAirInput,
    ) {
        assert!(
            humid_air
                .in_state(altitude, temperature, rel_humidity)
                .is_ok()
        );
    }

    #[rstest]
    fn in_state_same_inputs_returns_err(
        humid_air: HumidAir<Undefined>,
        altitude: HumidAirInput,
        pressure: HumidAirInput,
        rel_humidity: HumidAirInput,
    ) {
        assert!(matches!(
            humid_air
                .in_state(altitude, pressure, rel_humidity)
                .unwrap_err(),
            HumidAirStateError::InvalidInputs(_, _, _)
        ));
    }

    #[rstest]
    fn in_state_invalid_inputs_returns_err(
        humid_air: HumidAir<Undefined>,
        infinite_pressure: HumidAirInput,
        temperature: HumidAirInput,
        rel_humidity: HumidAirInput,
    ) {
        assert!(matches!(
            humid_air
                .in_state(infinite_pressure, temperature, rel_humidity)
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
