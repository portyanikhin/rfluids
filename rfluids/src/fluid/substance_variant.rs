use crate::error::FluidUpdateError;
use crate::fluid::BackendName;
use crate::io::{FluidInput, FluidInputPair, FluidUpdateRequest};
use crate::substance::{CustomMix, Substance};
use std::fmt::Debug;

/// CoolProp substance variant.
pub trait SubstanceVariant: BackendName + Debug + Clone {
    /// Try to parse two [`FluidInput`]s into [`FluidUpdateRequest`].
    ///
    /// **NB.** It's specific to the substance variant. For example,
    /// [`Substance`] (and its subsets) supports all [`FluidInputPair`]s,
    /// but [`CustomMix`] -- only [pressure/quality](FluidInputPair::PQ),
    /// [temperature/quality](FluidInputPair::QT) and
    /// [pressure/temperature](FluidInputPair::PT) inputs.
    ///
    /// # Args
    ///
    /// - `input1` -- first input property.
    /// - `input2` -- second input property.
    ///
    /// # Errors
    ///
    /// For invalid or unsupported inputs, a [`FluidUpdateError`] is returned.
    fn try_parse(
        input1: FluidInput,
        input2: FluidInput,
    ) -> Result<FluidUpdateRequest, FluidUpdateError>;
}

impl SubstanceVariant for Substance {
    fn try_parse(
        input1: FluidInput,
        input2: FluidInput,
    ) -> Result<FluidUpdateRequest, FluidUpdateError> {
        (input1, input2).try_into()
    }
}

impl SubstanceVariant for CustomMix {
    fn try_parse(
        input1: FluidInput,
        input2: FluidInput,
    ) -> Result<FluidUpdateRequest, FluidUpdateError> {
        let request: FluidUpdateRequest = (input1, input2).try_into()?;
        if !matches!(
            request.0,
            // Other input pairs are not currently supported for custom mixtures in CoolProp.
            FluidInputPair::PQ | FluidInputPair::QT | FluidInputPair::PT
        ) {
            return Err(FluidUpdateError::InvalidInputPair(
                input1.key(),
                input2.key(),
            ));
        }
        Ok(request)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod substance {
        use super::*;
        use approx::assert_relative_eq;
        use uom::si::f64::{Pressure, ThermodynamicTemperature};
        use uom::si::pressure::atmosphere;
        use uom::si::thermodynamic_temperature::degree_celsius;

        #[test]
        fn try_parse_two_valid_inputs_returns_ok() {
            let input1 =
                FluidInput::temperature(ThermodynamicTemperature::new::<degree_celsius>(30.0));
            let input2 = FluidInput::pressure(Pressure::new::<atmosphere>(2.0));
            let result = Substance::try_parse(input1, input2).unwrap();
            assert_eq!(result.0, FluidInputPair::PT);
            assert_relative_eq!(result.1, 202650.0);
            assert_relative_eq!(result.2, 303.15);
        }

        #[test]
        fn try_parse_two_same_inputs_returns_err() {
            let input =
                FluidInput::temperature(ThermodynamicTemperature::new::<degree_celsius>(30.0));
            assert_eq!(
                Substance::try_parse(input, input).unwrap_err(),
                FluidUpdateError::InvalidInputPair(input.key(), input.key())
            );
        }
    }

    mod custom_mix {
        use super::*;
        use approx::assert_relative_eq;
        use uom::si::available_energy::kilojoule_per_kilogram;
        use uom::si::f64::{AvailableEnergy, Pressure, ThermodynamicTemperature};
        use uom::si::pressure::kilopascal;
        use uom::si::thermodynamic_temperature::degree_celsius;

        #[test]
        fn try_parse_two_valid_inputs_returns_ok() {
            let input1 =
                FluidInput::temperature(ThermodynamicTemperature::new::<degree_celsius>(0.0));
            let input2 = FluidInput::pressure(Pressure::new::<kilopascal>(100.0));
            let result = CustomMix::try_parse(input1, input2).unwrap();
            assert_eq!(result.0, FluidInputPair::PT);
            assert_relative_eq!(result.1, 100e3);
            assert_relative_eq!(result.2, 273.15);
        }

        #[test]
        fn try_parse_two_same_inputs_returns_err() {
            let input = FluidInput::pressure(Pressure::new::<kilopascal>(100.0));
            assert_eq!(
                CustomMix::try_parse(input, input).unwrap_err(),
                FluidUpdateError::InvalidInputPair(input.key(), input.key())
            );
        }

        #[test]
        fn try_parse_unsupported_input_pair_returns_err() {
            let input1 = FluidInput::pressure(Pressure::new::<kilopascal>(100.0));
            let input2 =
                FluidInput::enthalpy(AvailableEnergy::new::<kilojoule_per_kilogram>(200.0));
            assert_eq!(
                CustomMix::try_parse(input1, input2).unwrap_err(),
                FluidUpdateError::InvalidInputPair(input1.key(), input2.key())
            );
        }
    }
}
