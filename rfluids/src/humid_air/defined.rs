// cSpell:disable

use super::{
    HumidAir, OutputResult, StateResult,
    common::{cached_output, guard},
};
use crate::io::{HumidAirInput, HumidAirParam};
use std::marker::PhantomData;

impl HumidAir {
    /// Absolute humidity **\[kg water/kg dry air\]**.
    ///
    /// # Errors
    ///
    /// If it's not available or calculation is failed,
    /// a [`HumidAirOutputError`](crate::error::HumidAirOutputError) is returned.
    pub fn abs_humidity(&mut self) -> OutputResult<f64> {
        self.non_negative_output(HumidAirParam::W)
    }

    /// Mass density per unit of humid air =
    /// `1` / [`specific_volume`](crate::humid_air::HumidAir::specific_volume)
    /// **\[kg humid air/m³\]**.
    ///
    /// # Errors
    ///
    /// If it's not available or calculation is failed,
    /// a [`HumidAirOutputError`](crate::error::HumidAirOutputError) is returned.
    pub fn density(&mut self) -> OutputResult<f64> {
        self.specific_volume().map(|value| 1.0 / value)
    }

    /// Mass density per unit of dry air =
    /// `1` / [`specific_volume_da`](crate::humid_air::HumidAir::specific_volume_da)
    /// **\[kg dry air/m³\]**.
    ///
    /// # Errors
    ///
    /// If it's not available or calculation is failed,
    /// a [`HumidAirOutputError`](crate::error::HumidAirOutputError) is returned.
    pub fn density_da(&mut self) -> OutputResult<f64> {
        self.specific_volume_da().map(|value| 1.0 / value)
    }

    /// Compressibility factor **\[dimensionless\]**.
    ///
    /// # Errors
    ///
    /// If it's not available or calculation is failed,
    /// a [`HumidAirOutputError`](crate::error::HumidAirOutputError) is returned.
    pub fn compressibility(&mut self) -> OutputResult<f64> {
        self.positive_output(HumidAirParam::Z)
    }

    /// Thermal conductivity **\[W/m/K\]**.
    ///
    /// # Errors
    ///
    /// If it's not available or calculation is failed,
    /// a [`HumidAirOutputError`](crate::error::HumidAirOutputError) is returned.
    pub fn conductivity(&mut self) -> OutputResult<f64> {
        self.positive_output(HumidAirParam::Conductivity)
    }

    /// Dew-point temperature **\[K\]**.
    ///
    /// # Errors
    ///
    /// If it's not available or calculation is failed,
    /// a [`HumidAirOutputError`](crate::error::HumidAirOutputError) is returned.
    pub fn dew_temperature(&mut self) -> OutputResult<f64> {
        self.positive_output(HumidAirParam::TDew)
    }

    /// Dynamic viscosity **\[Pa·s\]**.
    ///
    /// # Errors
    ///
    /// If it's not available or calculation is failed,
    /// a [`HumidAirOutputError`](crate::error::HumidAirOutputError) is returned.
    pub fn dynamic_viscosity(&mut self) -> OutputResult<f64> {
        self.positive_output(HumidAirParam::DynamicViscosity)
    }

    /// Specific enthalpy per unit of humid air **\[J/kg humid air\]**.
    ///
    /// # Errors
    ///
    /// If it's not available or calculation is failed,
    /// a [`HumidAirOutputError`](crate::error::HumidAirOutputError) is returned.
    pub fn enthalpy(&mut self) -> OutputResult<f64> {
        self.output(HumidAirParam::Hha)
    }

    /// Specific enthalpy per unit of dry air **\[J/kg dry air\]**.
    ///
    /// # Errors
    ///
    /// If it's not available or calculation is failed,
    /// a [`HumidAirOutputError`](crate::error::HumidAirOutputError) is returned.
    pub fn enthalpy_da(&mut self) -> OutputResult<f64> {
        self.output(HumidAirParam::Hda)
    }

    /// Specific entropy per unit of humid air **\[J/kg humid air/K\]**.
    ///
    /// # Errors
    ///
    /// If it's not available or calculation is failed,
    /// a [`HumidAirOutputError`](crate::error::HumidAirOutputError) is returned.
    pub fn entropy(&mut self) -> OutputResult<f64> {
        self.output(HumidAirParam::Sha)
    }

    /// Specific entropy per unit of dry air **\[J/kg dry air/K\]**.
    ///
    /// # Errors
    ///
    /// If it's not available or calculation is failed,
    /// a [`HumidAirOutputError`](crate::error::HumidAirOutputError) is returned.
    pub fn entropy_da(&mut self) -> OutputResult<f64> {
        self.output(HumidAirParam::Sda)
    }

    /// Pressure **\[Pa\]**.
    ///
    /// # Errors
    ///
    /// If it's not available or calculation is failed,
    /// a [`HumidAirOutputError`](crate::error::HumidAirOutputError) is returned.
    pub fn pressure(&mut self) -> OutputResult<f64> {
        self.positive_output(HumidAirParam::P)
    }

    /// Relative humidity **\[dimensionless, from 0 to 1\]**.
    ///
    /// # Errors
    ///
    /// If it's not available or calculation is failed,
    /// a [`HumidAirOutputError`](crate::error::HumidAirOutputError) is returned.
    pub fn rel_humidity(&mut self) -> OutputResult<f64> {
        let key = HumidAirParam::R;
        self.output(key)
            .and_then(|value| guard(key, value, |x| (0.0..=1.0).contains(&x)))
    }

    /// Specific heat at constant pressure per unit of humid air **\[J/kg humid air/K\]**.
    ///
    /// # Errors
    ///
    /// If it's not available or calculation is failed,
    /// a [`HumidAirOutputError`](crate::error::HumidAirOutputError) is returned.
    pub fn specific_heat(&mut self) -> OutputResult<f64> {
        self.positive_output(HumidAirParam::Cpha)
    }

    /// Specific heat at constant pressure per unit of dry air **\[J/kg dry air/K\]**.
    ///
    /// # Errors
    ///
    /// If it's not available or calculation is failed,
    /// a [`HumidAirOutputError`](crate::error::HumidAirOutputError) is returned.
    pub fn specific_heat_da(&mut self) -> OutputResult<f64> {
        self.positive_output(HumidAirParam::Cpda)
    }

    /// Specific heat at constant volume per unit of humid air **\[J/kg humid air/K\]**.
    ///
    /// # Errors
    ///
    /// If it's not available or calculation is failed,
    /// a [`HumidAirOutputError`](crate::error::HumidAirOutputError) is returned.
    pub fn specific_heat_const_volume(&mut self) -> OutputResult<f64> {
        self.positive_output(HumidAirParam::Cvha)
    }

    /// Specific heat at constant volume per unit of dry air **\[J/kg dry air/K\]**.
    ///
    /// # Errors
    ///
    /// If it's not available or calculation is failed,
    /// a [`HumidAirOutputError`](crate::error::HumidAirOutputError) is returned.
    pub fn specific_heat_const_volume_da(&mut self) -> OutputResult<f64> {
        self.positive_output(HumidAirParam::Cvda)
    }

    /// Specific volume per unit of humid air **\[m³/kg humid air\]**.
    ///
    /// # Errors
    ///
    /// If it's not available or calculation is failed,
    /// a [`HumidAirOutputError`](crate::error::HumidAirOutputError) is returned.
    pub fn specific_volume(&mut self) -> OutputResult<f64> {
        self.positive_output(HumidAirParam::Vha)
    }

    /// Specific volume per unit of dry air **\[m³/kg dry air\]**.
    ///
    /// # Errors
    ///
    /// If it's not available or calculation is failed,
    /// a [`HumidAirOutputError`](crate::error::HumidAirOutputError) is returned.
    pub fn specific_volume_da(&mut self) -> OutputResult<f64> {
        self.positive_output(HumidAirParam::Vda)
    }

    /// Dry-bulb temperature **\[K\]**.
    ///
    /// # Errors
    ///
    /// If it's not available or calculation is failed,
    /// a [`HumidAirOutputError`](crate::error::HumidAirOutputError) is returned.
    pub fn temperature(&mut self) -> OutputResult<f64> {
        self.positive_output(HumidAirParam::T)
    }

    /// Water mole fraction **\[mol water/mol humid air\]**.
    ///
    /// # Errors
    ///
    /// If it's not available or calculation is failed,
    /// a [`HumidAirOutputError`](crate::error::HumidAirOutputError) is returned.
    pub fn water_mole_fraction(&mut self) -> OutputResult<f64> {
        self.non_negative_output(HumidAirParam::PsiW)
    }

    /// Partial pressure of water vapor **\[Pa\]**.
    ///
    /// # Errors
    ///
    /// If it's not available or calculation is failed,
    /// a [`HumidAirOutputError`](crate::error::HumidAirOutputError) is returned.
    pub fn water_partial_pressure(&mut self) -> OutputResult<f64> {
        self.non_negative_output(HumidAirParam::Pw)
    }

    /// Wet-bulb temperature **\[K\]**.
    ///
    /// # Errors
    ///
    /// If it's not available or calculation is failed,
    /// a [`HumidAirOutputError`](crate::error::HumidAirOutputError) is returned.
    pub fn wet_bulb_temperature(&mut self) -> OutputResult<f64> {
        self.positive_output(HumidAirParam::TWetBulb)
    }

    /// Updates the thermodynamic state and returns a mutable reference to itself.
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
    use crate::{error::HumidAirStateError, test::test_output};
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
    fn relative_humidity(#[default(0.5)] value: f64) -> HumidAirInput {
        HumidAirInput::rel_humidity(value)
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

    #[fixture]
    fn invalid_humid_air(
        #[with(-1.0)] pressure: HumidAirInput,
        #[with(-300.0)] temperature: HumidAirInput,
        #[with(150.0)] relative_humidity: HumidAirInput,
    ) -> HumidAir {
        HumidAir::new()
            .in_state(pressure, temperature, relative_humidity)
            .unwrap()
    }

    test_output!(HumidAir: abs_humidity, humid_air: 7.293_697_701_992_549e-3, invalid_humid_air: Err);
    test_output!(HumidAir: density, humid_air: 1.199_359_276_772_349_3, invalid_humid_air: Err);
    test_output!(HumidAir: density_da, humid_air: 1.190_674_854_323_549_2, invalid_humid_air: Err);
    test_output!(HumidAir: compressibility, humid_air: 0.999_594_693_604_325_6, invalid_humid_air: Err);
    test_output!(HumidAir: conductivity, humid_air: 2.586_613_250_369_777_4e-2, invalid_humid_air: Err);
    test_output!(HumidAir: dew_temperature, humid_air: 282.424_425_814_578_2, invalid_humid_air: Err);
    test_output!(HumidAir: dynamic_viscosity, humid_air: 1.814_316_044_123_345e-5, invalid_humid_air: Err);
    test_output!(HumidAir: enthalpy, humid_air: 38_343.175_393_657_12, invalid_humid_air: Err);
    test_output!(HumidAir: enthalpy_da, humid_air: 38_622.838_923_912_93, invalid_humid_air: Err);
    test_output!(HumidAir: entropy, humid_air: 138.956_660_316_574_3, invalid_humid_air: Err);
    test_output!(HumidAir: entropy_da, humid_air: 139.970_168_190_601_87, invalid_humid_air: Err);
    test_output!(HumidAir: pressure, humid_air: 101_325.0, invalid_humid_air: Err);
    test_output!(HumidAir: rel_humidity, humid_air: 0.5, invalid_humid_air: Err);
    test_output!(HumidAir: specific_heat, humid_air: 1_012.467_815_774_874_7, invalid_humid_air: Err);
    test_output!(HumidAir: specific_heat_da, humid_air: 1_019.852_449_956_133_3, invalid_humid_air: Err);
    test_output!(HumidAir: specific_heat_const_volume, humid_air: 722.687_189_706_325_1, invalid_humid_air: Err);
    test_output!(HumidAir: specific_heat_const_volume_da, humid_air: 727.958_251_601_145_5, invalid_humid_air: Err);
    test_output!(HumidAir: specific_volume, humid_air: 0.833_778_517_719_182_3, invalid_humid_air: Err);
    test_output!(HumidAir: specific_volume_da, humid_air: 0.839_859_846_177_841_6, invalid_humid_air: Err);
    test_output!(HumidAir: temperature, humid_air: 293.15, invalid_humid_air: Err);
    test_output!(HumidAir: water_mole_fraction, humid_air: 1.159_130_506_217_982_9e-2, invalid_humid_air: Err);
    test_output!(HumidAir: water_partial_pressure, humid_air: 1_174.488_985_425_371, invalid_humid_air: Err);
    test_output!(HumidAir: wet_bulb_temperature, humid_air: 286.926_468_858_340_74, invalid_humid_air: Err);

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
