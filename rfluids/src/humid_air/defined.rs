// cSpell:disable

use super::common::{cached_output, guard};
use super::{HumidAir, OutputResult, StateResult};
use crate::io::HumidAirParam;
use crate::io::humid_air_input::HumidAirInput;
use std::marker::PhantomData;
use uom::si::available_energy::joule_per_kilogram;
use uom::si::dynamic_viscosity::pascal_second;
use uom::si::f64::{
    AvailableEnergy, DynamicViscosity, MassDensity, Pressure, Ratio, SpecificHeatCapacity,
    SpecificVolume, ThermalConductivity, ThermodynamicTemperature,
};
use uom::si::pressure::pascal;
use uom::si::ratio::ratio;
use uom::si::specific_heat_capacity::joule_per_kilogram_kelvin;
use uom::si::specific_volume::cubic_meter_per_kilogram;
use uom::si::thermal_conductivity::watt_per_meter_kelvin;
use uom::si::thermodynamic_temperature::kelvin;

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
    define_output!(
        non_negative_output,
        abs_humidity,
        W,
        Ratio,
        "Absolute humidity",
        "SI units: kg water/kg dry air",
        Ratio::new::<ratio>
    );

    #[doc = output_doc!(
        "Mass density per unit of humid air = `1` / [`specific_volume`](crate::humid_air::HumidAir::specific_volume)",
        "SI units: kg humid air/m³"
    )]
    pub fn density(&mut self) -> OutputResult<MassDensity> {
        self.specific_volume().map(uom::si::Quantity::recip)
    }

    #[doc = output_doc!(
        "Mass density per unit of dry air = `1` / [`specific_volume_da`](crate::humid_air::HumidAir::specific_volume_da)",
        "SI units: kg dry air/m³"
    )]
    pub fn density_da(&mut self) -> OutputResult<MassDensity> {
        self.specific_volume_da().map(uom::si::Quantity::recip)
    }

    define_output!(
        positive_output,
        compressibility,
        Z,
        f64,
        "Compressibility factor",
        "dimensionless"
    );

    define_output!(
        positive_output,
        conductivity,
        Conductivity,
        ThermalConductivity,
        "Thermal conductivity",
        "SI units: W/m/K",
        ThermalConductivity::new::<watt_per_meter_kelvin>
    );

    define_output!(
        positive_output,
        dew_temperature,
        TDew,
        ThermodynamicTemperature,
        "Dew-point temperature",
        "SI units: K",
        ThermodynamicTemperature::new::<kelvin>
    );

    define_output!(
        positive_output,
        dynamic_viscosity,
        DynamicViscosity,
        DynamicViscosity,
        "Dynamic viscosity",
        "SI units: Pa·s",
        DynamicViscosity::new::<pascal_second>
    );

    define_output!(
        output,
        enthalpy,
        Hha,
        AvailableEnergy,
        "Specific enthalpy per unit of humid air",
        "SI units: J/kg humid air",
        AvailableEnergy::new::<joule_per_kilogram>
    );

    define_output!(
        output,
        enthalpy_da,
        Hda,
        AvailableEnergy,
        "Specific enthalpy per unit of dry air",
        "SI units: J/kg dry air",
        AvailableEnergy::new::<joule_per_kilogram>
    );

    define_output!(
        output,
        entropy,
        Sha,
        SpecificHeatCapacity,
        "Specific entropy per unit of humid air",
        "SI units: J/kg humid air/K",
        SpecificHeatCapacity::new::<joule_per_kilogram_kelvin>
    );

    define_output!(
        output,
        entropy_da,
        Sda,
        SpecificHeatCapacity,
        "Specific entropy per unit of dry air",
        "SI units: J/kg dry air/K",
        SpecificHeatCapacity::new::<joule_per_kilogram_kelvin>
    );

    define_output!(
        positive_output,
        pressure,
        P,
        Pressure,
        "Pressure",
        "SI units: Pa",
        Pressure::new::<pascal>
    );

    #[doc = output_doc!(R, "Relative humidity", "SI units: dimensionless, from 0 to 1")]
    pub fn rel_humidity(&mut self) -> OutputResult<Ratio> {
        let key = HumidAirParam::R;
        self.output(key)
            .and_then(|value| guard(key, value, |x| (0.0..=1.0).contains(&x)))
            .map(Ratio::new::<ratio>)
    }

    define_output!(
        positive_output,
        specific_heat,
        Cpha,
        SpecificHeatCapacity,
        "Specific heat at constant pressure per unit of humid air",
        "SI units: J/kg humid air/K",
        SpecificHeatCapacity::new::<joule_per_kilogram_kelvin>
    );

    define_output!(
        positive_output,
        specific_heat_da,
        Cpda,
        SpecificHeatCapacity,
        "Specific heat at constant pressure per unit of dry air",
        "SI units: J/kg dry air/K",
        SpecificHeatCapacity::new::<joule_per_kilogram_kelvin>
    );

    define_output!(
        positive_output,
        specific_heat_const_volume,
        Cvha,
        SpecificHeatCapacity,
        "Specific heat at constant volume per unit of humid air",
        "SI units: J/kg humid air/K",
        SpecificHeatCapacity::new::<joule_per_kilogram_kelvin>
    );

    define_output!(
        positive_output,
        specific_heat_const_volume_da,
        Cvda,
        SpecificHeatCapacity,
        "Specific heat at constant volume per unit of dry air",
        "SI units: J/kg dry air/K",
        SpecificHeatCapacity::new::<joule_per_kilogram_kelvin>
    );

    define_output!(
        positive_output,
        specific_volume,
        Vha,
        SpecificVolume,
        "Specific volume per unit of humid air",
        "SI units: m³/kg humid air",
        SpecificVolume::new::<cubic_meter_per_kilogram>
    );

    define_output!(
        positive_output,
        specific_volume_da,
        Vda,
        SpecificVolume,
        "Specific volume per unit of dry air",
        "SI units: m³/kg dry air",
        SpecificVolume::new::<cubic_meter_per_kilogram>
    );

    define_output!(
        positive_output,
        temperature,
        T,
        ThermodynamicTemperature,
        "Dry-bulb temperature",
        "SI units: K",
        ThermodynamicTemperature::new::<kelvin>
    );

    define_output!(
        non_negative_output,
        water_mole_fraction,
        PsiW,
        Ratio,
        "Water mole fraction",
        "SI units: mol water/mol humid air",
        Ratio::new::<ratio>
    );

    define_output!(
        non_negative_output,
        water_partial_pressure,
        Pw,
        Pressure,
        "Partial pressure of water vapor",
        "SI units: Pa",
        Pressure::new::<pascal>
    );

    define_output!(
        positive_output,
        wet_bulb_temperature,
        TWetBulb,
        ThermodynamicTemperature,
        "Wet-bulb temperature",
        "SI units: K",
        ThermodynamicTemperature::new::<kelvin>
    );

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
    use crate::test::humid_air::test_output;
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

    test_output!(
        abs_humidity,
        humid_air,
        7.293_697_701_992_549e-3,
        invalid_humid_air
    );

    test_output!(
        density,
        humid_air,
        1.199_359_276_772_349_3,
        invalid_humid_air
    );

    test_output!(
        density_da,
        humid_air,
        1.190_674_854_323_549_2,
        invalid_humid_air
    );

    test_output!(
        f64,
        compressibility,
        humid_air,
        0.999_594_693_604_325_6,
        invalid_humid_air
    );

    test_output!(
        conductivity,
        humid_air,
        2.586_613_250_369_777_4e-2,
        invalid_humid_air
    );

    test_output!(
        dew_temperature,
        humid_air,
        282.424_425_814_578_2,
        invalid_humid_air
    );

    test_output!(
        dynamic_viscosity,
        humid_air,
        1.814_316_044_123_345e-5,
        invalid_humid_air
    );

    test_output!(
        enthalpy,
        humid_air,
        38_343.175_393_657_12,
        invalid_humid_air
    );

    test_output!(
        enthalpy_da,
        humid_air,
        38_622.838_923_912_93,
        invalid_humid_air
    );

    test_output!(entropy, humid_air, 138.956_660_316_574_3, invalid_humid_air);

    test_output!(
        entropy_da,
        humid_air,
        139.970_168_190_601_87,
        invalid_humid_air
    );

    test_output!(pressure, humid_air, 101_325.0, invalid_humid_air);
    test_output!(rel_humidity, humid_air, 0.5, invalid_humid_air);

    test_output!(
        specific_heat,
        humid_air,
        1_012.467_815_774_874_7,
        invalid_humid_air
    );

    test_output!(
        specific_heat_da,
        humid_air,
        1_019.852_449_956_133_3,
        invalid_humid_air
    );

    test_output!(
        specific_heat_const_volume,
        humid_air,
        722.687_189_706_325_1,
        invalid_humid_air
    );

    test_output!(
        specific_heat_const_volume_da,
        humid_air,
        727.958_251_601_145_5,
        invalid_humid_air
    );

    test_output!(
        specific_volume,
        humid_air,
        0.833_778_517_719_182_3,
        invalid_humid_air
    );

    test_output!(
        specific_volume_da,
        humid_air,
        0.839_859_846_177_841_6,
        invalid_humid_air
    );

    test_output!(temperature, humid_air, 293.15, invalid_humid_air);

    test_output!(
        water_mole_fraction,
        humid_air,
        1.159_130_506_217_982_9e-2,
        invalid_humid_air
    );

    test_output!(
        water_partial_pressure,
        humid_air,
        1_174.488_985_425_371,
        invalid_humid_air
    );

    test_output!(
        wet_bulb_temperature,
        humid_air,
        286.926_468_858_340_74,
        invalid_humid_air
    );

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
