//! [`Fluid`](crate::fluid::Fluid) input.
//!
//! # Examples
//!
//! ```
//! use rfluids::prelude::fluid::*;
//! use rfluids::uom::si::pressure::atmosphere;
//!
//! let pressure = fluid_input::pressure!(1.0, atmosphere);
//! let pressure_si = fluid_input::pressure!(101325.0);
//! assert_eq!(pressure, pressure_si);
//! assert_eq!(pressure, FluidInput::pressure(Pressure::new::<atmosphere>(1.0)));
//! assert_eq!(pressure_si, FluidInput::pressure_si(101325.0));
//! ```

use super::{define_input, define_input_macro, FluidParam, Input};
use crate::uom::si::f64::{
    AvailableEnergy, MassDensity, MolarConcentration, MolarEnergy, MolarHeatCapacity, Pressure,
    Ratio, SpecificHeatCapacity, ThermodynamicTemperature,
};

/// [`Fluid`](crate::fluid::Fluid) input.
///
/// # Examples
///
/// ```
/// use rfluids::prelude::fluid::*;
/// use rfluids::uom::si::pressure::atmosphere;
///
/// let pressure = fluid_input::pressure!(1.0, atmosphere);
/// let pressure_si = fluid_input::pressure!(101325.0);
/// assert_eq!(pressure, pressure_si);
/// assert_eq!(pressure, FluidInput::pressure(Pressure::new::<atmosphere>(1.0)));
/// assert_eq!(pressure_si, FluidInput::pressure_si(101325.0));
/// ```
pub type FluidInput = Input<FluidParam>;

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
    fluid,
    mass_density,
    gram_per_cubic_centimeter
);

define_input_macro!(
    fluid_input,
    FluidInput,
    enthalpy,
    AvailableEnergy,
    fluid,
    available_energy,
    kilojoule_per_kilogram
);

define_input_macro!(
    fluid_input,
    FluidInput,
    entropy,
    SpecificHeatCapacity,
    fluid,
    specific_heat_capacity,
    joule_per_kilogram_kelvin
);

define_input_macro!(
    fluid_input,
    FluidInput,
    internal_energy,
    AvailableEnergy,
    fluid,
    available_energy,
    kilojoule_per_kilogram
);

define_input_macro!(
    fluid_input,
    FluidInput,
    molar_density,
    MolarConcentration,
    fluid,
    molar_concentration,
    mole_per_cubic_meter
);

define_input_macro!(
    fluid_input,
    FluidInput,
    molar_enthalpy,
    MolarEnergy,
    fluid,
    molar_energy,
    kilojoule_per_mole
);

define_input_macro!(
    fluid_input,
    FluidInput,
    molar_entropy,
    MolarHeatCapacity,
    fluid,
    molar_heat_capacity,
    joule_per_kelvin_mole
);

define_input_macro!(
    fluid_input,
    FluidInput,
    molar_internal_energy,
    MolarEnergy,
    fluid,
    molar_energy,
    kilojoule_per_mole
);

define_input_macro!(
    fluid_input,
    FluidInput,
    pressure,
    Pressure,
    fluid,
    pressure,
    kilopascal
);

define_input_macro!(
    fluid_input,
    FluidInput,
    quality,
    Ratio,
    fluid,
    ratio,
    percent
);

define_input_macro!(
    fluid_input,
    FluidInput,
    temperature,
    ThermodynamicTemperature,
    fluid,
    thermodynamic_temperature,
    degree_celsius
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::test_input;
    use crate::uom::si::available_energy::joule_per_kilogram;
    use crate::uom::si::mass_density::kilogram_per_cubic_meter;
    use crate::uom::si::molar_concentration::mole_per_cubic_meter;
    use crate::uom::si::molar_energy::joule_per_mole;
    use crate::uom::si::molar_heat_capacity::joule_per_kelvin_mole;
    use crate::uom::si::pressure::pascal;
    use crate::uom::si::ratio::ratio;
    use crate::uom::si::specific_heat_capacity::joule_per_kilogram_kelvin;
    use crate::uom::si::thermodynamic_temperature::kelvin;

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

    test_input!(
        temperature,
        FluidInput,
        FluidParam::T,
        ThermodynamicTemperature,
        kelvin
    );
}
