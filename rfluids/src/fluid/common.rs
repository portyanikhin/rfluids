use crate::error::CoolPropError;
use crate::io::{FluidInput, FluidInputPair, FluidParam};
use crate::native::AbstractState;
use crate::Remember;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug, Copy, Clone, PartialEq)]
pub(crate) struct FluidUpdateRequest(pub FluidInputPair, pub f64, pub f64);

impl From<FluidUpdateRequest> for (FluidInput, FluidInput) {
    fn from(value: FluidUpdateRequest) -> Self {
        let keys: (FluidParam, FluidParam) = value.0.into();
        (
            FluidInput {
                key: keys.0,
                si_value: value.1,
            },
            FluidInput {
                key: keys.1,
                si_value: value.2,
            },
        )
    }
}

impl TryFrom<(FluidInput, FluidInput)> for FluidUpdateRequest {
    type Error = strum::ParseError;

    fn try_from(value: (FluidInput, FluidInput)) -> Result<Self, Self::Error> {
        let key = FluidInputPair::try_from((value.0.key, value.1.key))?;
        let (value1, value2) =
            if <(FluidParam, FluidParam)>::from(key) == (value.0.key, value.1.key) {
                (value.0.si_value, value.1.si_value)
            } else {
                (value.1.si_value, value.0.si_value)
            };
        Ok(Self(key, value1, value2))
    }
}

impl<K> Remember<&AbstractState, K> for HashMap<K, f64>
where
    K: Into<u8> + Copy + Eq + Hash,
{
    type Error = CoolPropError;

    fn remember(&mut self, src: &AbstractState, key: K) -> Result<f64, CoolPropError> {
        Ok(match self.entry(key) {
            Entry::Occupied(entry) => *entry.get(),
            Entry::Vacant(entry) => *entry.insert(src.keyed_output(key)?),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::uom::si::f64::{Pressure, ThermodynamicTemperature};
    use crate::uom::si::pressure::atmosphere;
    use crate::uom::si::thermodynamic_temperature::degree_celsius;
    use approx::assert_relative_eq;

    #[test]
    fn two_fluid_inputs_from_fluid_update_request_always_returns_expected_value() {
        let request = FluidUpdateRequest(FluidInputPair::PT, 101325.0, 293.15);
        let result = <(FluidInput, FluidInput)>::from(request);
        assert_eq!(result.0.key, FluidParam::P);
        assert_eq!(result.0.si_value, request.1);
        assert_eq!(result.1.key, FluidParam::T);
        assert_eq!(result.1.si_value, request.2);
    }

    #[test]
    fn try_from_two_valid_inputs_with_invariant_order_always_returns_ok() {
        let input1 = FluidInput::temperature(ThermodynamicTemperature::new::<degree_celsius>(20.0));
        let input2 = FluidInput::pressure(Pressure::new::<atmosphere>(1.0));
        let result1 = FluidUpdateRequest::try_from((input1, input2)).unwrap();
        let result2 = FluidUpdateRequest::try_from((input2, input1)).unwrap();
        assert_eq!(result1, result2);
        assert_eq!(result1.0, FluidInputPair::PT);
        assert_relative_eq!(result1.1, 101325.0);
        assert_relative_eq!(result1.2, 293.15);
    }

    #[test]
    fn try_from_two_invalid_inputs_always_returns_err() {
        let input = FluidInput::pressure(Pressure::new::<atmosphere>(1.0));
        assert!(FluidUpdateRequest::try_from((input, input)).is_err());
    }
}
