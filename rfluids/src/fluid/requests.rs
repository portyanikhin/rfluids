use crate::error::FluidStateError;
use crate::io::{FluidInput, FluidInputPair, FluidParam, Input};
use crate::substance::{BackendName, Substance};
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
                fractions: Some(vec![binary_mix.fraction.value]),
            },
            Substance::CustomMix(custom_mix) => {
                let mix = custom_mix.to_mole_based();
                let (components, fractions): (Vec<&str>, Vec<f64>) = mix
                    .components()
                    .iter()
                    .map(|component| (component.0.as_ref(), component.1.value))
                    .unzip();
                FluidCreateRequest {
                    name: Cow::Owned(components.join("&")),
                    backend_name: custom_mix.backend_name(),
                    fractions: Some(fractions),
                }
            }
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub(crate) struct FluidUpdateRequest(pub(crate) FluidInputPair, pub(crate) f64, pub(crate) f64);

impl From<FluidUpdateRequest> for (FluidInput, FluidInput) {
    fn from(value: FluidUpdateRequest) -> Self {
        let keys: (FluidParam, FluidParam) = value.0.into();
        (Input(keys.0, value.1), Input(keys.1, value.2))
    }
}

impl TryFrom<(FluidInput, FluidInput)> for FluidUpdateRequest {
    type Error = FluidStateError;

    fn try_from(value: (FluidInput, FluidInput)) -> Result<Self, Self::Error> {
        let key = FluidInputPair::try_from((value.0.key(), value.1.key()))
            .map_err(|_| FluidStateError::InvalidInputPair(value.0.key(), value.1.key()))?;
        if !value.0.si_value().is_finite() || !value.1.si_value().is_finite() {
            return Err(FluidStateError::InvalidInputValue);
        }
        let (value1, value2) =
            if <(FluidParam, FluidParam)>::from(key) == (value.0.key(), value.1.key()) {
                (value.0.si_value(), value.1.si_value())
            } else {
                (value.1.si_value(), value.0.si_value())
            };
        Ok(Self(key, value1, value2))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod create_request {
        use super::*;
        use crate::substance::*;
        use crate::uom::si::f64::Ratio;
        use crate::uom::si::ratio::percent;
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
            let propylene_glycol =
                BinaryMix::with_fraction(BinaryMixKind::MPG, Ratio::new::<percent>(40.0)).unwrap();
            let substance = Substance::from(propylene_glycol);
            let request = FluidCreateRequest::from(&substance);
            assert_eq!(request.name, Cow::Borrowed(BinaryMixKind::MPG.as_ref()));
            assert_eq!(request.backend_name, BinaryMixKind::MPG.backend_name());
            assert_eq!(request.fractions, Some(vec![0.4]));
        }

        #[test]
        fn from_custom_mix_substance_returns_expected_value() {
            let mix = CustomMix::mole_based(HashMap::from([
                (Pure::Water, Ratio::new::<percent>(80.0)),
                (Pure::Ethanol, Ratio::new::<percent>(20.0)),
            ]))
            .unwrap();
            let substance = Substance::from(mix.clone());
            let request = FluidCreateRequest::from(&substance);
            assert!([(Pure::Water, Pure::Ethanol), (Pure::Ethanol, Pure::Water)]
                .map(|x| Cow::Owned(format!("{}&{}", x.0.as_ref(), x.1.as_ref())))
                .contains(&request.name));
            assert_eq!(request.backend_name, mix.backend_name());
            assert!([Some(vec![0.8, 0.2]), Some(vec![0.2, 0.8])].contains(&request.fractions));
        }
    }

    mod update_request {
        use super::*;
        use crate::test::assert_relative_eq;
        use crate::uom::si::f64::{Pressure, ThermodynamicTemperature};
        use crate::uom::si::pressure::atmosphere;
        use crate::uom::si::thermodynamic_temperature::degree_celsius;
        use rstest::*;

        #[test]
        fn two_fluid_inputs_from_fluid_update_request_returns_expected_value() {
            let request = FluidUpdateRequest(FluidInputPair::PT, 101_325.0, 293.15);
            let result = <(FluidInput, FluidInput)>::from(request);
            assert_eq!(result.0.key(), FluidParam::P);
            assert_eq!(result.0.si_value(), request.1);
            assert_eq!(result.1.key(), FluidParam::T);
            assert_eq!(result.1.si_value(), request.2);
        }

        #[test]
        fn try_from_two_valid_inputs_with_invariant_order_returns_ok() {
            let input1 =
                FluidInput::temperature(ThermodynamicTemperature::new::<degree_celsius>(20.0));
            let input2 = FluidInput::pressure(Pressure::new::<atmosphere>(1.0));
            let result1 = FluidUpdateRequest::try_from((input1, input2)).unwrap();
            let result2 = FluidUpdateRequest::try_from((input2, input1)).unwrap();
            assert_eq!(result1, result2);
            assert_eq!(result1.0, FluidInputPair::PT);
            assert_relative_eq!(result1.1, 101_325.0);
            assert_relative_eq!(result1.2, 293.15);
        }

        #[test]
        fn try_from_two_same_inputs_returns_err() {
            let input = FluidInput::pressure(Pressure::new::<atmosphere>(1.0));
            assert_eq!(
                FluidUpdateRequest::try_from((input, input)).unwrap_err(),
                FluidStateError::InvalidInputPair(input.key(), input.key())
            );
        }

        #[rstest]
        fn try_from_non_finite_inputs_returns_err(
            #[values(f64::NAN, f64::INFINITY, -f64::INFINITY)] value1: f64,
            #[values(f64::NAN, f64::INFINITY, -f64::INFINITY, 1.0)] value2: f64,
        ) {
            let input1 =
                FluidInput::temperature(ThermodynamicTemperature::new::<degree_celsius>(value1));
            let input2 = FluidInput::pressure(Pressure::new::<atmosphere>(value2));
            assert_eq!(
                FluidUpdateRequest::try_from((input1, input2)).unwrap_err(),
                FluidStateError::InvalidInputValue
            );
        }
    }
}
