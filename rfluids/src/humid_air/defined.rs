use std::marker::PhantomData;

use super::common::{cached_output, guard};
use super::{HumidAir, OutputResult, StateResult};
use crate::io::HumidAirParam;
use crate::io::humid_air_input::HumidAirInput;

macro_rules! output_doc {
    ($key:ident, $description:literal, $units_description:literal) => {
        concat!(
            $description,
            "\n_(key: [`",
            stringify!($key),
            "`](HumidAirParam::",
            stringify!($key),
            "), ",
            $units_description,
            ")_.\n\n",
            "# Errors\n\n",
            "If it's not available or calculation is failed,\n",
            "a [`HumidAirOutputError`](crate::error::HumidAirOutputError) is returned.",
        )
    };
    ($description:literal, $units_description:literal) => {
        concat!(
            $description,
            "\n_(",
            $units_description,
            ")_.\n\n",
            "# Errors\n\n",
            "If it's not available or calculation is failed,\n",
            "a [`HumidAirOutputError`](crate::error::HumidAirOutputError) is returned.",
        )
    };
}

macro_rules! define_output {
    (
        $method:ident,
        $name:ident,
        $key:ident,
        $type:ty,
        $description:literal,
        $units_description:literal
        $(, $map:expr)?
    ) => {
        #[doc = output_doc!($key, $description, $units_description)]
        pub fn $name(&mut self) -> OutputResult<$type> {
            self.$method(HumidAirParam::$key)
                $(.map($map))?
        }
    };
}

impl HumidAir {
    /// Updates the thermodynamic state and returns a mutable reference to itself.
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
    /// - [`HumidAir::in_state`](crate::humid_air::HumidAir::in_state)
    pub fn update(
        &mut self,
        input1: HumidAirInput,
        input2: HumidAirInput,
        input3: HumidAirInput,
    ) -> StateResult<&mut Self> {
        self.inner_update(input1, input2, input3)?;
        Ok(self)
    }

    /// Returns a new instance in the specified thermodynamic state.
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
    pub fn in_state(
        &self,
        input1: HumidAirInput,
        input2: HumidAirInput,
        input3: HumidAirInput,
    ) -> StateResult<Self> {
        HumidAir::new().in_state(input1, input2, input3)
    }

    fn positive_output(&mut self, key: HumidAirParam) -> OutputResult<f64> {
        self.output(key)
            .and_then(|value| guard(key, value, |x| x > 0.0))
    }

    fn non_negative_output(&mut self, key: HumidAirParam) -> OutputResult<f64> {
        self.output(key)
            .and_then(|value| guard(key, value, |x| x >= 0.0))
    }

    fn output(&mut self, key: HumidAirParam) -> OutputResult<f64> {
        cached_output(&mut self.outputs, key, self.update_request.unwrap())
            .and_then(|value| guard(key, value, f64::is_finite))
    }
}

impl Clone for HumidAir {
    fn clone(&self) -> Self {
        Self {
            update_request: self.update_request,
            outputs: self.outputs.clone(),
            state: PhantomData,
        }
    }
}

impl PartialEq for HumidAir {
    fn eq(&self, other: &Self) -> bool {
        self.update_request == other.update_request
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
    fn humid_air(
        altitude: HumidAirInput,
        temperature: HumidAirInput,
        relative_humidity: HumidAirInput,
    ) -> HumidAir {
        HumidAir::new()
            .in_state(altitude, temperature, relative_humidity)
            .unwrap()
    }

    #[rstest]
    fn update_valid_inputs_returns_ok(
        mut humid_air: HumidAir,
        pressure: HumidAirInput,
        temperature: HumidAirInput,
        relative_humidity: HumidAirInput,
    ) {
        assert!(
            humid_air
                .update(pressure, temperature, relative_humidity)
                .is_ok()
        );
    }

    #[rstest]
    fn update_same_inputs_returns_err(
        mut humid_air: HumidAir,
        altitude: HumidAirInput,
        pressure: HumidAirInput,
        relative_humidity: HumidAirInput,
    ) {
        assert!(matches!(
            humid_air
                .update(altitude, pressure, relative_humidity)
                .unwrap_err(),
            HumidAirStateError::InvalidInputs(_, _, _)
        ));
    }

    #[rstest]
    fn update_invalid_inputs_returns_err(
        mut humid_air: HumidAir,
        infinite_pressure: HumidAirInput,
        temperature: HumidAirInput,
        relative_humidity: HumidAirInput,
    ) {
        assert!(matches!(
            humid_air
                .update(infinite_pressure, temperature, relative_humidity)
                .unwrap_err(),
            HumidAirStateError::InvalidInputValue
        ));
    }

    #[rstest]
    fn in_state_valid_inputs_returns_ok(
        humid_air: HumidAir,
        pressure: HumidAirInput,
        temperature: HumidAirInput,
        relative_humidity: HumidAirInput,
    ) {
        assert!(
            humid_air
                .in_state(pressure, temperature, relative_humidity)
                .is_ok()
        );
    }

    #[rstest]
    fn in_state_same_inputs_returns_err(
        humid_air: HumidAir,
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
        humid_air: HumidAir,
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
    fn clone_returns_new_instance(humid_air: HumidAir) {
        let clone = humid_air.clone();
        assert_eq!(clone, humid_air);
        assert_eq!(clone.update_request, humid_air.update_request);
        assert_eq!(clone.outputs, humid_air.outputs);
    }
}
