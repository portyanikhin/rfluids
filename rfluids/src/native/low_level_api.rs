use crate::error::CoolPropError;
use crate::native::common::{const_ptr_c_char, ErrorBuffer, COOLPROP};
use core::ffi::{c_char, c_long};

/// CoolProp thread safe low-level API.
#[derive(Debug)]
pub struct AbstractState {
    ptr: c_long,
}

impl AbstractState {
    /// Creates and returns a new [`AbstractState`] instance with specified backend and fluid names.
    ///
    /// # Args
    ///
    /// - `backend_name` -- name of the backend _(raw [`&str`](str)
    ///   or [`BackendName::backend_name`](crate::substance::BackendName::backend_name))_.
    /// - `fluid_names` -- names of the fluids separated by the `&` symbol
    ///   or just a single fluid name _(raw [`&str`](str),
    ///   [`Substance`](crate::substance::Substance) or its subset)_.
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
    /// use rfluids::native::AbstractState;
    ///
    /// let water = AbstractState::new("HEOS", "Water");
    /// assert!(water.is_ok());
    /// ```
    ///
    /// For incompressible binary mixtures:
    ///
    /// ```
    /// use rfluids::native::AbstractState;
    ///
    /// let propylene_glycol = AbstractState::new("INCOMP", "MPG");
    /// assert!(propylene_glycol.is_ok());
    /// ```
    ///
    /// For mixtures:
    ///
    /// ```
    /// use rfluids::native::AbstractState;
    ///
    /// let mixture = AbstractState::new("HEOS", "Water&Ethanol");
    /// assert!(mixture.is_ok());
    /// ```
    ///
    /// # See also
    ///
    /// - [CoolProp low-level API](https://coolprop.github.io/CoolProp/coolprop/LowLevelAPI.html)
    /// - [Pure and pseudo-pure substances](https://coolprop.github.io/CoolProp/fluid_properties/PurePseudoPure.html)
    /// - [Incompressible substances](https://coolprop.github.io/CoolProp/fluid_properties/Incomps.html)
    /// - [Predefined mixtures](https://coolprop.github.io/CoolProp/coolprop/HighLevelAPI.html#predefined-mixtures)
    /// - [`Substance`](crate::substance::Substance)
    pub fn new(
        backend_name: impl AsRef<str>,
        fluid_names: impl AsRef<str>,
    ) -> Result<AbstractState, CoolPropError> {
        let error = ErrorBuffer::default();
        let ptr = unsafe {
            COOLPROP.lock().unwrap().AbstractState_factory(
                const_ptr_c_char!(backend_name.as_ref().trim()),
                const_ptr_c_char!(fluid_names.as_ref().trim()),
                error.code,
                error.message.buffer,
                error.message.capacity,
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
    /// # Args
    ///
    /// - `fractions` -- fractions of the specified fluid
    ///   _(dimensionless, from 0 to 1 each)_.
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
    /// use rfluids::native::AbstractState;
    ///
    /// let mut propylene_glycol = AbstractState::new("INCOMP", "MPG").unwrap();
    /// let result = propylene_glycol.set_fractions(&[0.6]);
    /// assert!(result.is_ok());
    /// ```
    ///
    /// For mixtures:
    ///
    /// ```
    /// use rfluids::native::AbstractState;
    ///
    /// let mut mixture = AbstractState::new("HEOS", "Water&Ethanol").unwrap();
    /// let result = mixture.set_fractions(&[0.8, 0.2]);
    /// assert!(result.is_ok());
    /// ```
    pub fn set_fractions(&mut self, fractions: &[f64]) -> Result<(), CoolPropError> {
        let error = ErrorBuffer::default();
        unsafe {
            COOLPROP.lock().unwrap().AbstractState_set_fractions(
                self.ptr,
                fractions.as_ptr(),
                fractions.len() as c_long,
                error.code,
                error.message.buffer,
                error.message.capacity,
            );
        }
        Self::result((), error)
    }

    /// Update the state of the fluid.
    ///
    /// # Args
    ///
    /// - `input_pair_key` -- input pair key
    ///   _(raw [`u8`] or [`FluidInputPair`](crate::io::FluidInputPair))_.
    /// - `input1` -- value of the first input property _(in SI units)_.
    /// - `input2` -- value of the second input property _(in SI units)_.
    ///
    /// # Errors
    ///
    /// For invalid inputs, a [`CoolPropError`] is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use rfluids::io::FluidInputPair;
    /// use rfluids::native::AbstractState;
    ///
    /// let mut water = AbstractState::new("HEOS", "Water").unwrap();
    /// let result = water.update(FluidInputPair::PT, 101325.0, 293.15);
    /// assert!(result.is_ok());
    /// ```
    ///
    /// # See also
    ///
    /// - [`FluidInputPair`](crate::io::FluidInputPair)
    pub fn update(
        &mut self,
        input_pair_key: impl Into<u8>,
        input1: f64,
        input2: f64,
    ) -> Result<(), CoolPropError> {
        let error = ErrorBuffer::default();
        unsafe {
            COOLPROP.lock().unwrap().AbstractState_update(
                self.ptr,
                input_pair_key.into() as c_long,
                input1,
                input2,
                error.code,
                error.message.buffer,
                error.message.capacity,
            );
        }
        Self::result((), error)
    }

    /// Get an output parameter value.
    ///
    /// # Args
    ///
    /// - `key` -- output parameter key
    ///   _(raw [`u8`], [`FluidParam`](crate::io::FluidParam) or
    ///   [`FluidTrivialParam`](crate::io::FluidTrivialParam))_.
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
    /// To calculate the specific heat of saturated water vapor at _1 atm_:
    ///
    /// ```
    /// use approx::assert_relative_eq;
    /// use rfluids::io::{FluidInputPair, FluidParam};
    /// use rfluids::native::AbstractState;
    ///
    /// let mut water = AbstractState::new("HEOS", "Water").unwrap();
    /// water.update(FluidInputPair::PQ, 101325.0, 1.0).unwrap();
    /// let result = water.keyed_output(FluidParam::CpMass).unwrap();
    /// assert_relative_eq!(result, 2079.937085633241);
    /// ```
    ///
    /// ## Incompressible binary mixtures
    ///
    /// To calculate the dynamic viscosity of propylene glycol aqueous solution
    /// with _60 %_ mass fraction at _100 kPa_ and _-20 °C_:
    ///
    /// ```
    /// use approx::assert_relative_eq;
    /// use rfluids::io::{FluidInputPair, FluidParam};
    /// use rfluids::native::AbstractState;
    ///
    /// let mut propylene_glycol = AbstractState::new("INCOMP", "MPG").unwrap();
    /// propylene_glycol.set_fractions(&[0.6]).unwrap();
    /// propylene_glycol.update(FluidInputPair::PT, 100e3, 253.15).unwrap();
    /// let result = propylene_glycol.keyed_output(FluidParam::DynamicViscosity).unwrap();
    /// assert_relative_eq!(result, 0.13907391053938847);
    /// ```
    ///
    /// ## Mixtures
    ///
    /// To calculate the density of ethanol aqueous solution
    /// (with ethanol _20 %_ mole fraction) at _200 kPa_ and _4 °C_:
    ///
    /// ```
    /// use approx::assert_relative_eq;
    /// use rfluids::io::{FluidInputPair, FluidParam};
    /// use rfluids::native::AbstractState;
    ///
    /// let mut mixture = AbstractState::new("HEOS", "Water&Ethanol").unwrap();
    /// mixture.set_fractions(&[0.8, 0.2]).unwrap();
    /// mixture.update(FluidInputPair::PT, 200e3, 277.15).unwrap();
    /// let result = mixture.keyed_output(FluidParam::DMass).unwrap();
    /// assert_relative_eq!(result, 883.8826353773796);
    /// ```
    ///
    /// # See also
    ///
    /// - [`FluidParam`](crate::io::FluidParam)
    /// - [`FluidTrivialParam`](crate::io::FluidTrivialParam)
    pub fn keyed_output(&self, key: impl Into<u8>) -> Result<f64, CoolPropError> {
        let error = ErrorBuffer::default();
        let key = key.into();
        let value = unsafe {
            COOLPROP.lock().unwrap().AbstractState_keyed_output(
                self.ptr,
                key as c_long,
                error.code,
                error.message.buffer,
                error.message.capacity,
            )
        };
        Self::keyed_output_result(key, value, error)
    }

    /// Specify the phase state for all further calculations.
    ///
    /// # Args
    ///
    /// - `phase` -- phase state
    ///   _(raw [`&str`](str) or [`Phase`](crate::io::Phase))_.
    ///
    /// # Errors
    ///
    /// For invalid inputs, a [`CoolPropError`] is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use rfluids::io::{FluidInputPair, Phase};
    /// use rfluids::native::AbstractState;
    ///
    /// let mut water = AbstractState::new("HEOS", "Water").unwrap();
    /// water.specify_phase(Phase::Liquid).unwrap();
    /// let mut result = water.update(FluidInputPair::PT, 101325.0, 293.15);
    /// assert!(result.is_ok());
    /// water.specify_phase(Phase::Gas).unwrap();
    /// result = water.update(FluidInputPair::PT, 101325.0, 293.15);
    /// assert!(result.is_err());
    /// ```
    ///
    /// # See also
    ///
    /// - [Imposing the phase (optional)](https://coolprop.github.io/CoolProp/coolprop/HighLevelAPI.html#imposing-the-phase-optional)
    /// - [`Phase`](crate::io::Phase)
    pub fn specify_phase(&mut self, phase: impl AsRef<str>) -> Result<(), CoolPropError> {
        let error = ErrorBuffer::default();
        unsafe {
            COOLPROP.lock().unwrap().AbstractState_specify_phase(
                self.ptr,
                const_ptr_c_char!(phase.as_ref()),
                error.code,
                error.message.buffer,
                error.message.capacity,
            );
        }
        Self::result((), error)
    }

    /// Unspecify the phase state and go back to calculating it based on the inputs.
    ///
    /// # Examples
    ///
    /// ```
    /// use rfluids::io::{FluidInputPair, Phase};
    /// use rfluids::native::AbstractState;
    ///
    /// let mut water = AbstractState::new("HEOS", "Water").unwrap();
    /// water.specify_phase(Phase::Gas).unwrap();
    /// let mut result = water.update(FluidInputPair::PT, 101325.0, 293.15);
    /// assert!(result.is_err());
    /// water.unspecify_phase();
    /// result = water.update(FluidInputPair::PT, 101325.0, 293.15);
    /// assert!(result.is_ok());
    /// ```
    ///
    /// # See also
    ///
    /// - [Imposing the phase (optional)](https://coolprop.github.io/CoolProp/coolprop/HighLevelAPI.html#imposing-the-phase-optional)
    pub fn unspecify_phase(&mut self) {
        let error = ErrorBuffer::blank();
        unsafe {
            COOLPROP.lock().unwrap().AbstractState_unspecify_phase(
                self.ptr,
                error.code,
                error.message.buffer,
                error.message.capacity,
            );
        }
    }

    fn result<T>(value: T, error: ErrorBuffer) -> Result<T, CoolPropError> {
        let error_message: String = error.into();
        if error_message.trim().is_empty() {
            Ok(value)
        } else {
            Err(CoolPropError(error_message))
        }
    }

    fn keyed_output_result(key: u8, value: f64, error: ErrorBuffer) -> Result<f64, CoolPropError> {
        Self::result((), error)?;
        if !value.is_finite() {
            return Err(CoolPropError(format!(
                "Unable to get the output with key '{}' due to invalid or undefined state!",
                key
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
                error.message.capacity,
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::io::{FluidInputPair, FluidParam, Phase};
    use approx::assert_relative_eq;
    use rayon::prelude::*;
    use rstest::*;

    #[test]
    fn abstract_state_is_thread_safe() {
        let result: Vec<Result<f64, CoolPropError>> = (101_000..101_500)
            .into_par_iter()
            .map(move |p| {
                let mut sut = AbstractState::new("HEOS", "Water").unwrap();
                sut.specify_phase(Phase::TwoPhase).unwrap();
                sut.update(FluidInputPair::PQ, p as f64, 0.0).unwrap();
                sut.keyed_output(FluidParam::T)
            })
            .collect();
        assert!(result.iter().all(|r| r.is_ok()));
    }

    #[rstest]
    #[case("HEOS", "Water")]
    #[case("INCOMP", "MPG")]
    #[case("HEOS", "Water&Ethanol")]
    fn new_valid_inputs_returns_ok(#[case] backend_name: &str, #[case] fluid_names: &str) {
        let result = AbstractState::new(backend_name, fluid_names);
        assert!(result.is_ok());
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
    fn new_valid_inputs_returns_err(
        #[case] backend_name: &str,
        #[case] fluid_names: &str,
        #[case] expected_message: &str,
    ) {
        let result = AbstractState::new(backend_name, fluid_names);
        assert_eq!(result.unwrap_err().to_string(), expected_message);
    }

    #[test]
    fn set_fractions_valid_inputs_returns_ok() {
        let mut sut = AbstractState::new("HEOS", "Water&Ethanol").unwrap();
        let result = sut.set_fractions(&[0.6, 0.4]);
        assert!(result.is_ok());
    }

    #[test]
    fn set_fractions_invalid_inputs_returns_err() {
        let mut sut = AbstractState::new("HEOS", "Water&Ethanol").unwrap();
        let result = sut.set_fractions(&[0.6, 0.4, 0.6]);
        assert_eq!(
            result.unwrap_err().to_string(),
            "Error: size of mole fraction vector [3] \
            does not equal that of component vector [2]",
        );
    }

    #[test]
    fn update_valid_inputs_returns_ok() {
        let mut sut = AbstractState::new("HEOS", "Water").unwrap();
        let result = sut.update(FluidInputPair::PT, 101325.0, 293.15);
        assert!(result.is_ok());
    }

    #[test]
    fn update_invalid_inputs_returns_err() {
        let mut sut = AbstractState::new("HEOS", "Water").unwrap();
        let result = sut.update(FluidInputPair::PQ, 101325.0, -1.0);
        assert_eq!(
            result.unwrap_err().to_string(),
            "Error: Input vapor quality [Q] must be between 0 and 1"
        );
    }

    #[test]
    fn keyed_output_valid_state_returns_ok() {
        let mut sut = AbstractState::new("HEOS", "Water").unwrap();
        sut.update(FluidInputPair::PQ, 101325.0, 1.0).unwrap();
        let result = sut.keyed_output(FluidParam::CpMass);
        assert_relative_eq!(result.unwrap(), 2079.937085633241);
    }

    #[test]
    fn keyed_output_invalid_input_returns_err() {
        let sut = AbstractState::new("HEOS", "Water").unwrap();
        let result = sut.keyed_output(255);
        assert_eq!(
            result.unwrap_err().to_string(),
            "Error: Unable to match the key [255] in get_parameter_information for info [short]"
        );
    }

    #[test]
    fn keyed_output_non_trivial_with_not_defined_state_returns_err() {
        let sut = AbstractState::new("HEOS", "Water").unwrap();
        let result = sut.keyed_output(FluidParam::DMass);
        assert_eq!(
            result.unwrap_err().to_string(),
            "Unable to get the output with key '36' due to invalid or undefined state!"
        );
    }

    #[test]
    fn specify_phase_valid_input_specifies_phase_for_all_further_calculations() {
        let mut sut = AbstractState::new("HEOS", "Water").unwrap();
        sut.specify_phase(Phase::Liquid).unwrap();
        let mut result = sut.update(FluidInputPair::PT, 101325.0, 293.15);
        assert!(result.is_ok());
        sut.specify_phase(Phase::Gas).unwrap();
        result = sut.update(FluidInputPair::PT, 101325.0, 293.15);
        assert!(result.is_err());
    }

    #[test]
    fn specify_phase_invalid_input_returns_err() {
        let mut sut = AbstractState::new("HEOS", "Water").unwrap();
        let result = sut.specify_phase("Hello, World!");
        assert_eq!(
            result.unwrap_err().to_string(),
            "Error: Your input name [Hello, World!] is not valid \
            in get_phase_index (names are case sensitive)"
        );
    }

    #[test]
    fn unspecify_phase_unspecifies_phase_for_all_further_calculations() {
        let mut sut = AbstractState::new("HEOS", "Water").unwrap();
        sut.specify_phase(Phase::Gas).unwrap();
        let mut result = sut.update(FluidInputPair::PT, 101325.0, 293.15);
        assert!(result.is_err());
        sut.unspecify_phase();
        result = sut.update(FluidInputPair::PT, 101325.0, 293.15);
        assert!(result.is_ok());
    }
}
