use uom::si::molar_mass::kilogram_per_mole;

use super::requests::FluidUpdateRequest;
use super::Fluid;
use crate::error::FluidUpdateError;
use crate::io::{FluidInput, FluidTrivialParam};
use crate::state_variant::StateVariant;
use crate::substance::Substance;
use crate::uom::si::f64::{
    MassDensity, MolarConcentration, MolarMass, Pressure, ThermodynamicTemperature,
};
use crate::uom::si::mass_density::kilogram_per_cubic_meter;
use crate::uom::si::molar_concentration::mole_per_cubic_meter;
use crate::uom::si::pressure::pascal;
use crate::uom::si::thermodynamic_temperature::kelvin;
use std::collections::hash_map::Entry;

impl<S: StateVariant> Fluid<S> {
    /// Specified substance.
    pub fn substance(&self) -> &Substance {
        &self.substance
    }

    /// Acentric factor
    /// _(key: [`AcentricFactor`](FluidTrivialParam::AcentricFactor), dimensionless)_.
    ///
    /// If it's not available for the specified substance, returns [`None`].
    ///
    /// # Examples
    ///
    /// ```
    /// use approx::assert_relative_eq;
    /// use rfluids::prelude::fluid::*;
    ///
    /// let mut water = Fluid::from(Pure::Water);
    /// assert_relative_eq!(water.acentric_factor().unwrap(), 0.3442920843);
    ///
    /// let mut r444a = Fluid::from(PredefinedMix::R444A);
    /// assert!(r444a.acentric_factor().is_none());
    /// ```
    pub fn acentric_factor(&mut self) -> Option<f64> {
        self.trivial_output(FluidTrivialParam::AcentricFactor)
    }

    /// Critical point mass density
    /// _(key: [`DMassCritical`](FluidTrivialParam::DMassCritical), SI units: kg/m³)_.
    ///
    /// If it's not available for the specified substance, returns [`None`].
    ///
    /// # Examples
    ///
    /// ```
    /// use approx::assert_relative_eq;
    /// use rfluids::prelude::fluid::*;
    ///
    /// let mut water = Fluid::from(Pure::Water);
    /// assert_relative_eq!(water.critical_density().unwrap().value, 322.0);
    ///
    /// let mut r444a = Fluid::from(PredefinedMix::R444A);
    /// assert!(r444a.critical_density().is_none());
    /// ```
    pub fn critical_density(&mut self) -> Option<MassDensity> {
        // Due to CoolProp freeze
        if let Substance::PredefinedMix(_) = self.substance {
            return None;
        }
        Some(MassDensity::new::<kilogram_per_cubic_meter>(non_negative(
            self.trivial_output(FluidTrivialParam::DMassCritical),
        )?))
    }

    /// Critical point molar density
    /// _(key: [`DMolarCritical`](FluidTrivialParam::DMolarCritical), SI units: mol/m³)_.
    ///
    /// If it's not available for the specified substance, returns [`None`].
    ///
    /// # Examples
    ///
    /// ```
    /// use approx::assert_relative_eq;
    /// use rfluids::prelude::fluid::*;
    ///
    /// let mut water = Fluid::from(Pure::Water);
    /// assert_relative_eq!(
    ///     water.critical_molar_density().unwrap().value,
    ///     17873.72799560906
    /// );
    ///
    /// let mut r444a = Fluid::from(PredefinedMix::R444A);
    /// assert!(r444a.critical_molar_density().is_none());
    /// ```
    pub fn critical_molar_density(&mut self) -> Option<MolarConcentration> {
        // Due to CoolProp freeze
        if let Substance::PredefinedMix(_) = self.substance {
            return None;
        }
        Some(MolarConcentration::new::<mole_per_cubic_meter>(
            non_negative(self.trivial_output(FluidTrivialParam::DMolarCritical))?,
        ))
    }

    /// Critical point pressure
    /// _(key: [`PCritical`](FluidTrivialParam::PCritical), SI units: Pa)_.
    ///
    /// If it's not available for the specified substance, returns [`None`].
    ///
    /// # Examples
    ///
    /// ```
    /// use approx::assert_relative_eq;
    /// use rfluids::prelude::fluid::*;
    ///
    /// let mut water = Fluid::from(Pure::Water);
    /// assert_relative_eq!(water.critical_pressure().unwrap().value, 22.064e6);
    ///
    /// let mut r444a = Fluid::from(PredefinedMix::R444A);
    /// assert!(r444a.critical_pressure().is_none());
    /// ```
    pub fn critical_pressure(&mut self) -> Option<Pressure> {
        // Due to CoolProp freeze
        if let Substance::PredefinedMix(_) = self.substance {
            return None;
        }
        Some(Pressure::new::<pascal>(non_negative(
            self.trivial_output(FluidTrivialParam::PCritical),
        )?))
    }

    /// Critical point temperature
    /// _(key: [`TCritical`](FluidTrivialParam::TCritical), SI units: K)_.
    ///
    /// If it's not available for the specified substance, returns [`None`].
    ///
    /// # Examples
    ///
    /// ```
    /// use approx::assert_relative_eq;
    /// use rfluids::prelude::fluid::*;
    ///
    /// let mut water = Fluid::from(Pure::Water);
    /// assert_relative_eq!(water.critical_temperature().unwrap().value, 647.096);
    ///
    /// let mut r444a = Fluid::from(PredefinedMix::R444A);
    /// assert!(r444a.critical_temperature().is_none());
    /// ```
    pub fn critical_temperature(&mut self) -> Option<ThermodynamicTemperature> {
        // Due to CoolProp freeze
        if let Substance::PredefinedMix(_) = self.substance {
            return None;
        }
        Some(ThermodynamicTemperature::new::<kelvin>(non_negative(
            self.trivial_output(FluidTrivialParam::TCritical),
        )?))
    }

    /// Flammability hazard index
    /// _(key: [`FH`](FluidTrivialParam::FH), dimensionless)_.
    ///
    /// If it's not available for the specified substance, returns [`None`].
    ///
    /// # Examples
    ///
    /// ```
    /// use rfluids::prelude::fluid::*;
    /// use rfluids::uom::si::ratio::percent;
    ///
    /// let mut acetone = Fluid::from(Pure::Acetone);
    /// assert_eq!(acetone.flammability_hazard().unwrap(), 3.0);
    ///
    /// let mut water = Fluid::from(Pure::Water);
    /// assert_eq!(water.flammability_hazard().unwrap(), 0.0);
    ///
    /// let mut propylene_glycol = Fluid::from(
    ///     BinaryMix::try_from(BinaryMixKind::MPG, Ratio::new::<percent>(40.0)).unwrap(),
    /// );
    /// assert!(propylene_glycol.flammability_hazard().is_none());
    /// ```
    pub fn flammability_hazard(&mut self) -> Option<f64> {
        non_negative(self.trivial_output(FluidTrivialParam::FH))
    }

    /// Freezing temperature for incompressible mixtures
    /// _(key: [`TFreeze`](FluidTrivialParam::TFreeze), SI units: K)_.
    ///
    /// If it's not available for the specified substance, returns [`None`].
    ///
    /// # Examples
    ///
    /// ```
    /// use approx::assert_relative_eq;
    /// use rfluids::prelude::fluid::*;
    /// use rfluids::uom::si::ratio::percent;
    ///
    /// let mut water = Fluid::from(Pure::Water);
    /// assert!(water.freezing_temperature().is_none());
    ///
    /// let mut propylene_glycol = Fluid::from(
    ///     BinaryMix::try_from(BinaryMixKind::MPG, Ratio::new::<percent>(40.0)).unwrap(),
    /// );
    /// assert_relative_eq!(
    ///     propylene_glycol.freezing_temperature().unwrap().value,
    ///     252.58175495305838
    /// );
    /// ```
    pub fn freezing_temperature(&mut self) -> Option<ThermodynamicTemperature> {
        Some(ThermodynamicTemperature::new::<kelvin>(non_negative(
            self.trivial_output(FluidTrivialParam::TFreeze),
        )?))
    }

    /// 20-year global warming potential
    /// _(key: [`GWP20`](FluidTrivialParam::GWP20), dimensionless)_.
    ///
    /// If it's not available for the specified substance, returns [`None`].
    ///
    /// # Examples
    ///
    /// ```
    /// use rfluids::prelude::fluid::*;
    ///
    /// let mut water = Fluid::from(Pure::Water);
    /// assert!(water.gwp20().is_none());
    ///
    /// let mut r32 = Fluid::from(Pure::R32);
    /// assert_eq!(r32.gwp20().unwrap(), 2330.0);
    /// ```
    pub fn gwp20(&mut self) -> Option<f64> {
        non_negative(self.trivial_output(FluidTrivialParam::GWP20))
    }

    /// 100-year global warming potential
    /// _(key: [`GWP100`](FluidTrivialParam::GWP100), dimensionless)_.
    ///
    /// If it's not available for the specified substance, returns [`None`].
    ///
    /// # Examples
    ///
    /// ```
    /// use rfluids::prelude::fluid::*;
    ///
    /// let mut water = Fluid::from(Pure::Water);
    /// assert!(water.gwp100().is_none());
    ///
    /// let mut r32 = Fluid::from(Pure::R32);
    /// assert_eq!(r32.gwp100().unwrap(), 675.0);
    /// ```
    pub fn gwp100(&mut self) -> Option<f64> {
        non_negative(self.trivial_output(FluidTrivialParam::GWP100))
    }

    /// 500-year global warming potential
    /// _(key: [`GWP500`](FluidTrivialParam::GWP500), dimensionless)_.
    ///
    /// If it's not available for the specified substance, returns [`None`].
    ///
    /// # Examples
    ///
    /// ```
    /// use rfluids::prelude::fluid::*;
    ///
    /// let mut water = Fluid::from(Pure::Water);
    /// assert!(water.gwp500().is_none());
    ///
    /// let mut r32 = Fluid::from(Pure::R32);
    /// assert_eq!(r32.gwp500().unwrap(), 205.0);
    /// ```
    pub fn gwp500(&mut self) -> Option<f64> {
        non_negative(self.trivial_output(FluidTrivialParam::GWP500))
    }

    /// Health hazard index
    /// _(key: [`HH`](FluidTrivialParam::HH), dimensionless)_.
    ///
    /// If it's not available for the specified substance, returns [`None`].
    ///
    /// # Examples
    ///
    /// ```
    /// use rfluids::prelude::fluid::*;
    /// use rfluids::uom::si::ratio::percent;
    ///
    /// let mut acetone = Fluid::from(Pure::Acetone);
    /// assert_eq!(acetone.health_hazard().unwrap(), 2.0);
    ///
    /// let mut water = Fluid::from(Pure::Water);
    /// assert_eq!(water.health_hazard().unwrap(), 0.0);
    ///
    /// let mut propylene_glycol = Fluid::from(
    ///     BinaryMix::try_from(BinaryMixKind::MPG, Ratio::new::<percent>(40.0)).unwrap(),
    /// );
    /// assert!(propylene_glycol.health_hazard().is_none());
    /// ```
    pub fn health_hazard(&mut self) -> Option<f64> {
        non_negative(self.trivial_output(FluidTrivialParam::HH))
    }

    /// Maximum pressure
    /// _(key: [`PMax`](FluidTrivialParam::PMax), SI units: Pa)_.
    ///
    /// If it's not available for the specified substance, returns [`None`].
    ///
    /// # Examples
    ///
    /// ```
    /// use rfluids::prelude::fluid::*;
    /// use rfluids::uom::si::ratio::percent;
    ///
    /// let mut water = Fluid::from(Pure::Water);
    /// assert_eq!(water.max_pressure().unwrap().value, 1e9);
    ///
    /// let mut propylene_glycol = Fluid::from(
    ///     BinaryMix::try_from(BinaryMixKind::MPG, Ratio::new::<percent>(40.0)).unwrap(),
    /// );
    /// assert!(propylene_glycol.max_pressure().is_none());
    /// ```
    pub fn max_pressure(&mut self) -> Option<Pressure> {
        Some(Pressure::new::<pascal>(non_negative(
            self.trivial_output(FluidTrivialParam::PMax),
        )?))
    }

    /// Maximum temperature
    /// _(key: [`TMax`](FluidTrivialParam::TMax), SI units: K)_.
    ///
    /// # Examples
    ///
    /// ```
    /// use rfluids::prelude::fluid::*;
    ///
    /// let mut water = Fluid::from(Pure::Water);
    /// assert_eq!(water.max_temperature().value, 2e3);
    /// ```
    pub fn max_temperature(&mut self) -> ThermodynamicTemperature {
        ThermodynamicTemperature::new::<kelvin>(
            non_negative(self.trivial_output(FluidTrivialParam::TMax)).unwrap(),
        )
    }

    /// Minimum pressure
    /// _(key: [`PMin`](FluidTrivialParam::PMin), SI units: Pa)_.
    ///
    /// If it's not available for the specified substance, returns [`None`].
    ///
    /// # Examples
    ///
    /// ```
    /// use approx::assert_relative_eq;
    /// use rfluids::prelude::fluid::*;
    /// use rfluids::uom::si::ratio::percent;
    ///
    /// let mut water = Fluid::from(Pure::Water);
    /// assert_relative_eq!(
    ///     water.min_pressure().unwrap().value,
    ///     611.6548008968684
    /// );
    ///
    /// let mut propylene_glycol = Fluid::from(
    ///     BinaryMix::try_from(BinaryMixKind::MPG, Ratio::new::<percent>(40.0)).unwrap(),
    /// );
    /// assert!(propylene_glycol.min_pressure().is_none());
    /// ```
    pub fn min_pressure(&mut self) -> Option<Pressure> {
        Some(Pressure::new::<pascal>(non_negative(
            self.trivial_output(FluidTrivialParam::PMin),
        )?))
    }

    /// Minimum temperature
    /// _(key: [`TMin`](FluidTrivialParam::TMin), SI units: K)_.
    ///
    /// # Examples
    ///
    /// ```
    /// use rfluids::prelude::fluid::*;
    ///
    /// let mut water = Fluid::from(Pure::Water);
    /// assert_eq!(water.min_temperature().value, 273.16);
    /// ```
    pub fn min_temperature(&mut self) -> ThermodynamicTemperature {
        ThermodynamicTemperature::new::<kelvin>(
            non_negative(self.trivial_output(FluidTrivialParam::TMin)).unwrap(),
        )
    }

    /// Molar mass
    /// _(key: [`MolarMass`](FluidTrivialParam::MolarMass), SI units: kg/mol)_.
    ///
    /// If it's not available for the specified substance, returns [`None`].
    ///
    /// # Examples
    ///
    /// ```
    /// use approx::assert_relative_eq;
    /// use rfluids::prelude::fluid::*;
    /// use rfluids::uom::si::ratio::percent;
    ///
    /// let mut water = Fluid::from(Pure::Water);
    /// assert_relative_eq!(water.molar_mass().unwrap().value, 0.018015268);
    ///
    /// let mut propylene_glycol = Fluid::from(
    ///     BinaryMix::try_from(BinaryMixKind::MPG, Ratio::new::<percent>(40.0)).unwrap(),
    /// );
    /// assert!(propylene_glycol.molar_mass().is_none());
    /// ```
    pub fn molar_mass(&mut self) -> Option<MolarMass> {
        Some(MolarMass::new::<kilogram_per_mole>(non_negative(
            self.trivial_output(FluidTrivialParam::MolarMass),
        )?))
    }

    /// Ozone depletion potential
    /// _(key: [`ODP`](FluidTrivialParam::ODP), dimensionless)_.
    ///
    /// If it's not available for the specified substance, returns [`None`].
    ///
    /// # Examples
    ///
    /// ```
    /// use rfluids::prelude::fluid::*;
    ///
    /// let mut water = Fluid::from(Pure::Water);
    /// assert!(water.odp().is_none());
    ///
    /// let mut r32 = Fluid::from(Pure::R32);
    /// assert!(r32.odp().is_none());
    ///
    /// let mut r22 = Fluid::from(Pure::R22);
    /// assert_eq!(r22.odp().unwrap(), 0.05);
    /// ```
    pub fn odp(&mut self) -> Option<f64> {
        non_negative(self.trivial_output(FluidTrivialParam::ODP))
    }

    /// Physical hazard index
    /// _(key: [`PH`](FluidTrivialParam::PH), dimensionless)_.
    ///
    /// If it's not available for the specified substance, returns [`None`].
    ///
    /// # Examples
    ///
    /// ```
    /// use rfluids::prelude::fluid::*;
    /// use rfluids::uom::si::ratio::percent;
    ///
    /// let mut carbon_monoxide = Fluid::from(Pure::CarbonMonoxide);
    /// assert_eq!(carbon_monoxide.physical_hazard().unwrap(), 3.0);
    ///
    /// let mut water = Fluid::from(Pure::Water);
    /// assert_eq!(water.physical_hazard().unwrap(), 0.0);
    ///
    /// let mut propylene_glycol = Fluid::from(
    ///     BinaryMix::try_from(BinaryMixKind::MPG, Ratio::new::<percent>(40.0)).unwrap(),
    /// );
    /// assert!(propylene_glycol.physical_hazard().is_none());
    /// ```
    pub fn physical_hazard(&mut self) -> Option<f64> {
        non_negative(self.trivial_output(FluidTrivialParam::PH))
    }

    /// Reducing point mass density
    /// _(key: [`DMassReducing`](FluidTrivialParam::DMassReducing), SI units: kg/m³)_.
    ///
    /// If it's not available for the specified substance, returns [`None`].
    ///
    /// # Examples
    ///
    /// ```
    /// use approx::assert_relative_eq;
    /// use rfluids::prelude::fluid::*;
    /// use rfluids::uom::si::ratio::percent;
    ///
    /// let mut water = Fluid::from(Pure::Water);
    /// assert_relative_eq!(water.reducing_density().unwrap().value, 322.0);
    ///
    /// let mut propylene_glycol = Fluid::from(
    ///     BinaryMix::try_from(BinaryMixKind::MPG, Ratio::new::<percent>(40.0)).unwrap(),
    /// );
    /// assert!(propylene_glycol.reducing_density().is_none());
    /// ```
    pub fn reducing_density(&mut self) -> Option<MassDensity> {
        // Due to CoolProp bug
        density_from_molar_density(self.reducing_molar_density(), self.molar_mass())
    }

    /// Reducing point molar density
    /// _(key: [`DMolarReducing`](FluidTrivialParam::DMolarReducing), SI units: mol/m³)_.
    ///
    /// If it's not available for the specified substance, returns [`None`].
    ///
    /// # Examples
    ///
    /// ```
    /// use approx::assert_relative_eq;
    /// use rfluids::prelude::fluid::*;
    /// use rfluids::uom::si::ratio::percent;
    ///
    /// let mut water = Fluid::from(Pure::Water);
    /// assert_relative_eq!(
    ///     water.reducing_molar_density().unwrap().value,
    ///     17873.72799560906
    /// );
    ///
    /// let mut propylene_glycol = Fluid::from(
    ///     BinaryMix::try_from(BinaryMixKind::MPG, Ratio::new::<percent>(40.0)).unwrap(),
    /// );
    /// assert!(propylene_glycol.reducing_molar_density().is_none());
    /// ```
    pub fn reducing_molar_density(&mut self) -> Option<MolarConcentration> {
        Some(MolarConcentration::new::<mole_per_cubic_meter>(
            non_negative(self.trivial_output(FluidTrivialParam::DMolarReducing))?,
        ))
    }

    /// Reducing point pressure
    /// _(key: [`PReducing`](FluidTrivialParam::PReducing), SI units: Pa)_.
    ///
    /// If it's not available for the specified substance, returns [`None`].
    ///
    /// # Examples
    ///
    /// ```
    /// use approx::assert_relative_eq;
    /// use rfluids::prelude::fluid::*;
    /// use rfluids::uom::si::ratio::percent;
    ///
    /// let mut water = Fluid::from(Pure::Water);
    /// assert_relative_eq!(water.reducing_pressure().unwrap().value, 22.064e6);
    ///
    /// let mut propylene_glycol = Fluid::from(
    ///     BinaryMix::try_from(BinaryMixKind::MPG, Ratio::new::<percent>(40.0)).unwrap(),
    /// );
    /// assert!(propylene_glycol.reducing_pressure().is_none());
    /// ```
    pub fn reducing_pressure(&mut self) -> Option<Pressure> {
        Some(Pressure::new::<pascal>(non_negative(
            self.trivial_output(FluidTrivialParam::PReducing),
        )?))
    }

    /// Reducing point temperature
    /// _(key: [`TReducing`](FluidTrivialParam::TReducing), SI units: K)_.
    ///
    /// If it's not available for the specified substance, returns [`None`].
    ///
    /// # Examples
    ///
    /// ```
    /// use approx::assert_relative_eq;
    /// use rfluids::prelude::fluid::*;
    /// use rfluids::uom::si::ratio::percent;
    ///
    /// let mut water = Fluid::from(Pure::Water);
    /// assert_relative_eq!(water.reducing_temperature().unwrap().value, 647.096);
    ///
    /// let mut propylene_glycol = Fluid::from(
    ///     BinaryMix::try_from(BinaryMixKind::MPG, Ratio::new::<percent>(40.0)).unwrap(),
    /// );
    /// assert!(propylene_glycol.reducing_temperature().is_none());
    /// ```
    pub fn reducing_temperature(&mut self) -> Option<ThermodynamicTemperature> {
        Some(ThermodynamicTemperature::new::<kelvin>(non_negative(
            self.trivial_output(FluidTrivialParam::TReducing),
        )?))
    }

    /// Triple point pressure
    /// _(key: [`PTriple`](FluidTrivialParam::PTriple), SI units: Pa)_.
    ///
    /// If it's not available for the specified substance, returns [`None`].
    ///
    /// # Examples
    ///
    /// ```
    /// use approx::assert_relative_eq;
    /// use rfluids::prelude::fluid::*;
    /// use rfluids::uom::si::ratio::percent;
    ///
    /// let mut water = Fluid::from(Pure::Water);
    /// assert_relative_eq!(
    ///     water.triple_pressure().unwrap().value,
    ///     611.6548008968684
    /// );
    ///
    /// let mut propylene_glycol = Fluid::from(
    ///     BinaryMix::try_from(BinaryMixKind::MPG, Ratio::new::<percent>(40.0)).unwrap(),
    /// );
    /// assert!(propylene_glycol.triple_pressure().is_none());
    /// ```
    pub fn triple_pressure(&mut self) -> Option<Pressure> {
        Some(Pressure::new::<pascal>(non_negative(
            self.trivial_output(FluidTrivialParam::PTriple),
        )?))
    }

    /// Triple point temperature
    /// _(key: [`TTriple`](FluidTrivialParam::TTriple), SI units: K)_.
    ///
    /// If it's not available for the specified substance, returns [`None`].
    ///
    /// # Examples
    ///
    /// ```
    /// use approx::assert_relative_eq;
    /// use rfluids::prelude::fluid::*;
    /// use rfluids::uom::si::ratio::percent;
    ///
    /// let mut water = Fluid::from(Pure::Water);
    /// assert_relative_eq!(water.triple_temperature().unwrap().value, 273.16);
    ///
    /// let mut propylene_glycol = Fluid::from(
    ///     BinaryMix::try_from(BinaryMixKind::MPG, Ratio::new::<percent>(40.0)).unwrap(),
    /// );
    /// assert!(propylene_glycol.triple_temperature().is_none());
    /// ```
    pub fn triple_temperature(&mut self) -> Option<ThermodynamicTemperature> {
        Some(ThermodynamicTemperature::new::<kelvin>(non_negative(
            self.trivial_output(FluidTrivialParam::TTriple),
        )?))
    }

    pub(crate) fn inner_update(
        &mut self,
        input1: FluidInput,
        input2: FluidInput,
    ) -> Result<(), FluidUpdateError> {
        let request: FluidUpdateRequest = (input1, input2).try_into()?;
        self.backend.update(request.0, request.1, request.2)?;
        self.outputs.clear();
        self.outputs.insert(input1.key(), input1.si_value());
        self.outputs.insert(input2.key(), input2.si_value());
        self.update_request = Some(request);
        Ok(())
    }

    fn trivial_output(&mut self, key: FluidTrivialParam) -> Option<f64> {
        match self.trivial_outputs.entry(key) {
            Entry::Occupied(entry) => *entry.get(),
            Entry::Vacant(entry) => *entry.insert(self.backend.keyed_output(key).ok()),
        }
    }
}

fn non_negative(value: Option<f64>) -> Option<f64> {
    value.and_then(|v| if v >= 0.0 { Some(v) } else { None })
}

fn density_from_molar_density(
    molar_density: Option<MolarConcentration>,
    molar_mass: Option<MolarMass>,
) -> Option<MassDensity> {
    Some(molar_density? * molar_mass?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(None, None)]
    #[case(Some(-1.0), None)]
    #[case(Some(0.0), Some(0.0))]
    #[case(Some(1.0), Some(1.0))]
    fn non_negative_returns_none_for_negative_value(
        #[case] value: Option<f64>,
        #[case] expected: Option<f64>,
    ) {
        assert_eq!(non_negative(value), expected);
    }

    #[rstest]
    #[case(None, None, None)]
    #[case(Some(1.0), None, None)]
    #[case(None, Some(1.0), None)]
    #[case(Some(21.0), Some(2.0), Some(42.0))]
    fn density_from_molar_density_returns_expected_value(
        #[case] molar_density: Option<f64>,
        #[case] molar_mass: Option<f64>,
        #[case] expected: Option<f64>,
    ) {
        assert_eq!(
            density_from_molar_density(
                molar_density.map(MolarConcentration::new::<mole_per_cubic_meter>),
                molar_mass.map(MolarMass::new::<kilogram_per_mole>)
            )
            .map(|v| v.value),
            expected
        );
    }
}
