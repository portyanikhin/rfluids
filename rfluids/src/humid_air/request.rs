use std::collections::HashSet;

use super::HumidAirStateError;
use crate::io::HumidAirInput;

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct HumidAirUpdateRequest(
    pub(crate) HumidAirInput,
    pub(crate) HumidAirInput,
    pub(crate) HumidAirInput,
);

impl TryFrom<(HumidAirInput, HumidAirInput, HumidAirInput)> for HumidAirUpdateRequest {
    type Error = HumidAirStateError;

    fn try_from(
        inputs: (HumidAirInput, HumidAirInput, HumidAirInput),
    ) -> Result<Self, Self::Error> {
        let inputs_array: [HumidAirInput; 3] = inputs.into();
        let (keys, values) = (inputs_array.map(|x| x.key), inputs_array.map(|x| x.value));
        if HashSet::from(keys).len() != 3 {
            return Err(HumidAirStateError::InvalidInputs(keys[0], keys[1], keys[2]));
        }
        if !values.iter().all(|x| x.is_finite()) {
            return Err(HumidAirStateError::InvalidInputValue);
        }
        Ok(Self(inputs.0, inputs.1, inputs.2))
    }
}

#[cfg(test)]
mod tests {
    use rstest::*;

    use super::*;
    use crate::io::HumidAirParam;

    #[test]
    fn try_from_valid_inputs() {
        // Given
        let input1 = HumidAirInput::pressure(101_325.0);
        let input2 = HumidAirInput::temperature(293.15);
        let input3 = HumidAirInput::rel_humidity(0.5);

        // When
        let res = HumidAirUpdateRequest::try_from((input1, input2, input3)).unwrap();

        // Then
        assert_eq!(res.0, input1);
        assert_eq!(res.1, input2);
        assert_eq!(res.2, input3);
    }

    #[test]
    fn try_from_non_unique_inputs() {
        // Given
        let input1 = HumidAirInput::pressure(101_325.0);
        let input2 = HumidAirInput::altitude(300.0).unwrap();
        let input3 = HumidAirInput::rel_humidity(0.5);

        // When
        let res = HumidAirUpdateRequest::try_from((input1, input2, input3));

        // Then
        assert_eq!(
            res,
            Err(HumidAirStateError::InvalidInputs(
                HumidAirParam::P,
                HumidAirParam::P,
                HumidAirParam::R
            ))
        );
    }

    #[rstest]
    #[case(f64::INFINITY, 293.15, 0.5)]
    #[case(101_325.0, f64::INFINITY, 0.5)]
    #[case(101_325.0, 293.15, f64::INFINITY)]
    #[case(-f64::INFINITY, 293.15, 0.5)]
    #[case(101_325.0, -f64::INFINITY, 0.5)]
    #[case(101_325.0, 293.15, -f64::INFINITY)]
    #[case(f64::NAN, 293.15, 0.5)]
    #[case(101_325.0, f64::NAN, 0.5)]
    #[case(101_325.0, 293.15, f64::NAN)]
    fn try_from_non_finite_inputs(
        #[case] pressure: f64,
        #[case] temperature: f64,
        #[case] rel_humidity: f64,
    ) {
        // Given
        let input1 = HumidAirInput::pressure(pressure);
        let input2 = HumidAirInput::temperature(temperature);
        let input3 = HumidAirInput::rel_humidity(rel_humidity);

        // When
        let res = HumidAirUpdateRequest::try_from((input1, input2, input3));

        // Then
        assert_eq!(res, Err(HumidAirStateError::InvalidInputValue));
    }
}
