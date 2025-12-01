use super::FluidStateError;
use crate::{
    io::{FluidInput, FluidInputPair, FluidParam},
    substance::{BackendName, Substance},
};
use std::borrow::Cow;

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct FluidCreateRequest<'a> {
    pub(crate) name: Cow<'a, str>,
    pub(crate) backend_name: &'a str,
    pub(crate) fractions: Option<Vec<f64>>,
}

impl<'a> From<&'a Substance> for FluidCreateRequest<'a> {
    fn from(value: &'a Substance) -> Self {
        match value {
            Substance::Pure(pure) => Self {
                name: Cow::Borrowed(pure.as_ref()),
                backend_name: pure.backend_name(),
                fractions: None,
            },
            Substance::IncompPure(incomp_pure) => Self {
                name: Cow::Borrowed(incomp_pure.as_ref()),
                backend_name: incomp_pure.backend_name(),
                fractions: None,
            },
            Substance::PredefinedMix(predefined_mix) => Self {
                name: Cow::Borrowed(predefined_mix.as_ref()),
                backend_name: predefined_mix.backend_name(),
                fractions: None,
            },
            Substance::BinaryMix(binary_mix) => Self {
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
                Self {
                    name: Cow::Owned(components.join("&")),
                    backend_name: mix.backend_name(),
                    fractions: Some(fractions),
                }
            }
        }
    }
}

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
        fn from_pure() {
            // Given
            let r32 = Pure::R32;
            let substance = Substance::from(r32);

            // When
            let sut = FluidCreateRequest::from(&substance);

            // Then
            assert_eq!(sut.name, Cow::Borrowed(r32.as_ref()));
            assert_eq!(sut.backend_name, r32.backend_name());
            assert!(sut.fractions.is_none());
        }

        #[test]
        fn from_incomp_pure() {
            // Given
            let water = IncompPure::Water;
            let substance = Substance::from(water);

            // When
            let sut = FluidCreateRequest::from(&substance);

            // Then
            assert_eq!(sut.name, Cow::Borrowed(water.as_ref()));
            assert_eq!(sut.backend_name, water.backend_name());
            assert!(sut.fractions.is_none());
        }

        #[test]
        fn from_predefined_mix() {
            // Given
            let r444a = PredefinedMix::R444A;
            let substance = Substance::from(r444a);

            // When
            let sut = FluidCreateRequest::from(&substance);

            // Then
            assert_eq!(sut.name, Cow::Borrowed(r444a.as_ref()));
            assert_eq!(sut.backend_name, r444a.backend_name());
            assert!(sut.fractions.is_none());
        }

        #[test]
        fn from_binary_mix() {
            // Given
            let propylene_glycol = BinaryMixKind::MPG.with_fraction(0.4).unwrap();
            let substance = Substance::from(propylene_glycol);

            // When
            let sut = FluidCreateRequest::from(&substance);

            // Then
            assert_eq!(sut.name, Cow::Borrowed(BinaryMixKind::MPG.as_ref()));
            assert_eq!(sut.backend_name, BinaryMixKind::MPG.backend_name());
            assert_eq!(sut.fractions, Some(vec![0.4]));
        }

        #[test]
        fn from_custom_mix() {
            // Given
            let mix =
                CustomMix::mole_based(HashMap::from([(Pure::Water, 0.8), (Pure::Ethanol, 0.2)]))
                    .unwrap();
            let substance = Substance::from(mix.clone());

            // When
            let sut = FluidCreateRequest::from(&substance);

            // Then
            assert!(
                [(Pure::Water, Pure::Ethanol), (Pure::Ethanol, Pure::Water)]
                    .map(|x| Cow::Owned(format!("{}&{}", x.0.as_ref(), x.1.as_ref())))
                    .contains(&sut.name)
            );
            assert_eq!(sut.backend_name, mix.backend_name());
            assert!([Some(vec![0.8, 0.2]), Some(vec![0.2, 0.8])].contains(&sut.fractions));
        }
    }

    mod update_request {
        use super::*;
        use crate::test::assert_relative_eq;
        use rstest::*;

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
            assert_eq!(
                res,
                Err(FluidStateError::InvalidInputPair(input.key, input.key))
            );
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
}
