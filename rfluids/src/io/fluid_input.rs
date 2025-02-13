//! Fluid keyed input.

use super::{FluidParam, Input};
use crate::uom::si::f64::{
    AvailableEnergy, MassDensity, MolarConcentration, MolarEnergy, MolarHeatCapacity, Pressure,
    Ratio, SpecificHeatCapacity, ThermodynamicTemperature,
};

/// Fluid keyed input.
///
/// # Examples
///
/// ```
/// use rfluids::prelude::fluid::*;
/// use rfluids::uom::si::mass_density::gram_per_cubic_centimeter;
/// use rfluids::uom::si::pressure::atmosphere;
/// use rfluids::uom::si::thermodynamic_temperature::degree_celsius;
///
/// let pressure =
///     FluidInput::pressure(Pressure::new::<atmosphere>(1.0));
/// assert_eq!(pressure, FluidInput::pressure_si(101325.0));
///
/// let temperature =
///     FluidInput::temperature(ThermodynamicTemperature::new::<degree_celsius>(20.0));
/// assert_eq!(temperature, FluidInput::temperature_si(293.15));
///
/// let density =
///     FluidInput::density(MassDensity::new::<gram_per_cubic_centimeter>(1.0));
/// assert_eq!(density, FluidInput::density_si(1000.0));
/// ```
pub type FluidInput = Input<FluidParam>;

macro_rules! input_doc {
    ($key:ident, $description:literal, $units:literal) => {
        concat!(
            $description,
            " _(key: [`",
            stringify!($key),
            "`](FluidParam::",
            stringify!($key),
            "), SI units: ",
            $units,
            ")_."
        )
    };
    (si, $key:ident, $description:literal, $units:literal) => {
        concat!(
            $description,
            " _(key: [`",
            stringify!($key),
            "`](FluidParam::",
            stringify!($key),
            "))_ in SI units _(",
            $units,
            ")_."
        )
    };
}

macro_rules! define_input {
    ($name:ident, $key:ident, $type:ty, $description:literal, $units:literal) => {
        #[doc = input_doc!($key, $description, $units)]
        #[must_use]
        pub fn $name(value: $type) -> Self {
            Self(FluidParam::$key, value.value)
        }

        paste::paste! {
            #[doc = input_doc!(si, $key, $description, $units)]
            #[must_use]
            pub fn [<$name _si>](value: f64) -> Self {
                Self(FluidParam::$key, value)
            }
        }
    };
}

impl FluidInput {
    define_input!(density, DMass, MassDensity, "Mass density", "kg/m³");

    define_input!(
        enthalpy,
        HMass,
        AvailableEnergy,
        "Mass specific enthalpy",
        "J/kg"
    );

    define_input!(
        entropy,
        SMass,
        SpecificHeatCapacity,
        "Mass specific entropy",
        "J/kg/K"
    );

    define_input!(
        internal_energy,
        UMass,
        AvailableEnergy,
        "Mass specific internal energy",
        "J/kg"
    );

    define_input!(
        molar_density,
        DMolar,
        MolarConcentration,
        "Molar density",
        "mol/m³"
    );

    define_input!(
        molar_enthalpy,
        HMolar,
        MolarEnergy,
        "Molar specific enthalpy",
        "J/mol"
    );

    define_input!(
        molar_entropy,
        SMolar,
        MolarHeatCapacity,
        "Molar specific entropy",
        "J/mol/K"
    );

    define_input!(
        molar_internal_energy,
        UMolar,
        MolarEnergy,
        "Molar specific internal energy",
        "J/mol"
    );

    define_input!(pressure, P, Pressure, "Pressure", "Pa");

    define_input!(
        quality,
        Q,
        Ratio,
        "Vapor quality",
        "dimensionless, from 0 to 1"
    );

    define_input!(temperature, T, ThermodynamicTemperature, "Temperature", "K");
}

macro_rules! input_macro_doc {
    ($name:ident, $type:ident, $example_unit_path:literal, $example_unit:literal) => {
        concat!(
            "Shortcut for [`FluidInput::",
            stringify!($name),
            "`] and [`FluidInput::",
            stringify!($name),
            "_si`].",
            "\n\n# Args\n\n",
            "The first argument is the value of the input parameter _([`f64`])_, ",
            "and the second argument _(optional)_ is the unit of measure _(e.g., [`",
            $example_unit,
            "`](",
            $example_unit_path,
            "::",
            $example_unit,
            "))_.\n\nIf the unit of measure is not provided, ",
            "the value is assumed to be in SI units.",
            "\n\n# Examples\n\n",
            "```\n",
            "use rfluids::prelude::fluid::*;\n",
            "use rfluids::",
            $example_unit_path,
            "::",
            $example_unit,
            ";\n\n",
            "assert_eq!(\n",
            "    fluid_input::",
            stringify!($name),
            "!(42.0, ",
            $example_unit,
            "),\n",
            "    FluidInput::",
            stringify!($name),
            "(",
            stringify!($type),
            "::new::<",
            $example_unit,
            ">(42.0))\n",
            ");\n",
            "```\n\n",
            "```\n",
            "use rfluids::prelude::fluid::*;\n\n",
            "assert_eq!(\n",
            "    fluid_input::",
            stringify!($name),
            "!(42.0),\n",
            "    FluidInput::",
            stringify!($name),
            "_si(42.0)\n",
            ");\n",
        )
    };
}

macro_rules! define_input_macro {
    ($name:ident, $type:ident, $example_unit_path:literal, $example_unit:literal) => {
        #[doc = input_macro_doc!($name, $type, $example_unit_path, $example_unit)]
        #[macro_export]
        macro_rules! $name {
            ($value:expr, $unit:ty) => {
                $crate::io::FluidInput::$name($crate::uom::si::f64::$type::new::<$unit>($value))
            };
            ($value:expr) => {
                paste::paste! {
                    $crate::io::FluidInput::[<$name _si>]($value)
                }
            };
        }

        pub use $name;
    };
}

define_input_macro!(
    density,
    MassDensity,
    "uom::si::mass_density",
    "gram_per_cubic_centimeter"
);

define_input_macro!(
    enthalpy,
    AvailableEnergy,
    "uom::si::available_energy",
    "kilojoule_per_kilogram"
);

define_input_macro!(
    entropy,
    SpecificHeatCapacity,
    "uom::si::specific_heat_capacity",
    "joule_per_kilogram_kelvin"
);

define_input_macro!(
    internal_energy,
    AvailableEnergy,
    "uom::si::available_energy",
    "kilojoule_per_kilogram"
);

define_input_macro!(
    molar_density,
    MolarConcentration,
    "uom::si::molar_concentration",
    "mole_per_cubic_meter"
);

define_input_macro!(
    molar_enthalpy,
    MolarEnergy,
    "uom::si::molar_energy",
    "kilojoule_per_mole"
);

define_input_macro!(
    molar_entropy,
    MolarHeatCapacity,
    "uom::si::molar_heat_capacity",
    "joule_per_kelvin_mole"
);

define_input_macro!(
    molar_internal_energy,
    MolarEnergy,
    "uom::si::molar_energy",
    "kilojoule_per_mole"
);

define_input_macro!(pressure, Pressure, "uom::si::pressure", "kilopascal");

define_input_macro!(quality, Ratio, "uom::si::ratio", "percent");

define_input_macro!(
    temperature,
    ThermodynamicTemperature,
    "uom::si::thermodynamic_temperature",
    "degree_celsius"
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::uom::si::available_energy::joule_per_kilogram;
    use crate::uom::si::mass_density::kilogram_per_cubic_meter;
    use crate::uom::si::molar_concentration::mole_per_cubic_meter;
    use crate::uom::si::molar_energy::joule_per_mole;
    use crate::uom::si::molar_heat_capacity::joule_per_kelvin_mole;
    use crate::uom::si::pressure::pascal;
    use crate::uom::si::ratio::ratio;
    use crate::uom::si::specific_heat_capacity::joule_per_kilogram_kelvin;
    use crate::uom::si::thermodynamic_temperature::kelvin;

    #[test]
    fn density_returns_expected_key_and_si_value() {
        let sut = FluidInput::density(MassDensity::new::<kilogram_per_cubic_meter>(1.0));
        assert_eq!(sut.key(), FluidParam::DMass);
        assert_eq!(sut.si_value(), 1.0);
        assert_eq!(sut, FluidInput::density_si(1.0));
        assert_eq!(sut, density!(1.0, kilogram_per_cubic_meter));
        assert_eq!(sut, density!(1.0));
    }

    #[test]
    fn enthalpy_returns_expected_key_and_si_value() {
        let sut = FluidInput::enthalpy(AvailableEnergy::new::<joule_per_kilogram>(1.0));
        assert_eq!(sut.key(), FluidParam::HMass);
        assert_eq!(sut.si_value(), 1.0);
        assert_eq!(sut, FluidInput::enthalpy_si(1.0));
        assert_eq!(sut, enthalpy!(1.0, joule_per_kilogram));
        assert_eq!(sut, enthalpy!(1.0));
    }

    #[test]
    fn entropy_returns_expected_key_and_si_value() {
        let sut = FluidInput::entropy(SpecificHeatCapacity::new::<joule_per_kilogram_kelvin>(1.0));
        assert_eq!(sut.key(), FluidParam::SMass);
        assert_eq!(sut.si_value(), 1.0);
        assert_eq!(sut, FluidInput::entropy_si(1.0));
        assert_eq!(sut, entropy!(1.0, joule_per_kilogram_kelvin));
        assert_eq!(sut, entropy!(1.0));
    }

    #[test]
    fn internal_energy_returns_expected_key_and_si_value() {
        let sut = FluidInput::internal_energy(AvailableEnergy::new::<joule_per_kilogram>(1.0));
        assert_eq!(sut.key(), FluidParam::UMass);
        assert_eq!(sut.si_value(), 1.0);
        assert_eq!(sut, FluidInput::internal_energy_si(1.0));
        assert_eq!(sut, internal_energy!(1.0, joule_per_kilogram));
        assert_eq!(sut, internal_energy!(1.0));
    }

    #[test]
    fn molar_density_returns_expected_key_and_si_value() {
        let sut = FluidInput::molar_density(MolarConcentration::new::<mole_per_cubic_meter>(1.0));
        assert_eq!(sut.key(), FluidParam::DMolar);
        assert_eq!(sut.si_value(), 1.0);
        assert_eq!(sut, FluidInput::molar_density_si(1.0));
        assert_eq!(sut, molar_density!(1.0, mole_per_cubic_meter));
        assert_eq!(sut, molar_density!(1.0));
    }

    #[test]
    fn molar_enthalpy_returns_expected_key_and_si_value() {
        let sut = FluidInput::molar_enthalpy(MolarEnergy::new::<joule_per_mole>(1.0));
        assert_eq!(sut.key(), FluidParam::HMolar);
        assert_eq!(sut.si_value(), 1.0);
        assert_eq!(sut, FluidInput::molar_enthalpy_si(1.0));
        assert_eq!(sut, molar_enthalpy!(1.0, joule_per_mole));
        assert_eq!(sut, molar_enthalpy!(1.0));
    }

    #[test]
    fn molar_entropy_returns_expected_key_and_si_value() {
        let sut = FluidInput::molar_entropy(MolarHeatCapacity::new::<joule_per_kelvin_mole>(1.0));
        assert_eq!(sut.key(), FluidParam::SMolar);
        assert_eq!(sut.si_value(), 1.0);
        assert_eq!(sut, FluidInput::molar_entropy_si(1.0));
        assert_eq!(sut, molar_entropy!(1.0, joule_per_kelvin_mole));
        assert_eq!(sut, molar_entropy!(1.0));
    }

    #[test]
    fn molar_internal_energy_returns_expected_key_and_si_value() {
        let sut = FluidInput::molar_internal_energy(MolarEnergy::new::<joule_per_mole>(1.0));
        assert_eq!(sut.key(), FluidParam::UMolar);
        assert_eq!(sut.si_value(), 1.0);
        assert_eq!(sut, FluidInput::molar_internal_energy_si(1.0));
        assert_eq!(sut, molar_internal_energy!(1.0, joule_per_mole));
        assert_eq!(sut, molar_internal_energy!(1.0));
    }

    #[test]
    fn pressure_returns_expected_key_and_si_value() {
        let sut = FluidInput::pressure(Pressure::new::<pascal>(1.0));
        assert_eq!(sut.key(), FluidParam::P);
        assert_eq!(sut.si_value(), 1.0);
        assert_eq!(sut, FluidInput::pressure_si(1.0));
        assert_eq!(sut, pressure!(1.0, pascal));
        assert_eq!(sut, pressure!(1.0));
    }

    #[test]
    fn quality_returns_expected_key_and_si_value() {
        let sut = FluidInput::quality(Ratio::new::<ratio>(1.0));
        assert_eq!(sut.key(), FluidParam::Q);
        assert_eq!(sut.si_value(), 1.0);
        assert_eq!(sut, FluidInput::quality_si(1.0));
        assert_eq!(sut, quality!(1.0, ratio));
        assert_eq!(sut, quality!(1.0));
    }

    #[test]
    fn temperature_returns_expected_key_and_si_value() {
        let sut = FluidInput::temperature(ThermodynamicTemperature::new::<kelvin>(1.0));
        assert_eq!(sut.key(), FluidParam::T);
        assert_eq!(sut.si_value(), 1.0);
        assert_eq!(sut, FluidInput::temperature_si(1.0));
        assert_eq!(sut, temperature!(1.0, kelvin));
        assert_eq!(sut, temperature!(1.0));
    }
}
