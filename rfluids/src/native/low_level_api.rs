use core::ffi::c_long;
use std::{borrow::Cow, ffi::CString};

use coolprop_sys::COOLPROP;

use super::{CoolPropError, Result, common::ErrorBuffer};
use crate::substance::{Substance, SubstanceWithBackend};

/// `CoolProp` thread safe low-level API.
#[derive(Debug)]
pub struct AbstractState {
    ptr: c_long,
}

impl AbstractState {
    /// Creates and returns a new [`AbstractState`] instance
    /// with specified backend and substance names.
    ///
    /// # Arguments
    ///
    /// - `backend_name` -- name of the backend _(raw [`&str`](str) or
    ///   [`Backend::name`](crate::fluid::backend::Backend::name))_
    /// - `composition_id` -- names of the substance components separated by the `&` symbol or just
    ///   a single substance name _(raw [`&str`](str) or
    ///   [`Substance::composition_id`](crate::substance::Substance::composition_id))_
    ///
    /// # Errors
    ///
    /// Returns a [`CoolPropError`] for invalid inputs.
    ///
    /// # Examples
    ///
    /// For pure substances:
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
    /// - [CoolProp Low-Level API](https://coolprop.org/coolprop/LowLevelAPI.html)
    /// - [Pure and Pseudo-Pure Substances](https://coolprop.org/fluid_properties/PurePseudoPure.html)
    /// - [Incompressible Substances](https://coolprop.org/fluid_properties/Incompressibles.html)
    /// - [Predefined Mixtures](https://coolprop.org/coolprop/HighLevelAPI.html#predefined-mixtures)
    /// - [`Substance`](crate::substance::Substance)
    pub fn new(
        backend_name: impl AsRef<str>,
        composition_id: impl AsRef<str>,
    ) -> Result<AbstractState> {
        let backend_name = CString::new(backend_name.as_ref().trim()).unwrap();
        let composition_id = CString::new(composition_id.as_ref().trim()).unwrap();
        let mut err = ErrorBuffer::default();
        let ptr = unsafe {
            COOLPROP.lock().unwrap().AbstractState_factory(
                backend_name.as_ptr(),
                composition_id.as_ptr(),
                err.code_as_mut_ptr(),
                err.message.as_mut_ptr(),
                c_long::from(err.message.capacity()),
            )
        };
        res(Self { ptr }, err)
    }

    /// Set the fractions _(mole, mass or volume)_[^note].
    ///
    /// [^note]:  It will be defined automatically, depending on the specified backend.
    /// For example, the `"HEOS"` backend uses mole fractions, while the `"INCOMP"` backend
    /// uses mass or volume fractions, depending on which substance is specified.
    ///
    /// # Arguments
    ///
    /// - `fractions` -- substance fractions **\[dimensionless, from 0 to 1 each\]**
    ///
    /// # Errors
    ///
    /// Returns a [`CoolPropError`] for invalid inputs.
    ///
    /// # Examples
    ///
    /// For incompressible binary mixtures:
    ///
    /// ```
    /// use rfluids::prelude::*;
    ///
    /// let mut propylene_glycol = AbstractState::new("INCOMP", "MPG")?;
    /// let res = propylene_glycol.set_fractions(&[0.6]);
    /// assert!(res.is_ok());
    /// # Ok::<(), rfluids::native::CoolPropError>(())
    /// ```
    ///
    /// For custom mixtures:
    ///
    /// ```
    /// use rfluids::prelude::*;
    ///
    /// let mut mixture = AbstractState::new("HEOS", "Water&Ethanol")?;
    /// let res = mixture.set_fractions(&[0.8, 0.2]);
    /// assert!(res.is_ok());
    /// # Ok::<(), rfluids::native::CoolPropError>(())
    /// ```
    pub fn set_fractions(&mut self, fractions: &[f64]) -> Result<()> {
        let mut err = ErrorBuffer::default();
        unsafe {
            COOLPROP.lock().unwrap().AbstractState_set_fractions(
                self.ptr,
                fractions.as_ptr(),
                fractions.len() as c_long,
                err.code_as_mut_ptr(),
                err.message.as_mut_ptr(),
                c_long::from(err.message.capacity()),
            );
        }
        res((), err)
    }

    /// Update the state of the fluid.
    ///
    /// # Arguments
    ///
    /// - `input_pair_key` -- input pair key _(raw [`u8`] or
    ///   [`FluidInputPair`](crate::io::FluidInputPair))_
    /// - `input1` -- value of the first input property **\[SI units\]**
    /// - `input2` -- value of the second input property **\[SI units\]**
    ///
    /// # Errors
    ///
    /// Returns a [`CoolPropError`] for invalid inputs.
    ///
    /// # Examples
    ///
    /// ```
    /// use rfluids::prelude::*;
    ///
    /// let mut water = AbstractState::new("HEOS", "Water")?;
    /// let res = water.update(FluidInputPair::PT, 101_325.0, 293.15);
    /// assert!(res.is_ok());
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
        let mut err = ErrorBuffer::default();
        unsafe {
            COOLPROP.lock().unwrap().AbstractState_update(
                self.ptr,
                c_long::from(input_pair_key.into()),
                input1,
                input2,
                err.code_as_mut_ptr(),
                err.message.as_mut_ptr(),
                c_long::from(err.message.capacity()),
            );
        }
        res((), err)
    }

    /// Returns an output parameter value **\[SI units\]**
    ///
    /// # Arguments
    ///
    /// - `key` -- output parameter key _(raw [`u8`], [`FluidParam`](crate::io::FluidParam) or
    ///   [`FluidTrivialParam`](crate::io::FluidTrivialParam))_
    ///
    /// # Errors
    ///
    /// Returns a [`CoolPropError`] for non-trivial outputs with undefined state or invalid inputs.
    ///
    /// # Examples
    ///
    /// ## Pure substances
    ///
    /// To calculate the specific heat **\[J/kg/K\]** of saturated water vapor at _1 atm_:
    ///
    /// ```
    /// use approx::assert_relative_eq;
    /// use rfluids::prelude::*;
    ///
    /// let mut water = AbstractState::new("HEOS", "Water")?;
    /// water.update(FluidInputPair::PQ, 101_325.0, 1.0)?;
    /// let res = water.keyed_output(FluidParam::CpMass)?;
    /// assert_relative_eq!(res, 2_079.937_085_633_241, max_relative = 1e-6);
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
    /// let res = propylene_glycol.keyed_output(FluidParam::DynamicViscosity)?;
    /// assert_relative_eq!(res, 0.139_073_910_539_388_47, max_relative = 1e-6);
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
    /// let res = mixture.keyed_output(FluidParam::DMass)?;
    /// assert_relative_eq!(res, 883.882_635_377_379_6, max_relative = 1e-6);
    /// # Ok::<(), rfluids::native::CoolPropError>(())
    /// ```
    ///
    /// # See Also
    ///
    /// - [`FluidParam`](crate::io::FluidParam)
    /// - [`FluidTrivialParam`](crate::io::FluidTrivialParam)
    pub fn keyed_output(&self, key: impl Into<u8>) -> Result<f64> {
        let mut err = ErrorBuffer::default();
        let key = key.into();
        let value = unsafe {
            COOLPROP.lock().unwrap().AbstractState_keyed_output(
                self.ptr,
                c_long::from(key),
                err.code_as_mut_ptr(),
                err.message.as_mut_ptr(),
                c_long::from(err.message.capacity()),
            )
        };
        keyed_output(key, value, err)
    }

    /// Specify the phase state for all further calculations.
    ///
    /// # Arguments
    ///
    /// - `phase` -- phase state _(raw [`&str`](str) or [`Phase`](crate::io::Phase))_
    ///
    /// # Errors
    ///
    /// Returns a [`CoolPropError`] for invalid inputs.
    ///
    /// # Examples
    ///
    /// ```
    /// use rfluids::prelude::*;
    ///
    /// let mut water = AbstractState::new("HEOS", "Water")?;
    /// water.specify_phase(Phase::Liquid)?;
    /// let mut res = water.update(FluidInputPair::PT, 101_325.0, 293.15);
    /// assert!(res.is_ok());
    /// water.specify_phase(Phase::Gas)?;
    /// res = water.update(FluidInputPair::PT, 101_325.0, 293.15);
    /// assert!(res.is_err());
    /// # Ok::<(), rfluids::native::CoolPropError>(())
    /// ```
    ///
    /// # See Also
    ///
    /// - [Imposing the Phase (Optional)](https://coolprop.org/coolprop/HighLevelAPI.html#imposing-the-phase-optional)
    /// - [`Phase`](crate::io::Phase)
    pub fn specify_phase(&mut self, phase: impl AsRef<str>) -> Result<()> {
        let phase = CString::new(phase.as_ref()).unwrap();
        let mut err = ErrorBuffer::default();
        unsafe {
            COOLPROP.lock().unwrap().AbstractState_specify_phase(
                self.ptr,
                phase.as_ptr(),
                err.code_as_mut_ptr(),
                err.message.as_mut_ptr(),
                c_long::from(err.message.capacity()),
            );
        }
        res((), err)
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
    /// let mut res = water.update(FluidInputPair::PT, 101_325.0, 293.15);
    /// assert!(res.is_err());
    /// water.unspecify_phase();
    /// res = water.update(FluidInputPair::PT, 101_325.0, 293.15);
    /// assert!(res.is_ok());
    /// # Ok::<(), rfluids::native::CoolPropError>(())
    /// ```
    ///
    /// # See Also
    ///
    /// - [Imposing the Phase (Optional)](https://coolprop.org/coolprop/HighLevelAPI.html#imposing-the-phase-optional)
    pub fn unspecify_phase(&mut self) {
        let mut err = ErrorBuffer::blank();
        unsafe {
            COOLPROP.lock().unwrap().AbstractState_unspecify_phase(
                self.ptr,
                err.code_as_mut_ptr(),
                err.message.as_mut_ptr(),
                c_long::from(err.message.capacity()),
            );
        }
    }
}

impl TryFrom<&SubstanceWithBackend> for AbstractState {
    type Error = CoolPropError;

    fn try_from(value: &SubstanceWithBackend) -> Result<Self> {
        let (component_names, fractions): (Cow<'static, str>, Option<Vec<f64>>) =
            match &value.substance {
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
        let mut backend = AbstractState::new(value.backend.name(), component_names)?;
        if let Some(fractions) = fractions {
            backend.set_fractions(&fractions).unwrap();
        }
        Ok(backend)
    }
}

impl Drop for AbstractState {
    fn drop(&mut self) {
        let mut err = ErrorBuffer::blank();
        unsafe {
            COOLPROP.lock().unwrap().AbstractState_free(
                self.ptr,
                err.code_as_mut_ptr(),
                err.message.as_mut_ptr(),
                c_long::from(err.message.capacity()),
            );
        }
    }
}

fn res<T>(value: T, err: ErrorBuffer) -> Result<T> {
    let err: Option<CoolPropError> = err.into();
    err.map_or(Ok(value), Err)
}

fn keyed_output(key: u8, value: f64, err: ErrorBuffer) -> Result<f64> {
    res((), err)?;
    if !value.is_finite() {
        return Err(CoolPropError(format!(
            "Unable to get the output with key '{key}' due to invalid or undefined state!",
        )));
    }
    Ok(value)
}

#[cfg(test)]
mod tests {
    use rayon::prelude::*;
    use rstest::*;

    use super::*;
    use crate::{
        io::{FluidInputPair, FluidParam, Phase},
        test::assert_relative_eq,
    };

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
    fn new_valid_inputs(#[case] backend_name: &str, #[case] substance_names: &str) {
        // When
        let res = AbstractState::new(backend_name, substance_names);

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
        #[case] substance_names: &str,
        #[case] expected_message: &str,
    ) {
        // When
        let res = AbstractState::new(backend_name, substance_names);

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
