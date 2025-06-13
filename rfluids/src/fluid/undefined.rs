use super::{Fluid, StateResult};
use crate::{io::FluidInput, state_variant::Undefined};
use std::marker::PhantomData;

impl Fluid<Undefined> {
    /// Updates the thermodynamic state and returns itself with
    /// [`Defined`](crate::state_variant::Defined) state variant.
    ///
    /// # Args
    ///
    /// - `input1` -- first input property.
    /// - `input2` -- second input property.
    ///
    /// # Errors
    ///
    /// For invalid/unsupported inputs or invalid state,
    /// a [`FluidStateError`](crate::fluid::FluidStateError) is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use rfluids::fluid::StateResult;
    /// use rfluids::prelude::*;
    ///
    /// // After creation the `Fluid` instance has `Undefined` state variant
    /// let mut water: Fluid<Undefined> = Fluid::from(Pure::Water);
    ///
    /// // Calling `in_state` method will move the initial value and
    /// // perform conversion between `Undefined` and `Defined` state variants
    /// // (since `Defined` is the default state variant, it can be omitted)
    /// let mut water: Fluid = water.in_state(
    ///     FluidInput::pressure(101_325.0),
    ///     FluidInput::temperature(293.15),
    /// )?;
    ///
    /// // The `Fluid` instance now has `Defined` state variant
    /// // and it's thermodynamic state can be updated in place by calling `update` method
    /// // (which returns a mutable reference to the instance)
    /// let same_water_in_new_state: StateResult<&mut Fluid> = water.update(
    ///     FluidInput::pressure(202_650.0),
    ///     FluidInput::temperature(313.15),
    /// );
    /// assert!(same_water_in_new_state.is_ok());
    ///
    /// // Calling `in_state` method on `Fluid<Defined>` will return
    /// // a new instance in the specified thermodynamic state
    /// let new_water: StateResult<Fluid> = water.in_state(
    ///     FluidInput::pressure(405_300.0),
    ///     FluidInput::temperature(353.15),
    /// );
    /// assert!(new_water.is_ok());
    /// # Ok::<(), rfluids::Error>(())
    /// ```
    ///
    /// # See also
    ///
    /// - [`Fluid::update`](crate::fluid::Fluid::update)
    /// - [`Fluid::in_state`](crate::fluid::Fluid::in_state)
    pub fn in_state(mut self, input1: FluidInput, input2: FluidInput) -> StateResult<Fluid> {
        self.inner_update(input1, input2)?;
        Ok(Fluid {
            substance: self.substance,
            backend: self.backend,
            update_request: self.update_request,
            outputs: self.outputs,
            trivial_outputs: self.trivial_outputs,
            state: PhantomData,
        })
    }
}

impl Clone for Fluid<Undefined> {
    fn clone(&self) -> Self {
        let mut fluid = Fluid::try_from(self.substance.clone()).unwrap();
        fluid.trivial_outputs.clone_from(&self.trivial_outputs);
        fluid
    }
}

impl PartialEq for Fluid<Undefined> {
    fn eq(&self, other: &Self) -> bool {
        self.substance == other.substance
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        fluid::FluidStateError,
        substance::*,
        test::{SutFactory, test_output},
    };
    use rstest::*;

    struct Context {
        pressure: FluidInput,
        temperature: FluidInput,
        water: Pure,
        incomp_water: IncompPure,
        r22: Pure,
        r32: Pure,
        r444a: PredefinedMix,
        pg: BinaryMix,
    }

    impl<P: Into<Fluid<Undefined>>> SutFactory<P> for Context {
        type Sut = Fluid<Undefined>;

        fn sut(&self, payload: P) -> Self::Sut {
            payload.into()
        }
    }

    #[fixture]
    fn ctx() -> Context {
        Context {
            pressure: FluidInput::pressure(101_325.0),
            temperature: FluidInput::temperature(293.15),
            water: Pure::Water,
            incomp_water: IncompPure::Water,
            r22: Pure::R22,
            r32: Pure::R32,
            r444a: PredefinedMix::R444A,
            pg: BinaryMixKind::MPG.with_fraction(0.4).unwrap(),
        }
    }

    test_output!(acentric_factor, water: 0.344_292_084_3, r444a: Err);
    test_output!(critical_density, water: 322.0, r444a: Err, incomp_water: Err);
    test_output!(critical_molar_density, water: 17_873.727_995_609_06, r444a: Err, incomp_water: Err);
    test_output!(critical_pressure, water: 22.064e6, r444a: Err, incomp_water: Err);
    test_output!(critical_temperature, water: 647.096, r444a: Err, incomp_water: Err);
    test_output!(flammability_hazard, water: 0.0, incomp_water: Err);
    test_output!(freezing_temperature, pg: 252.581_754_953_058_38, water: Err);
    test_output!(gwp20, r32: 2330.0, water: Err);
    test_output!(gwp100, r32: 675.0, water: Err);
    test_output!(gwp500, r32: 205.0, water: Err);
    test_output!(health_hazard, water: 0.0, incomp_water: Err);
    test_output!(max_pressure, water: 1e9, incomp_water: Err);
    test_output!(max_temperature, water: 2e3, always_ok);
    test_output!(min_pressure, water: 611.654_800_896_868_4, incomp_water: Err);
    test_output!(min_temperature, water: 273.16, always_ok);
    test_output!(molar_mass, water: 0.018_015_268, incomp_water: Err);
    test_output!(odp, r22: 0.05, water: Err, incomp_water: Err);
    test_output!(physical_hazard, water: 0.0, incomp_water: Err);
    test_output!(reducing_density, water: 322.0, incomp_water: Err);
    test_output!(reducing_molar_density, water: 17_873.727_995_609_06, incomp_water: Err);
    test_output!(reducing_pressure, water: 22.064e6, incomp_water: Err);
    test_output!(reducing_temperature, water: 647.096, incomp_water: Err);
    test_output!(triple_pressure, water: 611.654_800_896_868_4, incomp_water: Err);
    test_output!(triple_temperature, water: 273.16, incomp_water: Err);

    #[rstest]
    fn substance(ctx: Context) {
        // Given
        let Context { water, .. } = ctx;
        let substance = Substance::from(water);
        let sut = ctx.sut(water);

        // When
        let res = sut.substance();

        // Then
        assert_eq!(*res, substance);
    }

    #[rstest]
    fn in_state_same_inputs(ctx: Context) {
        // Given
        let Context {
            pressure, water, ..
        } = ctx;
        let sut = ctx.sut(water);

        // When
        let res = sut.in_state(pressure, pressure);

        // Then
        assert_eq!(
            res,
            Err(FluidStateError::InvalidInputPair(
                pressure.key,
                pressure.key
            ))
        );
    }

    #[rstest]
    fn in_state_invalid_inputs(ctx: Context) {
        // Given
        let Context {
            temperature, water, ..
        } = ctx;
        let infinite_pressure = FluidInput::pressure(f64::INFINITY);
        let sut = ctx.sut(water);

        // When
        let res = sut.in_state(infinite_pressure, temperature);

        // Then
        assert_eq!(res, Err(FluidStateError::InvalidInputValue));
    }

    #[rstest]
    fn in_state_invalid_state(ctx: Context) {
        // Given
        let Context {
            temperature, water, ..
        } = ctx;
        let negative_pressure = FluidInput::pressure(-1.0);
        let sut = ctx.sut(water);

        // When
        let res = sut.in_state(negative_pressure, temperature);

        // Then
        assert!(matches!(res, Err(FluidStateError::UpdateFailed(_))));
    }

    #[rstest]
    fn clone(ctx: Context) {
        // Given
        let Context { water, .. } = ctx;
        let sut = ctx.sut(water);

        // When
        let clone = sut.clone();

        // Then
        assert_eq!(clone, sut);
        assert_eq!(clone.trivial_outputs, sut.trivial_outputs);
    }
}
