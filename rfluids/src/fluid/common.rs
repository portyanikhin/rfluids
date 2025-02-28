use super::OutputResult;
use crate::error::{CoolPropError, FluidOutputError};
use crate::io::{FluidParam, FluidTrivialParam};
use crate::native::AbstractState;
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

pub(crate) fn guard(key: Param, value: f64, ok: fn(f64) -> bool) -> OutputResult<f64> {
    if ok(value) {
        return Ok(value);
    }
    match key {
        Param::Trivial(key) => Err(FluidOutputError::UnavailableTrivialOutput(key)),
        Param::NonTrivial(key) => Err(FluidOutputError::UnavailableOutput(key)),
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(
        FluidTrivialParam::TCritical,
        -1.0,
        Err(FluidOutputError::UnavailableTrivialOutput(FluidTrivialParam::TCritical))
    )]
    #[case(FluidTrivialParam::TCritical, 0.0, Ok(0.0))]
    #[case(FluidTrivialParam::TCritical, 1.0, Ok(1.0))]
    #[case(
        FluidParam::T,
        -1.0,
        Err(FluidOutputError::UnavailableOutput(FluidParam::T))
    )]
    #[case(FluidParam::T, 0.0, Ok(0.0))]
    #[case(FluidParam::T, 1.0, Ok(1.0))]
    fn guard_returns_expected_value(
        #[case] key: impl Into<Param>,
        #[case] value: f64,
        #[case] expected: OutputResult<f64>,
    ) {
        assert_eq!(guard(key.into(), value, |x| x >= 0.0), expected);
    }
}
