use crate::{
    error::FluidStateError,
    io::{FluidInput, FluidInputPair, FluidParam},
    substance::{BackendName, Substance},
};
use std::borrow::Cow;

pub(crate) struct FluidCreateRequest<'a> {
    pub(crate) name: Cow<'a, str>,
    pub(crate) backend_name: &'a str,
    pub(crate) fractions: Option<Vec<f64>>,
}

impl<'a> From<&'a Substance> for FluidCreateRequest<'a> {
    fn from(value: &'a Substance) -> Self {
        match value {
            Substance::Pure(pure) => FluidCreateRequest {
                name: Cow::Borrowed(pure.as_ref()),
                backend_name: pure.backend_name(),
                fractions: None,
            },
            Substance::IncompPure(incomp_pure) => FluidCreateRequest {
                name: Cow::Borrowed(incomp_pure.as_ref()),
                backend_name: incomp_pure.backend_name(),
                fractions: None,
            },
            Substance::PredefinedMix(predefined_mix) => FluidCreateRequest {
                name: Cow::Borrowed(predefined_mix.as_ref()),
                backend_name: predefined_mix.backend_name(),
                fractions: None,
            },
            Substance::BinaryMix(binary_mix) => FluidCreateRequest {
                name: Cow::Borrowed(binary_mix.kind.as_ref()),
                backend_name: binary_mix.kind.backend_name(),
                fractions: Some(vec![binary_mix.fraction]),
            },
            Substance::CustomMix(custom_mix) => {
                let mix = custom_mix.clone().into_mole_based();
                let (components, fractions): (Vec<&str>, Vec<f64>) = mix
                    .components()
                    .iter()
                    .map(|component| (component.0.as_ref(), component.1))
                    .unzip();
                FluidCreateRequest {
                    name: Cow::Owned(components.join("&")),
                    backend_name: mix.backend_name(),
                    fractions: Some(fractions),
                }
            }
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub(crate) struct FluidUpdateRequest {
    pub(crate) input_pair: FluidInputPair,
    pub(crate) value1: f64,
    pub(crate) value2: f64,
}

impl From<FluidUpdateRequest> for (FluidInput, FluidInput) {
    fn from(request: FluidUpdateRequest) -> Self {
        let keys: (FluidParam, FluidParam) = request.input_pair.into();
        (
            FluidInput {
                key: keys.0,
                value: request.value1,
            },
            FluidInput {
                key: keys.1,
                value: request.value2,
            },
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
        Ok(Self {
            input_pair,
            value1,
            value2,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod create_request {
        use super::*;
        use crate::substance::*;
        use std::collections::HashMap;

        #[test]
        fn from_pure_substance_returns_expected_value() {
            let r32 = Pure::R32;
            let substance = Substance::from(r32);
            let request = FluidCreateRequest::from(&substance);
            assert_eq!(request.name, Cow::Borrowed(r32.as_ref()));
            assert_eq!(request.backend_name, r32.backend_name());
            assert!(request.fractions.is_none());
        }

        #[test]
        fn from_incomp_pure_substance_returns_expected_value() {
            let water = IncompPure::Water;
            let substance = Substance::from(water);
            let request = FluidCreateRequest::from(&substance);
            assert_eq!(request.name, Cow::Borrowed(water.as_ref()));
            assert_eq!(request.backend_name, water.backend_name());
            assert!(request.fractions.is_none());
        }

        #[test]
        fn from_predefined_mix_substance_returns_expected_value() {
            let r444a = PredefinedMix::R444A;
            let substance = Substance::from(r444a);
            let request = FluidCreateRequest::from(&substance);
            assert_eq!(request.name, Cow::Borrowed(r444a.as_ref()));
            assert_eq!(request.backend_name, r444a.backend_name());
            assert!(request.fractions.is_none());
        }

        #[test]
        fn from_binary_mix_substance_returns_expected_value() {
            let propylene_glycol = BinaryMixKind::MPG.with_fraction(0.4).unwrap();
            let substance = Substance::from(propylene_glycol);
            let request = FluidCreateRequest::from(&substance);
            assert_eq!(request.name, Cow::Borrowed(BinaryMixKind::MPG.as_ref()));
            assert_eq!(request.backend_name, BinaryMixKind::MPG.backend_name());
            assert_eq!(request.fractions, Some(vec![0.4]));
        }

        #[test]
        fn from_custom_mix_substance_returns_expected_value() {
            let mix =
                CustomMix::mole_based(HashMap::from([(Pure::Water, 0.8), (Pure::Ethanol, 0.2)]))
                    .unwrap();
            let substance = Substance::from(mix.clone());
            let request = FluidCreateRequest::from(&substance);
            assert!(
                [(Pure::Water, Pure::Ethanol), (Pure::Ethanol, Pure::Water)]
                    .map(|x| Cow::Owned(format!("{}&{}", x.0.as_ref(), x.1.as_ref())))
                    .contains(&request.name)
            );
            assert_eq!(request.backend_name, mix.backend_name());
            assert!([Some(vec![0.8, 0.2]), Some(vec![0.2, 0.8])].contains(&request.fractions));
        }
    }

    mod update_request {
        use super::*;
        use crate::test::assert_relative_eq;
        use rstest::*;

        #[test]
        fn two_fluid_inputs_from_fluid_update_request_returns_expected_value() {
            let request = FluidUpdateRequest {
                input_pair: FluidInputPair::PT,
                value1: 101_325.0,
                value2: 293.15,
            };
            let result = <(FluidInput, FluidInput)>::from(request);
            assert_eq!(result.0.key, FluidParam::P);
            assert_eq!(result.0.value, request.value1);
            assert_eq!(result.1.key, FluidParam::T);
            assert_eq!(result.1.value, request.value2);
        }

        #[test]
        fn try_from_two_valid_inputs_with_invariant_order_returns_ok() {
            let input1 = FluidInput::temperature(293.15);
            let input2 = FluidInput::pressure(101_325.0);
            let result1 = FluidUpdateRequest::try_from((input1, input2)).unwrap();
            let result2 = FluidUpdateRequest::try_from((input2, input1)).unwrap();
            assert_eq!(result1, result2);
            assert_eq!(result1.input_pair, FluidInputPair::PT);
            assert_relative_eq!(result1.value1, 101_325.0);
            assert_relative_eq!(result1.value2, 293.15);
        }

        #[test]
        fn try_from_two_same_inputs_returns_err() {
            let input = FluidInput::pressure(101_325.0);
            assert_eq!(
                FluidUpdateRequest::try_from((input, input)).unwrap_err(),
                FluidStateError::InvalidInputPair(input.key, input.key)
            );
        }

        #[rstest]
        fn try_from_non_finite_inputs_returns_err(
            #[values(f64::NAN, f64::INFINITY, -f64::INFINITY)] value1: f64,
            #[values(f64::NAN, f64::INFINITY, -f64::INFINITY, 101_325.0)] value2: f64,
        ) {
            let input1 = FluidInput::temperature(value1);
            let input2 = FluidInput::pressure(value2);
            assert_eq!(
                FluidUpdateRequest::try_from((input1, input2)).unwrap_err(),
                FluidStateError::InvalidInputValue
            );
        }
    }
}
