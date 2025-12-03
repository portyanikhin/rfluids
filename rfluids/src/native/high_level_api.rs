// cSpell:disable

use core::ffi::c_char;
use std::sync::MutexGuard;

use super::{
    CoolPropError, Result,
    common::{COOLPROP, MessageBuffer, const_ptr_c_char},
};

/// `CoolProp` thread safe high-level API.
pub struct CoolProp;

impl CoolProp {
    /// Returns a value that depends on the thermodynamic state of the fluid.
    ///
    /// # Arguments
    ///
    /// - `output_key` -- key of the output _(raw [`&str`](str) or
    ///   [`FluidParam`](crate::io::FluidParam))_
    /// - `input1_key` -- key of the first input property _(raw [`&str`](str) or
    ///   [`FluidParam`](crate::io::FluidParam))_
    /// - `input1_value` -- value of the first input property **\[SI units\]**
    /// - `input2_key` -- key of the second input property _(raw [`&str`](str)
    ///   or [`FluidParam`](crate::io::FluidParam))_
    /// - `input2_value` -- value of the second input property **\[SI units\]**
    /// - `fluid_name` -- name of the fluid _(raw [`&str`](str) or
    ///   [`Substance`](crate::substance::Substance) subset)_
    ///
    /// # Errors
    ///
    /// Returns a [`CoolPropError`] for invalid inputs.
    ///
    /// # Examples
    ///
    /// ## Pure fluids
    ///
    /// To calculate the specific heat **\[J/kg/K\]**
    /// of saturated water vapor at _1 atm_:
    ///
    /// ```
    /// use approx::assert_relative_eq;
    /// use rfluids::prelude::*;
    ///
    /// let result = CoolProp::props_si("C", "P", 101_325.0, "Q", 1.0, "Water")?;
    /// assert_relative_eq!(result, 2_079.937_085_633_241, max_relative = 1e-6);
    /// # Ok::<(), rfluids::native::CoolPropError>(())
    /// ```
    ///
    /// ## Incompressible binary mixtures
    ///
    /// To calculate the dynamic viscosity **\[Pa·s\]**
    /// of propylene glycol aqueous solution with _60 %_ mass fraction
    /// at _100 kPa_ and _-20 °C_:
    ///
    /// ```
    /// use approx::assert_relative_eq;
    /// use rfluids::prelude::*;
    ///
    /// let result =
    ///     CoolProp::props_si("V", "P", 100e3, "T", 253.15, "INCOMP::MPG-60%")?;
    /// assert_relative_eq!(result, 0.139_073_910_539_388_47, max_relative = 1e-6);
    /// # Ok::<(), rfluids::native::CoolPropError>(())
    /// ```
    ///
    /// ## Custom mixtures
    ///
    /// To calculate the density **\[kg/m³\]** of ethanol aqueous solution
    /// (with ethanol _40 %_ mass fraction) at _200 kPa_ and _4 °C_:
    ///
    /// ```
    /// use approx::assert_relative_eq;
    /// use rfluids::prelude::*;
    ///
    /// let result = CoolProp::props_si(
    ///     "D",
    ///     "P",
    ///     200e3,
    ///     "T",
    ///     277.15,
    ///     "HEOS::Water[0.6]&Ethanol[0.4]",
    /// )?;
    /// assert_relative_eq!(result, 859.529_660_279_914_7, max_relative = 1e-6);
    /// # Ok::<(), rfluids::native::CoolPropError>(())
    /// ```
    ///
    /// # See Also
    ///
    /// - [PropsSI function](https://coolprop.github.io/CoolProp/coolprop/HighLevelAPI.html#propssi-function)
    /// - [PropsSI inputs/outputs](https://coolprop.github.io/CoolProp/coolprop/HighLevelAPI.html#parameter-table)
    /// - [Pure and pseudo-pure substances](https://coolprop.github.io/CoolProp/fluid_properties/PurePseudoPure.html)
    /// - [Incompressible substances](https://coolprop.github.io/CoolProp/fluid_properties/Incomps.html)
    /// - [Predefined mixtures](https://coolprop.github.io/CoolProp/coolprop/HighLevelAPI.html#predefined-mixtures)
    /// - [`FluidParam`](crate::io::FluidParam)
    /// - [`Substance`](crate::substance::Substance)
    pub fn props_si(
        output_key: impl AsRef<str>,
        input1_key: impl AsRef<str>,
        input1_value: f64,
        input2_key: impl AsRef<str>,
        input2_value: f64,
        fluid_name: impl AsRef<str>,
    ) -> Result<f64> {
        let lock = COOLPROP.lock().unwrap();
        let value = unsafe {
            lock.PropsSI(
                const_ptr_c_char!(output_key.as_ref().trim()),
                const_ptr_c_char!(input1_key.as_ref().trim()),
                input1_value,
                const_ptr_c_char!(input2_key.as_ref().trim()),
                input2_value,
                const_ptr_c_char!(fluid_name.as_ref().trim()),
            )
        };
        Self::result(value, &lock)
    }

    /// Returns a value that depends on the thermodynamic state of humid air.
    ///
    /// # Arguments
    ///
    /// - `output_key` -- key of the output _(raw [`&str`](str) or
    ///   [`HumidAirParam`](crate::io::HumidAirParam))_
    /// - `input1_key` -- key of the first input property _(raw [`&str`](str) or
    ///   [`HumidAirParam`](crate::io::HumidAirParam))_
    /// - `input1_value` -- value of the first input property **\[SI units\]**
    /// - `input2_key` -- key of the second input property _(raw [`&str`](str)
    ///   or [`HumidAirParam`](crate::io::HumidAirParam))_
    /// - `input2_value` -- value of the second input property **\[SI units\]**
    /// - `input3_key` -- key of the third input property _(raw [`&str`](str) or
    ///   [`HumidAirParam`](crate::io::HumidAirParam))_
    /// - `input3_value` -- value of the third input property **\[SI units\]**
    ///
    /// # Errors
    ///
    /// Returns a [`CoolPropError`] for invalid inputs.
    ///
    /// # Examples
    ///
    /// To calculate the wet bulb temperature **\[K\]** of humid air
    /// at _100 kPa_, _30 °C_ and _50 %_ relative humidity:
    ///
    /// ```
    /// use approx::assert_relative_eq;
    /// use rfluids::prelude::*;
    ///
    /// let result = CoolProp::ha_props_si("B", "P", 100e3, "T", 303.15, "R", 0.5)?;
    /// assert_relative_eq!(result, 295.120_036_536_265_6, max_relative = 1e-6);
    /// # Ok::<(), rfluids::native::CoolPropError>(())
    /// ```
    ///
    /// # See Also
    ///
    /// - [HAPropsSI function](https://coolprop.github.io/CoolProp/fluid_properties/HumidAir.html)
    /// - [HAPropsSI inputs/outputs](https://coolprop.github.io/CoolProp/fluid_properties/HumidAir.html#table-of-inputs-outputs-to-hapropssi)
    /// - [`HumidAirParam`](crate::io::HumidAirParam)
    pub fn ha_props_si(
        output_key: impl AsRef<str>,
        input1_key: impl AsRef<str>,
        input1_value: f64,
        input2_key: impl AsRef<str>,
        input2_value: f64,
        input3_key: impl AsRef<str>,
        input3_value: f64,
    ) -> Result<f64> {
        let lock = COOLPROP.lock().unwrap();
        let value = unsafe {
            lock.HAPropsSI(
                const_ptr_c_char!(output_key.as_ref().trim()),
                const_ptr_c_char!(input1_key.as_ref().trim()),
                input1_value,
                const_ptr_c_char!(input2_key.as_ref().trim()),
                input2_value,
                const_ptr_c_char!(input3_key.as_ref().trim()),
                input3_value,
            )
        };
        Self::result(value, &lock)
    }

    /// Returns a value that doesn't depend on the thermodynamic state
    /// of the fluid _(trivial output)_.
    ///
    /// # Arguments
    ///
    /// - `output_key` -- key of the _trivial_ output _(raw [`&str`](str) or
    ///   [`FluidTrivialParam`](crate::io::FluidTrivialParam))_
    /// - `fluid_name` -- name of the fluid _(raw [`&str`](str) or
    ///   [`Substance`](crate::substance::Substance) subset)_
    ///
    /// # Errors
    ///
    /// Returns a [`CoolPropError`] for invalid inputs.
    ///
    /// # Examples
    ///
    /// Water critical point temperature **\[K\]**:
    ///
    /// ```
    /// use approx::assert_relative_eq;
    /// use rfluids::prelude::*;
    ///
    /// let result = CoolProp::props1_si("Tcrit", "Water")?;
    /// assert_relative_eq!(result, 647.096, max_relative = 1e-6);
    /// # Ok::<(), rfluids::native::CoolPropError>(())
    /// ```
    ///
    /// R32 100-year global warming potential **\[dimensionless\]**:
    ///
    /// ```
    /// use rfluids::prelude::*;
    ///
    /// let result = CoolProp::props1_si("GWP100", "R32")?;
    /// assert_eq!(result, 675.0);
    /// # Ok::<(), rfluids::native::CoolPropError>(())
    /// ```
    ///
    /// # See Also
    ///
    /// - [Props1SI function](https://coolprop.github.io/CoolProp/coolprop/HighLevelAPI.html#trivial-inputs)
    /// - [Props1SI outputs _(only those for which the value in the "Trivial" column is "True")_](https://coolprop.github.io/CoolProp/coolprop/HighLevelAPI.html#parameter-table)
    /// - [`FluidTrivialParam`](crate::io::FluidTrivialParam)
    /// - [`Substance`](crate::substance::Substance)
    pub fn props1_si(output_key: impl AsRef<str>, fluid_name: impl AsRef<str>) -> Result<f64> {
        let lock = COOLPROP.lock().unwrap();
        let value = unsafe {
            lock.Props1SI(
                const_ptr_c_char!(output_key.as_ref().trim()),
                const_ptr_c_char!(fluid_name.as_ref().trim()),
            )
        };
        Self::result(value, &lock)
    }

    fn result(value: f64, lock: &MutexGuard<coolprop_sys::bindings::CoolProp>) -> Result<f64> {
        if !value.is_finite() {
            let message = Self::get_error_message(lock);
            return Err(CoolPropError(message.unwrap_or("Unknown error".into())));
        }
        Ok(value)
    }

    fn get_error_message(lock: &MutexGuard<coolprop_sys::bindings::CoolProp>) -> Option<String> {
        let message = MessageBuffer::default();
        let _unused = unsafe {
            lock.get_global_param_string(
                const_ptr_c_char!("errstring"),
                message.buffer,
                message.capacity,
            )
        };
        let result: String = message.into();
        if result.trim().is_empty() {
            None
        } else {
            Some(result)
        }
    }
}

#[cfg(test)]
mod tests {
    use rayon::prelude::*;

    use super::*;
    use crate::test::assert_relative_eq;

    #[test]
    fn props_si_thread_safety() {
        // Given
        let substance = "Water";
        let pressure_range = 101_000..101_500;
        let quality = 0.0;

        // When
        let res: Vec<Result<f64>> = pressure_range
            .into_par_iter()
            .map(move |p| CoolProp::props_si("T", "P", p.into(), "Q", quality, substance))
            .collect();

        // Then
        assert!(res.iter().all(Result::is_ok));
    }

    #[test]
    fn props_si_water_density_in_standard_conditions() {
        // Given
        let substance = "Water";
        let pressure = 101_325.0;
        let temperature = 293.15;

        // When
        let res = CoolProp::props_si("D", "P", pressure, "T", temperature, substance).unwrap();

        // Then
        assert_relative_eq!(res, 998.207_150_467_928_4);
    }

    #[test]
    fn props_si_invalid_input() {
        // Given
        let substance = "Water";
        let pressure = 101_325.0;
        let negative_quality = -1.0;

        // When
        let res = CoolProp::props_si("D", "P", pressure, "Q", negative_quality, substance);

        // Then
        assert_eq!(
            res.unwrap_err().to_string(),
            "Input vapor quality [Q] must be between 0 and 1 : \
            PropsSI(\"D\",\"P\",101325,\"Q\",-1,\"Water\")"
        );
    }

    #[test]
    fn ha_props_si_thread_safety() {
        // Given
        let pressure_range = 101_000..101_500;
        let temperature = 293.15;
        let rel_humidity = 0.5;

        // When
        let res: Vec<Result<f64>> = pressure_range
            .into_par_iter()
            .map(move |p| {
                CoolProp::ha_props_si("W", "P", p.into(), "T", temperature, "R", rel_humidity)
            })
            .collect();

        // Then
        assert!(res.iter().all(Result::is_ok));
    }

    #[test]
    fn ha_props_si_humid_air_humidity_in_standard_conditions() {
        // Given
        let pressure = 101_325.0;
        let temperature = 293.15;
        let rel_humidity = 0.5;

        // When
        let res =
            CoolProp::ha_props_si("W", "P", pressure, "T", temperature, "R", rel_humidity).unwrap();

        // Then
        assert_relative_eq!(res, 7.293_697_701_992_549e-3);
    }

    #[test]
    fn ha_props_si_invalid_input() {
        // Given
        let pressure = 101_325.0;
        let temperature = 293.15;
        let negative_rel_humidity = -0.5;

        // When
        let res = CoolProp::ha_props_si(
            "W",
            "P",
            pressure,
            "T",
            temperature,
            "R",
            negative_rel_humidity,
        );

        // Then
        assert_eq!(
            res.unwrap_err().to_string(),
            "The input for key (7) with value (-0.5) \
            is outside the range of validity: (0) to (1)"
        );
    }

    #[test]
    fn props1_si_valid_input() {
        // Given
        let substance = "Water";
        let key = "Tcrit";

        // When
        let res = CoolProp::props1_si(key, substance).unwrap();

        // Then
        assert_relative_eq!(res, 647.096);
    }

    #[test]
    fn props1_si_invalid_input() {
        // Given
        let substance = "Water";
        let non_trivial_key = "T";

        // When
        let res = CoolProp::props1_si(non_trivial_key, substance);

        // Then
        assert_eq!(
            res.unwrap_err().to_string(),
            "Unable to use input parameter [T] in Props1SI for fluid Water; \
            error was Input pair variable is invalid and output(s) are non-trivial; \
            cannot do state update : PropsSI(\"T\",\"\",0,\"\",0,\"Water\")"
        );
    }

    #[test]
    fn result_valid() {
        // Given
        let valid = 42.0;

        // When
        let res = CoolProp::result(valid, &COOLPROP.lock().unwrap());

        // Then
        assert!(res.is_ok());
    }

    #[test]
    fn result_invalid() {
        // Given
        let invalid = f64::NAN;

        // When
        let res = CoolProp::result(invalid, &COOLPROP.lock().unwrap());

        // Then
        assert_eq!(res.unwrap_err().to_string(), "Unknown error");
    }
}
