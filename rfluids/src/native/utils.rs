use std::ffi::CString;

use coolprop_sys::COOLPROP;

use super::{
    CoolProp, Result,
    common::{MessageBuffer, get_error},
};
use crate::io::ConfigValue;

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
    /// # Examples
    ///
    /// ```
    /// use rfluids::prelude::*;
    ///
    /// let coolprop_version = CoolProp::get_global_param("version");
    /// assert!(coolprop_version.is_some());
    /// ```
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
        let param = CString::new(param).unwrap();
        let mut res = MessageBuffer::with_capacity(capacity);
        let lock = COOLPROP.lock().unwrap();
        let status = unsafe {
            lock.get_global_param_string(param.as_ptr(), res.as_mut_ptr(), res.capacity())
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
    ///   [`Substance::name`](crate::substance::Substance::name))_
    /// - `param` -- substance parameter key _(raw [`&str`](str) or
    ///   [`SubstanceParam`](crate::io::SubstanceParam))_
    ///
    /// Known parameter keys:
    ///
    /// - `"name"` -- substance name
    /// - `"aliases"` -- list of aliases for the substance _(comma-separated)_
    /// - `"REFPROP_name"` -- substance name used in `REFPROP`
    /// - `"CAS"` -- Chemical Abstracts Service (CAS) registry number
    /// - `"InChI"` -- International Chemical Identifier
    /// - `"InChIKey"` -- hashed version of the International Chemical Identifier
    /// - `"CHEMSPIDER_ID"` -- [`ChemSpider`](https://www.chemspider.com/) identifier
    /// - `"SMILES"` -- Simplified Molecular Input Line Entry System (SMILES) string
    /// - `"ASHRAE34"` -- ASHRAE Standard 34 safety rating
    /// - `"2DPNG_URL"` -- URL to a `2D` molecular structure image
    /// - `"BibTeX-XXX"` -- BibTeX key, where `XXX` is one of the following:
    ///     - `EOS` -- equation of state reference
    ///     - `CP0` -- ideal gas heat capacity equation reference
    ///     - `CONDUCTIVITY` -- thermal conductivity equation reference
    ///     - `MELTING_LINE` -- melting line equation reference
    ///     - `SURFACE_TENSION` -- surface tension equation reference
    ///     - `VISCOSITY` -- viscosity equation reference
    /// - `"pure"` -- `"true"` if the substance is pure, `"false"` otherwise
    /// - `"formula"` -- chemical formula of the substance in LaTeX form _(if available)_
    /// - `"JSON"` -- JSON representation of the substance properties and parameters
    ///
    /// # Examples
    ///
    /// ```
    /// use rfluids::prelude::*;
    ///
    /// let water_formula = CoolProp::get_substance_param("Water", "formula");
    /// assert_eq!(water_formula, Some("H_{2}O_{1}".to_string()));
    /// ```
    ///
    /// # See Also
    ///
    /// - [CoolProp High-Level API](https://coolprop.org/coolprop/HighLevelAPI.html)
    /// - [Substance Information](https://coolprop.org/coolprop/HighLevelAPI.html#fluid-information)
    /// - [`CoolPropLib.h` Reference](https://coolprop.org/_static/doxygen/html/_cool_prop_lib_8h.html)
    /// - [`Substance`](crate::substance::Substance)
    /// - [`SubstanceParam`](crate::io::SubstanceParam)
    pub fn get_substance_param(
        substance_name: impl AsRef<str>,
        param: impl AsRef<str>,
    ) -> Option<String> {
        let substance_name = substance_name.as_ref().trim();
        let param = param.as_ref().trim();
        let capacity = match param {
            "pure" => 6,
            "InChIKey" | "INCHI_Key" | "INCHIKEY" => 30,
            "name" | "REFPROP_name" | "REFPROPName" | "REFPROPname" | "CAS" | "CAS_number"
            | "ASHRAE34" => 100,
            "JSON" => 500_000,
            _ => 500,
        };
        let substance_name = CString::new(substance_name).unwrap();
        let param = CString::new(param).unwrap();
        let mut res = MessageBuffer::with_capacity(capacity);
        let lock = COOLPROP.lock().unwrap();
        let status = unsafe {
            lock.get_fluid_param_string(
                substance_name.as_ptr(),
                param.as_ptr(),
                res.as_mut_ptr(),
                res.capacity(),
            )
        };
        let res: String = res.into();
        if status != 1 || res.trim().is_empty() { None } else { Some(res) }
    }

    /// Sets a configuration key-value pair in `CoolProp`.
    ///
    /// Configuration keys control various aspects of `CoolProp` behavior (e.g., path settings,
    /// `REFPROP` integration, calculation behavior, etc.).
    ///
    /// The `value` argument can accept owned values ([`bool`], [`f64`], [`char`]) which are copied,
    /// or borrowed optional path references ([`&Path`](std::path::Path),
    /// [`&PathBuf`](std::path::PathBuf), [`Option<&Path>`](std::path::Path),
    /// [`Option<&PathBuf>`](std::path::PathBuf)) which must remain valid for the duration of
    /// the call.
    ///
    /// # Arguments
    ///
    /// - `key` -- configuration key _(raw [`&str`](str) or [`ConfigKey`](crate::io::ConfigKey))_
    /// - `value` -- configuration value _([`bool`], [`f64`], [`char`], [`&Path`](std::path::Path),
    ///   [`&PathBuf`](std::path::PathBuf), [`Option<&Path>`](std::path::Path), or
    ///   [`Option<&PathBuf>`](std::path::PathBuf), automatically converted to [`ConfigValue`])_
    ///
    /// Known configuration keys:
    ///
    /// - **Calculation Behavior:**
    ///     - `"ASSUME_CRITICAL_POINT_STABLE"` -- if `true`, evaluation of the stability of critical
    ///       point will be skipped and point will be assumed to be stable. **Default:** `false`
    ///     - `"CRITICAL_SPLINES_ENABLED"` -- if `true`, the critical splines will be used in the
    ///       near-vicinity of the critical point. **Default:** `true`
    ///     - `"CRITICAL_WITHIN_1UK"` -- if `true`, any temperature within `1 uK` of the critical
    ///       temperature will be considered to be **AT** the critical point. **Default:** `true`
    ///     - `"DONT_CHECK_PROPERTY_LIMITS"` -- if `true`, when possible, `CoolProp` will skip
    ///       checking whether values are inside the property limits. **Default:** `false`
    ///     - `"ENABLE_SUPERANCILLARIES"` -- if `true`, the superancillary functions will be used
    ///       for VLE of pure fluids. **Default:** `true`
    ///     - `"HENRYS_LAW_TO_GENERATE_VLE_GUESSES"` -- if `true`, when doing water-based mixture
    ///       dewpoint calculations, use Henry’s Law to generate guesses for liquid-phase
    ///       composition. **Default:** `false`
    ///     - `"NORMALIZE_GAS_CONSTANTS"` -- if `true`, for mixtures, the molar gas constant _(R)_
    ///       will be set to the CODATA value. **Default:** `true`
    ///     - `"OVERWRITE_BINARY_INTERACTION"` -- if `true`, and a pair of binary interaction pairs
    ///       to be added is already there, rather than not adding the binary interaction pair _(and
    ///       probably throwing an exception)_, overwrite it. **Default:** `false`
    ///     - `"OVERWRITE_DEPARTURE_FUNCTION"` -- if `true`, and a departure function to be added is
    ///       already there, rather than not adding the departure function _(and probably throwing
    ///       an exception)_, overwrite it. **Default:** `false`
    ///     - `"OVERWRITE_FLUIDS"` -- if `true`, and a fluid is added to the fluids library that is
    ///       already there, rather than not adding the fluid _(and probably throwing an
    ///       exception)_, overwrite it. **Default:** `false`
    ///     - `"PHASE_ENVELOPE_STARTING_PRESSURE_PA"` -- starting pressure in `Pa` for phase
    ///       envelope construction. **Default:** `100.0`
    ///     - `"R_U_CODATA"` -- the value for the ideal gas constant in `J/mol/K` according to
    ///       CODATA 2022. This value is used to harmonize all the ideal gas constants. This is
    ///       especially important in the critical region. **Default:** `8.314_462_618_153_24`
    ///     - `"SPINODAL_MINIMUM_DELTA"` -- the minimal delta to be used in tracing out the
    ///       spinodal; make sure that the EOS has a spinodal at this value of `delta=rho/rho_r`.
    ///       **Default:** `0.5`
    ///     - `"USE_GUESSES_IN_PROPSSI"` -- if `true`, calls to the vectorized versions of `PropsSI`
    ///       use the previous state as guess value while looping over the input vectors, only makes
    ///       sense when working with a single fluid and with points that are not too far from each
    ///       other. **Default:** `false`
    ///
    /// - **`REFPROP` Integration:**
    ///     - `"ALTERNATIVE_REFPROP_PATH"` -- an alternative path to be provided to the directory
    ///       that contains `REFPROP`’s fluids and mixtures directories. If provided, the `SETPATH`
    ///       function will be called with this directory prior to calling any `REFPROP` functions.
    ///       **Default:** `None`
    ///     - `"ALTERNATIVE_REFPROP_LIBRARY_PATH"` -- an alternative path to the shared library
    ///       file. If provided, it will be used to load `REFPROP`. **Default:** `None`
    ///     - `"ALTERNATIVE_REFPROP_HMX_BNC_PATH"` -- an alternative path to the `HMX.BNC` file. If
    ///       provided, it will be passed into `REFPROP`’s `SETUP` or `SETMIX` routines. _Default:
    ///       `None`
    ///     - `"REFPROP_DONT_ESTIMATE_INTERACTION_PARAMETERS"` --  if `true`, if the binary
    ///       interaction parameters in `REFPROP` are estimated, throw an error rather than silently
    ///       continuing. **Default:** `false`
    ///     - `"REFPROP_IGNORE_ERROR_ESTIMATED_INTERACTION_PARAMETERS"` -- if `true`, if the binary
    ///       interaction parameters in `REFPROP` are unable to be estimated, silently continue
    ///       rather than failing. **Default:** `false`
    ///     - `"REFPROP_USE_GERG"` -- if `true`, rather than using the highly-accurate pure fluid
    ///       equations of state, use the pure-fluid EOS from `GERG-2008`. **Default:** `false`
    ///     - `"REFPROP_USE_PENGROBINSON"` --  if `true`, rather than using the highly-accurate pure
    ///       fluid equations of state, use the Peng-Robinson EOS. **Default:** `false`
    ///
    /// - **Miscellaneous:**
    ///     - `"ALTERNATIVE_TABLES_DIRECTORY"` -- if provided, this path will be the root directory
    ///       for the tabular data. Otherwise, `${HOME}/.CoolProp/Tables` is used. **Default:**
    ///       `None`
    ///     - `"FLOAT_PUNCTUATION"` -- the delimiter to be used as the separator between the number
    ///       fraction. **Default:** `'.'`
    ///     - `"LIST_STRING_DELIMITER"` -- the delimiter to be used when converting a list of
    ///       strings to a string. **Default:** `','`
    ///     - `"MAXIMUM_TABLE_DIRECTORY_SIZE_IN_GB"` -- the maximum allowed size of the directory
    ///       that is used to store tabular data. **Default:** `1.0`
    ///     - `"SAVE_RAW_TABLES"` -- if `true`, the raw, uncompressed tables will also be written to
    ///       file. **Default:** `false`
    ///     - `"VTPR_ALWAYS_RELOAD_LIBRARY"` -- if `true`, the library will always be reloaded, no
    ///       matter what is currently loaded. **Default:** `false`
    ///     - `"VTPR_UNIFAC_PATH"` -- the path to the directory containing the UNIFAC JSON files.
    ///       Should be slash terminated. **Default:** `None`
    ///
    /// # Errors
    ///
    /// Returns [`CoolPropError`](crate::native::CoolPropError) if the configuration
    /// value is invalid.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::path::Path;
    ///
    /// use rfluids::prelude::*;
    ///
    /// // Set configuration values of different types
    /// assert!(CoolProp::set_config("DONT_CHECK_PROPERTY_LIMITS", true).is_ok());
    /// assert!(CoolProp::set_config("PHASE_ENVELOPE_STARTING_PRESSURE_PA", 1e5).is_ok());
    /// assert!(CoolProp::set_config("LIST_STRING_DELIMITER", ';').is_ok());
    /// assert!(
    ///     CoolProp::set_config("ALTERNATIVE_REFPROP_PATH", Path::new("/path/to/refprop")).is_ok()
    /// );
    ///
    /// // Attempt to set invalid configuration values
    /// assert!(CoolProp::set_config("DONT_CHECK_PROPERTY_LIMITS", 'y').is_err());
    /// assert!(CoolProp::set_config("PHASE_ENVELOPE_STARTING_PRESSURE_PA", '0').is_err());
    /// assert!(CoolProp::set_config("LIST_STRING_DELIMITER", 42.0).is_err());
    /// assert!(CoolProp::set_config("ALTERNATIVE_REFPROP_PATH", true).is_err());
    ///
    /// // Attempt to set non-existing configuration key.
    /// // CoolProp ignores nonexistent keys
    /// assert!(CoolProp::set_config("NON_EXISTING_KEY", false).is_ok());
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
    /// - [`ConfigKey`](crate::io::ConfigKey)
    /// - [`ConfigValue`]
    pub fn set_config<'a>(key: impl AsRef<str>, value: impl Into<ConfigValue<'a>>) -> Result<()> {
        let key = key.as_ref().trim();
        let value = value.into();
        set_config(key, &value)
    }
}

// Code coverage trick
fn set_config(key: &str, value: &ConfigValue) -> Result<()> {
    let key = CString::new(key).unwrap();
    let lock = COOLPROP.lock().unwrap();
    match value {
        ConfigValue::Bool(val) => unsafe {
            lock.set_config_bool(key.as_ptr(), *val);
        },
        ConfigValue::Float(val) => unsafe {
            lock.set_config_double(key.as_ptr(), *val);
        },
        ConfigValue::Char(val) => {
            let c_string = CString::new(val.to_string()).unwrap();
            unsafe {
                lock.set_config_string(key.as_ptr(), c_string.as_ptr());
            }
        }
        ConfigValue::OptionPath(val) => {
            let path = val.map_or_else(String::new, |p| p.to_string_lossy().into_owned());
            let c_string = CString::new(path).unwrap();
            unsafe {
                lock.set_config_string(key.as_ptr(), c_string.as_ptr());
            }
        }
    }
    let error = get_error(&lock);
    error.map_or(Ok(()), Err)
}

#[cfg(test)]
mod tests {
    use rstest::*;

    use super::*;
    use crate::{
        io::{GlobalParam::*, SubstanceParam::*},
        substance::Pure,
    };

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
    #[case(Name, true)]
    #[case(Aliases, true)]
    #[case(RefpropName, true)]
    #[case(Cas, true)]
    #[case(Inchi, true)]
    #[case(InchiKey, true)]
    #[case(ChemSpiderId, true)]
    #[case(Smiles, true)]
    #[case(Ashrae34, true)]
    #[case(TwoDPngUrl, true)]
    #[case(BibtexEos, true)]
    #[case(BibtexCp0, false)]
    #[case(BibtexConductivity, true)]
    #[case(BibtexMeltingLine, true)]
    #[case(BibtexSurfaceTension, true)]
    #[case(BibtexViscosity, true)]
    #[case(IsPure, true)]
    #[case(Formula, true)]
    #[case(Json, true)]
    #[case("name", true)]
    #[case("aliases", true)]
    #[case("REFPROP_name", true)]
    #[case("CAS", true)]
    #[case("InChI", true)]
    #[case("InChIKey", true)]
    #[case("CHEMSPIDER_ID", true)]
    #[case("SMILES", true)]
    #[case("ASHRAE34", true)]
    #[case("2DPNG_URL", true)]
    #[case("BibTeX-EOS", true)]
    #[case("BibTeX-CP0", false)]
    #[case("BibTeX-CONDUCTIVITY", true)]
    #[case("BibTeX-MELTING_LINE", true)]
    #[case("BibTeX-SURFACE_TENSION", true)]
    #[case("BibTeX-VISCOSITY", true)]
    #[case("pure", true)]
    #[case("formula", true)]
    #[case("JSON", true)]
    #[case("", false)]
    #[case(" ", false)]
    #[case("Hello, World!", false)]
    fn get_substance_param(#[case] param: impl AsRef<str>, #[case] is_some: bool) {
        // Given
        let substance = Pure::Water;

        // When
        let res = CoolProp::get_substance_param(substance, param);

        // Then
        assert_eq!(res.is_some(), is_some);
    }
}
