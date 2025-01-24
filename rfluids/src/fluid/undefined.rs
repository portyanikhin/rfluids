use crate::error::FluidUpdateError;
use crate::fluid::{Fluid, SubstanceVariant};
use crate::io::FluidInput;
use crate::state_variant::{Defined, Undefined};
use std::collections::HashMap;
use std::marker::PhantomData;

impl<T: SubstanceVariant> Fluid<T, Undefined> {
    /// Updates the state and returns itself with [`Defined`] state variant.
    ///
    /// # Args
    ///
    /// - input1 -- first input property.
    /// - input2 -- second input property.
    ///
    /// # Errors
    ///
    /// For invalid/unsupported inputs or invalid state,
    /// a [`FluidUpdateError`] is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use rfluids::error::FluidUpdateError;
    /// use rfluids::prelude::fluid::*;
    /// use rfluids::uom::si::pressure::atmosphere;
    /// use rfluids::uom::si::thermodynamic_temperature::degree_celsius;
    ///
    /// // After creation Fluid has Undefined state variant
    /// let mut water: Fluid<_, Undefined> = Fluid::from(Pure::Water);
    ///
    /// // First update will move value above and
    /// // perform conversion between Undefined and Defined state variants
    /// let mut water: Fluid<_, Defined> = water.update(
    ///     FluidInput::pressure(Pressure::new::<atmosphere>(1.0)),
    ///     FluidInput::temperature(ThermodynamicTemperature::new::<degree_celsius>(20.0)),
    /// ).unwrap();
    ///
    /// // Secondary updates will work in place and
    /// // return mutable reference to the Fluid
    /// let result: Result<&mut Fluid<_, Defined>, FluidUpdateError> = water.update(
    ///     FluidInput::pressure(Pressure::new::<atmosphere>(2.0)),
    ///     FluidInput::temperature(ThermodynamicTemperature::new::<degree_celsius>(40.0)),
    /// );
    /// assert!(result.is_ok());
    /// ```
    pub fn update(
        mut self,
        input1: FluidInput,
        input2: FluidInput,
    ) -> Result<Fluid<T, Defined>, FluidUpdateError> {
        self.inner_update(input1, input2)?;
        Ok(Fluid {
            substance: self.substance,
            backend: self.backend,
            update_request: self.update_request,
            trivial_outputs: self.trivial_outputs,
            outputs: HashMap::new(),
            state: PhantomData,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::uom::si::available_energy::kilojoule_per_kilogram;
    use crate::uom::si::f64::{AvailableEnergy, Pressure, ThermodynamicTemperature};
    use crate::uom::si::pressure::atmosphere;
    use crate::uom::si::thermodynamic_temperature::degree_celsius;
    use rstest::*;

    #[fixture]
    fn enthalpy(#[default(100.0)] value: f64) -> FluidInput {
        FluidInput::enthalpy(AvailableEnergy::new::<kilojoule_per_kilogram>(value))
    }

    #[fixture]
    fn temperature(#[default(20.0)] value: f64) -> FluidInput {
        FluidInput::temperature(ThermodynamicTemperature::new::<degree_celsius>(value))
    }

    #[fixture]
    fn pressure(#[default(1.0)] value: f64) -> FluidInput {
        FluidInput::pressure(Pressure::new::<atmosphere>(value))
    }

    #[fixture]
    fn infinite_pressure(#[with(f64::INFINITY)] pressure: FluidInput) -> FluidInput {
        pressure
    }

    #[fixture]
    fn negative_pressure(#[with(-1.0)] pressure: FluidInput) -> FluidInput {
        pressure
    }

    mod substance {
        use super::*;
        use crate::error::FluidUpdateError;
        use crate::substance::{Pure, Substance};

        #[fixture]
        fn sut() -> Fluid<Substance, Undefined> {
            Fluid::from(Pure::Water)
        }

        #[rstest]
        fn update_valid_inputs_returns_ok(
            sut: Fluid<Substance, Undefined>,
            temperature: FluidInput,
            pressure: FluidInput,
        ) {
            assert!(sut.update(temperature, pressure).is_ok());
        }

        #[rstest]
        fn update_same_inputs_returns_err(sut: Fluid<Substance, Undefined>, pressure: FluidInput) {
            assert_eq!(
                sut.update(pressure, pressure).unwrap_err(),
                FluidUpdateError::InvalidInputPair(pressure.key(), pressure.key())
            );
        }

        #[rstest]
        fn update_invalid_inputs_returns_err(
            sut: Fluid<Substance, Undefined>,
            temperature: FluidInput,
            infinite_pressure: FluidInput,
        ) {
            assert_eq!(
                sut.update(temperature, infinite_pressure).unwrap_err(),
                FluidUpdateError::InvalidInputValue
            );
        }

        #[rstest]
        fn update_invalid_state_returns_err(
            sut: Fluid<Substance, Undefined>,
            temperature: FluidInput,
            negative_pressure: FluidInput,
        ) {
            assert!(matches!(
                sut.update(temperature, negative_pressure).unwrap_err(),
                FluidUpdateError::InvalidState(_)
            ));
        }
    }

    mod custom_mix {
        use super::*;
        use crate::error::FluidUpdateError;
        use crate::substance::{CustomMix, Pure};
        use uom::si::f64::Ratio;
        use uom::si::ratio::percent;

        #[fixture]
        fn sut() -> Fluid<CustomMix, Undefined> {
            Fluid::try_from(
                CustomMix::mass_based(HashMap::from([
                    (Pure::Water.into(), Ratio::new::<percent>(70.0)),
                    (Pure::Ethanol.into(), Ratio::new::<percent>(30.0)),
                ]))
                .unwrap(),
            )
            .unwrap()
        }

        #[rstest]
        fn update_valid_inputs_returns_ok(
            sut: Fluid<CustomMix, Undefined>,
            temperature: FluidInput,
            pressure: FluidInput,
        ) {
            assert!(sut.update(temperature, pressure).is_ok());
        }

        #[rstest]
        fn update_same_inputs_returns_err(sut: Fluid<CustomMix, Undefined>, pressure: FluidInput) {
            assert_eq!(
                sut.update(pressure, pressure).unwrap_err(),
                FluidUpdateError::InvalidInputPair(pressure.key(), pressure.key())
            );
        }

        #[rstest]
        fn update_unsupported_inputs_returns_err(
            sut: Fluid<CustomMix, Undefined>,
            pressure: FluidInput,
            enthalpy: FluidInput,
        ) {
            assert_eq!(
                sut.update(pressure, enthalpy).unwrap_err(),
                FluidUpdateError::InvalidInputPair(pressure.key(), enthalpy.key())
            );
        }

        #[rstest]
        fn update_invalid_inputs_returns_err(
            sut: Fluid<CustomMix, Undefined>,
            temperature: FluidInput,
            infinite_pressure: FluidInput,
        ) {
            assert_eq!(
                sut.update(temperature, infinite_pressure).unwrap_err(),
                FluidUpdateError::InvalidInputValue
            );
        }

        #[rstest]
        fn update_invalid_state_returns_err(
            sut: Fluid<CustomMix, Undefined>,
            temperature: FluidInput,
            negative_pressure: FluidInput,
        ) {
            assert!(matches!(
                sut.update(temperature, negative_pressure).unwrap_err(),
                FluidUpdateError::InvalidState(_)
            ));
        }
    }
}
