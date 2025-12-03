use std::{collections::HashMap, marker::PhantomData};

use super::{Fluid, FluidBuildError, StateResult, request::FluidCreateRequest};
use crate::{
    io::FluidInput, native::AbstractState, state_variant::Undefined, substance::Substance,
};

#[bon::bon]
impl Fluid<Undefined> {
    /// Builds a new [`Fluid`] instance
    /// with [`Undefined`] state variant from any [`Substance`].
    ///
    /// This method provides advanced control over backend selection.
    /// For most use cases, prefer using [`From`]/[`TryFrom`] trait
    /// implementations instead.
    ///
    /// # Arguments
    ///
    /// - `substance` -- substance for which to calculate properties
    /// - `with_backend` -- `CoolProp` backend to be used (e.g.,
    ///   `"HEOS"`, `"INCOMP"`, `"REFPROP"`, `"IF97"`, etc.). If
    ///   provided, overrides the default one defined for the
    ///   substance
    ///
    /// # Errors
    ///
    /// Returns a [`FluidBuildError`] for invalid backend names
    /// or unsupported custom mixtures.
    ///
    /// # Examples
    ///
    /// ```
    /// use rfluids::prelude::*;
    ///
    /// // Using default backend (HEOS for pure fluids)
    /// let water = Fluid::builder().substance(Pure::Water).build()?;
    /// assert_eq!(water.backend_name(), "HEOS");
    ///
    /// // Overriding backend (using IF97 instead of default HEOS)
    /// let if97_water = Fluid::builder()
    ///     .substance(Pure::Water)
    ///     .with_backend("IF97")
    ///     .build()?;
    /// assert_eq!(if97_water.backend_name(), "IF97");
    ///
    /// // The two fluids are not equal since they use different backends
    /// assert_ne!(water, if97_water);
    /// # Ok::<(), rfluids::Error>(())
    /// ```
    ///
    /// # See Also
    ///
    /// If you don't need to override the backend name, consider
    /// using:
    /// - [`Fluid::from`] -- simpler way to create [`Fluid`] from
    ///   [`Pure`](crate::substance::Pure),
    ///   [`IncompPure`](crate::substance::IncompPure),
    ///   [`PredefinedMix`](crate::substance::PredefinedMix), or
    ///   [`BinaryMix`](crate::substance::BinaryMix)
    /// - [`Fluid::try_from`] -- for creating [`Fluid`] from any
    ///   [`Substance`] (including
    ///   [`CustomMix`](crate::substance::CustomMix))
    #[builder]
    pub fn new(
        /// Substance for which to calculate properties.
        #[builder(into)]
        substance: Substance,
        /// `CoolProp` backend to be used
        /// (e.g., `"HEOS"`, `"INCOMP"`, `"REFPROP"`, `"IF97"`, etc.).
        /// If provided, overrides the default one defined for the
        /// substance.
        #[builder(with = |backend_name: impl AsRef<str>| backend_name.as_ref().trim().to_string())]
        with_backend: Option<String>,
    ) -> Result<Self, FluidBuildError> {
        let request = FluidCreateRequest::new(&substance, with_backend.clone());
        let mut backend = AbstractState::new(&request.backend_name, request.substance_name)
            .map_err(|err| {
                if err.to_string().to_lowercase().contains("backend") {
                    FluidBuildError::InvalidBackend(err)
                } else {
                    FluidBuildError::UnsupportedCustomMix(err)
                }
            })?;
        if let Some(fractions) = request.fractions {
            backend.set_fractions(&fractions).unwrap();
        }
        Ok(Self {
            substance,
            backend,
            custom_backend_name: with_backend,
            update_request: None,
            outputs: HashMap::new(),
            trivial_outputs: HashMap::new(),
            state: PhantomData,
        })
    }

    /// Updates the thermodynamic state and returns itself with
    /// [`Defined`](crate::state_variant::Defined) state variant.
    ///
    /// # Arguments
    ///
    /// - `input1` -- first input property
    /// - `input2` -- second input property
    ///
    /// # Errors
    ///
    /// Returns a [`FluidStateError`](crate::fluid::FluidStateError)
    /// for invalid/unsupported inputs or invalid state.
    ///
    /// # Examples
    ///
    /// ```
    /// use rfluids::{fluid::StateResult, prelude::*};
    ///
    /// // After creation, the `Fluid` instance has `Undefined` state variant
    /// let mut water: Fluid<Undefined> = Fluid::from(Pure::Water);
    ///
    /// // Calling `in_state` will move the initial value and
    /// // perform conversion between `Undefined` and `Defined` state variants
    /// // (since `Defined` is the default state variant, it can be omitted)
    /// let mut water: Fluid = water.in_state(
    ///     FluidInput::pressure(101_325.0),
    ///     FluidInput::temperature(293.15),
    /// )?;
    ///
    /// // The `Fluid` instance now has `Defined` state variant
    /// // and its thermodynamic state can be updated in place by calling `update`
    /// // (which returns a mutable reference to the instance)
    /// let same_water_in_new_state: StateResult<&mut Fluid> = water.update(
    ///     FluidInput::pressure(202_650.0),
    ///     FluidInput::temperature(313.15),
    /// );
    /// assert!(same_water_in_new_state.is_ok());
    ///
    /// // Calling `in_state` on `Fluid<Defined>` will return
    /// // a new instance in the specified thermodynamic state
    /// let new_water: StateResult<Fluid> = water.in_state(
    ///     FluidInput::pressure(405_300.0),
    ///     FluidInput::temperature(353.15),
    /// );
    /// assert!(new_water.is_ok());
    /// # Ok::<(), rfluids::Error>(())
    /// ```
    ///
    /// # See Also
    ///
    /// - [`Fluid::update`](crate::fluid::Fluid::update)
    /// - [`Fluid::in_state`](crate::fluid::Fluid::in_state)
    pub fn in_state(mut self, input1: FluidInput, input2: FluidInput) -> StateResult<Fluid> {
        self.inner_update(input1, input2)?;
        Ok(Fluid {
            substance: self.substance,
            backend: self.backend,
            custom_backend_name: self.custom_backend_name,
            update_request: self.update_request,
            outputs: self.outputs,
            trivial_outputs: self.trivial_outputs,
            state: PhantomData,
        })
    }
}

impl Clone for Fluid<Undefined> {
    fn clone(&self) -> Self {
        let mut fluid = Fluid::builder()
            .substance(self.substance.clone())
            .maybe_with_backend(self.custom_backend_name.clone())
            .build()
            .unwrap();
        fluid.trivial_outputs.clone_from(&self.trivial_outputs);
        fluid
    }
}

impl PartialEq for Fluid<Undefined> {
    fn eq(&self, other: &Self) -> bool {
        self.substance == other.substance && self.backend_name() == other.backend_name()
    }
}

#[cfg(test)]
mod tests {
    use rstest::*;

    use super::*;
    use crate::{
        fluid::FluidStateError,
        substance::*,
        test::{SutFactory, test_output},
    };

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
    fn builder(ctx: Context) {
        // Given
        let Context { water, .. } = ctx;

        // When
        let default = Fluid::builder().substance(water).build().unwrap();
        let custom = Fluid::builder()
            .substance(water)
            .with_backend("IF97")
            .build()
            .unwrap();

        // Then
        assert_eq!(default.substance(), &Substance::from(water));
        assert_eq!(default.backend_name(), "HEOS");
        assert_eq!(custom.substance(), &Substance::from(water));
        assert_eq!(custom.backend_name(), "IF97");
        assert_ne!(default, custom);
    }

    #[rstest]
    fn builder_invalid_backend(ctx: Context) {
        // Given
        let Context { water, .. } = ctx;

        // When
        let res = Fluid::builder()
            .substance(water)
            .with_backend("Hello, World!")
            .build();

        // Then
        assert!(matches!(res, Err(FluidBuildError::InvalidBackend(_))));
    }

    #[rstest]
    fn builder_unsupported_custom_mix() {
        // Given
        let unsupported_mix =
            CustomMix::mass_based([(Pure::Orthohydrogen, 0.6), (Pure::R32, 0.4)]).unwrap();

        // When
        let res = Fluid::builder()
            .substance(unsupported_mix)
            .with_backend("HEOS")
            .build();

        // Then
        assert!(matches!(res, Err(FluidBuildError::UnsupportedCustomMix(_))));
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
