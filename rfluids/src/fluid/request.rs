use std::borrow::Cow;

use super::{
    FluidStateError,
    backend::{Backend, DefaultBackend},
};
use crate::{
    io::{FluidInput, FluidInputPair, FluidParam},
    substance::Substance,
};

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct FluidCreateRequest {
    pub(crate) substance_name: Cow<'static, str>,
    pub(crate) backend_name: Cow<'static, str>,
    pub(crate) fractions: Option<Vec<f64>>,
}

impl FluidCreateRequest {
    #[must_use]
    pub(crate) fn new(substance: &Substance, backend: Option<Backend>) -> Self {
        let (substance_name, fractions): (Cow<'static, str>, Option<Vec<f64>>) = match substance {
            Substance::Pure(pure) => (Cow::Borrowed(pure.into()), None),
            Substance::IncompPure(incomp_pure) => (Cow::Borrowed(incomp_pure.into()), None),
            Substance::PredefinedMix(predefined_mix) => {
                (Cow::Borrowed(predefined_mix.into()), None)
            }
            Substance::BinaryMix(binary_mix) => {
                (Cow::Borrowed(binary_mix.kind.into()), Some(vec![binary_mix.fraction]))
            }
            Substance::CustomMix(custom_mix) => {
                let mix = custom_mix.clone().into_mole_based();
                let (components, fractions): (Vec<&str>, Vec<f64>) = mix
                    .components()
                    .iter()
                    .map(|component| (component.0.as_ref(), component.1))
                    .unzip();
                (Cow::Owned(components.join("&")), Some(fractions))
            }
        };
        let backend_name = backend.unwrap_or_else(|| substance.default_backend()).name();
        Self { substance_name, backend_name, fractions }
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
    use super::*;

    mod create_request {
        use rstest::*;

        use super::*;
        use crate::{fluid::backend::*, substance::*};

        #[rstest]
        #[case(
            Pure::R32,
            None,
            FluidCreateRequest {
                substance_name: Cow::Borrowed("R32"),
                backend_name: Cow::Borrowed("HEOS"),
                fractions: None,
            }
        )]
        #[case(
            Pure::Water,
            Some(BaseBackend::If97.into()),
            FluidCreateRequest {
                substance_name: Cow::Borrowed("Water"),
                backend_name: Cow::Borrowed("IF97"),
                fractions: None,
            }
        )]
        #[case(
            IncompPure::Water,
            None,
            FluidCreateRequest {
                substance_name: Cow::Borrowed("Water"),
                backend_name: Cow::Borrowed("INCOMP"),
                fractions: None,
            }
        )]
        #[case(
            IncompPure::Water,
            Some(BaseBackend::Heos.with(TabularMethod::Ttse)),
            FluidCreateRequest {
                substance_name: Cow::Borrowed("Water"),
                backend_name: Cow::Owned("TTSE&HEOS".to_string()),
                fractions: None,
            }
        )]
        #[case(
            PredefinedMix::R444A,
            None,
            FluidCreateRequest {
                substance_name: Cow::Borrowed("R444A.mix"),
                backend_name: Cow::Borrowed("HEOS"),
                fractions: None,
            }
        )]
        #[case(
            PredefinedMix::R444A,
            Some(BaseBackend::Pr.into()),
            FluidCreateRequest {
                substance_name: Cow::Borrowed("R444A.mix"),
                backend_name: Cow::Borrowed("PR"),
                fractions: None,
            }
        )]
        #[case(
            BinaryMixKind::MPG.with_fraction(0.4).unwrap(),
            None,
            FluidCreateRequest {
                substance_name: Cow::Borrowed("MPG"),
                backend_name: Cow::Borrowed("INCOMP"),
                fractions: Some(vec![0.4]),
            }
        )]
        #[case(
            BinaryMixKind::MPG.with_fraction(0.4).unwrap(),
            Some(BaseBackend::Heos.with(TabularMethod::Bicubic)),
            FluidCreateRequest {
                substance_name: Cow::Borrowed("MPG"),
                backend_name: Cow::Owned("BICUBIC&HEOS".to_string()),
                fractions: Some(vec![0.4]),
            }
        )]
        fn new(
            #[case] substance: impl Into<Substance>,
            #[case] backend: Option<Backend>,
            #[case] expected: FluidCreateRequest,
        ) {
            // Given
            let substance: Substance = substance.into();

            // When
            let sut = FluidCreateRequest::new(&substance, backend);

            // Then
            assert_eq!(sut, expected);
        }

        #[rstest]
        #[case(None, Cow::Borrowed("HEOS"))]
        #[case(
            Some(BaseBackend::Refprop.with(TabularMethod::Ttse)),
            Cow::Owned("TTSE&REFPROP".to_string())
        )]
        fn new_custom_mix(#[case] backend: Option<Backend>, #[case] expected: Cow<str>) {
            // Given
            let mix = CustomMix::mole_based([(Pure::Water, 0.8), (Pure::Ethanol, 0.2)]).unwrap();
            let substance = Substance::from(mix.clone());

            // When
            let sut = FluidCreateRequest::new(&substance, backend);

            // Then
            assert!(["Water&Ethanol", "Ethanol&Water"].contains(&sut.substance_name.as_ref()));
            assert_eq!(sut.backend_name, expected);
            assert!([Some(vec![0.8, 0.2]), Some(vec![0.2, 0.8])].contains(&sut.fractions));
        }
    }

    mod update_request {
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
}
