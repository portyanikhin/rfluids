use coolprop_sys::COOLPROP;

use super::{
    CoolProp,
    common::{MessageBuffer, const_ptr_c_char},
};

impl CoolProp {
    /// Returns the current `CoolProp` debug level.
    ///
    /// The debug level controls the verbosity of debugging output from `CoolProp`.
    /// Valid range is `0-10`:
    ///
    /// - `0` -- debugging output is disabled
    /// - `> 0` -- debugging output is enabled, with larger values producing more verbose output
    ///
    /// # Examples
    ///
    /// ```
    /// use rfluids::prelude::*;
    ///
    /// // By default, debugging output is disabled
    /// let debug_level = CoolProp::get_debug_level();
    /// assert_eq!(debug_level, 0);
    /// ```
    ///
    /// # See Also
    ///
    /// - [`CoolProp::set_debug_level`](Self::set_debug_level)
    #[must_use]
    pub fn get_debug_level() -> i32 {
        unsafe { COOLPROP.lock().unwrap().get_debug_level() }
    }

    /// Sets the `CoolProp` debug level.
    ///
    /// Controls the verbosity of debugging output for subsequent `CoolProp` operations.
    ///
    /// # Arguments
    ///
    /// - `level` -- debug level to set (valid range: `0-10`, larger values produce more verbose
    ///   output)
    ///   - `0` -- disable debugging output
    ///   - `> 0` -- enable debugging output with corresponding verbosity level
    ///
    ///
    /// # Examples
    ///
    /// ```
    /// use rfluids::prelude::*;
    ///
    /// // Set maximum verbosity
    /// CoolProp::set_debug_level(10);
    /// assert_eq!(CoolProp::get_debug_level(), 10);
    ///
    /// // Disable debugging output
    /// CoolProp::set_debug_level(0);
    /// assert_eq!(CoolProp::get_debug_level(), 0);
    /// ```
    ///
    /// # See Also
    ///
    /// - [`CoolProp::get_debug_level`](Self::get_debug_level)
    pub fn set_debug_level(level: i32) {
        unsafe { COOLPROP.lock().unwrap().set_debug_level(level) }
    }

    /// Returns a global string parameter from `CoolProp`.
    ///
    /// Returns [`None`] if the parameter key is invalid or the value could not be retrieved.
    ///
    /// # Arguments
    ///
    /// - `param` -- global parameter key _(raw [`&str`](str))_
    ///
    /// Known parameter keys:
    ///
    /// - `"version"` -- `CoolProp` library version
    /// - `"gitrevision"` -- `git` commit hash of the library build
    /// - `"HOME"` -- `CoolProp` home directory path
    /// - `"REFPROP_version"` -- `REFPROP` version, if available
    /// - `"errstring"` -- pending error message
    /// - `"warnstring"` -- pending warning message
    /// - `"fluids_list"` -- list of all available pure and pseudo-pure fluids _(comma-separated)_
    /// - `"incompressible_list_pure"` -- list of pure incompressible fluids _(comma-separated)_
    /// - `"incompressible_list_solution"` -- list of incompressible mixtures _(comma-separated)_
    /// - `"predefined_mixtures"` -- list of predefined mixtures _(comma-separated)_
    /// - `"cubic_fluids_list"` -- list of fluids with cubic EOS _(comma-separated)_
    /// - `"mixture_binary_pairs_list"` -- list of binary pairs for mixtures _(comma-separated)_
    /// - `"parameter_list"` -- list of all available fluid parameters _(comma-separated)_
    /// - `"cubic_fluids_schema"` -- JSON schema for fluids with cubic EOS
    /// - `"pcsaft_fluids_schema"` -- JSON schema for PC-SAFT fluids
    ///
    /// # See Also
    ///
    /// - [CoolProp High-Level API](https://coolprop.org/coolprop/HighLevelAPI.html)
    /// - [`CoolPropLib.h` Reference](https://coolprop.org/_static/doxygen/html/_cool_prop_lib_8h.html)
    pub fn get_global_param_string(param: impl AsRef<str>) -> Option<String> {
        let param = param.as_ref().trim();
        let capacity = if ["version", "gitrevision", "HOME", "REFPROP_version"].contains(&param) {
            100
        } else {
            30_000
        };
        let res = MessageBuffer::with_capacity(capacity);
        let lock = COOLPROP.lock().unwrap();
        let status = unsafe {
            lock.get_global_param_string(const_ptr_c_char!(param), res.buffer, res.capacity)
        };
        let res: String = res.into();
        if status != 1 || res.trim().is_empty() { None } else { Some(res) }
    }
}

#[cfg(test)]
mod tests {
    use rstest::*;

    use super::*;

    #[test]
    fn get_debug_level() {
        // When
        let res = CoolProp::get_debug_level();

        // Then
        assert_eq!(res, 0);
    }

    #[rstest]
    #[case(3)]
    #[case(-42)]
    #[case(0)]
    fn set_debug_level(#[case] value: i32) {
        // When
        CoolProp::set_debug_level(value);
        let res = CoolProp::get_debug_level();

        // Then
        assert_eq!(res, value);
    }

    #[rstest]
    #[case("version", true)]
    #[case("gitrevision", true)]
    #[case("fluids_list", true)]
    #[case("incompressible_list_pure", true)]
    #[case("incompressible_list_solution", true)]
    #[case("mixture_binary_pairs_list", true)]
    #[case("parameter_list", true)]
    #[case("predefined_mixtures", true)]
    #[case("HOME", true)]
    #[case("cubic_fluids_schema", true)]
    #[case("cubic_fluids_list", true)]
    #[case("pcsaft_fluids_schema", true)]
    #[case("", false)]
    #[case(" ", false)]
    #[case("Hello, World!", false)]
    fn get_global_param_string(#[case] param: &str, #[case] is_some: bool) {
        // When
        let res = CoolProp::get_global_param_string(param);

        // Then
        assert_eq!(res.is_some(), is_some);
    }
}
