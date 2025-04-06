use super::{OutputResult, requests::HumidAirUpdateRequest};
use crate::{
    error::HumidAirOutputError,
    io::{HumidAirParam, Input},
    native::CoolProp,
};
use std::collections::HashMap;

pub(crate) fn cached_output(
    cache: &mut HashMap<HumidAirParam, OutputResult<f64>>,
    key: HumidAirParam,
    request: HumidAirUpdateRequest,
) -> OutputResult<f64> {
    match cache
        .entry(key)
        .or_insert_with(|| {
            CoolProp::ha_props_si(
                key,
                request.0.key(),
                request.0.si_value(),
                request.1.key(),
                request.1.si_value(),
                request.2.key(),
                request.2.si_value(),
            )
            .map_err(|e| HumidAirOutputError::CalculationFailed(key, e))
        })
        .as_ref()
    {
        Ok(&x) => Ok(x),
        Err(e) => Err(e.clone()),
    }
}

pub(crate) fn guard(key: HumidAirParam, value: f64, ok: fn(f64) -> bool) -> OutputResult<f64> {
    if ok(value) {
        return Ok(value);
    }
    Err(HumidAirOutputError::UnavailableOutput(key))
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(
        HumidAirParam::W, -1.0, Err(HumidAirOutputError::UnavailableOutput(HumidAirParam::W))
    )]
    #[case(HumidAirParam::W, 0.0, Ok(0.0))]
    #[case(HumidAirParam::W, 1.0, Ok(1.0))]
    fn guard_returns_expected_value(
        #[case] key: HumidAirParam,
        #[case] value: f64,
        #[case] expected: OutputResult<f64>,
    ) {
        assert_eq!(guard(key, value, |x| x >= 0.0), expected);
    }
}
