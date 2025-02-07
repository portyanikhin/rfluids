use super::OutputResult;
use crate::error::{CoolPropError, FluidOutputError};
use crate::io::{FluidParam, FluidTrivialParam};
use crate::native::AbstractState;
use crate::uom::si::f64::{MassDensity, MolarConcentration, MolarMass};
use std::collections::HashMap;
use std::hash::Hash;

pub(crate) fn cached_output<K>(
    cache: &mut HashMap<K, OutputResult<f64>>,
    backend: &mut AbstractState,
    key: K,
    f: impl FnOnce(CoolPropError) -> FluidOutputError,
) -> OutputResult<f64>
where
    K: Into<u8> + Eq + Hash + Copy,
{
    match cache
        .entry(key)
        .or_insert_with(|| backend.keyed_output(key).map_err(f))
        .as_ref()
    {
        Ok(&x) => Ok(x),
        Err(e) => Err(e.clone()),
    }
}

#[derive(Clone, Copy)]
pub(crate) enum Param {
    Trivial(FluidTrivialParam),
    NonTrivial(FluidParam),
}

impl From<FluidTrivialParam> for Param {
    fn from(value: FluidTrivialParam) -> Self {
        Param::Trivial(value)
    }
}

impl From<FluidParam> for Param {
    fn from(value: FluidParam) -> Self {
        Param::NonTrivial(value)
    }
}

pub(crate) fn non_negative(key: Param, value: f64) -> OutputResult<f64> {
    if value >= 0.0 {
        return Ok(value);
    }
    match key {
        Param::Trivial(key) => Err(FluidOutputError::UnavailableTrivialOutput(key)),
        Param::NonTrivial(key) => Err(FluidOutputError::UnavailableOutput(key)),
    }
}

pub(crate) fn density_from_molar_density(
    molar_density: OutputResult<MolarConcentration>,
    molar_mass: OutputResult<MolarMass>,
) -> OutputResult<MassDensity> {
    Ok(molar_density? * molar_mass?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::uom::si::molar_concentration::mole_per_cubic_meter;
    use crate::uom::si::molar_mass::kilogram_per_mole;
    use rstest::*;

    #[rstest]
    #[case(
        FluidTrivialParam::TCritical,
        -1.0,
        Err(FluidOutputError::UnavailableTrivialOutput(FluidTrivialParam::TCritical))
    )]
    #[case(FluidTrivialParam::TCritical, 0.0, Ok(0.0))]
    #[case(FluidTrivialParam::TCritical, 1.0, Ok(1.0))]
    #[rstest]
    #[case(
        FluidParam::T,
        -1.0,
        Err(FluidOutputError::UnavailableOutput(FluidParam::T))
    )]
    #[case(FluidParam::T, 0.0, Ok(0.0))]
    #[case(FluidParam::T, 1.0, Ok(1.0))]
    fn non_negative_returns_expected_value(
        #[case] key: impl Into<Param>,
        #[case] value: f64,
        #[case] expected: OutputResult<f64>,
    ) {
        assert_eq!(non_negative(key.into(), value), expected);
    }

    #[rstest]
    #[case(
        Err(FluidOutputError::UnavailableTrivialOutput(FluidTrivialParam::DMolarReducing)),
        Err(FluidOutputError::UnavailableTrivialOutput(FluidTrivialParam::MolarMass)),
        Err(FluidOutputError::UnavailableTrivialOutput(FluidTrivialParam::DMolarReducing))
    )]
    #[case(
        Ok(1.0),
        Err(FluidOutputError::UnavailableTrivialOutput(FluidTrivialParam::MolarMass)),
        Err(FluidOutputError::UnavailableTrivialOutput(FluidTrivialParam::MolarMass))
    )]
    #[case(
        Err(FluidOutputError::UnavailableTrivialOutput(FluidTrivialParam::DMolarReducing)),
        Ok(1.0),
        Err(FluidOutputError::UnavailableTrivialOutput(FluidTrivialParam::DMolarReducing))
    )]
    #[case(Ok(21.0), Ok(2.0), Ok(42.0))]
    fn density_from_molar_density_returns_expected_value(
        #[case] molar_density: OutputResult<f64>,
        #[case] molar_mass: OutputResult<f64>,
        #[case] expected: OutputResult<f64>,
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
