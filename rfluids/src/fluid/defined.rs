// cSpell:disable

use super::common::cached_output;
use super::{Fluid, OutputResult, StateResult};
use crate::error::FluidOutputError;
use crate::io::{FluidInput, FluidParam};
use crate::uom::si::available_energy::joule_per_kilogram;
use crate::uom::si::f64::AvailableEnergy;

impl Fluid {
    /// Ideal gas Helmholtz energy contribution
    /// _(key: [`Alpha0`](FluidParam::Alpha0), dimensionless)_.
    ///
    /// # Errors
    ///
    /// If it's not available (calculation is failed),
    /// a [`FluidOutputError`] is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use approx::assert_relative_eq;
    /// use rfluids::prelude::fluid::*;
    /// use rfluids::uom::si::pressure::atmosphere;
    /// use rfluids::uom::si::ratio::percent;
    /// use rfluids::uom::si::thermodynamic_temperature::degree_celsius;
    ///
    /// let pressure = FluidInput::pressure(Pressure::new::<atmosphere>(1.0));
    /// let temperature =
    ///     FluidInput::temperature(ThermodynamicTemperature::new::<degree_celsius>(20.0));
    /// let mut water = Fluid::from(Pure::Water).in_state(pressure, temperature)?;
    /// assert_relative_eq!(water.alpha0()?, 9.942698150834108);
    ///
    /// let mut propylene_glycol = Fluid::from(
    ///     BinaryMix::with_fraction(BinaryMixKind::MPG, Ratio::new::<percent>(40.0))?,
    /// ).in_state(pressure, temperature)?;
    /// assert!(propylene_glycol.alpha0().is_err());
    /// # Ok::<(), rfluids::error::Error>(())
    /// ```
    pub fn alpha0(&mut self) -> OutputResult<f64> {
        self.output(FluidParam::Alpha0)
    }

    /// Residual Helmholtz energy contribution
    /// _(key: [`AlphaR`](FluidParam::AlphaR), dimensionless)_.
    ///
    /// # Errors
    ///
    /// If it's not available (calculation is failed),
    /// a [`FluidOutputError`] is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use approx::assert_relative_eq;
    /// use rfluids::prelude::fluid::*;
    /// use rfluids::uom::si::pressure::atmosphere;
    /// use rfluids::uom::si::ratio::percent;
    /// use rfluids::uom::si::thermodynamic_temperature::degree_celsius;
    ///
    /// let pressure = FluidInput::pressure(Pressure::new::<atmosphere>(1.0));
    /// let temperature =
    ///     FluidInput::temperature(ThermodynamicTemperature::new::<degree_celsius>(20.0));
    /// let mut water = Fluid::from(Pure::Water).in_state(pressure, temperature)?;
    /// assert_relative_eq!(water.alphar()?, -9.964888981266709);
    ///
    /// let mut propylene_glycol = Fluid::from(
    ///     BinaryMix::with_fraction(BinaryMixKind::MPG, Ratio::new::<percent>(40.0))?,
    /// ).in_state(pressure, temperature)?;
    /// assert!(propylene_glycol.alphar().is_err());
    /// # Ok::<(), rfluids::error::Error>(())
    /// ```
    pub fn alphar(&mut self) -> OutputResult<f64> {
        self.output(FluidParam::AlphaR)
    }

    /// Mass specific enthalpy
    /// _(key: [`HMass`](FluidParam::HMass), SI units: J/kg)_.
    ///
    /// # Errors
    ///
    /// If it's not available or calculation is failed,
    /// a [`FluidOutputError`] is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use approx::assert_relative_eq;
    /// use rfluids::prelude::fluid::*;
    /// use rfluids::uom::si::available_energy::kilojoule_per_kilogram;
    /// use rfluids::uom::si::pressure::atmosphere;
    /// use rfluids::uom::si::thermodynamic_temperature::degree_celsius;
    ///
    /// let mut water = Fluid::from(Pure::Water).in_state(
    ///     FluidInput::pressure(Pressure::new::<atmosphere>(1.0)),
    ///     FluidInput::temperature(ThermodynamicTemperature::new::<degree_celsius>(20.0)),
    /// )?;
    /// assert_relative_eq!(water.enthalpy()?.value, 84007.3008506628);
    /// assert_relative_eq!(
    ///     water.enthalpy()?.get::<kilojoule_per_kilogram>(),
    ///     84.0073008506628
    /// );
    /// # Ok::<(), rfluids::error::Error>(())
    /// ```
    pub fn enthalpy(&mut self) -> Result<AvailableEnergy, FluidOutputError> {
        self.output(FluidParam::HMass)
            .map(AvailableEnergy::new::<joule_per_kilogram>)
    }

    /// Updates the thermodynamic state and returns a mutable reference to itself.
    ///
    /// # Args
    ///
    /// - input1 -- first input property.
    /// - input2 -- second input property.
    ///
    /// # Errors
    ///
    /// For invalid/unsupported inputs or invalid state,
    /// a [`FluidStateError`](crate::error::FluidStateError) is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use rfluids::error::FluidStateError;
    /// use rfluids::prelude::fluid::*;
    /// use rfluids::uom::si::pressure::atmosphere;
    /// use rfluids::uom::si::thermodynamic_temperature::degree_celsius;
    ///
    /// // After creation the `Fluid` instance has `Undefined` state variant
    /// let mut water: Fluid<Undefined> = Fluid::from(Pure::Water);
    ///
    /// // Calling `in_state` method will move the initial value and
    /// // perform conversion between `Undefined` and `Defined` state variants
    /// // (since `Defined` is the default state variant, it can be omitted)
    /// let mut water: Fluid = water.in_state(
    ///     FluidInput::pressure(Pressure::new::<atmosphere>(1.0)),
    ///     FluidInput::temperature(ThermodynamicTemperature::new::<degree_celsius>(20.0)),
    /// )?;
    ///
    /// // The `Fluid` instance now has `Defined` state variant
    /// // and it's thermodynamic state can be updated in place by calling `update` method
    /// // (which returns a mutable reference to the instance)
    /// let same_water_in_new_state: StateResult<&mut Fluid> = water.update(
    ///     FluidInput::pressure(Pressure::new::<atmosphere>(2.0)),
    ///     FluidInput::temperature(ThermodynamicTemperature::new::<degree_celsius>(40.0)),
    /// );
    /// assert!(same_water_in_new_state.is_ok());
    ///
    /// // Calling `in_state` method on `Fluid<Defined>` will return
    /// // a new instance in the specified thermodynamic state
    /// let new_water: StateResult<Fluid> = water.in_state(
    ///     FluidInput::pressure(Pressure::new::<atmosphere>(4.0)),
    ///     FluidInput::temperature(ThermodynamicTemperature::new::<degree_celsius>(80.0)),
    /// );
    /// assert!(new_water.is_ok());
    /// # Ok::<(), rfluids::error::Error>(())
    /// ```
    ///
    /// # See also
    ///
    /// - [`Fluid::in_state`](crate::fluid::Fluid::in_state)
    pub fn update(&mut self, input1: FluidInput, input2: FluidInput) -> StateResult<&mut Self> {
        self.inner_update(input1, input2)?;
        Ok(self)
    }

    /// Returns a new instance in the specified thermodynamic state.
    ///
    /// # Args
    ///
    /// - input1 -- first input property.
    /// - input2 -- second input property.
    ///
    /// # Errors
    ///
    /// For invalid/unsupported inputs or invalid state,
    /// a [`FluidStateError`](crate::error::FluidStateError) is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use rfluids::error::FluidStateError;
    /// use rfluids::prelude::fluid::*;
    /// use rfluids::uom::si::pressure::atmosphere;
    /// use rfluids::uom::si::thermodynamic_temperature::degree_celsius;
    ///
    /// // After creation the `Fluid` instance has `Undefined` state variant
    /// let mut water: Fluid<Undefined> = Fluid::from(Pure::Water);
    ///
    /// // Calling `in_state` method will move the initial value and
    /// // perform conversion between `Undefined` and `Defined` state variants
    /// // (since `Defined` is the default state variant, it can be omitted)
    /// let mut water: Fluid = water.in_state(
    ///     FluidInput::pressure(Pressure::new::<atmosphere>(1.0)),
    ///     FluidInput::temperature(ThermodynamicTemperature::new::<degree_celsius>(20.0)),
    /// )?;
    ///
    /// // The `Fluid` instance now has `Defined` state variant
    /// // and it's thermodynamic state can be updated in place by calling `update` method
    /// // (which returns a mutable reference to the instance)
    /// let same_water_in_new_state: StateResult<&mut Fluid> = water.update(
    ///     FluidInput::pressure(Pressure::new::<atmosphere>(2.0)),
    ///     FluidInput::temperature(ThermodynamicTemperature::new::<degree_celsius>(40.0)),
    /// );
    /// assert!(same_water_in_new_state.is_ok());
    ///
    /// // Calling `in_state` method on `Fluid<Defined>` will return
    /// // a new instance in the specified thermodynamic state
    /// let new_water: StateResult<Fluid> = water.in_state(
    ///     FluidInput::pressure(Pressure::new::<atmosphere>(4.0)),
    ///     FluidInput::temperature(ThermodynamicTemperature::new::<degree_celsius>(80.0)),
    /// );
    /// assert!(new_water.is_ok());
    /// # Ok::<(), rfluids::error::Error>(())
    /// ```
    ///
    /// # See also
    ///
    /// - [`Fluid::update`](crate::fluid::Fluid::update)
    pub fn in_state(&self, input1: FluidInput, input2: FluidInput) -> StateResult<Self> {
        let mut fluid = Fluid::try_from(self.substance.clone())
            .unwrap()
            .in_state(input1, input2)?;
        fluid.trivial_outputs.clone_from(&self.trivial_outputs);
        Ok(fluid)
    }

    fn output(&mut self, key: FluidParam) -> OutputResult<f64> {
        cached_output(&mut self.outputs, &mut self.backend, key, |e| {
            FluidOutputError::CalculationFailed(key, e)
        })
    }
}

impl Clone for Fluid {
    fn clone(&self) -> Self {
        let inputs: (FluidInput, FluidInput) = self.update_request.unwrap().into();
        let mut fluid = Fluid::try_from(self.substance.clone())
            .unwrap()
            .in_state(inputs.0, inputs.1)
            .unwrap();
        fluid.outputs.clone_from(&self.outputs);
        fluid.trivial_outputs.clone_from(&self.trivial_outputs);
        fluid
    }
}

impl PartialEq for Fluid {
    fn eq(&self, other: &Self) -> bool {
        self.substance == other.substance && self.update_request == other.update_request
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::FluidStateError;
    use crate::fluid::common::tests::test_output;
    use crate::substance::*;
    use crate::uom::si::f64::{Pressure, Ratio, ThermodynamicTemperature};
    use crate::uom::si::pressure::atmosphere;
    use crate::uom::si::ratio::percent;
    use crate::uom::si::thermodynamic_temperature::degree_celsius;
    use rstest::*;

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

    #[fixture]
    fn water(temperature: FluidInput, pressure: FluidInput) -> Fluid {
        Fluid::from(Pure::Water)
            .in_state(temperature, pressure)
            .unwrap()
    }

    #[fixture]
    fn r22(temperature: FluidInput, pressure: FluidInput) -> Fluid {
        Fluid::from(Pure::R22)
            .in_state(temperature, pressure)
            .unwrap()
    }

    #[fixture]
    fn r32(temperature: FluidInput, pressure: FluidInput) -> Fluid {
        Fluid::from(Pure::R32)
            .in_state(temperature, pressure)
            .unwrap()
    }

    #[fixture]
    fn incomp_water(temperature: FluidInput, pressure: FluidInput) -> Fluid {
        Fluid::from(IncompPure::Water)
            .in_state(temperature, pressure)
            .unwrap()
    }

    #[fixture]
    fn r444a(temperature: FluidInput, pressure: FluidInput) -> Fluid {
        Fluid::from(PredefinedMix::R444A)
            .in_state(temperature, pressure)
            .unwrap()
    }

    #[fixture]
    fn propylene_glycol(temperature: FluidInput, pressure: FluidInput) -> Fluid {
        Fluid::from(
            BinaryMix::with_fraction(BinaryMixKind::MPG, Ratio::new::<percent>(40.0)).unwrap(),
        )
        .in_state(temperature, pressure)
        .unwrap()
    }

    #[rstest]
    fn substance_returns_entered_value(temperature: FluidInput, pressure: FluidInput) {
        let water = Pure::Water;
        let substance = Substance::from(water);
        let sut = Fluid::from(water).in_state(temperature, pressure).unwrap();
        assert_eq!(sut.substance(), &substance);
    }

    test_output!(Fluid, f64, acentric_factor, water, 0.344_292_084_3, r444a);
    test_output!(Fluid, critical_density, water, 322.0, r444a, incomp_water);

    test_output!(
        Fluid,
        critical_molar_density,
        water,
        17_873.727_995_609_06,
        r444a,
        incomp_water
    );

    test_output!(
        Fluid,
        critical_pressure,
        water,
        22.064e6,
        r444a,
        incomp_water
    );

    test_output!(
        Fluid,
        critical_temperature,
        water,
        647.096,
        r444a,
        incomp_water
    );

    test_output!(Fluid, f64, flammability_hazard, water, 0.0, incomp_water);

    test_output!(
        Fluid,
        freezing_temperature,
        propylene_glycol,
        252.581_754_953_058_38,
        water
    );

    test_output!(Fluid, f64, gwp20, r32, 2330.0, water);
    test_output!(Fluid, f64, gwp100, r32, 675.0, water);
    test_output!(Fluid, f64, gwp500, r32, 205.0, water);
    test_output!(Fluid, f64, health_hazard, water, 0.0, incomp_water);
    test_output!(Fluid, max_pressure, water, 1e9, incomp_water);
    test_output!(Fluid, always_ok, max_temperature, water, 2e3);

    test_output!(
        Fluid,
        min_pressure,
        water,
        611.654_800_896_868_4,
        incomp_water
    );

    test_output!(Fluid, always_ok, min_temperature, water, 273.16);
    test_output!(Fluid, molar_mass, water, 0.018_015_268, incomp_water);
    test_output!(Fluid, f64, odp, r22, 0.05, water, incomp_water);
    test_output!(Fluid, f64, physical_hazard, water, 0.0, incomp_water);
    test_output!(Fluid, reducing_density, water, 322.0, incomp_water);

    test_output!(
        Fluid,
        reducing_molar_density,
        water,
        17_873.727_995_609_06,
        incomp_water
    );

    test_output!(Fluid, reducing_pressure, water, 22.064e6, incomp_water);
    test_output!(Fluid, reducing_temperature, water, 647.096, incomp_water);

    test_output!(
        Fluid,
        triple_pressure,
        water,
        611.654_800_896_868_4,
        incomp_water
    );

    test_output!(Fluid, triple_temperature, water, 273.16, incomp_water);
    test_output!(Fluid, enthalpy, water, 84_007.300_850_662_8);

    test_output!(
        Fluid,
        f64,
        alpha0,
        water,
        9.942_698_150_834_108,
        propylene_glycol
    );

    test_output!(
        Fluid,
        f64,
        alphar,
        water,
        -9.964_888_981_266_709,
        propylene_glycol
    );

    #[rstest]
    fn update_valid_inputs_returns_ok(
        mut water: Fluid,
        temperature: FluidInput,
        pressure: FluidInput,
    ) {
        assert!(water.update(temperature, pressure).is_ok());
    }

    #[rstest]
    fn update_same_inputs_returns_err(mut water: Fluid, pressure: FluidInput) {
        assert_eq!(
            water.update(pressure, pressure).unwrap_err(),
            FluidStateError::InvalidInputPair(pressure.key(), pressure.key())
        );
    }

    #[rstest]
    fn update_invalid_inputs_returns_err(
        mut water: Fluid,
        temperature: FluidInput,
        infinite_pressure: FluidInput,
    ) {
        assert_eq!(
            water.update(temperature, infinite_pressure).unwrap_err(),
            FluidStateError::InvalidInputValue
        );
    }

    #[rstest]
    fn update_invalid_state_returns_err(
        mut water: Fluid,
        temperature: FluidInput,
        negative_pressure: FluidInput,
    ) {
        assert!(matches!(
            water.update(temperature, negative_pressure).unwrap_err(),
            FluidStateError::UpdateFailed(_)
        ));
    }

    #[rstest]
    fn in_state_valid_inputs_returns_ok(
        water: Fluid,
        temperature: FluidInput,
        pressure: FluidInput,
    ) {
        assert!(water.in_state(temperature, pressure).is_ok());
    }

    #[rstest]
    fn in_state_same_inputs_returns_err(water: Fluid, pressure: FluidInput) {
        assert_eq!(
            water.in_state(pressure, pressure).unwrap_err(),
            FluidStateError::InvalidInputPair(pressure.key(), pressure.key())
        );
    }

    #[rstest]
    fn in_state_invalid_inputs_returns_err(
        water: Fluid,
        temperature: FluidInput,
        infinite_pressure: FluidInput,
    ) {
        assert_eq!(
            water.in_state(temperature, infinite_pressure).unwrap_err(),
            FluidStateError::InvalidInputValue
        );
    }

    #[rstest]
    fn in_state_invalid_state_returns_err(
        water: Fluid,
        temperature: FluidInput,
        negative_pressure: FluidInput,
    ) {
        assert!(matches!(
            water.in_state(temperature, negative_pressure).unwrap_err(),
            FluidStateError::UpdateFailed(_)
        ));
    }

    #[rstest]
    fn clone_returns_new_instance(water: Fluid) {
        let clone = water.clone();
        assert_eq!(clone, water);
        assert_eq!(clone.outputs, water.outputs);
        assert_eq!(clone.trivial_outputs, water.trivial_outputs);
    }
}
