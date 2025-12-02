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
    /// # Arguments
    ///
    /// - `input1` -- first input property
    /// - `input2` -- second input property
    /// - `input3` -- third input property
    ///
    /// # Errors
    ///
    /// Returns a [`HumidAirStateError`](crate::humid_air::HumidAirStateError)
    /// for invalid inputs.
    ///
    /// # Examples
    ///
    /// ```
    /// use rfluids::humid_air::StateResult;
    /// use rfluids::prelude::*;
    ///
    /// // After creation, the `HumidAir` instance has `Undefined` state variant
    /// let mut humid_air: HumidAir<Undefined> = HumidAir::new();
    ///
    /// // Calling `in_state` will move the initial value and
    /// // perform conversion between `Undefined` and `Defined` state variants
    /// // (since `Defined` is the default state variant, it can be omitted)
    /// let mut humid_air: HumidAir = humid_air.in_state(
    ///     HumidAirInput::altitude(0.0)?,
    ///     HumidAirInput::temperature(293.15),
    ///     HumidAirInput::rel_humidity(0.5),
    /// )?;
    ///
    /// // The `HumidAir` instance now has `Defined` state variant
    /// // and its thermodynamic state can be updated in place by calling `update`
    /// // (which returns a mutable reference to the instance)
    /// let same_humid_air_in_new_state: StateResult<&mut HumidAir> = humid_air.update(
    ///     HumidAirInput::pressure(202_650.0),
    ///     HumidAirInput::temperature(313.15),
    ///     HumidAirInput::rel_humidity(0.75),
    /// );
    /// assert!(same_humid_air_in_new_state.is_ok());
    ///
    /// // Calling `in_state` on `HumidAir<Defined>` will return
    /// // a new instance in the specified thermodynamic state
    /// let new_humid_air: StateResult<HumidAir> = humid_air.in_state(
    ///     HumidAirInput::pressure(405_300.0),
    ///     HumidAirInput::temperature(353.15),
    ///     HumidAirInput::rel_humidity(1.0),
    /// );
    /// assert!(new_humid_air.is_ok());
    /// # Ok::<(), rfluids::Error>(())
    /// ```
    ///
    /// # See Also
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
    use crate::{humid_air::HumidAirStateError, io::HumidAirInput};
    use rstest::*;

    struct Context {
        altitude: HumidAirInput,
        pressure: HumidAirInput,
        temperature: HumidAirInput,
        rel_humidity: HumidAirInput,
    }

    #[fixture]
    fn ctx() -> Context {
        Context {
            altitude: HumidAirInput::altitude(0.0).unwrap(),
            pressure: HumidAirInput::pressure(101_325.0),
            temperature: HumidAirInput::temperature(293.15),
            rel_humidity: HumidAirInput::rel_humidity(0.5),
        }
    }

    #[test]
    fn new() {
        // Given
        let sut = HumidAir::new();

        // When
        let (outputs, update_request) = (sut.outputs.clone(), sut.update_request);

        // Then
        assert_eq!(sut, HumidAir::default());
        assert!(update_request.is_none());
        assert!(outputs.is_empty());
    }

    #[rstest]
    fn in_state_valid_inputs(ctx: Context) {
        // Given
        let Context {
            altitude,
            temperature,
            rel_humidity,
            ..
        } = ctx;
        let sut = HumidAir::new();

        // When
        let res = sut.in_state(altitude, temperature, rel_humidity);

        // Then
        assert!(res.is_ok());
    }

    #[rstest]
    fn in_state_same_inputs(ctx: Context) {
        // Given
        let Context {
            altitude,
            pressure,
            rel_humidity,
            ..
        } = ctx;
        let sut = HumidAir::new();

        // When
        let res = sut.in_state(altitude, pressure, rel_humidity);

        // Then
        assert!(matches!(
            res,
            Err(HumidAirStateError::InvalidInputs(_, _, _))
        ));
    }

    #[rstest]
    fn in_state_invalid_inputs(ctx: Context) {
        // Given
        let Context {
            temperature,
            rel_humidity,
            ..
        } = ctx;
        let infinite_pressure = HumidAirInput::pressure(f64::INFINITY);
        let sut = HumidAir::new();

        // When
        let res = sut.in_state(infinite_pressure, temperature, rel_humidity);

        // Then
        assert_eq!(res, Err(HumidAirStateError::InvalidInputValue));
    }

    #[test]
    fn clone() {
        // Given
        let sut = HumidAir::new();

        // When
        let clone = sut.clone();

        // Then
        assert_eq!(clone, sut);
    }
}
