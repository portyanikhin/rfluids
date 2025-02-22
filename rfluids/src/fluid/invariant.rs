use super::common::{cached_output, guard};
use super::requests::FluidUpdateRequest;
use super::{Fluid, OutputResult, StateResult};
use crate::error::FluidOutputError;
use crate::io::fluid_input::FluidInput;
use crate::io::{FluidTrivialParam, Input};
use crate::ops::mul;
use crate::state_variant::StateVariant;
use crate::substance::Substance;
use crate::uom::si::f64::{
    MassDensity, MolarConcentration, MolarMass, Pressure, ThermodynamicTemperature,
};
use crate::uom::si::mass_density::kilogram_per_cubic_meter;
use crate::uom::si::molar_concentration::mole_per_cubic_meter;
use crate::uom::si::molar_mass::kilogram_per_mole;
use crate::uom::si::pressure::pascal;
use crate::uom::si::thermodynamic_temperature::kelvin;

macro_rules! trivial_output_doc {
    (always_ok, $key:ident, $description:literal, $units_description:literal) => {
        concat!(
            $description,
            "\n_(key: [`",
            stringify!($key),
            "`](FluidTrivialParam::",
            stringify!($key),
            "), ",
            $units_description,
            ")_.",
        )
    };
    ($key:ident, $description:literal, $units_description:literal) => {
        concat!(
            trivial_output_doc!(always_ok, $key, $description, $units_description),
            "\n\n# Errors\n\n",
            "If it's not available for the specified substance,\n",
            "a [`FluidOutputError`] is returned.",
        )
    };
}

macro_rules! define_trivial_output {
    (
        $method:ident,
        $name:ident,
        $key:ident,
        $type:ty,
        $description:literal,
        $units_description:literal
        $(, $map:expr)?
    ) => {
        #[doc = trivial_output_doc!($key, $description, $units_description)]
        pub fn $name(&mut self) -> OutputResult<$type> {
            self.$method(FluidTrivialParam::$key)
                $(.map($map))?
        }
    };
    (
        not_available_for_predefined_mix,
        $method:ident,
        $name:ident,
        $key:ident,
        $type:ty,
        $description:literal,
        $units_description:literal
        $(, $map:expr)?
    ) => {
        #[doc = trivial_output_doc!($key, $description, $units_description)]
        pub fn $name(&mut self) -> OutputResult<$type> {
            let key = FluidTrivialParam::$key;
            // Due to CoolProp freeze
            if let Substance::PredefinedMix(_) = self.substance {
                return Err(FluidOutputError::UnavailableTrivialOutput(key));
            }
            self.$method(key)
                $(.map($map))?
        }
    };
    (
        always_ok,
        $method:ident,
        $name:ident,
        $key:ident,
        $type:ty,
        $description:literal,
        $units_description:literal,
        $map:expr
    ) => {
        #[doc = trivial_output_doc!(always_ok, $key, $description, $units_description)]
        pub fn $name(&mut self) -> $type {
            $map(self.$method(FluidTrivialParam::$key).unwrap())
        }
    };
}

impl<S: StateVariant> Fluid<S> {
    /// Specified substance.
    #[must_use]
    pub fn substance(&self) -> &Substance {
        &self.substance
    }

    define_trivial_output!(
        trivial_output,
        acentric_factor,
        AcentricFactor,
        f64,
        "Acentric factor",
        "dimensionless"
    );

    define_trivial_output!(
        not_available_for_predefined_mix,
        positive_trivial_output,
        critical_density,
        DMassCritical,
        MassDensity,
        "Critical point mass density",
        "SI units: kg/m³",
        MassDensity::new::<kilogram_per_cubic_meter>
    );

    define_trivial_output!(
        not_available_for_predefined_mix,
        positive_trivial_output,
        critical_molar_density,
        DMolarCritical,
        MolarConcentration,
        "Critical point molar density",
        "SI units: mol/m³",
        MolarConcentration::new::<mole_per_cubic_meter>
    );

    define_trivial_output!(
        not_available_for_predefined_mix,
        positive_trivial_output,
        critical_pressure,
        PCritical,
        Pressure,
        "Critical point pressure",
        "SI units: Pa",
        Pressure::new::<pascal>
    );

    define_trivial_output!(
        not_available_for_predefined_mix,
        positive_trivial_output,
        critical_temperature,
        TCritical,
        ThermodynamicTemperature,
        "Critical point temperature",
        "SI units: K",
        ThermodynamicTemperature::new::<kelvin>
    );

    define_trivial_output!(
        non_negative_trivial_output,
        flammability_hazard,
        FH,
        f64,
        "Flammability hazard index",
        "dimensionless"
    );

    define_trivial_output!(
        positive_trivial_output,
        freezing_temperature,
        TFreeze,
        ThermodynamicTemperature,
        "Freezing temperature for incompressible mixtures",
        "SI units: K",
        ThermodynamicTemperature::new::<kelvin>
    );

    define_trivial_output!(
        non_negative_trivial_output,
        gwp20,
        GWP20,
        f64,
        "20-year global warming potential",
        "dimensionless"
    );

    define_trivial_output!(
        non_negative_trivial_output,
        gwp100,
        GWP100,
        f64,
        "100-year global warming potential",
        "dimensionless"
    );

    define_trivial_output!(
        non_negative_trivial_output,
        gwp500,
        GWP500,
        f64,
        "500-year global warming potential",
        "dimensionless"
    );

    define_trivial_output!(
        non_negative_trivial_output,
        health_hazard,
        HH,
        f64,
        "Health hazard index",
        "dimensionless"
    );

    define_trivial_output!(
        positive_trivial_output,
        max_pressure,
        PMax,
        Pressure,
        "Maximum pressure",
        "SI units: Pa",
        Pressure::new::<pascal>
    );

    define_trivial_output!(
        always_ok,
        positive_trivial_output,
        max_temperature,
        TMax,
        ThermodynamicTemperature,
        "Maximum temperature",
        "SI units: K",
        ThermodynamicTemperature::new::<kelvin>
    );

    define_trivial_output!(
        positive_trivial_output,
        min_pressure,
        PMin,
        Pressure,
        "Minimum pressure",
        "SI units: Pa",
        Pressure::new::<pascal>
    );

    define_trivial_output!(
        always_ok,
        positive_trivial_output,
        min_temperature,
        TMin,
        ThermodynamicTemperature,
        "Minimum temperature",
        "SI units: K",
        ThermodynamicTemperature::new::<kelvin>
    );

    define_trivial_output!(
        positive_trivial_output,
        molar_mass,
        MolarMass,
        MolarMass,
        "Molar mass",
        "SI units: kg/mol",
        MolarMass::new::<kilogram_per_mole>
    );

    define_trivial_output!(
        non_negative_trivial_output,
        odp,
        ODP,
        f64,
        "Ozone depletion potential",
        "dimensionless"
    );

    define_trivial_output!(
        non_negative_trivial_output,
        physical_hazard,
        PH,
        f64,
        "Physical hazard index",
        "dimensionless"
    );

    #[doc = trivial_output_doc!(DMassReducing, "Reducing point mass density", "SI units: kg/m³")]
    pub fn reducing_density(&mut self) -> OutputResult<MassDensity> {
        // Due to CoolProp bug
        mul(self.reducing_molar_density(), self.molar_mass())
    }

    define_trivial_output!(
        positive_trivial_output,
        reducing_molar_density,
        DMolarReducing,
        MolarConcentration,
        "Reducing point molar density",
        "SI units: mol/m³",
        MolarConcentration::new::<mole_per_cubic_meter>
    );

    define_trivial_output!(
        positive_trivial_output,
        reducing_pressure,
        PReducing,
        Pressure,
        "Reducing point pressure",
        "SI units: Pa",
        Pressure::new::<pascal>
    );

    define_trivial_output!(
        positive_trivial_output,
        reducing_temperature,
        TReducing,
        ThermodynamicTemperature,
        "Reducing point temperature",
        "SI units: K",
        ThermodynamicTemperature::new::<kelvin>
    );

    define_trivial_output!(
        positive_trivial_output,
        triple_pressure,
        PTriple,
        Pressure,
        "Triple point pressure",
        "SI units: Pa",
        Pressure::new::<pascal>
    );

    define_trivial_output!(
        positive_trivial_output,
        triple_temperature,
        TTriple,
        ThermodynamicTemperature,
        "Triple point temperature",
        "SI units: K",
        ThermodynamicTemperature::new::<kelvin>
    );

    pub(crate) fn inner_update(
        &mut self,
        input1: FluidInput,
        input2: FluidInput,
    ) -> StateResult<()> {
        let request: FluidUpdateRequest = (input1, input2).try_into()?;
        self.backend.update(request.0, request.1, request.2)?;
        self.outputs.clear();
        self.outputs.insert(input1.key(), Ok(input1.si_value()));
        self.outputs.insert(input2.key(), Ok(input2.si_value()));
        self.update_request = Some(request);
        Ok(())
    }

    fn positive_trivial_output(&mut self, key: FluidTrivialParam) -> OutputResult<f64> {
        self.trivial_output(key)
            .and_then(|value| guard(key.into(), value, |x| x > 0.0))
    }

    fn non_negative_trivial_output(&mut self, key: FluidTrivialParam) -> OutputResult<f64> {
        self.trivial_output(key)
            .and_then(|value| guard(key.into(), value, |x| x >= 0.0))
    }

    fn trivial_output(&mut self, key: FluidTrivialParam) -> OutputResult<f64> {
        cached_output(&mut self.trivial_outputs, &mut self.backend, key, |_| {
            FluidOutputError::UnavailableTrivialOutput(key)
        })
    }
}
