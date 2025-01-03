use crate::native::common::{const_ptr_c_char, CoolPropError, ErrorBuffer, COOLPROP};
use crate::native::enums::*;
use core::ffi::{c_char, c_long};

/// CoolProp thread safe low-level API.
#[derive(Debug)]
pub struct AbstractState {
    ptr: c_long,
}

impl AbstractState {
    /// Creates and returns a new [`AbstractState`] instance with specified backend and fluid names.
    ///
    /// For invalid inputs, a [`CoolPropError`] is returned.
    ///
    /// - `backend_name` — name of the backend.
    /// - `fluid_names` — names of the fluids separated by the `&` symbol
    ///   (or just a single fluid name).
    ///
    /// # Examples
    ///
    /// For pure fluids:
    ///
    /// ```
    /// use fluids_rs::native::AbstractState;
    ///
    /// let water = AbstractState::new("HEOS", "Water");
    /// assert!(water.is_ok());
    /// ```
    ///
    /// For incompressible binary mixtures:
    ///
    /// ```
    /// use fluids_rs::native::AbstractState;
    ///
    /// let propylene_glycol = AbstractState::new("INCOMP", "MPG");
    /// assert!(propylene_glycol.is_ok());
    /// ```
    ///
    /// For mixtures:
    ///
    /// ```
    /// use fluids_rs::native::AbstractState;
    ///
    /// let mixture = AbstractState::new("HEOS", "Water&Ethanol");
    /// assert!(mixture.is_ok());
    /// ```
    ///
    /// # See also
    ///
    /// - [CoolProp low-level API](https://coolprop.github.io/CoolProp/coolprop/LowLevelAPI.html)
    /// - [Pure and pseudo-pure fluids](https://coolprop.github.io/CoolProp/fluid_properties/PurePseudoPure.html)
    /// - [Incompressible binary mixtures](https://coolprop.github.io/CoolProp/fluid_properties/Incompressibles.html)
    /// - [Predefined mixtures](https://coolprop.github.io/CoolProp/coolprop/HighLevelAPI.html#predefined-mixtures)
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

    /// Set the fractions (mole, mass or volume — it will be defined automatically).
    ///
    /// For invalid inputs, a [`CoolPropError`] is returned.
    ///
    /// - `fractions` — specified fractions _(from 0 to 1 each)_.
    ///
    /// # Examples
    ///
    /// For incompressible binary mixtures:
    ///
    /// ```
    /// use fluids_rs::native::AbstractState;
    ///
    /// let propylene_glycol = AbstractState::new("INCOMP", "MPG").unwrap();
    /// let result = propylene_glycol.set_fractions(&[0.6]);
    /// assert!(result.is_ok());
    /// ```
    ///
    /// For mixtures:
    ///
    /// ```
    /// use fluids_rs::native::AbstractState;
    ///
    /// let mixture = AbstractState::new("HEOS", "Water&Ethanol").unwrap();
    /// let result = mixture.set_fractions(&[0.6, 0.4]);
    /// assert!(result.is_ok());
    /// ```
    pub fn set_fractions(&self, fractions: &[f64]) -> Result<(), CoolPropError> {
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

    /// Update the state of fluid.
    ///
    /// For invalid inputs, a [`CoolPropError`] is returned.
    ///
    /// - `input_pair` — CoolProp input pair.
    /// - `input1` — value of the first input property _(in SI units)_.
    /// - `input2` — value of the second input property _(in SI units)_.
    ///
    /// # Examples
    ///
    /// ```
    /// use fluids_rs::native::{AbstractState, InputPair};
    ///
    /// let water = AbstractState::new("HEOS", "Water").unwrap();
    /// let result = water.update(InputPair::PT, 101325.0, 293.15);
    /// assert!(result.is_ok());
    /// ```
    pub fn update(
        &self,
        input_pair: InputPair,
        input1: f64,
        input2: f64,
    ) -> Result<(), CoolPropError> {
        let error = ErrorBuffer::default();
        unsafe {
            COOLPROP.lock().unwrap().AbstractState_update(
                self.ptr,
                input_pair as c_long,
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
    /// For invalid state or for non-trivial outputs with undefined state,
    /// a [`CoolPropError`] is returned.
    ///
    /// - `key` — specified CoolProp output parameter.
    ///
    /// # Examples
    ///
    /// ## Pure fluids
    ///
    /// To calculate the specific heat of saturated water vapor at _1 atm_:
    ///
    /// ```
    /// use approx::assert_relative_eq;
    /// use fluids_rs::native::{AbstractState, InputPair, Parameter};
    ///
    /// let water = AbstractState::new("HEOS", "Water").unwrap();
    /// water.update(InputPair::PQ, 101325.0, 1.0).unwrap();
    /// let result = water.keyed_output(Parameter::CpMass).unwrap();
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
    /// use fluids_rs::native::{AbstractState, InputPair, Parameter};
    ///
    /// let propylene_glycol = AbstractState::new("INCOMP", "MPG").unwrap();
    /// propylene_glycol.set_fractions(&[0.6]).unwrap();
    /// propylene_glycol.update(InputPair::PT, 100e3, 253.15).unwrap();
    /// let result = propylene_glycol.keyed_output(Parameter::DynamicViscosity).unwrap();
    /// assert_relative_eq!(result, 0.13907391053938847);
    /// ```
    ///
    /// ## Mixtures
    ///
    /// To calculate the density of ethanol aqueous solution
    /// (with ethanol _40 %_ mass fraction) at _200 kPa_ and _4 °C_:
    ///
    /// ```
    /// use approx::assert_relative_eq;
    /// use fluids_rs::native::{AbstractState, InputPair, Parameter};
    ///
    /// let mixture = AbstractState::new("HEOS", "Water&Ethanol").unwrap();
    /// mixture.set_fractions(&[0.6, 0.4]).unwrap();
    /// mixture.update(InputPair::PT, 200e3, 277.15).unwrap();
    /// let result = mixture.keyed_output(Parameter::DMass).unwrap();
    /// assert_relative_eq!(result, 859.5296602799147);
    /// ```
    pub fn keyed_output(&self, key: Parameter) -> Result<f64, CoolPropError> {
        let error = ErrorBuffer::default();
        let value = unsafe {
            COOLPROP.lock().unwrap().AbstractState_keyed_output(
                self.ptr,
                key as c_long,
                error.code,
                error.message.buffer,
                error.message.capacity,
            )
        };
        Self::validated_keyed_output(key, value)
    }

    /// Specify the phase state for all further calculations.
    ///
    /// - `phase` — specified phase state.
    ///
    /// # Examples
    ///
    /// ```
    /// use fluids_rs::native::{AbstractState, InputPair, Phase};
    ///
    /// let water = AbstractState::new("HEOS", "Water").unwrap();
    /// water.specify_phase(Phase::Liquid);
    /// let mut result = water.update(InputPair::PT, 101325.0, 293.15);
    /// assert!(result.is_ok());
    /// water.specify_phase(Phase::Gas);
    /// result = water.update(InputPair::PT, 101325.0, 293.15);
    /// assert!(result.is_err());
    /// ```
    ///
    /// # See also
    ///
    /// - [Imposing the phase (optional)](https://coolprop.github.io/CoolProp/coolprop/HighLevelAPI.html#imposing-the-phase-optional)
    pub fn specify_phase(&self, phase: Phase) {
        let error = ErrorBuffer::default();
        let phase_name: &'static str = phase.into();
        unsafe {
            COOLPROP.lock().unwrap().AbstractState_specify_phase(
                self.ptr,
                const_ptr_c_char!(phase_name),
                error.code,
                error.message.buffer,
                error.message.capacity,
            );
        }
    }

    /// Unspecify the phase state and go back to calculating it based on the inputs.
    ///
    /// # Examples
    ///
    /// ```
    /// use fluids_rs::native::{AbstractState, InputPair, Phase};
    ///
    /// let water = AbstractState::new("HEOS", "Water").unwrap();
    /// water.specify_phase(Phase::Gas);
    /// let mut result = water.update(InputPair::PT, 101325.0, 293.15);
    /// assert!(result.is_err());
    /// water.unspecify_phase();
    /// result = water.update(InputPair::PT, 101325.0, 293.15);
    /// assert!(result.is_ok());
    /// ```
    ///
    /// # See also
    ///
    /// - [Imposing the phase (optional)](https://coolprop.github.io/CoolProp/coolprop/HighLevelAPI.html#imposing-the-phase-optional)
    pub fn unspecify_phase(&self) {
        let error = ErrorBuffer::default();
        unsafe {
            COOLPROP.lock().unwrap().AbstractState_unspecify_phase(
                self.ptr,
                error.code,
                error.message.buffer,
                error.message.capacity,
            );
        }
    }

    fn result<T>(result: T, error_buffer: ErrorBuffer) -> Result<T, CoolPropError> {
        let error_message: String = error_buffer.into();
        if error_message.trim().is_empty() {
            Ok(result)
        } else {
            Err(CoolPropError(error_message))
        }
    }

    fn validated_keyed_output(key: Parameter, value: f64) -> Result<f64, CoolPropError> {
        if !value.is_finite() {
            return Err(CoolPropError(format!(
                "Unable to get the value of '{:?}' due to invalid or undefined state!",
                key
            )));
        }
        Ok(value)
    }
}

impl Drop for AbstractState {
    fn drop(&mut self) {
        let error = ErrorBuffer::default();
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
    use approx::assert_relative_eq;
    use rstest::*;

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
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), expected_message);
    }

    #[test]
    fn set_fractions_valid_inputs_returns_ok() {
        let sut = AbstractState::new("HEOS", "Water&Ethanol").unwrap();
        let result = sut.set_fractions(&[0.6, 0.4]);
        assert!(result.is_ok());
    }

    #[test]
    fn set_fractions_invalid_inputs_returns_err() {
        let sut = AbstractState::new("HEOS", "Water&Ethanol").unwrap();
        let result = sut.set_fractions(&[0.6, 0.4, 0.6]);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Error: size of mole fraction vector [3] \
            does not equal that of component vector [2]",
        );
    }

    #[test]
    fn update_valid_inputs_returns_ok() {
        let sut = AbstractState::new("HEOS", "Water").unwrap();
        let result = sut.update(InputPair::PT, 101325.0, 293.15);
        assert!(result.is_ok());
    }

    #[test]
    fn update_invalid_inputs_returns_err() {
        let sut = AbstractState::new("HEOS", "Water").unwrap();
        let result = sut.update(InputPair::PQ, 101325.0, -1.0);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Error: Input vapor quality [Q] must be between 0 and 1"
        );
    }

    #[test]
    fn keyed_output_valid_state_returns_ok() {
        let water = AbstractState::new("HEOS", "Water").unwrap();
        water.update(InputPair::PQ, 101325.0, 1.0).unwrap();
        let result = water.keyed_output(Parameter::CpMass);
        assert!(result.is_ok());
        assert_relative_eq!(result.unwrap(), 2079.937085633241);
    }

    #[test]
    fn keyed_output_non_trivial_with_not_defined_state_returns_err() {
        let water = AbstractState::new("HEOS", "Water").unwrap();
        let result = water.keyed_output(Parameter::DMass);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Unable to get the value of 'DMass' due to invalid or undefined state!"
        );
    }

    #[test]
    fn specify_phase_always_specifies_phase_for_all_further_calculations() {
        let sut = AbstractState::new("HEOS", "Water").unwrap();
        sut.specify_phase(Phase::Liquid);
        let mut result = sut.update(InputPair::PT, 101325.0, 293.15);
        assert!(result.is_ok());
        sut.specify_phase(Phase::Gas);
        result = sut.update(InputPair::PT, 101325.0, 293.15);
        assert!(result.is_err());
    }

    #[test]
    fn unspecify_phase_always_unspecifies_phase_for_all_further_calculations() {
        let sut = AbstractState::new("HEOS", "Water").unwrap();
        sut.specify_phase(Phase::Gas);
        let mut result = sut.update(InputPair::PT, 101325.0, 293.15);
        assert!(result.is_err());
        sut.unspecify_phase();
        result = sut.update(InputPair::PT, 101325.0, 293.15);
        assert!(result.is_ok());
    }
}
