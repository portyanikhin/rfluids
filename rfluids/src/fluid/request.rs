use super::FluidStateError;
use crate::io::{FluidInput, FluidInputPair, FluidParam};

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct FluidUpdateRequest {
    pub(crate) input_pair: FluidInputPair,
    pub(crate) value1: f64,
    pub(crate) value2: f64,
}

impl From<FluidUpdateRequest> for (FluidInput, FluidInput) {
    fn from(request: FluidUpdateRequest) -> Self {
        let keys: (FluidParam, FluidParam) = request.input_pair.into();
        (
            FluidInput { key: keys.0, value: request.value1 },
            FluidInput { key: keys.1, value: request.value2 },
        )
    }
}

impl TryFrom<(FluidInput, FluidInput)> for FluidUpdateRequest {
    type Error = FluidStateError;

    fn try_from(inputs: (FluidInput, FluidInput)) -> Result<Self, Self::Error> {
        let input_pair = FluidInputPair::try_from((inputs.0.key, inputs.1.key))
            .map_err(|_| FluidStateError::InvalidInputPair(inputs.0.key, inputs.1.key))?;
        if !inputs.0.value.is_finite() || !inputs.1.value.is_finite() {
            return Err(FluidStateError::InvalidInputValue);
        }
        let (value1, value2) =
            if <(FluidParam, FluidParam)>::from(input_pair) == (inputs.0.key, inputs.1.key) {
                (inputs.0.value, inputs.1.value)
            } else {
                (inputs.1.value, inputs.0.value)
            };
        Ok(Self { input_pair, value1, value2 })
    }
}

#[cfg(test)]
mod tests {
    use rstest::*;

    use super::*;
    use crate::test::assert_relative_eq;

    #[test]
    fn into_inputs() {
        // Given
        let sut = FluidUpdateRequest {
            input_pair: FluidInputPair::PT,
            value1: 101_325.0,
            value2: 293.15,
        };

        // When
        let res: (FluidInput, FluidInput) = sut.into();

        // Then
        assert_eq!(res.0.key, FluidParam::P);
        assert_eq!(res.0.value, sut.value1);
        assert_eq!(res.1.key, FluidParam::T);
        assert_eq!(res.1.value, sut.value2);
    }

    #[test]
    fn try_from_valid_inputs_with_invariant_order() {
        // Given
        let input1 = FluidInput::temperature(293.15);
        let input2 = FluidInput::pressure(101_325.0);

        // When
        let res1 = FluidUpdateRequest::try_from((input1, input2)).unwrap();
        let res2 = FluidUpdateRequest::try_from((input2, input1)).unwrap();

        // Then
        assert_eq!(res1, res2);
        assert_eq!(res1.input_pair, FluidInputPair::PT);
        assert_relative_eq!(res1.value1, 101_325.0);
        assert_relative_eq!(res1.value2, 293.15);
    }

    #[test]
    fn try_from_same_inputs() {
        // Given
        let input = FluidInput::pressure(101_325.0);

        // When
        let res = FluidUpdateRequest::try_from((input, input));

        // Then
        assert_eq!(res, Err(FluidStateError::InvalidInputPair(input.key, input.key)));
    }

    #[rstest]
    fn try_from_non_finite_inputs(
        #[values(f64::NAN, f64::INFINITY, -f64::INFINITY)] temperature: f64,
        #[values(f64::NAN, f64::INFINITY, -f64::INFINITY, 101_325.0)] pressure: f64,
    ) {
        // Given
        let input1 = FluidInput::temperature(temperature);
        let input2 = FluidInput::pressure(pressure);

        // When
        let res = FluidUpdateRequest::try_from((input1, input2));

        // Then
        assert_eq!(res, Err(FluidStateError::InvalidInputValue));
    }
}
