//! `HumidAir` input.
//!
//! # Examples
//!
//! ```
//! use rfluids::prelude::humid_air::*;
//! use rfluids::uom::si::pressure::atmosphere;
//!
//! let pressure = humid_air_input::pressure!(1.0, atmosphere);
//! let pressure_si = humid_air_input::pressure!(101325.0);
//! assert_eq!(pressure, pressure_si);
//! assert_eq!(pressure, HumidAirInput::pressure(Pressure::new::<atmosphere>(1.0)));
//! assert_eq!(pressure_si, HumidAirInput::pressure_si(101325.0));
//! ```

use super::{define_input, define_input_macro, HumidAirParam, Input};
use crate::uom::si::f64::{
    AvailableEnergy, MassDensity, Pressure, Ratio, SpecificHeatCapacity, SpecificVolume,
    ThermodynamicTemperature,
};

/// `HumidAir` input.
///
/// # Examples
///
/// ```
/// use rfluids::prelude::humid_air::*;
/// use rfluids::uom::si::pressure::atmosphere;
///
/// let pressure = humid_air_input::pressure!(1.0, atmosphere);
/// let pressure_si = humid_air_input::pressure!(101325.0);
/// assert_eq!(pressure, pressure_si);
/// assert_eq!(pressure, HumidAirInput::pressure(Pressure::new::<atmosphere>(1.0)));
/// assert_eq!(pressure_si, HumidAirInput::pressure_si(101325.0));
/// ```
pub type HumidAirInput = Input<HumidAirParam>;

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

    /// Mass density per unit of humid air _(SI units: kg humid air/m³)_.
    ///
    /// **NB.** It will be converted to the
    /// [`specific_volume`](crate::io::humid_air_input::HumidAirInput::specific_volume),
    /// since there is no specific [`HumidAirParam`] for this.
    ///
    /// # See also
    ///
    /// - [`humid_air_input::density!`](crate::io::humid_air_input::density) macro
    #[must_use]
    pub fn density(value: MassDensity) -> Self {
        Self::density_si(value.value)
    }

    /// Mass density per unit of humid air in SI units _(kg humid air/m³)_.
    ///
    /// **NB.** It will be converted to the
    /// [`specific_volume_si`](crate::io::humid_air_input::HumidAirInput::specific_volume_si),
    /// since there is no specific [`HumidAirParam`] for this.
    ///
    /// # See also
    ///
    /// - [`humid_air_input::density!`](crate::io::humid_air_input::density) macro
    #[must_use]
    pub fn density_si(value: f64) -> Self {
        Self::specific_volume_si(1.0 / value)
    }

    /// Mass density per unit of dry air _(SI units: kg dry air/m³)_.
    ///
    /// **NB.** It will be converted to the
    /// [`specific_volume_da`](crate::io::humid_air_input::HumidAirInput::specific_volume_da),
    /// since there is no specific [`HumidAirParam`] for this.
    ///
    /// # See also
    ///
    /// - [`humid_air_input::density_da!`](crate::io::humid_air_input::density_da) macro
    #[must_use]
    pub fn density_da(value: MassDensity) -> Self {
        Self::density_da_si(value.value)
    }

    /// Mass density per unit of dry air in SI units _(kg dry air/m³)_.
    ///
    /// **NB.** It will be converted to the
    /// [`specific_volume_da_si`](crate::io::humid_air_input::HumidAirInput::specific_volume_da_si),
    /// since there is no specific [`HumidAirParam`] for this.
    ///
    /// # See also
    ///
    /// - [`humid_air_input::density_da!`](crate::io::humid_air_input::density_da) macro
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
    humid_air,
    ratio,
    part_per_thousand
);

define_input_macro!(
    humid_air_input,
    HumidAirInput,
    density,
    MassDensity,
    humid_air,
    mass_density,
    gram_per_cubic_centimeter
);

define_input_macro!(
    humid_air_input,
    HumidAirInput,
    density_da,
    MassDensity,
    humid_air,
    mass_density,
    gram_per_cubic_centimeter
);

define_input_macro!(
    humid_air_input,
    HumidAirInput,
    dew_temperature,
    ThermodynamicTemperature,
    humid_air,
    thermodynamic_temperature,
    degree_celsius
);

define_input_macro!(
    humid_air_input,
    HumidAirInput,
    enthalpy,
    AvailableEnergy,
    humid_air,
    available_energy,
    kilojoule_per_kilogram
);

define_input_macro!(
    humid_air_input,
    HumidAirInput,
    enthalpy_da,
    AvailableEnergy,
    humid_air,
    available_energy,
    kilojoule_per_kilogram
);

define_input_macro!(
    humid_air_input,
    HumidAirInput,
    entropy,
    SpecificHeatCapacity,
    humid_air,
    specific_heat_capacity,
    kilojoule_per_kilogram_kelvin
);

define_input_macro!(
    humid_air_input,
    HumidAirInput,
    entropy_da,
    SpecificHeatCapacity,
    humid_air,
    specific_heat_capacity,
    kilojoule_per_kilogram_kelvin
);

define_input_macro!(
    humid_air_input,
    HumidAirInput,
    pressure,
    Pressure,
    humid_air,
    pressure,
    kilopascal
);

define_input_macro!(
    humid_air_input,
    HumidAirInput,
    rel_humidity,
    Ratio,
    humid_air,
    ratio,
    percent
);

define_input_macro!(
    humid_air_input,
    HumidAirInput,
    specific_volume,
    SpecificVolume,
    humid_air,
    specific_volume,
    cubic_meter_per_kilogram
);

define_input_macro!(
    humid_air_input,
    HumidAirInput,
    specific_volume_da,
    SpecificVolume,
    humid_air,
    specific_volume,
    cubic_meter_per_kilogram
);

define_input_macro!(
    humid_air_input,
    HumidAirInput,
    temperature,
    ThermodynamicTemperature,
    humid_air,
    thermodynamic_temperature,
    degree_celsius
);

define_input_macro!(
    humid_air_input,
    HumidAirInput,
    water_mole_fraction,
    Ratio,
    humid_air,
    ratio,
    part_per_thousand
);

define_input_macro!(
    humid_air_input,
    HumidAirInput,
    water_partial_pressure,
    Pressure,
    humid_air,
    pressure,
    kilopascal
);

define_input_macro!(
    humid_air_input,
    HumidAirInput,
    wet_bulb_temperature,
    ThermodynamicTemperature,
    humid_air,
    thermodynamic_temperature,
    degree_celsius
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::test_input;
    use crate::uom::si::available_energy::joule_per_kilogram;
    use crate::uom::si::mass_density::kilogram_per_cubic_meter;
    use crate::uom::si::pressure::pascal;
    use crate::uom::si::ratio::ratio;
    use crate::uom::si::specific_heat_capacity::joule_per_kilogram_kelvin;
    use crate::uom::si::specific_volume::cubic_meter_per_kilogram;
    use crate::uom::si::thermodynamic_temperature::kelvin;

    test_input!(abs_humidity, HumidAirInput, HumidAirParam::W, Ratio, ratio);

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
