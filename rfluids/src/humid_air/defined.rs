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
    /// a [`HumidAirOutputError`](crate::humid_air::HumidAirOutputError) is returned.
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
    /// a [`HumidAirOutputError`](crate::humid_air::HumidAirOutputError) is returned.
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
    /// a [`HumidAirOutputError`](crate::humid_air::HumidAirOutputError) is returned.
    pub fn density_da(&mut self) -> OutputResult<f64> {
        self.specific_volume_da().map(|value| 1.0 / value)
    }

    /// Compressibility factor **\[dimensionless\]**.
    ///
    /// # Errors
    ///
    /// If it's not available or calculation is failed,
    /// a [`HumidAirOutputError`](crate::humid_air::HumidAirOutputError) is returned.
    pub fn compressibility(&mut self) -> OutputResult<f64> {
        self.positive_output(HumidAirParam::Z)
    }

    /// Thermal conductivity **\[W/m/K\]**.
    ///
    /// # Errors
    ///
    /// If it's not available or calculation is failed,
    /// a [`HumidAirOutputError`](crate::humid_air::HumidAirOutputError) is returned.
    pub fn conductivity(&mut self) -> OutputResult<f64> {
        self.positive_output(HumidAirParam::Conductivity)
    }

    /// Dew-point temperature **\[K\]**.
    ///
    /// # Errors
    ///
    /// If it's not available or calculation is failed,
    /// a [`HumidAirOutputError`](crate::humid_air::HumidAirOutputError) is returned.
    pub fn dew_temperature(&mut self) -> OutputResult<f64> {
        self.positive_output(HumidAirParam::TDew)
    }

    /// Dynamic viscosity **\[Pa·s\]**.
    ///
    /// # Errors
    ///
    /// If it's not available or calculation is failed,
    /// a [`HumidAirOutputError`](crate::humid_air::HumidAirOutputError) is returned.
    pub fn dynamic_viscosity(&mut self) -> OutputResult<f64> {
        self.positive_output(HumidAirParam::DynamicViscosity)
    }

    /// Specific enthalpy per unit of humid air **\[J/kg humid air\]**.
    ///
    /// # Errors
    ///
    /// If it's not available or calculation is failed,
    /// a [`HumidAirOutputError`](crate::humid_air::HumidAirOutputError) is returned.
    pub fn enthalpy(&mut self) -> OutputResult<f64> {
        self.output(HumidAirParam::Hha)
    }

    /// Specific enthalpy per unit of dry air **\[J/kg dry air\]**.
    ///
    /// # Errors
    ///
    /// If it's not available or calculation is failed,
    /// a [`HumidAirOutputError`](crate::humid_air::HumidAirOutputError) is returned.
    pub fn enthalpy_da(&mut self) -> OutputResult<f64> {
        self.output(HumidAirParam::Hda)
    }

    /// Specific entropy per unit of humid air **\[J/kg humid air/K\]**.
    ///
    /// # Errors
    ///
    /// If it's not available or calculation is failed,
    /// a [`HumidAirOutputError`](crate::humid_air::HumidAirOutputError) is returned.
    pub fn entropy(&mut self) -> OutputResult<f64> {
        self.output(HumidAirParam::Sha)
    }

    /// Specific entropy per unit of dry air **\[J/kg dry air/K\]**.
    ///
    /// # Errors
    ///
    /// If it's not available or calculation is failed,
    /// a [`HumidAirOutputError`](crate::humid_air::HumidAirOutputError) is returned.
    pub fn entropy_da(&mut self) -> OutputResult<f64> {
        self.output(HumidAirParam::Sda)
    }

    /// Pressure **\[Pa\]**.
    ///
    /// # Errors
    ///
    /// If it's not available or calculation is failed,
    /// a [`HumidAirOutputError`](crate::humid_air::HumidAirOutputError) is returned.
    pub fn pressure(&mut self) -> OutputResult<f64> {
        self.positive_output(HumidAirParam::P)
    }

    /// Relative humidity **\[dimensionless, from 0 to 1\]**.
    ///
    /// # Errors
    ///
    /// If it's not available or calculation is failed,
    /// a [`HumidAirOutputError`](crate::humid_air::HumidAirOutputError) is returned.
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
    /// a [`HumidAirOutputError`](crate::humid_air::HumidAirOutputError) is returned.
    pub fn specific_heat(&mut self) -> OutputResult<f64> {
        self.positive_output(HumidAirParam::Cpha)
    }

    /// Specific heat at constant pressure per unit of dry air **\[J/kg dry air/K\]**.
    ///
    /// # Errors
    ///
    /// If it's not available or calculation is failed,
    /// a [`HumidAirOutputError`](crate::humid_air::HumidAirOutputError) is returned.
    pub fn specific_heat_da(&mut self) -> OutputResult<f64> {
        self.positive_output(HumidAirParam::Cpda)
    }

    /// Specific heat at constant volume per unit of humid air **\[J/kg humid air/K\]**.
    ///
    /// # Errors
    ///
    /// If it's not available or calculation is failed,
    /// a [`HumidAirOutputError`](crate::humid_air::HumidAirOutputError) is returned.
    pub fn specific_heat_const_volume(&mut self) -> OutputResult<f64> {
        self.positive_output(HumidAirParam::Cvha)
    }

    /// Specific heat at constant volume per unit of dry air **\[J/kg dry air/K\]**.
    ///
    /// # Errors
    ///
    /// If it's not available or calculation is failed,
    /// a [`HumidAirOutputError`](crate::humid_air::HumidAirOutputError) is returned.
    pub fn specific_heat_const_volume_da(&mut self) -> OutputResult<f64> {
        self.positive_output(HumidAirParam::Cvda)
    }

    /// Specific volume per unit of humid air **\[m³/kg humid air\]**.
    ///
    /// # Errors
    ///
    /// If it's not available or calculation is failed,
    /// a [`HumidAirOutputError`](crate::humid_air::HumidAirOutputError) is returned.
    pub fn specific_volume(&mut self) -> OutputResult<f64> {
        self.positive_output(HumidAirParam::Vha)
    }

    /// Specific volume per unit of dry air **\[m³/kg dry air\]**.
    ///
    /// # Errors
    ///
    /// If it's not available or calculation is failed,
    /// a [`HumidAirOutputError`](crate::humid_air::HumidAirOutputError) is returned.
    pub fn specific_volume_da(&mut self) -> OutputResult<f64> {
        self.positive_output(HumidAirParam::Vda)
    }

    /// Dry-bulb temperature **\[K\]**.
    ///
    /// # Errors
    ///
    /// If it's not available or calculation is failed,
    /// a [`HumidAirOutputError`](crate::humid_air::HumidAirOutputError) is returned.
    pub fn temperature(&mut self) -> OutputResult<f64> {
        self.positive_output(HumidAirParam::T)
    }

    /// Water mole fraction **\[mol water/mol humid air\]**.
    ///
    /// # Errors
    ///
    /// If it's not available or calculation is failed,
    /// a [`HumidAirOutputError`](crate::humid_air::HumidAirOutputError) is returned.
    pub fn water_mole_fraction(&mut self) -> OutputResult<f64> {
        self.non_negative_output(HumidAirParam::PsiW)
    }

    /// Partial pressure of water vapor **\[Pa\]**.
    ///
    /// # Errors
    ///
    /// If it's not available or calculation is failed,
    /// a [`HumidAirOutputError`](crate::humid_air::HumidAirOutputError) is returned.
    pub fn water_partial_pressure(&mut self) -> OutputResult<f64> {
        self.non_negative_output(HumidAirParam::Pw)
    }

    /// Wet-bulb temperature **\[K\]**.
    ///
    /// # Errors
    ///
    /// If it's not available or calculation is failed,
    /// a [`HumidAirOutputError`](crate::humid_air::HumidAirOutputError) is returned.
    pub fn wet_bulb_temperature(&mut self) -> OutputResult<f64> {
        self.positive_output(HumidAirParam::TWetBulb)
    }

    /// Updates the thermodynamic state and returns a mutable reference to itself.
    ///
    /// # Arguments
    ///
    /// - `input1` -- first input property
    /// - `input2` -- second input property
    /// - `input3` -- third input property
    ///
    /// # Errors
    ///
    /// For invalid inputs,
    /// a [`HumidAirStateError`](crate::humid_air::HumidAirStateError) is returned.
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
    /// # Ok::<(), rfluids::Error>(())
    /// ```
    ///
    /// # See Also
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
    /// # Arguments
    ///
    /// - `input1` -- first input property
    /// - `input2` -- second input property
    /// - `input3` -- third input property
    ///
    /// # Errors
    ///
    /// For invalid inputs,
    /// a [`HumidAirStateError`](crate::humid_air::HumidAirStateError) is returned.
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
    /// # Ok::<(), rfluids::Error>(())
    /// ```
    ///
    /// # See Also
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
    use crate::{
        humid_air::HumidAirStateError,
        test::{SutFactory, test_output},
    };
    use rstest::*;

    struct Context {
        altitude: HumidAirInput,
        pressure: HumidAirInput,
        temperature: HumidAirInput,
        relative_humidity: HumidAirInput,
        valid: (HumidAirInput, HumidAirInput, HumidAirInput),
        invalid: (HumidAirInput, HumidAirInput, HumidAirInput),
    }

    impl SutFactory<(HumidAirInput, HumidAirInput, HumidAirInput)> for Context {
        type Sut = HumidAir;

        fn sut(&self, payload: (HumidAirInput, HumidAirInput, HumidAirInput)) -> Self::Sut {
            HumidAir::new()
                .in_state(payload.0, payload.1, payload.2)
                .unwrap()
        }
    }

    #[fixture]
    fn ctx() -> Context {
        let altitude = HumidAirInput::altitude(0.0).unwrap();
        let pressure = HumidAirInput::pressure(101_325.0);
        let temperature = HumidAirInput::temperature(293.15);
        let relative_humidity = HumidAirInput::rel_humidity(0.5);
        Context {
            altitude,
            pressure,
            temperature,
            relative_humidity,
            valid: (altitude, temperature, relative_humidity),
            invalid: (
                HumidAirInput::pressure(-1.0),
                HumidAirInput::temperature(-1.0),
                HumidAirInput::rel_humidity(1.5),
            ),
        }
    }

    test_output!(abs_humidity, valid: 7.293_697_701_992_549e-3, invalid: Err);
    test_output!(density, valid: 1.199_359_276_772_349_3, invalid: Err);
    test_output!(density_da, valid: 1.190_674_854_323_549_2, invalid: Err);
    test_output!(compressibility, valid: 0.999_594_693_604_325_6, invalid: Err);
    test_output!(conductivity, valid: 2.586_613_250_369_777_4e-2, invalid: Err);
    test_output!(dew_temperature, valid: 282.424_425_814_578_2, invalid: Err);
    test_output!(dynamic_viscosity, valid: 1.814_316_044_123_345e-5, invalid: Err);
    test_output!(enthalpy, valid: 38_343.175_393_657_12, invalid: Err);
    test_output!(enthalpy_da, valid: 38_622.838_923_912_93, invalid: Err);
    test_output!(entropy, valid: 138.956_660_316_574_3, invalid: Err);
    test_output!(entropy_da, valid: 139.970_168_190_601_87, invalid: Err);
    test_output!(pressure, valid: 101_325.0, invalid: Err);
    test_output!(rel_humidity, valid: 0.5, invalid: Err);
    test_output!(specific_heat, valid: 1_012.467_815_774_874_7, invalid: Err);
    test_output!(specific_heat_da, valid: 1_019.852_449_956_133_3, invalid: Err);
    test_output!(specific_heat_const_volume, valid: 722.687_189_706_325_1, invalid: Err);
    test_output!(specific_heat_const_volume_da, valid: 727.958_251_601_145_5, invalid: Err);
    test_output!(specific_volume, valid: 0.833_778_517_719_182_3, invalid: Err);
    test_output!(specific_volume_da, valid: 0.839_859_846_177_841_6, invalid: Err);
    test_output!(temperature, valid: 293.15, invalid: Err);
    test_output!(water_mole_fraction, valid: 1.159_130_506_217_982_9e-2, invalid: Err);
    test_output!(water_partial_pressure, valid: 1_174.488_985_425_371, invalid: Err);
    test_output!(wet_bulb_temperature, valid: 286.926_468_858_340_74, invalid: Err);

    #[rstest]
    fn update_valid_inputs(ctx: Context) {
        // Given
        let Context {
            pressure,
            temperature,
            relative_humidity,
            valid,
            ..
        } = ctx;
        let mut sut = ctx.sut(valid);

        // When
        let res = sut.update(pressure, temperature, relative_humidity);

        // Then
        assert!(res.is_ok());
    }

    #[rstest]
    fn update_same_inputs(ctx: Context) {
        // Given
        let Context {
            altitude,
            pressure,
            relative_humidity,
            valid,
            ..
        } = ctx;
        let mut sut = ctx.sut(valid);

        // When
        let res = sut.update(altitude, pressure, relative_humidity);

        // Then
        assert!(matches!(
            res,
            Err(HumidAirStateError::InvalidInputs(_, _, _))
        ));
    }

    #[rstest]
    fn update_invalid_inputs(ctx: Context) {
        // Given
        let Context {
            temperature,
            relative_humidity,
            valid,
            ..
        } = ctx;
        let infinite_pressure = HumidAirInput::pressure(f64::INFINITY);
        let mut sut = ctx.sut(valid);

        // When
        let res = sut.update(infinite_pressure, temperature, relative_humidity);

        // Then
        assert_eq!(res, Err(HumidAirStateError::InvalidInputValue));
    }

    #[rstest]
    fn in_state_valid_inputs(ctx: Context) {
        // Given
        let Context {
            pressure,
            temperature,
            relative_humidity,
            valid,
            ..
        } = ctx;
        let sut = ctx.sut(valid);

        // When
        let res = sut.in_state(pressure, temperature, relative_humidity);

        // Then
        assert!(res.is_ok());
    }

    #[rstest]
    fn in_state_same_inputs(ctx: Context) {
        // Given
        let Context {
            altitude,
            pressure,
            relative_humidity,
            valid,
            ..
        } = ctx;
        let sut = ctx.sut(valid);

        // When
        let res = sut.in_state(altitude, pressure, relative_humidity);

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
            relative_humidity,
            valid,
            ..
        } = ctx;
        let infinite_pressure = HumidAirInput::pressure(f64::INFINITY);
        let sut = ctx.sut(valid);

        // When
        let res = sut.in_state(infinite_pressure, temperature, relative_humidity);

        // Then
        assert_eq!(res, Err(HumidAirStateError::InvalidInputValue));
    }

    #[rstest]
    fn clone(ctx: Context) {
        // Given
        let Context { valid, .. } = ctx;
        let sut = ctx.sut(valid);

        // When
        let clone = sut.clone();

        // Then
        assert_eq!(clone, sut);
        assert_eq!(clone.update_request, sut.update_request);
        assert_eq!(clone.outputs, sut.outputs);
    }
}
