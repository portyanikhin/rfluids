//! [`HumidAir`](crate::humid_air::HumidAir) input.
//!
//! # Examples
//!
//! ```
//! use rfluids::prelude::*;
//! use uom::si::pressure::atmosphere;
//!
//! let pressure = humid_air_input::pressure!(1.0, atmosphere);
//! let pressure_si = humid_air_input::pressure!(101325.0);
//! assert_eq!(pressure, pressure_si);
//! assert_eq!(pressure, HumidAirInput::pressure(Pressure::new::<atmosphere>(1.0)));
//! assert_eq!(pressure_si, HumidAirInput::pressure_si(101325.0));
//! ```

use super::{HumidAirParam, define_input, define_input_macro, impl_input};
use crate::error::AltitudeError;
use uom::si::f64::{
    AvailableEnergy, Length, MassDensity, Pressure, Ratio, SpecificHeatCapacity, SpecificVolume,
    ThermodynamicTemperature,
};

/// [`HumidAir`](crate::humid_air::HumidAir) input.
///
/// # Examples
///
/// ```
/// use rfluids::prelude::*;
/// use uom::si::pressure::atmosphere;
///
/// let pressure = humid_air_input::pressure!(1.0, atmosphere);
/// let pressure_si = humid_air_input::pressure!(101325.0);
/// assert_eq!(pressure, pressure_si);
/// assert_eq!(pressure, HumidAirInput::pressure(Pressure::new::<atmosphere>(1.0)));
/// assert_eq!(pressure_si, HumidAirInput::pressure_si(101325.0));
/// ```
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct HumidAirInput(pub(crate) HumidAirParam, pub(crate) f64);

impl_input!(HumidAirInput, HumidAirParam);

impl HumidAirInput {
    define_input!(
        humid_air_input,
        abs_humidity,
        HumidAirParam,
        W,
        Ratio,
        "Absolute humidity",
        "kg water/kg dry air"
    );

    /// Altitude above sea level _(SI units: m)_.
    ///
    /// **NB.** It will be converted to the
    /// [`pressure`](Self::pressure)
    /// _(according to ASHRAE Fundamentals Handbook)_,
    /// since there is no specific [`HumidAirParam`] for this.
    ///
    /// # Errors
    ///
    /// For values out of possible range _\[-5 000; 10 000\] m_,
    /// an [`AltitudeError`] will be returned.
    ///
    /// # See also
    ///
    /// - [`humid_air_input::altitude!`](crate::io::humid_air_input::altitude) macro
    /// - [`altitude_si`](Self::altitude_si)
    pub fn altitude(value: Length) -> Result<Self, AltitudeError> {
        Self::altitude_si(value.value)
    }

    /// Altitude above sea level in SI units _(m)_.
    ///
    /// **NB.** It will be converted to the
    /// [`pressure_si`](Self::pressure_si)
    /// _(according to ASHRAE Fundamentals Handbook)_,
    /// since there is no specific [`HumidAirParam`] for this.
    ///
    /// # Errors
    ///
    /// For values out of possible range _\[-5 000; 10 000\] m_,
    /// an [`AltitudeError`] will be returned.
    ///
    /// # See also
    ///
    /// - [`humid_air_input::altitude!`](crate::io::humid_air_input::altitude) macro
    /// - [`altitude`](Self::altitude)
    pub fn altitude_si(value: f64) -> Result<Self, AltitudeError> {
        if !(-5_000.0..=10_000.0).contains(&value) {
            return Err(AltitudeError::OutOfRange(value));
        }
        Ok(Self::pressure_si(
            101_325.0 * (1.0 - 2.255_77e-5 * value).powf(5.255_9),
        ))
    }

    /// Mass density per unit of humid air _(SI units: kg humid air/m³)_.
    ///
    /// **NB.** It will be converted to the
    /// [`specific_volume`](Self::specific_volume),
    /// since there is no specific [`HumidAirParam`] for this.
    ///
    /// # See also
    ///
    /// - [`humid_air_input::density!`](crate::io::humid_air_input::density) macro
    /// - [`density_si`](Self::density_si)
    #[must_use]
    pub fn density(value: MassDensity) -> Self {
        Self::density_si(value.value)
    }

    /// Mass density per unit of humid air in SI units _(kg humid air/m³)_.
    ///
    /// **NB.** It will be converted to the
    /// [`specific_volume_si`](Self::specific_volume_si),
    /// since there is no specific [`HumidAirParam`] for this.
    ///
    /// # See also
    ///
    /// - [`humid_air_input::density!`](crate::io::humid_air_input::density) macro
    /// - [`density`](Self::density)
    #[must_use]
    pub fn density_si(value: f64) -> Self {
        Self::specific_volume_si(1.0 / value)
    }

    /// Mass density per unit of dry air _(SI units: kg dry air/m³)_.
    ///
    /// **NB.** It will be converted to the
    /// [`specific_volume_da`](Self::specific_volume_da),
    /// since there is no specific [`HumidAirParam`] for this.
    ///
    /// # See also
    ///
    /// - [`humid_air_input::density_da!`](crate::io::humid_air_input::density_da) macro
    /// - [`density_da_si`](Self::density_da_si)
    #[must_use]
    pub fn density_da(value: MassDensity) -> Self {
        Self::density_da_si(value.value)
    }

    /// Mass density per unit of dry air in SI units _(kg dry air/m³)_.
    ///
    /// **NB.** It will be converted to the
    /// [`specific_volume_da_si`](Self::specific_volume_da_si),
    /// since there is no specific [`HumidAirParam`] for this.
    ///
    /// # See also
    ///
    /// - [`humid_air_input::density_da!`](crate::io::humid_air_input::density_da) macro
    /// - [`density_da`](Self::density_da)
    #[must_use]
    pub fn density_da_si(value: f64) -> Self {
        Self::specific_volume_da_si(1.0 / value)
    }

    define_input!(
        humid_air_input,
        dew_temperature,
        HumidAirParam,
        TDew,
        ThermodynamicTemperature,
        "Dew-point temperature",
        "K"
    );

    define_input!(
        humid_air_input,
        enthalpy,
        HumidAirParam,
        Hha,
        AvailableEnergy,
        "Specific enthalpy per unit of humid air",
        "J/kg humid air"
    );

    define_input!(
        humid_air_input,
        enthalpy_da,
        HumidAirParam,
        Hda,
        AvailableEnergy,
        "Specific enthalpy per unit of dry air",
        "J/kg dry air"
    );

    define_input!(
        humid_air_input,
        entropy,
        HumidAirParam,
        Sha,
        SpecificHeatCapacity,
        "Specific entropy per unit of humid air",
        "J/kg humid air/K"
    );

    define_input!(
        humid_air_input,
        entropy_da,
        HumidAirParam,
        Sda,
        SpecificHeatCapacity,
        "Specific entropy per unit of dry air",
        "J/kg dry air/K"
    );

    define_input!(
        humid_air_input,
        pressure,
        HumidAirParam,
        P,
        Pressure,
        "Pressure",
        "Pa"
    );

    define_input!(
        humid_air_input,
        rel_humidity,
        HumidAirParam,
        R,
        Ratio,
        "Relative humidity",
        "dimensionless, from 0 to 1"
    );

    define_input!(
        humid_air_input,
        specific_volume,
        HumidAirParam,
        Vha,
        SpecificVolume,
        "Specific volume per unit of humid air",
        "m³/kg humid air"
    );

    define_input!(
        humid_air_input,
        specific_volume_da,
        HumidAirParam,
        Vda,
        SpecificVolume,
        "Specific volume per unit of dry air",
        "m³/kg dry air"
    );

    define_input!(
        humid_air_input,
        temperature,
        HumidAirParam,
        T,
        ThermodynamicTemperature,
        "Dry-bulb temperature",
        "K"
    );

    define_input!(
        humid_air_input,
        water_mole_fraction,
        HumidAirParam,
        PsiW,
        Ratio,
        "Water mole fraction",
        "mol water/mol humid air"
    );

    define_input!(
        humid_air_input,
        water_partial_pressure,
        HumidAirParam,
        Pw,
        Pressure,
        "Partial pressure of water vapor",
        "Pa"
    );

    define_input!(
        humid_air_input,
        wet_bulb_temperature,
        HumidAirParam,
        TWetBulb,
        ThermodynamicTemperature,
        "Wet-bulb temperature",
        "K"
    );
}

define_input_macro!(
    humid_air_input,
    HumidAirInput,
    abs_humidity,
    Ratio,
    ratio,
    part_per_thousand
);

define_input_macro!(
    humid_air_input,
    HumidAirInput,
    altitude,
    Length,
    length,
    meter
);

define_input_macro!(
    humid_air_input,
    HumidAirInput,
    density,
    MassDensity,
    mass_density,
    gram_per_cubic_centimeter
);

define_input_macro!(
    humid_air_input,
    HumidAirInput,
    density_da,
    MassDensity,
    mass_density,
    gram_per_cubic_centimeter
);

define_input_macro!(
    humid_air_input,
    HumidAirInput,
    dew_temperature,
    ThermodynamicTemperature,
    thermodynamic_temperature,
    degree_celsius
);

define_input_macro!(
    humid_air_input,
    HumidAirInput,
    enthalpy,
    AvailableEnergy,
    available_energy,
    kilojoule_per_kilogram
);

define_input_macro!(
    humid_air_input,
    HumidAirInput,
    enthalpy_da,
    AvailableEnergy,
    available_energy,
    kilojoule_per_kilogram
);

define_input_macro!(
    humid_air_input,
    HumidAirInput,
    entropy,
    SpecificHeatCapacity,
    specific_heat_capacity,
    kilojoule_per_kilogram_kelvin
);

define_input_macro!(
    humid_air_input,
    HumidAirInput,
    entropy_da,
    SpecificHeatCapacity,
    specific_heat_capacity,
    kilojoule_per_kilogram_kelvin
);

define_input_macro!(
    humid_air_input,
    HumidAirInput,
    pressure,
    Pressure,
    pressure,
    kilopascal
);

define_input_macro!(
    humid_air_input,
    HumidAirInput,
    rel_humidity,
    Ratio,
    ratio,
    percent
);

define_input_macro!(
    humid_air_input,
    HumidAirInput,
    specific_volume,
    SpecificVolume,
    specific_volume,
    cubic_meter_per_kilogram
);

define_input_macro!(
    humid_air_input,
    HumidAirInput,
    specific_volume_da,
    SpecificVolume,
    specific_volume,
    cubic_meter_per_kilogram
);

define_input_macro!(
    humid_air_input,
    HumidAirInput,
    temperature,
    ThermodynamicTemperature,
    thermodynamic_temperature,
    degree_celsius
);

define_input_macro!(
    humid_air_input,
    HumidAirInput,
    water_mole_fraction,
    Ratio,
    ratio,
    part_per_thousand
);

define_input_macro!(
    humid_air_input,
    HumidAirInput,
    water_partial_pressure,
    Pressure,
    pressure,
    kilopascal
);

define_input_macro!(
    humid_air_input,
    HumidAirInput,
    wet_bulb_temperature,
    ThermodynamicTemperature,
    thermodynamic_temperature,
    degree_celsius
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::io::Input;
    use crate::test::{assert_relative_eq, test_input};
    use rstest::*;
    use uom::si::available_energy::joule_per_kilogram;
    use uom::si::length::meter;
    use uom::si::mass_density::kilogram_per_cubic_meter;
    use uom::si::pressure::pascal;
    use uom::si::ratio::ratio;
    use uom::si::specific_heat_capacity::joule_per_kilogram_kelvin;
    use uom::si::specific_volume::cubic_meter_per_kilogram;
    use uom::si::thermodynamic_temperature::kelvin;

    test_input!(abs_humidity, HumidAirInput, HumidAirParam::W, Ratio, ratio);

    #[rstest]
    #[case(10_000.0, 26_436.098_351_416_622)]
    #[case(0.0, 101_325.0)]
    #[case(-5_000.0, 177_687.447_332_308_8)]
    fn altitude_valid_value_returns_ok(#[case] altitude: f64, #[case] pressure: f64) {
        let sut = HumidAirInput::altitude(Length::new::<meter>(altitude)).unwrap();
        assert_eq!(sut.key(), HumidAirParam::P);
        assert_relative_eq!(sut.si_value(), pressure);
        assert_eq!(sut, HumidAirInput::altitude_si(altitude).unwrap());
        assert_eq!(sut, altitude!(altitude, meter).unwrap());
    }

    #[rstest]
    #[case(10_000.0 + 1e-6)]
    #[case(-5_000.0 - 1e-6)]
    fn altitude_invalid_value_returns_err(#[case] invalid_value: f64) {
        let result = HumidAirInput::altitude(Length::new::<meter>(invalid_value));
        assert_eq!(
            result.unwrap_err(),
            AltitudeError::OutOfRange(invalid_value)
        );
    }

    #[test]
    fn density_returns_expected_key_and_si_value() {
        let sut = HumidAirInput::density(MassDensity::new::<kilogram_per_cubic_meter>(2.0));
        assert_eq!(sut.key(), HumidAirParam::Vha);
        assert_eq!(sut.si_value(), 0.5);
        assert_eq!(sut, HumidAirInput::density_si(2.0));
        assert_eq!(sut, density!(2.0, kilogram_per_cubic_meter));
        assert_eq!(sut, density!(2.0));
    }

    #[test]
    fn density_da_returns_expected_key_and_si_value() {
        let sut = HumidAirInput::density_da(MassDensity::new::<kilogram_per_cubic_meter>(2.0));
        assert_eq!(sut.key(), HumidAirParam::Vda);
        assert_eq!(sut.si_value(), 0.5);
        assert_eq!(sut, HumidAirInput::density_da_si(2.0));
        assert_eq!(sut, density_da!(2.0, kilogram_per_cubic_meter));
        assert_eq!(sut, density_da!(2.0));
    }

    test_input!(
        dew_temperature,
        HumidAirInput,
        HumidAirParam::TDew,
        ThermodynamicTemperature,
        kelvin
    );

    test_input!(
        enthalpy,
        HumidAirInput,
        HumidAirParam::Hha,
        AvailableEnergy,
        joule_per_kilogram
    );

    test_input!(
        enthalpy_da,
        HumidAirInput,
        HumidAirParam::Hda,
        AvailableEnergy,
        joule_per_kilogram
    );

    test_input!(
        entropy,
        HumidAirInput,
        HumidAirParam::Sha,
        SpecificHeatCapacity,
        joule_per_kilogram_kelvin
    );

    test_input!(
        entropy_da,
        HumidAirInput,
        HumidAirParam::Sda,
        SpecificHeatCapacity,
        joule_per_kilogram_kelvin
    );

    test_input!(pressure, HumidAirInput, HumidAirParam::P, Pressure, pascal);

    test_input!(rel_humidity, HumidAirInput, HumidAirParam::R, Ratio, ratio);

    test_input!(
        specific_volume,
        HumidAirInput,
        HumidAirParam::Vha,
        SpecificVolume,
        cubic_meter_per_kilogram
    );

    test_input!(
        specific_volume_da,
        HumidAirInput,
        HumidAirParam::Vda,
        SpecificVolume,
        cubic_meter_per_kilogram
    );

    test_input!(
        temperature,
        HumidAirInput,
        HumidAirParam::T,
        ThermodynamicTemperature,
        kelvin
    );

    test_input!(
        water_mole_fraction,
        HumidAirInput,
        HumidAirParam::PsiW,
        Ratio,
        ratio
    );

    test_input!(
        water_partial_pressure,
        HumidAirInput,
        HumidAirParam::Pw,
        Pressure,
        pascal
    );

    test_input!(
        wet_bulb_temperature,
        HumidAirInput,
        HumidAirParam::TWetBulb,
        ThermodynamicTemperature,
        kelvin
    );
}
