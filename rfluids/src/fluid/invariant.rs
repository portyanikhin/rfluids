use super::common::{cached_output, density_from_molar_density, non_negative};
use super::requests::FluidUpdateRequest;
use super::{Fluid, OutputResult, StateResult};
use crate::error::FluidOutputError;
use crate::io::{FluidInput, FluidTrivialParam};
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

impl<S: StateVariant> Fluid<S> {
    /// Specified substance.
    #[must_use]
    pub fn substance(&self) -> &Substance {
        &self.substance
    }

    /// Acentric factor
    /// _(key: [`AcentricFactor`](FluidTrivialParam::AcentricFactor), dimensionless)_.
    ///
    /// # Errors
    ///
    /// If it's not available for the specified substance,
    /// a [`FluidOutputError`] is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use approx::assert_relative_eq;
    /// use rfluids::prelude::fluid::*;
    ///
    /// let mut water = Fluid::from(Pure::Water);
    /// assert_relative_eq!(water.acentric_factor()?, 0.3442920843);
    ///
    /// let mut r444a = Fluid::from(PredefinedMix::R444A);
    /// assert!(r444a.acentric_factor().is_err());
    /// # Ok::<(), rfluids::error::Error>(())
    /// ```
    pub fn acentric_factor(&mut self) -> OutputResult<f64> {
        self.trivial_output(FluidTrivialParam::AcentricFactor)
    }

    /// Critical point mass density
    /// _(key: [`DMassCritical`](FluidTrivialParam::DMassCritical), SI units: kg/m³)_.
    ///
    /// # Errors
    ///
    /// If it's not available for the specified substance,
    /// a [`FluidOutputError`] is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use approx::assert_relative_eq;
    /// use rfluids::prelude::fluid::*;
    /// use rfluids::uom::si::mass_density::gram_per_cubic_centimeter;
    ///
    /// let mut water = Fluid::from(Pure::Water);
    /// assert_relative_eq!(water.critical_density()?.value, 322.0);
    /// assert_relative_eq!(
    ///     water.critical_density()?.get::<gram_per_cubic_centimeter>(),
    ///     0.322
    /// );
    ///
    /// let mut r444a = Fluid::from(PredefinedMix::R444A);
    /// assert!(r444a.critical_density().is_err());
    /// # Ok::<(), rfluids::error::Error>(())
    /// ```
    pub fn critical_density(&mut self) -> OutputResult<MassDensity> {
        let key = FluidTrivialParam::DMassCritical;
        // Due to CoolProp freeze
        if let Substance::PredefinedMix(_) = self.substance {
            return Err(FluidOutputError::UnavailableTrivialOutput(key));
        }
        self.non_negative_trivial_output(key)
            .map(MassDensity::new::<kilogram_per_cubic_meter>)
    }

    /// Critical point molar density
    /// _(key: [`DMolarCritical`](FluidTrivialParam::DMolarCritical), SI units: mol/m³)_.
    ///
    /// # Errors
    ///
    /// If it's not available for the specified substance,
    /// a [`FluidOutputError`] is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use approx::assert_relative_eq;
    /// use rfluids::prelude::fluid::*;
    /// use rfluids::uom::si::molar_concentration::mole_per_liter;
    ///
    /// let mut water = Fluid::from(Pure::Water);
    /// assert_relative_eq!(
    ///     water.critical_molar_density()?.value,
    ///     17873.72799560906
    /// );
    /// assert_relative_eq!(
    ///     water.critical_molar_density()?.get::<mole_per_liter>(),
    ///     17.87372799560906
    /// );
    ///
    /// let mut r444a = Fluid::from(PredefinedMix::R444A);
    /// assert!(r444a.critical_molar_density().is_err());
    /// # Ok::<(), rfluids::error::Error>(())
    /// ```
    pub fn critical_molar_density(&mut self) -> OutputResult<MolarConcentration> {
        let key = FluidTrivialParam::DMolarCritical;
        // Due to CoolProp freeze
        if let Substance::PredefinedMix(_) = self.substance {
            return Err(FluidOutputError::UnavailableTrivialOutput(key));
        }
        self.non_negative_trivial_output(key)
            .map(MolarConcentration::new::<mole_per_cubic_meter>)
    }

    /// Critical point pressure
    /// _(key: [`PCritical`](FluidTrivialParam::PCritical), SI units: Pa)_.
    ///
    /// # Errors
    ///
    /// If it's not available for the specified substance,
    /// a [`FluidOutputError`] is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use approx::assert_relative_eq;
    /// use rfluids::prelude::fluid::*;
    /// use rfluids::uom::si::pressure::megapascal;
    ///
    /// let mut water = Fluid::from(Pure::Water);
    /// assert_relative_eq!(water.critical_pressure()?.value, 22.064e6);
    /// assert_relative_eq!(
    ///     water.critical_pressure()?.get::<megapascal>(),
    ///     22.064
    /// );
    ///
    /// let mut r444a = Fluid::from(PredefinedMix::R444A);
    /// assert!(r444a.critical_pressure().is_err());
    /// # Ok::<(), rfluids::error::Error>(())
    /// ```
    pub fn critical_pressure(&mut self) -> OutputResult<Pressure> {
        let key = FluidTrivialParam::PCritical;
        // Due to CoolProp freeze
        if let Substance::PredefinedMix(_) = self.substance {
            return Err(FluidOutputError::UnavailableTrivialOutput(key));
        }
        self.non_negative_trivial_output(key)
            .map(Pressure::new::<pascal>)
    }

    /// Critical point temperature
    /// _(key: [`TCritical`](FluidTrivialParam::TCritical), SI units: K)_.
    ///
    /// # Errors
    ///
    /// If it's not available for the specified substance,
    /// a [`FluidOutputError`] is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use approx::assert_relative_eq;
    /// use rfluids::prelude::fluid::*;
    /// use rfluids::uom::si::thermodynamic_temperature::degree_celsius;
    ///
    /// let mut water = Fluid::from(Pure::Water);
    /// assert_relative_eq!(water.critical_temperature()?.value, 647.096);
    /// assert_relative_eq!(
    ///     water.critical_temperature()?.get::<degree_celsius>(),
    ///     373.946
    /// );
    ///
    /// let mut r444a = Fluid::from(PredefinedMix::R444A);
    /// assert!(r444a.critical_temperature().is_err());
    /// # Ok::<(), rfluids::error::Error>(())
    /// ```
    pub fn critical_temperature(&mut self) -> OutputResult<ThermodynamicTemperature> {
        let key = FluidTrivialParam::TCritical;
        // Due to CoolProp freeze
        if let Substance::PredefinedMix(_) = self.substance {
            return Err(FluidOutputError::UnavailableTrivialOutput(key));
        }
        self.non_negative_trivial_output(key)
            .map(ThermodynamicTemperature::new::<kelvin>)
    }

    /// Flammability hazard index
    /// _(key: [`FH`](FluidTrivialParam::FH), dimensionless)_.
    ///
    /// # Errors
    ///
    /// If it's not available for the specified substance,
    /// a [`FluidOutputError`] is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use rfluids::prelude::fluid::*;
    /// use rfluids::uom::si::ratio::percent;
    ///
    /// let mut acetone = Fluid::from(Pure::Acetone);
    /// assert_eq!(acetone.flammability_hazard()?, 3.0);
    ///
    /// let mut water = Fluid::from(Pure::Water);
    /// assert_eq!(water.flammability_hazard()?, 0.0);
    ///
    /// let mut propylene_glycol = Fluid::from(
    ///     BinaryMix::with_fraction(BinaryMixKind::MPG, Ratio::new::<percent>(40.0))?,
    /// );
    /// assert!(propylene_glycol.flammability_hazard().is_err());
    /// # Ok::<(), rfluids::error::Error>(())
    /// ```
    pub fn flammability_hazard(&mut self) -> OutputResult<f64> {
        self.non_negative_trivial_output(FluidTrivialParam::FH)
    }

    /// Freezing temperature for incompressible mixtures
    /// _(key: [`TFreeze`](FluidTrivialParam::TFreeze), SI units: K)_.
    ///
    /// # Errors
    ///
    /// If it's not available for the specified substance,
    /// a [`FluidOutputError`] is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use approx::assert_relative_eq;
    /// use rfluids::prelude::fluid::*;
    /// use rfluids::uom::si::ratio::percent;
    /// use rfluids::uom::si::thermodynamic_temperature::degree_celsius;
    ///
    /// let mut water = Fluid::from(Pure::Water);
    /// assert!(water.freezing_temperature().is_err());
    ///
    /// let mut propylene_glycol = Fluid::from(
    ///     BinaryMix::with_fraction(BinaryMixKind::MPG, Ratio::new::<percent>(40.0))?,
    /// );
    /// assert_relative_eq!(
    ///     propylene_glycol.freezing_temperature()?.value,
    ///     252.58175495305838
    /// );
    /// assert_relative_eq!(
    ///     propylene_glycol.freezing_temperature()?.get::<degree_celsius>(),
    ///     -20.5682450469416
    /// );
    /// # Ok::<(), rfluids::error::Error>(())
    /// ```
    pub fn freezing_temperature(&mut self) -> OutputResult<ThermodynamicTemperature> {
        self.non_negative_trivial_output(FluidTrivialParam::TFreeze)
            .map(ThermodynamicTemperature::new::<kelvin>)
    }

    /// 20-year global warming potential
    /// _(key: [`GWP20`](FluidTrivialParam::GWP20), dimensionless)_.
    ///
    /// # Errors
    ///
    /// If it's not available for the specified substance,
    /// a [`FluidOutputError`] is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use rfluids::prelude::fluid::*;
    ///
    /// let mut water = Fluid::from(Pure::Water);
    /// assert!(water.gwp20().is_err());
    ///
    /// let mut r32 = Fluid::from(Pure::R32);
    /// assert_eq!(r32.gwp20()?, 2330.0);
    /// # Ok::<(), rfluids::error::Error>(())
    /// ```
    pub fn gwp20(&mut self) -> OutputResult<f64> {
        self.non_negative_trivial_output(FluidTrivialParam::GWP20)
    }

    /// 100-year global warming potential
    /// _(key: [`GWP100`](FluidTrivialParam::GWP100), dimensionless)_.
    ///
    /// # Errors
    ///
    /// If it's not available for the specified substance,
    /// a [`FluidOutputError`] is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use rfluids::prelude::fluid::*;
    ///
    /// let mut water = Fluid::from(Pure::Water);
    /// assert!(water.gwp100().is_err());
    ///
    /// let mut r32 = Fluid::from(Pure::R32);
    /// assert_eq!(r32.gwp100()?, 675.0);
    /// # Ok::<(), rfluids::error::Error>(())
    /// ```
    pub fn gwp100(&mut self) -> OutputResult<f64> {
        self.non_negative_trivial_output(FluidTrivialParam::GWP100)
    }

    /// 500-year global warming potential
    /// _(key: [`GWP500`](FluidTrivialParam::GWP500), dimensionless)_.
    ///
    /// # Errors
    ///
    /// If it's not available for the specified substance,
    /// a [`FluidOutputError`] is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use rfluids::prelude::fluid::*;
    ///
    /// let mut water = Fluid::from(Pure::Water);
    /// assert!(water.gwp500().is_err());
    ///
    /// let mut r32 = Fluid::from(Pure::R32);
    /// assert_eq!(r32.gwp500()?, 205.0);
    /// # Ok::<(), rfluids::error::Error>(())
    /// ```
    pub fn gwp500(&mut self) -> OutputResult<f64> {
        self.non_negative_trivial_output(FluidTrivialParam::GWP500)
    }

    /// Health hazard index
    /// _(key: [`HH`](FluidTrivialParam::HH), dimensionless)_.
    ///
    /// # Errors
    ///
    /// If it's not available for the specified substance,
    /// a [`FluidOutputError`] is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use rfluids::prelude::fluid::*;
    /// use rfluids::uom::si::ratio::percent;
    ///
    /// let mut acetone = Fluid::from(Pure::Acetone);
    /// assert_eq!(acetone.health_hazard()?, 2.0);
    ///
    /// let mut water = Fluid::from(Pure::Water);
    /// assert_eq!(water.health_hazard()?, 0.0);
    ///
    /// let mut propylene_glycol = Fluid::from(
    ///     BinaryMix::with_fraction(BinaryMixKind::MPG, Ratio::new::<percent>(40.0))?,
    /// );
    /// assert!(propylene_glycol.health_hazard().is_err());
    /// # Ok::<(), rfluids::error::Error>(())
    pub fn health_hazard(&mut self) -> OutputResult<f64> {
        self.non_negative_trivial_output(FluidTrivialParam::HH)
    }

    /// Maximum pressure
    /// _(key: [`PMax`](FluidTrivialParam::PMax), SI units: Pa)_.
    ///
    /// # Errors
    ///
    /// If it's not available for the specified substance,
    /// a [`FluidOutputError`] is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use approx::assert_relative_eq;
    /// use rfluids::prelude::fluid::*;
    /// use rfluids::uom::si::ratio::percent;
    /// use rfluids::uom::si::pressure::megapascal;
    ///
    /// let mut water = Fluid::from(Pure::Water);
    /// assert_relative_eq!(water.max_pressure()?.value, 1e9);
    /// assert_relative_eq!(
    ///     water.max_pressure()?.get::<megapascal>(),
    ///     1000.0
    /// );
    ///
    /// let mut propylene_glycol = Fluid::from(
    ///     BinaryMix::with_fraction(BinaryMixKind::MPG, Ratio::new::<percent>(40.0))?,
    /// );
    /// assert!(propylene_glycol.max_pressure().is_err());
    /// # Ok::<(), rfluids::error::Error>(())
    /// ```
    pub fn max_pressure(&mut self) -> OutputResult<Pressure> {
        self.non_negative_trivial_output(FluidTrivialParam::PMax)
            .map(Pressure::new::<pascal>)
    }

    /// Maximum temperature
    /// _(key: [`TMax`](FluidTrivialParam::TMax), SI units: K)_.
    ///
    /// # Examples
    ///
    /// ```
    /// use approx::assert_relative_eq;
    /// use rfluids::prelude::fluid::*;
    /// use rfluids::uom::si::thermodynamic_temperature::degree_celsius;
    ///
    /// let mut water = Fluid::from(Pure::Water);
    /// assert_relative_eq!(water.max_temperature().value, 2e3);
    /// assert_relative_eq!(
    ///     water.max_temperature().get::<degree_celsius>(),
    ///     1726.85
    /// );
    /// # Ok::<(), rfluids::error::Error>(())
    /// ```
    pub fn max_temperature(&mut self) -> ThermodynamicTemperature {
        ThermodynamicTemperature::new::<kelvin>(
            self.non_negative_trivial_output(FluidTrivialParam::TMax)
                .unwrap(),
        )
    }

    /// Minimum pressure
    /// _(key: [`PMin`](FluidTrivialParam::PMin), SI units: Pa)_.
    ///
    /// # Errors
    ///
    /// If it's not available for the specified substance,
    /// a [`FluidOutputError`] is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use approx::assert_relative_eq;
    /// use rfluids::prelude::fluid::*;
    /// use rfluids::uom::si::ratio::percent;
    /// use rfluids::uom::si::pressure::kilopascal;
    ///
    /// let mut water = Fluid::from(Pure::Water);
    /// assert_relative_eq!(
    ///     water.min_pressure()?.value,
    ///     611.6548008968684
    /// );
    /// assert_relative_eq!(
    ///     water.min_pressure()?.get::<kilopascal>(),
    ///     0.6116548008968684
    /// );
    ///
    /// let mut propylene_glycol = Fluid::from(
    ///     BinaryMix::with_fraction(BinaryMixKind::MPG, Ratio::new::<percent>(40.0))?,
    /// );
    /// assert!(propylene_glycol.min_pressure().is_err());
    /// # Ok::<(), rfluids::error::Error>(())
    /// ```
    pub fn min_pressure(&mut self) -> OutputResult<Pressure> {
        self.non_negative_trivial_output(FluidTrivialParam::PMin)
            .map(Pressure::new::<pascal>)
    }

    /// Minimum temperature
    /// _(key: [`TMin`](FluidTrivialParam::TMin), SI units: K)_.
    ///
    /// # Examples
    ///
    /// ```
    /// use approx::assert_relative_eq;
    /// use rfluids::prelude::fluid::*;
    /// use rfluids::uom::si::thermodynamic_temperature::degree_celsius;
    ///
    /// let mut water = Fluid::from(Pure::Water);
    /// assert_relative_eq!(water.min_temperature().value, 273.16);
    /// assert_relative_eq!(
    ///     water.min_temperature().get::<degree_celsius>(),
    ///     0.01,
    ///     epsilon = 1e-6
    /// );
    /// # Ok::<(), rfluids::error::Error>(())
    /// ```
    pub fn min_temperature(&mut self) -> ThermodynamicTemperature {
        ThermodynamicTemperature::new::<kelvin>(
            self.non_negative_trivial_output(FluidTrivialParam::TMin)
                .unwrap(),
        )
    }

    /// Molar mass
    /// _(key: [`MolarMass`](FluidTrivialParam::MolarMass), SI units: kg/mol)_.
    ///
    /// # Errors
    ///
    /// If it's not available for the specified substance,
    /// a [`FluidOutputError`] is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use approx::assert_relative_eq;
    /// use rfluids::prelude::fluid::*;
    /// use rfluids::uom::si::molar_mass::gram_per_mole;
    /// use rfluids::uom::si::ratio::percent;
    ///
    /// let mut water = Fluid::from(Pure::Water);
    /// assert_relative_eq!(water.molar_mass()?.value, 0.018015268);
    /// assert_relative_eq!(
    ///     water.molar_mass()?.get::<gram_per_mole>(),
    ///     18.015268
    /// );
    ///
    /// let mut propylene_glycol = Fluid::from(
    ///     BinaryMix::with_fraction(BinaryMixKind::MPG, Ratio::new::<percent>(40.0))?,
    /// );
    /// assert!(propylene_glycol.molar_mass().is_err());
    /// # Ok::<(), rfluids::error::Error>(())
    /// ```
    pub fn molar_mass(&mut self) -> OutputResult<MolarMass> {
        self.non_negative_trivial_output(FluidTrivialParam::MolarMass)
            .map(MolarMass::new::<kilogram_per_mole>)
    }

    /// Ozone depletion potential
    /// _(key: [`ODP`](FluidTrivialParam::ODP), dimensionless)_.
    ///
    /// # Errors
    ///
    /// If it's not available for the specified substance,
    /// a [`FluidOutputError`] is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use rfluids::prelude::fluid::*;
    ///
    /// let mut water = Fluid::from(Pure::Water);
    /// assert!(water.odp().is_err());
    ///
    /// let mut r32 = Fluid::from(Pure::R32);
    /// assert!(r32.odp().is_err());
    ///
    /// let mut r22 = Fluid::from(Pure::R22);
    /// assert_eq!(r22.odp()?, 0.05);
    /// # Ok::<(), rfluids::error::Error>(())
    /// ```
    pub fn odp(&mut self) -> OutputResult<f64> {
        self.non_negative_trivial_output(FluidTrivialParam::ODP)
    }

    /// Physical hazard index
    /// _(key: [`PH`](FluidTrivialParam::PH), dimensionless)_.
    ///
    /// # Errors
    ///
    /// If it's not available for the specified substance,
    /// a [`FluidOutputError`] is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use rfluids::prelude::fluid::*;
    /// use rfluids::uom::si::ratio::percent;
    ///
    /// let mut carbon_monoxide = Fluid::from(Pure::CarbonMonoxide);
    /// assert_eq!(carbon_monoxide.physical_hazard()?, 3.0);
    ///
    /// let mut water = Fluid::from(Pure::Water);
    /// assert_eq!(water.physical_hazard()?, 0.0);
    ///
    /// let mut propylene_glycol = Fluid::from(
    ///     BinaryMix::with_fraction(BinaryMixKind::MPG, Ratio::new::<percent>(40.0))?,
    /// );
    /// assert!(propylene_glycol.physical_hazard().is_err());
    /// # Ok::<(), rfluids::error::Error>(())
    pub fn physical_hazard(&mut self) -> OutputResult<f64> {
        self.non_negative_trivial_output(FluidTrivialParam::PH)
    }

    /// Reducing point mass density
    /// _(key: [`DMassReducing`](FluidTrivialParam::DMassReducing), SI units: kg/m³)_.
    ///
    /// # Errors
    ///
    /// If it's not available for the specified substance,
    /// a [`FluidOutputError`] is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use approx::assert_relative_eq;
    /// use rfluids::prelude::fluid::*;
    /// use rfluids::uom::si::mass_density::gram_per_cubic_centimeter;
    /// use rfluids::uom::si::ratio::percent;
    ///
    /// let mut water = Fluid::from(Pure::Water);
    /// assert_relative_eq!(water.reducing_density()?.value, 322.0);
    /// assert_relative_eq!(
    ///     water.reducing_density()?.get::<gram_per_cubic_centimeter>(),
    ///     0.322
    /// );
    ///
    /// let mut propylene_glycol = Fluid::from(
    ///     BinaryMix::with_fraction(BinaryMixKind::MPG, Ratio::new::<percent>(40.0))?,
    /// );
    /// assert!(propylene_glycol.reducing_density().is_err());
    /// # Ok::<(), rfluids::error::Error>(())
    /// ```
    pub fn reducing_density(&mut self) -> OutputResult<MassDensity> {
        // Due to CoolProp bug
        density_from_molar_density(self.reducing_molar_density(), self.molar_mass())
    }

    /// Reducing point molar density
    /// _(key: [`DMolarReducing`](FluidTrivialParam::DMolarReducing), SI units: mol/m³)_.
    ///
    /// # Errors
    ///
    /// If it's not available for the specified substance,
    /// a [`FluidOutputError`] is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use approx::assert_relative_eq;
    /// use rfluids::prelude::fluid::*;
    /// use rfluids::uom::si::molar_concentration::mole_per_liter;
    /// use rfluids::uom::si::ratio::percent;
    ///
    /// let mut water = Fluid::from(Pure::Water);
    /// assert_relative_eq!(
    ///     water.reducing_molar_density()?.value,
    ///     17873.72799560906
    /// );
    /// assert_relative_eq!(
    ///     water.reducing_molar_density()?.get::<mole_per_liter>(),
    ///     17.87372799560906
    /// );
    ///
    /// let mut propylene_glycol = Fluid::from(
    ///     BinaryMix::with_fraction(BinaryMixKind::MPG, Ratio::new::<percent>(40.0))?,
    /// );
    /// assert!(propylene_glycol.reducing_molar_density().is_err());
    /// # Ok::<(), rfluids::error::Error>(())
    /// ```
    pub fn reducing_molar_density(&mut self) -> OutputResult<MolarConcentration> {
        self.non_negative_trivial_output(FluidTrivialParam::DMolarReducing)
            .map(MolarConcentration::new::<mole_per_cubic_meter>)
    }

    /// Reducing point pressure
    /// _(key: [`PReducing`](FluidTrivialParam::PReducing), SI units: Pa)_.
    ///
    /// # Errors
    ///
    /// If it's not available for the specified substance,
    /// a [`FluidOutputError`] is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use approx::assert_relative_eq;
    /// use rfluids::prelude::fluid::*;
    /// use rfluids::uom::si::pressure::megapascal;
    /// use rfluids::uom::si::ratio::percent;
    ///
    /// let mut water = Fluid::from(Pure::Water);
    /// assert_relative_eq!(water.reducing_pressure()?.value, 22.064e6);
    /// assert_relative_eq!(
    ///     water.reducing_pressure()?.get::<megapascal>(),
    ///     22.064
    /// );
    ///
    /// let mut propylene_glycol = Fluid::from(
    ///     BinaryMix::with_fraction(BinaryMixKind::MPG, Ratio::new::<percent>(40.0))?,
    /// );
    /// assert!(propylene_glycol.reducing_pressure().is_err());
    /// # Ok::<(), rfluids::error::Error>(())
    pub fn reducing_pressure(&mut self) -> OutputResult<Pressure> {
        self.non_negative_trivial_output(FluidTrivialParam::PReducing)
            .map(Pressure::new::<pascal>)
    }

    /// Reducing point temperature
    /// _(key: [`TReducing`](FluidTrivialParam::TReducing), SI units: K)_.
    ///
    /// # Errors
    ///
    /// If it's not available for the specified substance,
    /// a [`FluidOutputError`] is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use approx::assert_relative_eq;
    /// use rfluids::prelude::fluid::*;
    /// use rfluids::uom::si::thermodynamic_temperature::degree_celsius;
    /// use rfluids::uom::si::ratio::percent;
    ///
    /// let mut water = Fluid::from(Pure::Water);
    /// assert_relative_eq!(water.reducing_temperature()?.value, 647.096);
    /// assert_relative_eq!(
    ///     water.reducing_temperature()?.get::<degree_celsius>(),
    ///     373.946
    /// );
    ///
    /// let mut propylene_glycol = Fluid::from(
    ///     BinaryMix::with_fraction(BinaryMixKind::MPG, Ratio::new::<percent>(40.0))?,
    /// );
    /// assert!(propylene_glycol.reducing_temperature().is_err());
    /// # Ok::<(), rfluids::error::Error>(())
    /// ```
    pub fn reducing_temperature(&mut self) -> OutputResult<ThermodynamicTemperature> {
        self.non_negative_trivial_output(FluidTrivialParam::TReducing)
            .map(ThermodynamicTemperature::new::<kelvin>)
    }

    /// Triple point pressure
    /// _(key: [`PTriple`](FluidTrivialParam::PTriple), SI units: Pa)_.
    ///
    /// # Errors
    ///
    /// If it's not available for the specified substance,
    /// a [`FluidOutputError`] is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use approx::assert_relative_eq;
    /// use rfluids::prelude::fluid::*;
    /// use rfluids::uom::si::pressure::kilopascal;
    /// use rfluids::uom::si::ratio::percent;
    ///
    /// let mut water = Fluid::from(Pure::Water);
    /// assert_relative_eq!(
    ///     water.triple_pressure()?.value,
    ///     611.6548008968684
    /// );
    /// assert_relative_eq!(
    ///     water.min_pressure()?.get::<kilopascal>(),
    ///     0.6116548008968684
    /// );
    ///
    /// let mut propylene_glycol = Fluid::from(
    ///     BinaryMix::with_fraction(BinaryMixKind::MPG, Ratio::new::<percent>(40.0))?,
    /// );
    /// assert!(propylene_glycol.triple_pressure().is_err());
    /// # Ok::<(), rfluids::error::Error>(())
    /// ```
    pub fn triple_pressure(&mut self) -> OutputResult<Pressure> {
        self.non_negative_trivial_output(FluidTrivialParam::PTriple)
            .map(Pressure::new::<pascal>)
    }

    /// Triple point temperature
    /// _(key: [`TTriple`](FluidTrivialParam::TTriple), SI units: K)_.
    ///
    /// # Errors
    ///
    /// If it's not available for the specified substance,
    /// a [`FluidOutputError`] is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use approx::assert_relative_eq;
    /// use rfluids::prelude::fluid::*;
    /// use rfluids::uom::si::thermodynamic_temperature::degree_celsius;
    /// use rfluids::uom::si::ratio::percent;
    ///
    /// let mut water = Fluid::from(Pure::Water);
    /// assert_relative_eq!(water.triple_temperature()?.value, 273.16);
    /// assert_relative_eq!(
    ///     water.triple_temperature()?.get::<degree_celsius>(),
    ///     0.01,
    ///     epsilon = 1e-6
    /// );
    ///
    /// let mut propylene_glycol = Fluid::from(
    ///     BinaryMix::with_fraction(BinaryMixKind::MPG, Ratio::new::<percent>(40.0))?,
    /// );
    /// assert!(propylene_glycol.triple_temperature().is_err());
    /// # Ok::<(), rfluids::error::Error>(())
    /// ```
    pub fn triple_temperature(&mut self) -> OutputResult<ThermodynamicTemperature> {
        self.non_negative_trivial_output(FluidTrivialParam::TTriple)
            .map(ThermodynamicTemperature::new::<kelvin>)
    }

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

    fn non_negative_trivial_output(&mut self, key: FluidTrivialParam) -> OutputResult<f64> {
        self.trivial_output(key)
            .and_then(|value| non_negative(key.into(), value))
    }

    fn trivial_output(&mut self, key: FluidTrivialParam) -> OutputResult<f64> {
        cached_output(&mut self.trivial_outputs, &mut self.backend, key, |_| {
            FluidOutputError::UnavailableTrivialOutput(key)
        })
    }
}
