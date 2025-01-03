use crate::native::common::{const_ptr_c_char, CoolPropError, MessageBuffer, COOLPROP};
use core::ffi::c_char;
use std::sync::MutexGuard;

/// CoolProp thread safe high-level API.
pub struct CoolProp;

impl CoolProp {
    /// Returns a value that depends on the thermodynamic state
    /// of pure/pseudo-pure fluids or mixtures.
    ///
    /// For undefined fluid states or invalid inputs, a [`CoolPropError`] is returned.
    ///
    /// - `output_name` — name of the output.
    /// - `input1_name` — name of the first input property.
    /// - `input1_value` — value of the first input property _(in SI units)_.
    /// - `input2_name` — name of the second input property.
    /// - `input2_value` — value of the second input property _(in SI units)_.
    /// - `fluid_name` — name of the fluid.
    ///
    /// # Examples
    ///
    /// ## Pure fluids
    ///
    /// To calculate the specific heat of saturated water vapor at _1 atm_:
    ///
    /// ```
    /// use approx::assert_relative_eq;
    /// use fluids_rs::native::CoolProp;
    ///
    /// let result = CoolProp::props_si("C", "P", 101325.0, "Q", 1.0, "Water").unwrap();
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
    /// use fluids_rs::native::CoolProp;
    ///
    /// let result = CoolProp::props_si("V", "P", 100e3, "T", 253.15, "INCOMP::MPG-60%").unwrap();
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
    /// use fluids_rs::native::CoolProp;
    ///
    /// let result = CoolProp::props_si(
    ///     "D",
    ///     "P",
    ///     200e3,
    ///     "T",
    ///     277.15,
    ///     "HEOS::Water[0.6]&Ethanol[0.4]",
    /// )
    /// .unwrap();
    /// assert_relative_eq!(result, 859.5296602799147);
    /// ```
    ///
    /// # See also
    ///
    /// - [PropsSI function](https://coolprop.github.io/CoolProp/coolprop/HighLevelAPI.html#propssi-function)
    /// - [PropsSI inputs/outputs](https://coolprop.github.io/CoolProp/coolprop/HighLevelAPI.html#parameter-table)
    /// - [Pure and pseudo-pure fluids](https://coolprop.github.io/CoolProp/fluid_properties/PurePseudoPure.html)
    /// - [Incompressible binary mixtures](https://coolprop.github.io/CoolProp/fluid_properties/Incompressibles.html)
    /// - [Predefined mixtures](https://coolprop.github.io/CoolProp/coolprop/HighLevelAPI.html#predefined-mixtures)
    pub fn props_si(
        output_name: impl AsRef<str>,
        input1_name: impl AsRef<str>,
        input1_value: f64,
        input2_name: impl AsRef<str>,
        input2_value: f64,
        fluid_name: impl AsRef<str>,
    ) -> Result<f64, CoolPropError> {
        let lock = COOLPROP.lock().unwrap();
        let result = unsafe {
            lock.PropsSI(
                const_ptr_c_char!(output_name.as_ref().trim()),
                const_ptr_c_char!(input1_name.as_ref().trim()),
                input1_value,
                const_ptr_c_char!(input2_name.as_ref().trim()),
                input2_value,
                const_ptr_c_char!(fluid_name.as_ref().trim()),
            )
        };
        Self::validate_result(result, lock)?;
        Ok(result)
    }

    /// Returns a value that depends on the thermodynamic state of humid air.
    ///
    /// For undefined humid air states or invalid inputs, a [`CoolPropError`] is returned.
    ///
    /// - `output_name` — name of the output.
    /// - `input1_name` — name of the first input property.
    /// - `input1_value` — value of the first input property _(in SI units)_.
    /// - `input2_name` — name of the second input property.
    /// - `input2_value` — value of the second input property _(in SI units)_.
    /// - `input3_name` — name of the third input property.
    /// - `input3_value` — value of the third input property _(in SI units)_.
    ///
    /// # Examples
    ///
    /// To calculate the wet bulb temperature of humid air
    /// at _100 kPa_, _30 °C_ and _50 %_ relative humidity:
    ///
    /// ```
    /// use approx::assert_relative_eq;
    /// use fluids_rs::native::CoolProp;
    ///
    /// let result = CoolProp::ha_props_si("B", "P", 100e3, "T", 303.15, "R", 0.5).unwrap();
    /// assert_relative_eq!(result, 295.1200365362656);
    /// ```
    ///
    /// # See also
    ///
    /// - [HAPropsSI function](https://coolprop.github.io/CoolProp/fluid_properties/HumidAir.html)
    /// - [HAPropsSI inputs/outputs](https://coolprop.github.io/CoolProp/fluid_properties/HumidAir.html#table-of-inputs-outputs-to-hapropssi)
    pub fn ha_props_si(
        output_name: impl AsRef<str>,
        input1_name: impl AsRef<str>,
        input1_value: f64,
        input2_name: impl AsRef<str>,
        input2_value: f64,
        input3_name: impl AsRef<str>,
        input3_value: f64,
    ) -> Result<f64, CoolPropError> {
        let lock = COOLPROP.lock().unwrap();
        let result = unsafe {
            lock.HAPropsSI(
                const_ptr_c_char!(output_name.as_ref().trim()),
                const_ptr_c_char!(input1_name.as_ref().trim()),
                input1_value,
                const_ptr_c_char!(input2_name.as_ref().trim()),
                input2_value,
                const_ptr_c_char!(input3_name.as_ref().trim()),
                input3_value,
            )
        };
        Self::validate_result(result, lock)?;
        Ok(result)
    }

    //noinspection SpellCheckingInspection
    /// Returns a value that do not depend on the thermodynamic state
    /// of pure/pseudo-pure fluids or mixtures.
    ///
    /// For invalid inputs, a [`CoolPropError`] is returned.
    ///
    /// - `output_name` — name of the _trivial_ output.
    /// - `fluid_name` — name of the fluid.
    ///
    /// # Examples
    ///
    /// Water critical point temperature _(K)_:
    ///
    /// ```
    /// use approx::assert_relative_eq;
    /// use fluids_rs::native::CoolProp;
    ///
    /// let result = CoolProp::props1_si("Tcrit", "Water").unwrap();
    /// assert_relative_eq!(result, 647.096);
    /// ```
    ///
    /// R32 100-year global warming potential _(dimensionless)_:
    ///
    /// ```
    /// use approx::assert_relative_eq;
    /// use fluids_rs::native::CoolProp;
    ///
    /// let result = CoolProp::props1_si("GWP100", "R32").unwrap();
    /// assert_relative_eq!(result, 675.0);
    /// ```
    ///
    /// # See also
    ///
    /// - [Props1SI function](https://coolprop.github.io/CoolProp/coolprop/HighLevelAPI.html#trivial-inputs)
    /// - [Props1SI outputs _(only those for which the value in the "Trivial" column is "True")_](https://coolprop.github.io/CoolProp/coolprop/HighLevelAPI.html#parameter-table)
    pub fn props1_si(
        output_name: impl AsRef<str>,
        fluid_name: impl AsRef<str>,
    ) -> Result<f64, CoolPropError> {
        let lock = COOLPROP.lock().unwrap();
        let result = unsafe {
            lock.Props1SI(
                const_ptr_c_char!(output_name.as_ref().trim()),
                const_ptr_c_char!(fluid_name.as_ref().trim()),
            )
        };
        Self::validate_result(result, lock)?;
        Ok(result)
    }

    fn validate_result(
        result: f64,
        lock: MutexGuard<coolprop_sys::bindings::CoolProp>,
    ) -> Result<(), CoolPropError> {
        if !result.is_finite() {
            let message = Self::get_error_message(lock);
            return Err(CoolPropError(
                message.unwrap_or("Unknown error".to_string()),
            ));
        }
        Ok(())
    }

    fn get_error_message(lock: MutexGuard<coolprop_sys::bindings::CoolProp>) -> Option<String> {
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
    use super::*;
    use approx::assert_relative_eq;
    use rayon::prelude::*;

    #[test]
    fn props_si_invalid_input_returns_err() {
        let result = CoolProp::props_si("D", "P", 101325.0, "Q", -1.0, "Water");
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Input vapor quality [Q] must be between 0 and 1 : \
            PropsSI(\"D\",\"P\",101325,\"Q\",-1,\"Water\")"
        );
    }

    #[test]
    fn props_si_water_density_in_standard_conditions_returns_ok() {
        let result = CoolProp::props_si("D", "P", 101325.0, "T", 293.15, "Water");
        assert!(result.is_ok());
        assert_relative_eq!(result.unwrap(), 998.2071504679284);
    }

    #[test]
    fn props_si_is_thread_safe() {
        let result: Vec<Result<f64, CoolPropError>> = (101_000..101_500)
            .into_par_iter()
            .map(move |p| CoolProp::props_si("T", "P", p as f64, "Q", 0.0, "Water"))
            .collect();
        assert!(result.iter().all(|r| r.is_ok()));
    }

    #[test]
    fn ha_props_si_invalid_input_returns_err() {
        let result = CoolProp::ha_props_si("W", "P", 101325.0, "T", 293.15, "R", -0.5);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "The input for key (7) with value (-0.5) \
            is outside the range of validity: (0) to (1)"
        );
    }

    #[test]
    fn ha_props_si_humid_air_humidity_in_standard_conditions_returns_ok() {
        let result = CoolProp::ha_props_si("W", "P", 101325.0, "T", 293.15, "R", 0.5);
        assert!(result.is_ok());
        assert_relative_eq!(result.unwrap(), 0.007293697701992549);
    }

    #[test]
    fn ha_props_si_is_thread_safe() {
        let result: Vec<Result<f64, CoolPropError>> = (101_000..101_500)
            .into_par_iter()
            .map(move |p| CoolProp::ha_props_si("W", "P", p as f64, "T", 293.15, "R", 0.5))
            .collect();
        assert!(result.iter().all(|r| r.is_ok()));
    }

    //noinspection SpellCheckingInspection
    #[test]
    fn props1_si_valid_input_returns_ok() {
        let result = CoolProp::props1_si("Tcrit", "Water");
        assert!(result.is_ok());
        assert_relative_eq!(result.unwrap(), 647.096);
    }

    #[test]
    fn props1_si_invalid_input_returns_err() {
        let result = CoolProp::props1_si("T", "Water");
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Unable to use input parameter [T] in Props1SI for fluid Water; \
            error was Input pair variable is invalid and output(s) are non-trivial; \
            cannot do state update : PropsSI(\"T\",\"\",0,\"\",0,\"Water\")"
        );
    }

    #[test]
    fn validate_result_valid_number_returns_ok() {
        let result = CoolProp::validate_result(42.0, COOLPROP.lock().unwrap());
        assert!(result.is_ok());
    }

    #[test]
    fn validate_result_invalid_number_returns_err() {
        let result = CoolProp::validate_result(f64::NAN, COOLPROP.lock().unwrap());
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Unknown error");
    }
}
