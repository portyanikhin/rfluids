use std::path::{Path, PathBuf};

/// `CoolProp` configuration keys.
///
/// # Examples
///
/// Conversion between [`&str`](str):
///
/// ```
/// use std::str::FromStr;
///
/// use rfluids::prelude::*;
///
/// assert_eq!(ConfigKey::EnableSuperancillaries.as_ref(), "ENABLE_SUPERANCILLARIES");
/// assert_eq!(
///     ConfigKey::from_str("ENABLE_SUPERANCILLARIES"),
///     Ok(ConfigKey::EnableSuperancillaries)
/// );
/// assert_eq!(
///     ConfigKey::try_from("EnableSuperancillaries"),
///     Ok(ConfigKey::EnableSuperancillaries)
/// );
/// ```
///
/// # See Also
///
/// - [CoolProp High-Level API](https://coolprop.org/coolprop/HighLevelAPI.html)
/// - [`CoolProp` Configuration](https://coolprop.org/coolprop/Configuration.html)
/// - [`CoolProp::set_config`](crate::native::CoolProp::set_config)
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    PartialEq,
    strum_macros::AsRefStr,
    strum_macros::EnumString,
    strum_macros::FromRepr,
    strum_macros::IntoStaticStr,
)]
#[strum(ascii_case_insensitive)]
pub enum ConfigKey {
    /// If `true`, evaluation of the stability of critical point will be skipped and point will be
    /// assumed to be stable.
    ///
    /// **Type:** [`bool`]
    ///
    /// **Default:** `false`
    #[strum(to_string = "ASSUME_CRITICAL_POINT_STABLE", serialize = "AssumeCriticalPointIsStable")]
    AssumeCriticalPointIsStable,

    /// If `true`, the critical splines will be used in the near-vicinity of the critical point.
    ///
    /// **Type:** [`bool`]
    ///
    /// **Default:** `true`
    #[strum(to_string = "CRITICAL_SPLINES_ENABLED", serialize = "CriticalSplinesEnabled")]
    CriticalSplinesEnabled,

    /// if `true`, any temperature within `1 uK` of the critical temperature will be considered to
    /// be **AT** the critical point.
    ///
    /// **Type:** [`bool`]
    ///
    /// **Default:** `true`
    #[strum(to_string = "CRITICAL_WITHIN_1UK", serialize = "CriticalWithin1Uk")]
    CriticalWithin1Uk,

    /// If `true`, when possible, `CoolProp` will skip checking whether values are inside the
    /// property limits.
    ///
    /// **Type:** [`bool`]
    ///
    /// **Default:** `false`
    #[strum(to_string = "DONT_CHECK_PROPERTY_LIMITS", serialize = "DontCheckPropLimits")]
    DontCheckPropLimits,

    /// if `true`, the superancillary functions will be used for VLE of pure fluids.
    ///
    /// **Type:** [`bool`]
    ///
    /// **Default:** `true`
    #[strum(to_string = "ENABLE_SUPERANCILLARIES", serialize = "EnableSuperancillaries")]
    EnableSuperancillaries,

    /// If `true`, when doing water-based mixture dewpoint calculations, use Henry’s Law to generate
    /// guesses for liquid-phase composition.
    ///
    /// **Type:** [`bool`]
    ///
    /// **Default:** `false`
    #[strum(
        to_string = "HENRYS_LAW_TO_GENERATE_VLE_GUESSES",
        serialize = "HenrysLawToGenerateVleGuesses"
    )]
    HenrysLawToGenerateVleGuesses,

    /// If `true`, for mixtures, the molar gas constant _(R)_ will be set to the CODATA value.
    ///
    /// **Type:** [`bool`]
    ///
    /// **Default:** `true`
    #[strum(to_string = "NORMALIZE_GAS_CONSTANTS", serialize = "NormalizeGasConstants")]
    NormalizeGasConstants,

    /// If `true`, and a pair of binary interaction pairs to be added is already there, rather than
    /// not adding the binary interaction pair _(and probably throwing an exception)_, overwrite it.
    ///
    /// **Type:** [`bool`]
    ///
    /// **Default:** `false`
    #[strum(to_string = "OVERWRITE_BINARY_INTERACTION", serialize = "OverwriteBinaryInteraction")]
    OverwriteBinaryInteraction,

    /// if `true`, and a departure function to be added is already there, rather than not adding the
    /// departure function _(and probably throwing an exception)_, overwrite it.
    ///
    /// **Type:** [`bool`]
    ///
    /// **Default:** `false`
    #[strum(to_string = "OVERWRITE_DEPARTURE_FUNCTION", serialize = "OverwriteDepartureFn")]
    OverwriteDepartureFn,

    /// If `true`, and a substance is added to the substances library that is already there, rather
    /// than not adding the substance _(and probably throwing an exception)_, overwrite it.
    ///
    /// **Type:** [`bool`]
    ///
    /// **Default:** `false`
    #[strum(to_string = "OVERWRITE_FLUIDS", serialize = "OverwriteSubstances")]
    OverwriteSubstances,

    /// Starting pressure in `Pa` for phase envelope construction.
    ///
    /// **Type:** [`f64`]
    ///
    /// **Default:** `100.0`
    #[strum(
        to_string = "PHASE_ENVELOPE_STARTING_PRESSURE_PA",
        serialize = "PhaseEnvelopeStartPressurePa"
    )]
    PhaseEnvelopeStartPressurePa,

    /// The value for the ideal gas constant in `J/mol/K` according to CODATA 2022. This value is
    /// used to harmonize all the ideal gas constants. This is especially important in the critical
    /// region.
    ///
    /// **Type:** [`f64`]
    ///
    /// **Default:** `8.314_462_618_153_24`
    #[strum(to_string = "R_U_CODATA", serialize = "RUCodata")]
    RUCodata,

    /// The minimal delta to be used in tracing out the spinodal; make sure that the EOS has a
    /// spinodal at this value of `delta=rho/rho_r`.
    ///
    /// **Type:** [`f64`]
    ///
    /// **Default:** `0.5`
    #[strum(to_string = "SPINODAL_MINIMUM_DELTA", serialize = "SpinodalMinDelta")]
    SpinodalMinDelta,

    /// If `true`, calls to the vectorized versions of `PropsSI` use the previous state as guess
    /// value while looping over the input vectors, only makes sense when working with a single
    /// fluid and with points that are not too far from each other.
    ///
    /// **Type:** [`bool`]
    ///
    /// **Default:** `false`
    #[strum(to_string = "USE_GUESSES_IN_PROPSSI", serialize = "UseGuessesInPropsSi")]
    UseGuessesInPropsSi,

    /// An alternative path to be provided to the directory that contains `REFPROP`’s fluids and
    /// mixtures directories. If provided, the `SETPATH` function will be called with this directory
    /// prior to calling any `REFPROP` functions.
    ///
    /// **Type:** [`Option<&Path>`](std::path::Path)
    ///
    /// **Default:** `None`
    #[strum(to_string = "ALTERNATIVE_REFPROP_PATH", serialize = "AltRefpropPath")]
    AltRefpropPath,

    /// An alternative path to the shared library file. If provided, it will be used to load
    /// `REFPROP`.
    ///
    /// **Type:** [`Option<&Path>`](std::path::Path)
    ///
    /// **Default:** `None`
    #[strum(to_string = "ALTERNATIVE_REFPROP_LIBRARY_PATH", serialize = "AltRefpropLibPath")]
    AltRefpropLibPath,

    /// An alternative path to the `HMX.BNC` file. If provided, it will be passed into `REFPROP`’s
    /// `SETUP` or `SETMIX` routines.
    ///
    /// **Type:** [`Option<&Path>`](std::path::Path)
    ///
    /// **Default:** `None`
    #[strum(to_string = "ALTERNATIVE_REFPROP_HMX_BNC_PATH", serialize = "AltRefpropHmxBncPath")]
    AltRefpropHmxBncPath,

    /// If `true`, if the binary interaction parameters in `REFPROP` are estimated, throw an error
    /// rather than silently continuing.
    ///
    /// **Type:** [`bool`]
    ///
    /// **Default:** `false`
    #[strum(
        to_string = "REFPROP_DONT_ESTIMATE_INTERACTION_PARAMETERS",
        serialize = "RefpropDontEstimateInteractionParams"
    )]
    RefpropDontEstimateInteractionParams,

    /// If `true`, if the binary interaction parameters in `REFPROP` are unable to be estimated,
    /// silently continue rather than failing.
    ///
    /// **Type:** [`bool`]
    ///
    /// **Default:** `false`
    #[strum(
        to_string = "REFPROP_IGNORE_ERROR_ESTIMATED_INTERACTION_PARAMETERS",
        serialize = "RefpropIgnoreErrorEstimatedInteractionParams"
    )]
    RefpropIgnoreErrorEstimatedInteractionParams,

    /// If `true`, rather than using the highly-accurate pure fluid equations of state, use the
    /// pure-fluid EOS from `GERG-2008`.
    ///
    /// **Type:** [`bool`]
    ///
    /// **Default:** `false`
    #[strum(to_string = "REFPROP_USE_GERG", serialize = "RefpropUseGerg")]
    RefpropUseGerg,

    /// If `true`, rather than using the highly-accurate pure fluid equations of state, use the
    /// Peng-Robinson EOS.
    ///
    /// **Type:** [`bool`]
    ///
    /// **Default:** `false`
    #[strum(to_string = "REFPROP_USE_PENGROBINSON", serialize = "RefpropUsePengRobinson")]
    RefpropUsePengRobinson,

    /// If provided, this path will be the root directory for the tabular data. Otherwise,
    /// `${HOME}/.CoolProp/Tables` is used.
    ///
    /// **Type:** [`Option<&Path>`](std::path::Path)
    ///
    /// **Default:** `None`
    #[strum(to_string = "ALTERNATIVE_TABLES_DIRECTORY", serialize = "AltTablesPath")]
    AltTablesPath,

    /// The delimiter to be used as the separator between the number fraction.
    ///
    /// **Type:** [`char`]
    ///
    /// **Default:** `'.'`
    #[strum(to_string = "FLOAT_PUNCTUATION", serialize = "FloatPunctuation")]
    FloatPunctuation,

    /// The delimiter to be used when converting a list of strings to a string.
    ///
    /// **Type:** [`char`]
    ///
    /// **Default:** `','`
    #[strum(to_string = "LIST_STRING_DELIMITER", serialize = "ListPunctuation")]
    ListPunctuation,

    /// The maximum allowed size of the directory that is used to store tabular data.
    ///
    /// **Type:** [`f64`]
    ///
    /// **Default:** `1.0`
    #[strum(to_string = "MAXIMUM_TABLE_DIRECTORY_SIZE_IN_GB", serialize = "MaxTableDirSizeInGb")]
    MaxTableDirSizeInGb,

    /// If `true`, the raw, uncompressed tables will also be written to file.
    ///
    /// **Type:** [`bool`]
    ///
    /// **Default:** `false`
    #[strum(to_string = "SAVE_RAW_TABLES", serialize = "SaveRawTables")]
    SaveRawTables,

    /// If `true`, the library will always be reloaded, no matter what is currently loaded.
    ///
    /// **Type:** [`bool`]
    ///
    /// **Default:** `false`
    #[strum(to_string = "VTPR_ALWAYS_RELOAD_LIBRARY", serialize = "VtPrAlwaysReloadLib")]
    VtPrAlwaysReloadLib,

    /// The path to the directory containing the UNIFAC JSON files. Should be slash terminated.
    ///
    /// **Type:** [`Option<&Path>`](std::path::Path)
    ///
    /// **Default:** `None`
    #[strum(to_string = "VTPR_UNIFAC_PATH", serialize = "VtPrUnifacPath")]
    VtPrUnifacPath,
}

/// `CoolProp` configuration value.
///
/// # See Also
///
/// - [CoolProp High-Level API](https://coolprop.org/coolprop/HighLevelAPI.html)
/// - [`CoolProp` Configuration](https://coolprop.org/coolprop/Configuration.html)
/// - [`CoolProp::set_config`](crate::native::CoolProp::set_config)
#[derive(Clone, Debug, PartialEq)]
pub enum ConfigValue<'a> {
    /// Boolean.
    Bool(bool),
    /// Floating-point.
    Float(f64),
    /// Character.
    Char(char),
    /// Optional path.
    OptionPath(Option<&'a Path>),
}

impl From<bool> for ConfigValue<'_> {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<&bool> for ConfigValue<'_> {
    fn from(value: &bool) -> Self {
        Self::Bool(*value)
    }
}

impl From<f64> for ConfigValue<'_> {
    fn from(value: f64) -> Self {
        Self::Float(value)
    }
}

impl From<&f64> for ConfigValue<'_> {
    fn from(value: &f64) -> Self {
        Self::Float(*value)
    }
}

impl From<char> for ConfigValue<'_> {
    fn from(value: char) -> Self {
        Self::Char(value)
    }
}

impl From<&char> for ConfigValue<'_> {
    fn from(value: &char) -> Self {
        Self::Char(*value)
    }
}

impl<'a> From<Option<&'a Path>> for ConfigValue<'a> {
    fn from(value: Option<&'a Path>) -> Self {
        Self::OptionPath(value)
    }
}

impl<'a> From<Option<&'a PathBuf>> for ConfigValue<'a> {
    fn from(value: Option<&'a PathBuf>) -> Self {
        Self::from(value.map(PathBuf::as_path))
    }
}

impl<'a> From<&'a Option<PathBuf>> for ConfigValue<'a> {
    fn from(value: &'a Option<PathBuf>) -> Self {
        Self::from(value.as_ref())
    }
}

impl<'a> From<&'a Path> for ConfigValue<'a> {
    fn from(value: &'a Path) -> Self {
        Self::from(Some(value))
    }
}

impl<'a> From<&'a PathBuf> for ConfigValue<'a> {
    fn from(value: &'a PathBuf) -> Self {
        Self::from(Some(value.as_path()))
    }
}

#[cfg(test)]
mod tests {
    use rstest::*;

    use super::*;

    mod key {
        use std::str::FromStr;

        use super::{ConfigKey::*, *};

        #[rstest]
        #[case(AssumeCriticalPointIsStable, "ASSUME_CRITICAL_POINT_STABLE")]
        #[case(CriticalSplinesEnabled, "CRITICAL_SPLINES_ENABLED")]
        #[case(CriticalWithin1Uk, "CRITICAL_WITHIN_1UK")]
        #[case(DontCheckPropLimits, "DONT_CHECK_PROPERTY_LIMITS")]
        #[case(EnableSuperancillaries, "ENABLE_SUPERANCILLARIES")]
        #[case(HenrysLawToGenerateVleGuesses, "HENRYS_LAW_TO_GENERATE_VLE_GUESSES")]
        #[case(NormalizeGasConstants, "NORMALIZE_GAS_CONSTANTS")]
        #[case(OverwriteBinaryInteraction, "OVERWRITE_BINARY_INTERACTION")]
        #[case(OverwriteDepartureFn, "OVERWRITE_DEPARTURE_FUNCTION")]
        #[case(OverwriteSubstances, "OVERWRITE_FLUIDS")]
        #[case(PhaseEnvelopeStartPressurePa, "PHASE_ENVELOPE_STARTING_PRESSURE_PA")]
        #[case(RUCodata, "R_U_CODATA")]
        #[case(SpinodalMinDelta, "SPINODAL_MINIMUM_DELTA")]
        #[case(UseGuessesInPropsSi, "USE_GUESSES_IN_PROPSSI")]
        #[case(AltRefpropPath, "ALTERNATIVE_REFPROP_PATH")]
        #[case(AltRefpropLibPath, "ALTERNATIVE_REFPROP_LIBRARY_PATH")]
        #[case(AltRefpropHmxBncPath, "ALTERNATIVE_REFPROP_HMX_BNC_PATH")]
        #[case(
            RefpropDontEstimateInteractionParams,
            "REFPROP_DONT_ESTIMATE_INTERACTION_PARAMETERS"
        )]
        #[case(
            RefpropIgnoreErrorEstimatedInteractionParams,
            "REFPROP_IGNORE_ERROR_ESTIMATED_INTERACTION_PARAMETERS"
        )]
        #[case(RefpropUseGerg, "REFPROP_USE_GERG")]
        #[case(RefpropUsePengRobinson, "REFPROP_USE_PENGROBINSON")]
        #[case(AltTablesPath, "ALTERNATIVE_TABLES_DIRECTORY")]
        #[case(FloatPunctuation, "FLOAT_PUNCTUATION")]
        #[case(ListPunctuation, "LIST_STRING_DELIMITER")]
        #[case(MaxTableDirSizeInGb, "MAXIMUM_TABLE_DIRECTORY_SIZE_IN_GB")]
        #[case(SaveRawTables, "SAVE_RAW_TABLES")]
        #[case(VtPrAlwaysReloadLib, "VTPR_ALWAYS_RELOAD_LIBRARY")]
        #[case(VtPrUnifacPath, "VTPR_UNIFAC_PATH")]
        fn as_str(#[case] sut: ConfigKey, #[case] expected: &str) {
            // When
            let str = sut.as_ref();
            let static_str: &'static str = sut.into();

            // Then
            assert_eq!(str, expected);
            assert_eq!(static_str, expected);
        }

        #[rstest]
        #[case(
            vec!["ASSUME_CRITICAL_POINT_STABLE", "AssumeCriticalPointIsStable"],
            AssumeCriticalPointIsStable
        )]
        #[case(vec!["CRITICAL_SPLINES_ENABLED", "CriticalSplinesEnabled"], CriticalSplinesEnabled)]
        #[case(vec!["CRITICAL_WITHIN_1UK", "CriticalWithin1Uk"], CriticalWithin1Uk)]
        #[case(vec!["DONT_CHECK_PROPERTY_LIMITS", "DontCheckPropLimits"], DontCheckPropLimits)]
        #[case(vec!["ENABLE_SUPERANCILLARIES", "EnableSuperancillaries"], EnableSuperancillaries)]
        #[case(
            vec!["HENRYS_LAW_TO_GENERATE_VLE_GUESSES", "HenrysLawToGenerateVleGuesses"],
            HenrysLawToGenerateVleGuesses
        )]
        #[case(vec!["NORMALIZE_GAS_CONSTANTS", "NormalizeGasConstants"], NormalizeGasConstants)]
        #[case(
            vec!["OVERWRITE_BINARY_INTERACTION", "OverwriteBinaryInteraction"],
            OverwriteBinaryInteraction
        )]
        #[case(vec!["OVERWRITE_DEPARTURE_FUNCTION", "OverwriteDepartureFn"], OverwriteDepartureFn)]
        #[case(vec!["OVERWRITE_FLUIDS", "OverwriteSubstances"], OverwriteSubstances)]
        #[case(
            vec!["PHASE_ENVELOPE_STARTING_PRESSURE_PA", "PhaseEnvelopeStartPressurePa"],
            PhaseEnvelopeStartPressurePa
        )]
        #[case(vec!["R_U_CODATA", "RUCodata"], RUCodata)]
        #[case(vec!["SPINODAL_MINIMUM_DELTA", "SpinodalMinDelta"], SpinodalMinDelta)]
        #[case(vec!["USE_GUESSES_IN_PROPSSI", "UseGuessesInPropsSi"], UseGuessesInPropsSi)]
        #[case(vec!["ALTERNATIVE_REFPROP_PATH", "AltRefpropPath"], AltRefpropPath)]
        #[case(vec!["ALTERNATIVE_REFPROP_LIBRARY_PATH", "AltRefpropLibPath"], AltRefpropLibPath)]
        #[case(vec!["ALTERNATIVE_REFPROP_HMX_BNC_PATH", "AltRefpropHmxBncPath"], AltRefpropHmxBncPath)]
        #[case(
            vec![
                "REFPROP_DONT_ESTIMATE_INTERACTION_PARAMETERS",
                "RefpropDontEstimateInteractionParams"
            ],
            RefpropDontEstimateInteractionParams
        )]
        #[case(
            vec![
                "REFPROP_IGNORE_ERROR_ESTIMATED_INTERACTION_PARAMETERS",
                "RefpropIgnoreErrorEstimatedInteractionParams"
            ],
            RefpropIgnoreErrorEstimatedInteractionParams
        )]
        #[case(vec!["REFPROP_USE_GERG", "RefpropUseGerg"], RefpropUseGerg)]
        #[case(vec!["REFPROP_USE_PENGROBINSON", "RefpropUsePengRobinson"], RefpropUsePengRobinson)]
        #[case(vec!["ALTERNATIVE_TABLES_DIRECTORY", "AltTablesPath"], AltTablesPath)]
        #[case(vec!["FLOAT_PUNCTUATION", "FloatPunctuation"], FloatPunctuation)]
        #[case(vec!["LIST_STRING_DELIMITER", "ListPunctuation"], ListPunctuation)]
        #[case(
            vec!["MAXIMUM_TABLE_DIRECTORY_SIZE_IN_GB", "MaxTableDirSizeInGb"],
            MaxTableDirSizeInGb
        )]
        #[case(vec!["SAVE_RAW_TABLES", "SaveRawTables"], SaveRawTables)]
        #[case(vec!["VTPR_ALWAYS_RELOAD_LIBRARY", "VtPrAlwaysReloadLib"], VtPrAlwaysReloadLib)]
        #[case(vec!["VTPR_UNIFAC_PATH", "VtPrUnifacPath"], VtPrUnifacPath)]
        fn from_valid_str<'a>(#[case] valid: Vec<&'a str>, #[case] expected: ConfigKey) {
            for s in valid {
                // When
                let res1 = ConfigKey::from_str(s).unwrap();
                let res2 = ConfigKey::try_from(s).unwrap();

                // Then
                assert_eq!(res1, expected);
                assert_eq!(res2, expected);
            }
        }

        #[rstest]
        #[case("")]
        #[case("Hello, World!")]
        fn from_invalid_str(#[case] invalid: &str) {
            // When
            let res1 = ConfigKey::from_str(invalid);
            let res2 = ConfigKey::try_from(invalid);

            // Then
            assert!(res1.is_err());
            assert!(res2.is_err());
        }
    }

    mod value {
        use super::*;

        #[test]
        fn from_bool() {
            // Given
            let value = true;

            // When
            let res = ConfigValue::from(value);

            // Then
            assert_eq!(res, ConfigValue::Bool(value));
        }

        #[test]
        fn from_bool_ref() {
            // Given
            let value = true;

            // When
            let res = ConfigValue::from(&value);

            // Then
            assert_eq!(res, ConfigValue::Bool(value));
        }

        #[test]
        fn from_float() {
            // Given
            let value = 42.0;

            // When
            let res = ConfigValue::from(value);

            // Then
            assert_eq!(res, ConfigValue::Float(value));
        }

        #[test]
        fn from_float_ref() {
            // Given
            let value = 42.0;

            // When
            let res = ConfigValue::from(&value);

            // Then
            assert_eq!(res, ConfigValue::Float(value));
        }

        #[test]
        fn from_char() {
            // Given
            let value = '.';

            // When
            let res = ConfigValue::from(value);

            // Then
            assert_eq!(res, ConfigValue::Char(value));
        }

        #[test]
        fn from_char_ref() {
            // Given
            let value = '.';

            // When
            let res = ConfigValue::from(&value);

            // Then
            assert_eq!(res, ConfigValue::Char(value));
        }

        #[test]
        fn from_path() {
            // Given
            let value = Path::new("foo/bar");

            // When
            let res = ConfigValue::from(value);

            // Then
            assert_eq!(res, ConfigValue::OptionPath(Some(value)));
        }

        #[test]
        fn from_path_buf_ref() {
            // Given
            let value = PathBuf::from("foo/bar");

            // When
            let res = ConfigValue::from(&value);

            // Then
            assert_eq!(res, ConfigValue::OptionPath(Some(value.as_path())));
        }

        #[rstest]
        #[case(None)]
        #[case(Some(Path::new("foo/bar")))]
        fn from_option_path(#[case] value: Option<&Path>) {
            // When
            let res = ConfigValue::from(value);

            // Then
            assert_eq!(res, ConfigValue::OptionPath(value));
        }

        #[rstest]
        #[case(None)]
        #[case(Some(PathBuf::from("foo/bar")))]
        fn from_option_path_buf_ref(#[case] value: Option<PathBuf>) {
            // Given
            let value = value.as_ref();

            // When
            let res = ConfigValue::from(value);

            // Then
            assert_eq!(res, ConfigValue::OptionPath(value.map(PathBuf::as_path)));
        }

        #[rstest]
        #[case(None)]
        #[case(Some(PathBuf::from("foo/bar")))]
        fn from_ref_to_option_path_buf(#[case] value: Option<PathBuf>) {
            // When
            let res = ConfigValue::from(&value);

            // Then
            assert_eq!(res, ConfigValue::OptionPath(value.as_deref()));
        }
    }
}
