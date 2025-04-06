//! [`Fluid`](crate::fluid::Fluid) input.
//!
//! # Examples
//!
//! ```
//! use rfluids::prelude::*;
//! use uom::si::pressure::atmosphere;
//!
//! let pressure = fluid_input::pressure!(1.0, atmosphere);
//! let pressure_si = fluid_input::pressure!(101325.0);
//! assert_eq!(pressure, pressure_si);
//! assert_eq!(pressure, FluidInput::pressure(Pressure::new::<atmosphere>(1.0)));
//! assert_eq!(pressure_si, FluidInput::pressure_si(101325.0));
//! ```

use super::{FluidParam, define_input, define_input_macro, impl_input};
use uom::si::f64::{
    AvailableEnergy, MassDensity, MolarConcentration, MolarEnergy, MolarHeatCapacity, Pressure,
    Ratio, SpecificHeatCapacity, SpecificVolume, ThermodynamicTemperature,
};

/// [`Fluid`](crate::fluid::Fluid) input.
///
/// # Examples
///
/// ```
/// use rfluids::prelude::*;
/// use uom::si::pressure::atmosphere;
///
/// let pressure = fluid_input::pressure!(1.0, atmosphere);
/// let pressure_si = fluid_input::pressure!(101325.0);
/// assert_eq!(pressure, pressure_si);
/// assert_eq!(pressure, FluidInput::pressure(Pressure::new::<atmosphere>(1.0)));
/// assert_eq!(pressure_si, FluidInput::pressure_si(101325.0));
/// ```
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct FluidInput(pub(crate) FluidParam, pub(crate) f64);

impl_input!(FluidInput, FluidParam);

impl FluidInput {
    define_input!(
        fluid_input,
        density,
        FluidParam,
        DMass,
        MassDensity,
        "Mass density",
        "kg/m³"
    );

    define_input!(
        fluid_input,
        enthalpy,
        FluidParam,
        HMass,
        AvailableEnergy,
        "Mass specific enthalpy",
        "J/kg"
    );

    define_input!(
        fluid_input,
        entropy,
        FluidParam,
        SMass,
        SpecificHeatCapacity,
        "Mass specific entropy",
        "J/kg/K"
    );

    define_input!(
        fluid_input,
        internal_energy,
        FluidParam,
        UMass,
        AvailableEnergy,
        "Mass specific internal energy",
        "J/kg"
    );

    define_input!(
        fluid_input,
        molar_density,
        FluidParam,
        DMolar,
        MolarConcentration,
        "Molar density",
        "mol/m³"
    );

    define_input!(
        fluid_input,
        molar_enthalpy,
        FluidParam,
        HMolar,
        MolarEnergy,
        "Molar specific enthalpy",
        "J/mol"
    );

    define_input!(
        fluid_input,
        molar_entropy,
        FluidParam,
        SMolar,
        MolarHeatCapacity,
        "Molar specific entropy",
        "J/mol/K"
    );

    define_input!(
        fluid_input,
        molar_internal_energy,
        FluidParam,
        UMolar,
        MolarEnergy,
        "Molar specific internal energy",
        "J/mol"
    );

    define_input!(
        fluid_input,
        pressure,
        FluidParam,
        P,
        Pressure,
        "Pressure",
        "Pa"
    );

    define_input!(
        fluid_input,
        quality,
        FluidParam,
        Q,
        Ratio,
        "Vapor quality",
        "dimensionless, from 0 to 1"
    );

    /// Specific volume _(SI units: m³/kg)_.
    ///
    /// **NB.** It will be converted to the [`density`](Self::density),
    /// since there is no specific [`FluidParam`] for this.
    ///
    /// # See also
    ///
    /// - [`fluid_input::specific_volume!`](crate::io::fluid_input::specific_volume) macro
    /// - [`specific_volume_si`](Self::specific_volume_si)
    #[must_use]
    pub fn specific_volume(value: SpecificVolume) -> Self {
        Self::specific_volume_si(value.value)
    }

    /// Specific volume in SI units _(m³/kg)_.
    ///
    /// **NB.** It will be converted to the [`density_si`](Self::density_si),
    /// since there is no specific [`FluidParam`] for this.
    ///
    /// # See also
    ///
    /// - [`fluid_input::specific_volume!`](crate::io::fluid_input::specific_volume) macro
    /// - [`specific_volume`](Self::specific_volume)
    #[must_use]
    pub fn specific_volume_si(value: f64) -> Self {
        Self::density_si(1.0 / value)
    }

    define_input!(
        fluid_input,
        temperature,
        FluidParam,
        T,
        ThermodynamicTemperature,
        "Temperature",
        "K"
    );
}

define_input_macro!(
    fluid_input,
    FluidInput,
    density,
    MassDensity,
    mass_density,
    gram_per_cubic_centimeter
);

define_input_macro!(
    fluid_input,
    FluidInput,
    enthalpy,
    AvailableEnergy,
    available_energy,
    kilojoule_per_kilogram
);

define_input_macro!(
    fluid_input,
    FluidInput,
    entropy,
    SpecificHeatCapacity,
    specific_heat_capacity,
    joule_per_kilogram_kelvin
);

define_input_macro!(
    fluid_input,
    FluidInput,
    internal_energy,
    AvailableEnergy,
    available_energy,
    kilojoule_per_kilogram
);

define_input_macro!(
    fluid_input,
    FluidInput,
    molar_density,
    MolarConcentration,
    molar_concentration,
    mole_per_cubic_meter
);

define_input_macro!(
    fluid_input,
    FluidInput,
    molar_enthalpy,
    MolarEnergy,
    molar_energy,
    kilojoule_per_mole
);

define_input_macro!(
    fluid_input,
    FluidInput,
    molar_entropy,
    MolarHeatCapacity,
    molar_heat_capacity,
    joule_per_kelvin_mole
);

define_input_macro!(
    fluid_input,
    FluidInput,
    molar_internal_energy,
    MolarEnergy,
    molar_energy,
    kilojoule_per_mole
);

define_input_macro!(
    fluid_input,
    FluidInput,
    pressure,
    Pressure,
    pressure,
    kilopascal
);

define_input_macro!(fluid_input, FluidInput, quality, Ratio, ratio, percent);

define_input_macro!(
    fluid_input,
    FluidInput,
    specific_volume,
    SpecificVolume,
    specific_volume,
    cubic_meter_per_kilogram
);

define_input_macro!(
    fluid_input,
    FluidInput,
    temperature,
    ThermodynamicTemperature,
    thermodynamic_temperature,
    degree_celsius
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{io::Input, test::test_input};
    use uom::si::{
        available_energy::joule_per_kilogram, mass_density::kilogram_per_cubic_meter,
        molar_concentration::mole_per_cubic_meter, molar_energy::joule_per_mole,
        molar_heat_capacity::joule_per_kelvin_mole, pressure::pascal, ratio::ratio,
        specific_heat_capacity::joule_per_kilogram_kelvin,
        specific_volume::cubic_meter_per_kilogram, thermodynamic_temperature::kelvin,
    };

    test_input!(
        density,
        FluidInput,
        FluidParam::DMass,
        MassDensity,
        kilogram_per_cubic_meter
    );

    test_input!(
        enthalpy,
        FluidInput,
        FluidParam::HMass,
        AvailableEnergy,
        joule_per_kilogram
    );

    test_input!(
        entropy,
        FluidInput,
        FluidParam::SMass,
        SpecificHeatCapacity,
        joule_per_kilogram_kelvin
    );

    test_input!(
        internal_energy,
        FluidInput,
        FluidParam::UMass,
        AvailableEnergy,
        joule_per_kilogram
    );

    test_input!(
        molar_density,
        FluidInput,
        FluidParam::DMolar,
        MolarConcentration,
        mole_per_cubic_meter
    );

    test_input!(
        molar_enthalpy,
        FluidInput,
        FluidParam::HMolar,
        MolarEnergy,
        joule_per_mole
    );

    test_input!(
        molar_entropy,
        FluidInput,
        FluidParam::SMolar,
        MolarHeatCapacity,
        joule_per_kelvin_mole
    );

    test_input!(
        molar_internal_energy,
        FluidInput,
        FluidParam::UMolar,
        MolarEnergy,
        joule_per_mole
    );

    test_input!(pressure, FluidInput, FluidParam::P, Pressure, pascal);
    test_input!(quality, FluidInput, FluidParam::Q, Ratio, ratio);

    #[test]
    fn specific_volume_returns_expected_key_and_si_value() {
        let sut = FluidInput::specific_volume(SpecificVolume::new::<cubic_meter_per_kilogram>(2.0));
        assert_eq!(sut.key(), FluidParam::DMass);
        assert_eq!(sut.si_value(), 0.5);
        assert_eq!(sut, FluidInput::specific_volume_si(2.0));
        assert_eq!(sut, specific_volume!(2.0, cubic_meter_per_kilogram));
        assert_eq!(sut, specific_volume!(2.0));
    }

    test_input!(
        temperature,
        FluidInput,
        FluidParam::T,
        ThermodynamicTemperature,
        kelvin
    );
}
