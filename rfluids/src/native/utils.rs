use coolprop_sys::COOLPROP;

use super::{
    CoolProp, Result,
    common::{MessageBuffer, const_ptr_c_char, get_error},
};

const MIN_DEBUG_LEVEL: u8 = 0;
const MAX_DEBUG_LEVEL: u8 = 10;

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
    /// ```no_run
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
    pub fn get_debug_level() -> u8 {
        let level = unsafe { COOLPROP.lock().unwrap().get_debug_level() };
        level.clamp(MIN_DEBUG_LEVEL.into(), MAX_DEBUG_LEVEL.into()) as u8
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
    /// ```no_run
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
    pub fn set_debug_level(level: u8) {
        unsafe {
            COOLPROP
                .lock()
                .unwrap()
                .set_debug_level(level.clamp(MIN_DEBUG_LEVEL, MAX_DEBUG_LEVEL).into());
        }
    }

    /// Returns a global string parameter from `CoolProp`.
    ///
    /// Returns [`None`] if the parameter key is invalid or the value could not be retrieved.
    ///
    /// # Arguments
    ///
    /// - `param` -- global parameter key _(raw [`&str`](str) or
    ///   [`GlobalParam`](crate::io::GlobalParam))_
    ///
    /// Known parameter keys:
    ///
    /// - `"version"` -- `CoolProp` library version
    /// - `"gitrevision"` -- `git` commit hash of the `CoolProp` build
    /// - `"HOME"` -- `CoolProp` home directory path
    /// - `"REFPROP_version"` -- `REFPROP` version, if available
    /// - `"errstring"` -- pending error message
    /// - `"warnstring"` -- pending warning message
    /// - `"fluids_list"` -- list of all available pure and pseudo-pure substances
    ///   _(comma-separated)_
    /// - `"incompressible_list_pure"` -- list of all available pure incompressible substances
    ///   _(comma-separated)_
    /// - `"incompressible_list_solution"` -- list of all available incompressible binary mixtures
    ///   _(comma-separated)_
    /// - `"predefined_mixtures"` -- list of all available predefined mixtures _(comma-separated)_
    /// - `"cubic_fluids_list"` -- list of all substances for which a cubic EOS is applicable
    ///   _(comma-separated)_
    /// - `"mixture_binary_pairs_list"` -- list of binary pairs for mixtures _(comma-separated)_
    /// - `"parameter_list"` -- list of all available fluid parameters _(comma-separated)_
    /// - `"cubic_fluids_schema"` -- JSON schema for substances with cubic EOS
    /// - `"pcsaft_fluids_schema"` -- JSON schema for substances with PC-SAFT EOS
    ///
    /// # See Also
    ///
    /// - [CoolProp High-Level API](https://coolprop.org/coolprop/HighLevelAPI.html)
    /// - [`CoolPropLib.h` Reference](https://coolprop.org/_static/doxygen/html/_cool_prop_lib_8h.html)
    /// - [`GlobalParam`](crate::io::GlobalParam)
    pub fn get_global_param(param: impl AsRef<str>) -> Option<String> {
        let param = param.as_ref().trim();
        let capacity = match param {
            "version" | "gitrevision" | "HOME" | "REFPROP_version" => 100,
            "errstring" | "warnstring" => 500,
            _ => 30_000,
        };
        let res = MessageBuffer::with_capacity(capacity);
        let lock = COOLPROP.lock().unwrap();
        let status = unsafe {
            lock.get_global_param_string(const_ptr_c_char!(param), res.buffer, res.capacity)
        };
        let res: String = res.into();
        if status != 1 || res.trim().is_empty() { None } else { Some(res) }
    }

    /// Returns a string parameter for a specific substance from `CoolProp`.
    ///
    /// Returns [`None`] if the input is invalid or the value could not be retrieved.
    ///
    /// # Arguments
    ///
    /// - `substance_name` -- name of the substance _(raw [`&str`](str) or
    ///   [`Substance`](crate::substance::Substance) subset)_
    /// - `param` -- substance parameter key _(raw [`&str`](str) or
    ///   [`SubstanceParam`](crate::io::SubstanceParam))_
    ///
    /// Known parameter keys:
    ///
    /// - `"aliases"` -- list of aliases for the substance _(comma-separated)_
    /// - `"CAS"` -- CAS number
    /// - `"ASHRAE34"` -- ASHRAE Standard 34 safety rating
    /// - `"REFPROP_name"` -- substance name used in `REFPROP`
    /// - `"BibTeX-XXX"` -- BibTeX key, where `XXX` is one of the following:
    ///     - `EOS` -- equation of state reference
    ///     - `CP0` -- ideal gas heat capacity equation reference
    ///     - `CONDUCTIVITY` -- thermal conductivity equation reference
    ///     - `MELTING_LINE` -- melting line equation reference
    ///     - `SURFACE_TENSION` -- surface tension equation reference
    ///     - `VISCOSITY` -- viscosity equation reference
    /// - `"pure"` -- `"true"` if the substance is pure, `"false"` otherwise
    /// - `"formula"` -- chemical formula of the substance in LaTeX form _(if available)_
    ///
    /// # See Also
    ///
    /// - [CoolProp High-Level API](https://coolprop.org/coolprop/HighLevelAPI.html)
    /// - [Substance Information](https://coolprop.org/coolprop/HighLevelAPI.html#fluid-information)
    /// - [`CoolPropLib.h` Reference](https://coolprop.org/_static/doxygen/html/_cool_prop_lib_8h.html)
    /// - [`SubstanceParam`](crate::io::SubstanceParam)
    pub fn get_substance_param(
        substance_name: impl AsRef<str>,
        param: impl AsRef<str>,
    ) -> Option<String> {
        let res = MessageBuffer::default();
        let lock = COOLPROP.lock().unwrap();
        let status = unsafe {
            lock.get_fluid_param_string(
                const_ptr_c_char!(substance_name.as_ref().trim()),
                const_ptr_c_char!(param.as_ref().trim()),
                res.buffer,
                res.capacity,
            )
        };
        let res: String = res.into();
        if status != 1 || res.trim().is_empty() { None } else { Some(res) }
    }

    /// Sets a configuration key-value pair in `CoolProp`.
    ///
    /// Configuration keys control various aspects of `CoolProp` behavior (e.g., path settings,
    /// property limits checking, gas constant normalization, etc.).
    ///
    /// # Arguments
    ///
    /// - `key` -- configuration key name _(raw [`&str`](str))_
    /// - `value` -- configuration value _(raw [`bool`], [`f64`], [`String`], or [`&str`](str),
    ///   automatically converted to [`ConfigValue`])_
    ///
    /// Known configuration keys:
    ///
    /// - **Calculation Behavior:**
    ///     - `"ASSUME_CRITICAL_POINT_STABLE"` -- if `true`, evaluation of the stability of critical
    ///       point will be skipped and point will be assumed to be stable. _Default: `false`_
    ///     - `"CRITICAL_SPLINES_ENABLED"` -- if `true`, the critical splines will be used in the
    ///       near-vicinity of the critical point. _Default: `true`_
    ///     - `"CRITICAL_WITHIN_1UK"` -- if `true`, any temperature within `1 uK` of the critical
    ///       temperature will be considered to be **AT** the critical point. _Default: `true`_
    ///     - `"DONT_CHECK_PROPERTY_LIMITS"` -- if `true`, when possible, `CoolProp` will skip
    ///       checking whether values are inside the property limits. _Default: `false`_
    ///     - `"ENABLE_SUPERANCILLARIES"` -- if `true`, the superancillary functions will be used
    ///       for VLE of pure fluids. _Default: `true`_
    ///     - `"HENRYS_LAW_TO_GENERATE_VLE_GUESSES"` -- if `true`, when doing water-based mixture
    ///       dewpoint calculations, use Henry’s Law to generate guesses for liquid-phase
    ///       composition. _Default: `false`_
    ///     - `"NORMALIZE_GAS_CONSTANTS"` -- if `true`, for mixtures, the molar gas constant _(R)_
    ///       will be set to the CODATA value. _Default: `true`_
    ///     - `"OVERWRITE_BINARY_INTERACTION"` -- if `true`, and a pair of binary interaction pairs
    ///       to be added is already there, rather than not adding the binary interaction pair _(and
    ///       probably throwing an exception)_, overwrite it. _Default: `false`_
    ///     - `"OVERWRITE_DEPARTURE_FUNCTION"` -- if `true`, and a departure function to be added is
    ///       already there, rather than not adding the departure function _(and probably throwing
    ///       an exception)_, overwrite it. _Default: `false`_
    ///     - `"OVERWRITE_FLUIDS"` -- if `true`, and a fluid is added to the fluids library that is
    ///       already there, rather than not adding the fluid _(and probably throwing an
    ///       exception)_, overwrite it. _Default: `false`_
    ///     - `"PHASE_ENVELOPE_STARTING_PRESSURE_PA"` -- starting pressure in `Pa` for phase
    ///       envelope construction. _Default: `100.0`_
    ///     - `"R_U_CODATA"` -- the value for the ideal gas constant in `J/mol/K` according to
    ///       CODATA 2022. This value is used to harmonize all the ideal gas constants. This is
    ///       especially important in the critical region. _Default: `8.314_462_618_153_24`_
    ///     - `"SPINODAL_MINIMUM_DELTA"` -- the minimal delta to be used in tracing out the
    ///       spinodal; make sure that the EOS has a spinodal at this value of `delta=rho/rho_r`.
    ///       _Default: `0.5`_
    ///     - `"USE_GUESSES_IN_PROPSSI"` -- if `true`, calls to the vectorized versions of `PropsSI`
    ///       use the previous state as guess value while looping over the input vectors, only makes
    ///       sense when working with a single fluid and with points that are not too far from each
    ///       other. _Default: `false`_
    ///
    /// - **`REFPROP` Integration:**
    ///     - `"ALTERNATIVE_REFPROP_PATH"` -- an alternative path to be provided to the directory
    ///       that contains `REFPROP`’s fluids and mixtures directories. If provided, the `SETPATH`
    ///       function will be called with this directory prior to calling any `REFPROP` functions.
    ///       _Default: `""`_
    ///     - `"ALTERNATIVE_REFPROP_LIBRARY_PATH"` -- an alternative path to the shared library
    ///       file. If provided, it will be used to load `REFPROP`. _Default: `""`_
    ///     - `"ALTERNATIVE_REFPROP_HMX_BNC_PATH"` -- an alternative path to the `HMX.BNC` file. If
    ///       provided, it will be passed into `REFPROP`’s `SETUP` or `SETMIX` routines. _Default:
    ///       `""`_
    ///     - `"REFPROP_DONT_ESTIMATE_INTERACTION_PARAMETERS"` --  if `true`, if the binary
    ///       interaction parameters in `REFPROP` are estimated, throw an error rather than silently
    ///       continuing. _Default: `false`_
    ///     - `"REFPROP_IGNORE_ERROR_ESTIMATED_INTERACTION_PARAMETERS"` -- if `true`, if the binary
    ///       interaction parameters in `REFPROP` are unable to be estimated, silently continue
    ///       rather than failing. _Default: `false`
    ///     - `"REFPROP_USE_GERG"` -- if `true`, rather than using the highly-accurate pure fluid
    ///       equations of state, use the pure-fluid EOS from `GERG-2008`. _Default: `false`_
    ///     - `"REFPROP_USE_PENGROBINSON"` --  if `true`, rather than using the highly-accurate pure
    ///       fluid equations of state, use the Peng-Robinson EOS. _Default: `false`_
    ///
    /// - **Miscellaneous:**
    ///     - `"ALTERNATIVE_TABLES_DIRECTORY"` -- if provided, this path will be the root directory
    ///       for the tabular data. Otherwise, `${HOME}/.CoolProp/Tables` is used. _Default: `""`_
    ///     - `"FLOAT_PUNCTUATION"` -- the first character of this string will be used as the
    ///       separator between the number fraction. _Default: `"."`_
    ///     - `"LIST_STRING_DELIMITER"` -- the delimiter to be used when converting a list of
    ///       strings to a string. _Default: `","`_
    ///     - `"MAXIMUM_TABLE_DIRECTORY_SIZE_IN_GB"` -- the maximum allowed size of the directory
    ///       that is used to store tabular data. _Default: `1.0`_
    ///     - `"SAVE_RAW_TABLES"` -- if `true`, the raw, uncompressed tables will also be written to
    ///       file. _Default: `false`_
    ///     - `"VTPR_ALWAYS_RELOAD_LIBRARY"` -- if `true`, the library will always be reloaded, no
    ///       matter what is currently loaded. _Default: `false`_
    ///     - `"VTPR_UNIFAC_PATH"` -- the path to the directory containing the UNIFAC JSON files.
    ///       Should be slash terminated. _Default: `""`_
    ///
    /// # Errors
    ///
    /// Returns [`CoolPropError`](crate::native::CoolPropError)
    /// if the configuration key/value is invalid or the value could not be set.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rfluids::prelude::*;
    ///
    /// // Set configuration values of different types
    /// assert!(CoolProp::set_config("DONT_CHECK_PROPERTY_LIMITS", true).is_ok());
    /// assert!(CoolProp::set_config("PHASE_ENVELOPE_STARTING_PRESSURE_PA", 1e5).is_ok());
    /// assert!(CoolProp::set_config("ALTERNATIVE_REFPROP_PATH", "/path/to/refprop").is_ok());
    ///
    /// // Attempt to set invalid configuration values
    /// assert!(CoolProp::set_config("DONT_CHECK_PROPERTY_LIMITS", "yes").is_err());
    /// assert!(CoolProp::set_config("PHASE_ENVELOPE_STARTING_PRESSURE_PA", "1e5").is_err());
    /// assert!(CoolProp::set_config("ALTERNATIVE_REFPROP_PATH", true).is_err());
    ///
    /// // Attempt to set non-existing configuration key
    /// assert!(CoolProp::set_config("NON_EXISTING_KEY", "value").is_ok()); // CoolProp ignores nonexistent keys
    /// ```
    ///
    /// # Notes
    ///
    /// - Configuration variables can also be set via environment variables by prefixing the key
    ///   name with `COOLPROP_` _(e.g., `COOLPROP_DONT_CHECK_PROPERTY_LIMITS`)_
    /// - Configuration changes affect all subsequent `CoolProp` operations
    ///
    /// # See Also
    ///
    /// - [`CoolProp` Configuration](https://coolprop.org/coolprop/Configuration.html)
    /// - [`CoolPropLib.h` Reference](https://coolprop.org/_static/doxygen/html/_cool_prop_lib_8h.html)
    /// - [`ConfigValue`]
    pub fn set_config(key: impl AsRef<str>, value: impl Into<ConfigValue>) -> Result<()> {
        let key = key.as_ref().trim();
        let value = value.into();
        set_config(key, value)
    }
}

fn set_config(key: &str, value: ConfigValue) -> Result<()> {
    let lock = COOLPROP.lock().unwrap();
    match value {
        ConfigValue::Bool(val) => unsafe {
            lock.set_config_bool(const_ptr_c_char!(key), val);
        },
        ConfigValue::Float(val) => unsafe {
            lock.set_config_double(const_ptr_c_char!(key), val);
        },
        ConfigValue::String(val) => unsafe {
            lock.set_config_string(const_ptr_c_char!(key), const_ptr_c_char!(val.trim()));
        },
    }
    let error = get_error(&lock);
    error.map_or(Ok(()), Err)
}

/// `CoolProp` configuration value.
#[derive(Clone, Debug, PartialEq)]
pub enum ConfigValue {
    /// Boolean value.
    Bool(bool),
    /// Floating-point value.
    Float(f64),
    /// String value.
    String(String),
}

impl From<bool> for ConfigValue {
    fn from(value: bool) -> Self {
        ConfigValue::Bool(value)
    }
}
impl From<f64> for ConfigValue {
    fn from(value: f64) -> Self {
        ConfigValue::Float(value)
    }
}

impl From<String> for ConfigValue {
    fn from(value: String) -> Self {
        ConfigValue::String(value)
    }
}

impl From<&str> for ConfigValue {
    fn from(value: &str) -> Self {
        Self::from(value.to_string())
    }
}

#[cfg(test)]
mod tests {
    use rstest::*;

    use super::*;
    use crate::{io::GlobalParam::*, substance::Pure};

    #[test]
    fn get_debug_level() {
        // When
        let res = CoolProp::get_debug_level();

        // Then
        assert_eq!(res, MIN_DEBUG_LEVEL);
    }

    #[test]
    fn set_debug_level() {
        // When
        CoolProp::set_debug_level(MIN_DEBUG_LEVEL);
        let res = CoolProp::get_debug_level();

        // Then
        assert_eq!(res, MIN_DEBUG_LEVEL);
    }

    #[rstest]
    #[case(Version, true)]
    #[case(GitRevision, true)]
    #[case(HomePath, true)]
    #[case(PendingError, false)]
    #[case(PendingWarning, false)]
    #[case(PureList, true)]
    #[case(IncompPureList, true)]
    #[case(BinaryMixList, true)]
    #[case(PredefinedMixList, true)]
    #[case(CubicList, true)]
    #[case(MixBinaryPairsList, true)]
    #[case(ParamList, true)]
    #[case(CubicJsonSchema, true)]
    #[case(PcSaftJsonSchema, true)]
    #[case("version", true)]
    #[case("gitrevision", true)]
    #[case("HOME", true)]
    #[case("errstring", false)]
    #[case("warnstring", false)]
    #[case("fluids_list", true)]
    #[case("incompressible_list_pure", true)]
    #[case("incompressible_list_solution", true)]
    #[case("predefined_mixtures", true)]
    #[case("cubic_fluids_list", true)]
    #[case("mixture_binary_pairs_list", true)]
    #[case("parameter_list", true)]
    #[case("cubic_fluids_schema", true)]
    #[case("pcsaft_fluids_schema", true)]
    #[case("", false)]
    #[case(" ", false)]
    #[case("Hello, World!", false)]
    fn get_global_param(#[case] param: impl AsRef<str>, #[case] is_some: bool) {
        // When
        let res = CoolProp::get_global_param(param);

        // Then
        assert_eq!(res.is_some(), is_some);
    }

    #[rstest]
    #[case("aliases", true)]
    #[case("CAS", true)]
    #[case("ASHRAE34", true)]
    #[case("REFPROP_name", true)]
    #[case("BibTeX-EOS", true)]
    #[case("BibTeX-CP0", false)]
    #[case("BibTeX-CONDUCTIVITY", true)]
    #[case("BibTeX-MELTING_LINE", true)]
    #[case("BibTeX-SURFACE_TENSION", true)]
    #[case("BibTeX-VISCOSITY", true)]
    #[case("pure", true)]
    #[case("formula", true)]
    #[case("", false)]
    #[case(" ", false)]
    #[case("Hello, World!", false)]
    fn get_substance_param(#[case] param: &str, #[case] is_some: bool) {
        // Given
        let substance = Pure::Water;

        // When
        let res = CoolProp::get_substance_param(substance, param);

        // Then
        assert_eq!(res.is_some(), is_some);
    }
}
