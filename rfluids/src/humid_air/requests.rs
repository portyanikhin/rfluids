use std::collections::HashSet;

use crate::error::HumidAirStateError;
use crate::io::Input;
use crate::io::humid_air_input::HumidAirInput;

#[derive(Debug, Copy, Clone, PartialEq)]
pub(crate) struct HumidAirUpdateRequest(HumidAirInput, HumidAirInput, HumidAirInput);

impl TryFrom<(HumidAirInput, HumidAirInput, HumidAirInput)> for HumidAirUpdateRequest {
    type Error = HumidAirStateError;

    fn try_from(value: (HumidAirInput, HumidAirInput, HumidAirInput)) -> Result<Self, Self::Error> {
        let inputs: [HumidAirInput; 3] = value.into();
        let (keys, values) = (inputs.map(|x| x.key()), inputs.map(|x| x.si_value()));
        if HashSet::from(keys).len() != 3 {
            return Err(HumidAirStateError::InvalidInputs(keys[0], keys[1], keys[2]));
        }
        if !values.iter().all(|x| x.is_finite()) {
            return Err(HumidAirStateError::InvalidInputValue);
        }
        Ok(Self(value.0, value.1, value.2))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::io::{HumidAirParam, humid_air_input};
    use crate::uom::si::length::meter;
    use crate::uom::si::pressure::atmosphere;
    use crate::uom::si::ratio::percent;
    use crate::uom::si::thermodynamic_temperature::degree_celsius;
    use rstest::*;

    #[test]
    fn try_from_valid_inputs_returns_ok() {
        let input1 = humid_air_input::pressure!(1.0, atmosphere);
        let input2 = humid_air_input::temperature!(20.0, degree_celsius);
        let input3 = humid_air_input::rel_humidity!(50.0, percent);
        let result = HumidAirUpdateRequest::try_from((input1, input2, input3)).unwrap();
        assert_eq!(result.0, input1);
        assert_eq!(result.1, input2);
        assert_eq!(result.2, input3);
    }

    #[test]
    fn try_from_non_unique_inputs_returns_err() {
        let input1 = humid_air_input::pressure!(1.0, atmosphere);
        let input2 = humid_air_input::altitude!(300.0, meter).unwrap();
        let input3 = humid_air_input::rel_humidity!(50.0, percent);
        let result = HumidAirUpdateRequest::try_from((input1, input2, input3));
        assert_eq!(
            result.unwrap_err(),
            HumidAirStateError::InvalidInputs(HumidAirParam::P, HumidAirParam::P, HumidAirParam::R)
        );
    }

    #[rstest]
    #[case(f64::INFINITY, 20.0, 50.0)]
    #[case(1.0, f64::INFINITY, 50.0)]
    #[case(1.0, 20.0, f64::INFINITY)]
    #[case(-f64::INFINITY, 20.0, 50.0)]
    #[case(1.0, -f64::INFINITY, 50.0)]
    #[case(1.0, 20.0, -f64::INFINITY)]
    #[case(f64::NAN, 20.0, 50.0)]
    #[case(1.0, f64::NAN, 50.0)]
    #[case(1.0, 20.0, f64::NAN)]
    fn try_from_non_finite_inputs_returns_err(
        #[case] pressure: f64,
        #[case] temperature: f64,
        #[case] rel_humidity: f64,
    ) {
        let input1 = humid_air_input::pressure!(pressure, atmosphere);
        let input2 = humid_air_input::temperature!(temperature, degree_celsius);
        let input3 = humid_air_input::rel_humidity!(rel_humidity, percent);
        let result = HumidAirUpdateRequest::try_from((input1, input2, input3));
        assert_eq!(result.unwrap_err(), HumidAirStateError::InvalidInputValue);
    }
}
