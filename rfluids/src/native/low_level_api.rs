use super::{
    CoolPropError, Result,
    common::{COOLPROP, ErrorBuffer, const_ptr_c_char},
};
use core::ffi::{c_char, c_long};

/// `CoolProp` thread safe low-level API.
#[derive(Debug)]
pub struct AbstractState {
    ptr: c_long,
}

impl AbstractState {
    /// Creates and returns a new [`AbstractState`] instance
    /// with specified backend and fluid names.
    ///
    /// # Arguments
    ///
    /// - `backend_name` -- name of the backend _(raw [`&str`](str)
    ///   or [`BackendName::backend_name`](crate::substance::BackendName::backend_name))_
    /// - `fluid_names` -- names of the fluids separated by the `&` symbol or just a single
    ///   fluid name _(raw [`&str`](str) or [`Substance`](crate::substance::Substance) subset)_
    ///
    /// # Errors
    ///
    /// For invalid inputs, a [`CoolPropError`] is returned.
    ///
    /// # Examples
    ///
    /// For pure fluids:
    ///
    /// ```
    /// use rfluids::prelude::*;
    ///
    /// let water = AbstractState::new("HEOS", "Water");
    /// assert!(water.is_ok());
    /// ```
    ///
    /// For incompressible binary mixtures:
    ///
    /// ```
    /// use rfluids::prelude::*;
    ///
    /// let propylene_glycol = AbstractState::new("INCOMP", "MPG");
    /// assert!(propylene_glycol.is_ok());
    /// ```
    ///
    /// For mixtures:
    ///
    /// ```
    /// use rfluids::prelude::*;
    ///
    /// let mixture = AbstractState::new("HEOS", "Water&Ethanol");
    /// assert!(mixture.is_ok());
    /// ```
    ///
    /// # See Also
    ///
    /// - [CoolProp low-level API](https://coolprop.github.io/CoolProp/coolprop/LowLevelAPI.html)
    /// - [Pure and pseudo-pure substances](https://coolprop.github.io/CoolProp/fluid_properties/PurePseudoPure.html)
    /// - [Incompressible substances](https://coolprop.github.io/CoolProp/fluid_properties/Incomps.html)
    /// - [Predefined mixtures](https://coolprop.github.io/CoolProp/coolprop/HighLevelAPI.html#predefined-mixtures)
    /// - [`Substance`](crate::substance::Substance)
    pub fn new(
        backend_name: impl AsRef<str>,
        fluid_names: impl AsRef<str>,
    ) -> Result<AbstractState> {
        let error = ErrorBuffer::default();
        let ptr = unsafe {
            COOLPROP.lock().unwrap().AbstractState_factory(
                const_ptr_c_char!(backend_name.as_ref().trim()),
                const_ptr_c_char!(fluid_names.as_ref().trim()),
                error.code,
                error.message.buffer,
                c_long::from(error.message.capacity),
            )
        };
        Self::result(Self { ptr }, error)
    }

    /// Set the fractions _(mole, mass or volume)_[^note].
    ///
    /// [^note]:  It will be defined automatically, depending on the specified backend.
    /// For example, the `"HEOS"` backend uses mole fractions and the `"INCOMP"` backend --
    /// mass or volume fractions, depending on which substance is specified.
    ///
    /// # Arguments
    ///
    /// - `fractions` -- fractions of the specified fluid
    ///   **\[dimensionless, from 0 to 1 each\]**
    ///
    /// # Errors
    ///
    /// For invalid inputs, a [`CoolPropError`] is returned.
    ///
    /// # Examples
    ///
    /// For incompressible binary mixtures:
    ///
    /// ```
    /// use rfluids::prelude::*;
    ///
    /// let mut propylene_glycol = AbstractState::new("INCOMP", "MPG")?;
    /// let result = propylene_glycol.set_fractions(&[0.6]);
    /// assert!(result.is_ok());
    /// # Ok::<(), rfluids::native::CoolPropError>(())
    /// ```
    ///
    /// For custom mixtures:
    ///
    /// ```
    /// use rfluids::prelude::*;
    ///
    /// let mut mixture = AbstractState::new("HEOS", "Water&Ethanol")?;
    /// let result = mixture.set_fractions(&[0.8, 0.2]);
    /// assert!(result.is_ok());
    /// # Ok::<(), rfluids::native::CoolPropError>(())
    /// ```
    pub fn set_fractions(&mut self, fractions: &[f64]) -> Result<()> {
        let error = ErrorBuffer::default();
        unsafe {
            COOLPROP.lock().unwrap().AbstractState_set_fractions(
                self.ptr,
                fractions.as_ptr(),
                fractions.len() as c_long,
                error.code,
                error.message.buffer,
                c_long::from(error.message.capacity),
            );
        }
        Self::result((), error)
    }

    /// Update the state of the fluid.
    ///
    /// # Arguments
    ///
    /// - `input_pair_key` -- input pair key
    ///   _(raw [`u8`] or [`FluidInputPair`](crate::io::FluidInputPair))_
    /// - `input1` -- value of the first input property **\[SI units\]**
    /// - `input2` -- value of the second input property **\[SI units\]**
    ///
    /// # Errors
    ///
    /// For invalid inputs, a [`CoolPropError`] is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use rfluids::prelude::*;
    ///
    /// let mut water = AbstractState::new("HEOS", "Water")?;
    /// let result = water.update(FluidInputPair::PT, 101_325.0, 293.15);
    /// assert!(result.is_ok());
    /// # Ok::<(), rfluids::native::CoolPropError>(())
    /// ```
    ///
    /// # See Also
    ///
    /// - [`FluidInputPair`](crate::io::FluidInputPair)
    pub fn update(
        &mut self,
        input_pair_key: impl Into<u8>,
        input1: f64,
        input2: f64,
    ) -> Result<()> {
        let error = ErrorBuffer::default();
        unsafe {
            COOLPROP.lock().unwrap().AbstractState_update(
                self.ptr,
                c_long::from(input_pair_key.into()),
                input1,
                input2,
                error.code,
                error.message.buffer,
                c_long::from(error.message.capacity),
            );
        }
        Self::result((), error)
    }

    /// Returns an output parameter value **\[SI units\]**
    ///
    /// # Arguments
    ///
    /// - `key` -- output parameter key
    ///   _(raw [`u8`], [`FluidParam`](crate::io::FluidParam) or
    ///   [`FluidTrivialParam`](crate::io::FluidTrivialParam))_
    ///
    /// # Errors
    ///
    /// For non-trivial outputs with undefined state or invalid inputs,
    /// a [`CoolPropError`] is returned.
    ///
    /// # Examples
    ///
    /// ## Pure fluids
    ///
    /// To calculate the specific heat **\[J/kg/K\]** of saturated water vapor at _1 atm_:
    ///
    /// ```
    /// use approx::assert_relative_eq;
    /// use rfluids::prelude::*;
    ///
    /// let mut water = AbstractState::new("HEOS", "Water")?;
    /// water.update(FluidInputPair::PQ, 101_325.0, 1.0)?;
    /// let result = water.keyed_output(FluidParam::CpMass)?;
    /// assert_relative_eq!(result, 2_079.937_085_633_241, max_relative = 1e-6);
    /// # Ok::<(), rfluids::native::CoolPropError>(())
    /// ```
    ///
    /// ## Incompressible binary mixtures
    ///
    /// To calculate the dynamic viscosity **\[Pa·s\]** of propylene glycol aqueous solution
    /// with _60 %_ mass fraction at _100 kPa_ and _-20 °C_:
    ///
    /// ```
    /// use approx::assert_relative_eq;
    /// use rfluids::prelude::*;
    ///
    /// let mut propylene_glycol = AbstractState::new("INCOMP", "MPG")?;
    /// propylene_glycol.set_fractions(&[0.6])?;
    /// propylene_glycol.update(FluidInputPair::PT, 100e3, 253.15)?;
    /// let result = propylene_glycol.keyed_output(FluidParam::DynamicViscosity)?;
    /// assert_relative_eq!(result, 0.139_073_910_539_388_47, max_relative = 1e-6);
    /// # Ok::<(), rfluids::native::CoolPropError>(())
    /// ```
    ///
    /// ## Custom mixtures
    ///
    /// To calculate the density **\[kg/m³\]** of ethanol aqueous solution
    /// (with ethanol _20 %_ mole fraction) at _200 kPa_ and _4 °C_:
    ///
    /// ```
    /// use approx::assert_relative_eq;
    /// use rfluids::prelude::*;
    ///
    /// let mut mixture = AbstractState::new("HEOS", "Water&Ethanol")?;
    /// mixture.set_fractions(&[0.8, 0.2])?;
    /// mixture.update(FluidInputPair::PT, 200e3, 277.15)?;
    /// let result = mixture.keyed_output(FluidParam::DMass)?;
    /// assert_relative_eq!(result, 883.882_635_377_379_6, max_relative = 1e-6);
    /// # Ok::<(), rfluids::native::CoolPropError>(())
    /// ```
    ///
    /// # See Also
    ///
    /// - [`FluidParam`](crate::io::FluidParam)
    /// - [`FluidTrivialParam`](crate::io::FluidTrivialParam)
    pub fn keyed_output(&self, key: impl Into<u8>) -> Result<f64> {
        let error = ErrorBuffer::default();
        let key = key.into();
        let value = unsafe {
            COOLPROP.lock().unwrap().AbstractState_keyed_output(
                self.ptr,
                c_long::from(key),
                error.code,
                error.message.buffer,
                c_long::from(error.message.capacity),
            )
        };
        Self::keyed_output_result(key, value, error)
    }

    /// Specify the phase state for all further calculations.
    ///
    /// # Arguments
    ///
    /// - `phase` -- phase state
    ///   _(raw [`&str`](str) or [`Phase`](crate::io::Phase))_
    ///
    /// # Errors
    ///
    /// For invalid inputs, a [`CoolPropError`] is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use rfluids::prelude::*;
    ///
    /// let mut water = AbstractState::new("HEOS", "Water")?;
    /// water.specify_phase(Phase::Liquid)?;
    /// let mut result = water.update(FluidInputPair::PT, 101_325.0, 293.15);
    /// assert!(result.is_ok());
    /// water.specify_phase(Phase::Gas)?;
    /// result = water.update(FluidInputPair::PT, 101_325.0, 293.15);
    /// assert!(result.is_err());
    /// # Ok::<(), rfluids::native::CoolPropError>(())
    /// ```
    ///
    /// # See Also
    ///
    /// - [Imposing the phase (optional)](https://coolprop.github.io/CoolProp/coolprop/HighLevelAPI.html#imposing-the-phase-optional)
    /// - [`Phase`](crate::io::Phase)
    pub fn specify_phase(&mut self, phase: impl AsRef<str>) -> Result<()> {
        let error = ErrorBuffer::default();
        unsafe {
            COOLPROP.lock().unwrap().AbstractState_specify_phase(
                self.ptr,
                const_ptr_c_char!(phase.as_ref()),
                error.code,
                error.message.buffer,
                c_long::from(error.message.capacity),
            );
        }
        Self::result((), error)
    }

    /// Unspecify the phase state and go back to calculating it based on the inputs.
    ///
    /// # Examples
    ///
    /// ```
    /// use rfluids::prelude::*;
    ///
    /// let mut water = AbstractState::new("HEOS", "Water")?;
    /// water.specify_phase(Phase::Gas)?;
    /// let mut result = water.update(FluidInputPair::PT, 101_325.0, 293.15);
    /// assert!(result.is_err());
    /// water.unspecify_phase();
    /// result = water.update(FluidInputPair::PT, 101_325.0, 293.15);
    /// assert!(result.is_ok());
    /// # Ok::<(), rfluids::native::CoolPropError>(())
    /// ```
    ///
    /// # See Also
    ///
    /// - [Imposing the phase (optional)](https://coolprop.github.io/CoolProp/coolprop/HighLevelAPI.html#imposing-the-phase-optional)
    pub fn unspecify_phase(&mut self) {
        let error = ErrorBuffer::blank();
        unsafe {
            COOLPROP.lock().unwrap().AbstractState_unspecify_phase(
                self.ptr,
                error.code,
                error.message.buffer,
                c_long::from(error.message.capacity),
            );
        }
    }

    fn result<T>(value: T, error: ErrorBuffer) -> Result<T> {
        let error_message: String = error.into();
        if error_message.trim().is_empty() {
            Ok(value)
        } else {
            Err(CoolPropError(error_message))
        }
    }

    fn keyed_output_result(key: u8, value: f64, error: ErrorBuffer) -> Result<f64> {
        Self::result((), error)?;
        if !value.is_finite() {
            return Err(CoolPropError(format!(
                "Unable to get the output with key '{key}' due to invalid or undefined state!",
            )));
        }
        Ok(value)
    }
}

impl Drop for AbstractState {
    fn drop(&mut self) {
        let error = ErrorBuffer::blank();
        unsafe {
            COOLPROP.lock().unwrap().AbstractState_free(
                self.ptr,
                error.code,
                error.message.buffer,
                c_long::from(error.message.capacity),
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        io::{FluidInputPair, FluidParam, Phase},
        test::assert_relative_eq,
    };
    use rayon::prelude::*;
    use rstest::*;

    #[test]
    fn thread_safety() {
        // Given
        let backend = "HEOS";
        let substance = "Water";
        let pressure_range = 101_000..101_500;
        let quality = 0.0;

        // When
        let res: Vec<Result<f64>> = pressure_range
            .into_par_iter()
            .map(move |p| {
                let mut sut = AbstractState::new(backend, substance).unwrap();
                sut.specify_phase(Phase::TwoPhase).unwrap();
                sut.update(FluidInputPair::PQ, p.into(), quality).unwrap();
                sut.keyed_output(FluidParam::T)
            })
            .collect();

        // Then
        assert!(res.iter().all(Result::is_ok));
    }

    #[rstest]
    #[case("HEOS", "Water")]
    #[case("INCOMP", "MPG")]
    #[case("HEOS", "Water&Ethanol")]
    fn new_valid_inputs(#[case] backend_name: &str, #[case] fluid_names: &str) {
        // When
        let res = AbstractState::new(backend_name, fluid_names);

        // Then
        assert!(res.is_ok());
    }

    #[rstest]
    #[case(
        "Hello, World!",
        "Water",
        "Error: Invalid backend name [Hello, World!] to factory function"
    )]
    #[case(
        "INCOMP",
        "Hello, World!",
        "Error: key [Hello, World!] was not found in string_to_index_map \
        in JSONIncompressibleLibrary"
    )]
    #[case(
        "HEOS",
        "Water+Ethanol",
        "Error: key [Water+Ethanol] was not found in string_to_index_map \
        in JSONFluidLibrary"
    )]
    fn new_invalid_inputs(
        #[case] backend_name: &str,
        #[case] fluid_names: &str,
        #[case] expected_message: &str,
    ) {
        // When
        let res = AbstractState::new(backend_name, fluid_names);

        // Then
        assert_eq!(res.unwrap_err().to_string(), expected_message);
    }

    #[test]
    fn set_fractions_valid_input() {
        // Given
        let mut sut = AbstractState::new("HEOS", "Water&Ethanol").unwrap();

        // When
        let res = sut.set_fractions(&[0.6, 0.4]);

        // Then
        assert!(res.is_ok());
    }

    #[test]
    fn set_fractions_invalid_input() {
        // Given
        let mut sut = AbstractState::new("HEOS", "Water&Ethanol").unwrap();

        // When
        let res = sut.set_fractions(&[0.6, 0.4, 0.6]);

        // Then
        assert_eq!(
            res.unwrap_err().to_string(),
            "Error: size of mole fraction vector [3] \
            does not equal that of component vector [2]",
        );
    }

    #[test]
    fn update_valid_inputs() {
        // Given
        let mut sut = AbstractState::new("HEOS", "Water").unwrap();

        // When
        let res = sut.update(FluidInputPair::PT, 101_325.0, 293.15);

        // Then
        assert!(res.is_ok());
    }

    #[test]
    fn update_invalid_inputs() {
        // Given
        let mut sut = AbstractState::new("HEOS", "Water").unwrap();

        // When
        let res = sut.update(FluidInputPair::PQ, 101_325.0, -1.0);

        // Then
        assert_eq!(
            res.unwrap_err().to_string(),
            "Error: Input vapor quality [Q] must be between 0 and 1"
        );
    }

    #[test]
    fn keyed_output_valid_state() {
        // Given
        let mut sut = AbstractState::new("HEOS", "Water").unwrap();
        sut.update(FluidInputPair::PQ, 101_325.0, 1.0).unwrap();

        // When
        let res = sut.keyed_output(FluidParam::CpMass).unwrap();

        // Then
        assert_relative_eq!(res, 2_079.937_085_633_241);
    }

    #[test]
    fn keyed_output_invalid_input() {
        // Given
        let sut = AbstractState::new("HEOS", "Water").unwrap();

        // When
        let res = sut.keyed_output(255);

        // Then
        assert_eq!(
            res.unwrap_err().to_string(),
            "Error: Unable to match the key [255] in get_parameter_information for info [short]"
        );
    }

    #[test]
    fn keyed_output_non_trivial_with_not_defined_state() {
        // Given
        let sut = AbstractState::new("HEOS", "Water").unwrap();

        // When
        let res = sut.keyed_output(FluidParam::DMass);

        // Then
        assert_eq!(
            res.unwrap_err().to_string(),
            "Unable to get the output with key '39' due to invalid or undefined state!"
        );
    }

    #[test]
    fn specify_phase_valid() {
        // Given
        let mut sut = AbstractState::new("HEOS", "Water").unwrap();

        // When
        sut.specify_phase(Phase::Liquid).unwrap();
        let res1 = sut.update(FluidInputPair::PT, 101_325.0, 293.15);
        sut.specify_phase(Phase::Gas).unwrap();
        let res2 = sut.update(FluidInputPair::PT, 101_325.0, 293.15);

        // Then
        assert!(res1.is_ok());
        assert!(res2.is_err());
    }

    #[test]
    fn specify_phase_invalid() {
        // Given
        let mut sut = AbstractState::new("HEOS", "Water").unwrap();

        // When
        let res = sut.specify_phase("Hello, World!");

        // Then
        assert_eq!(
            res.unwrap_err().to_string(),
            "Error: Your input name [Hello, World!] is not valid \
            in get_phase_index (names are case sensitive)"
        );
    }

    #[test]
    fn unspecify_phase() {
        // Given
        let mut sut = AbstractState::new("HEOS", "Water").unwrap();

        // When
        sut.specify_phase(Phase::Gas).unwrap();
        let res1 = sut.update(FluidInputPair::PT, 101_325.0, 293.15);
        sut.unspecify_phase();
        let res2 = sut.update(FluidInputPair::PT, 101_325.0, 293.15);

        // Then
        assert!(res1.is_err());
        assert!(res2.is_ok());
    }
}
