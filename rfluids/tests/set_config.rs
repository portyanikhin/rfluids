use rfluids::prelude::{ConfigKey::*, *};
use rstest::*;

#[rstest]
#[case(AssumeCriticalPointIsStable)]
#[case(CriticalSplinesEnabled)]
#[case(CriticalWithin1Uk)]
#[case(DontCheckPropLimits)]
#[case(EnableSuperancillaries)]
#[case(HenrysLawToGenerateVleGuesses)]
#[case(NormalizeGasConstants)]
#[case(OverwriteBinaryInteraction)]
#[case(OverwriteDepartureFn)]
#[case(OverwriteSubstances)]
#[case(UseGuessesInPropsSi)]
#[case(RefpropDontEstimateInteractionParams)]
#[case(RefpropIgnoreErrorEstimatedInteractionParams)]
#[case(RefpropUseGerg)]
#[case(RefpropUsePengRobinson)]
#[case(SaveRawTables)]
#[case(VtprAlwaysReloadLib)]
#[case("ASSUME_CRITICAL_POINT_STABLE")]
#[case("CRITICAL_SPLINES_ENABLED")]
#[case("CRITICAL_WITHIN_1UK")]
#[case("DONT_CHECK_PROPERTY_LIMITS")]
#[case("ENABLE_SUPERANCILLARIES")]
#[case("HENRYS_LAW_TO_GENERATE_VLE_GUESSES")]
#[case("NORMALIZE_GAS_CONSTANTS")]
#[case("OVERWRITE_BINARY_INTERACTION")]
#[case("OVERWRITE_DEPARTURE_FUNCTION")]
#[case("OVERWRITE_FLUIDS")]
#[case("USE_GUESSES_IN_PROPSSI")]
#[case("REFPROP_DONT_ESTIMATE_INTERACTION_PARAMETERS")]
#[case("REFPROP_IGNORE_ERROR_ESTIMATED_INTERACTION_PARAMETERS")]
#[case("REFPROP_USE_GERG")]
#[case("REFPROP_USE_PENGROBINSON")]
#[case("SAVE_RAW_TABLES")]
#[case("VTPR_ALWAYS_RELOAD_LIBRARY")]
fn set_config_bool(#[case] key: impl AsRef<str> + Clone) {
    // Given
    let value = true;

    // When
    let res1 = CoolProp::set_config(key.clone(), value);
    let res2 = CoolProp::set_config(key, !value);

    // Then
    assert!(res1.is_ok());
    assert!(res2.is_ok());
}

#[rstest]
#[case(PhaseEnvelopeStartPressurePa)]
#[case(RUCodata)]
#[case(SpinodalMinDelta)]
#[case(MaxTableDirSizeInGb)]
#[case("PHASE_ENVELOPE_STARTING_PRESSURE_PA")]
#[case("R_U_CODATA")]
#[case("SPINODAL_MINIMUM_DELTA")]
#[case("MAXIMUM_TABLE_DIRECTORY_SIZE_IN_GB")]
fn set_config_float(#[case] key: impl AsRef<str>) {
    // Given
    let value = 42.0;

    // When
    let res = CoolProp::set_config(key, value);

    // Then
    assert!(res.is_ok());
}

#[rstest]
#[case(AltRefpropPath)]
#[case(AltRefpropLibPath)]
#[case(AltRefpropHmxBncPath)]
#[case(AltTablesPath)]
#[case(FloatPunctuation)]
#[case(ListPunctuation)]
#[case(VtprUnifacPath)]
#[case("ALTERNATIVE_REFPROP_PATH")]
#[case("ALTERNATIVE_REFPROP_LIBRARY_PATH")]
#[case("ALTERNATIVE_REFPROP_HMX_BNC_PATH")]
#[case("ALTERNATIVE_TABLES_DIRECTORY")]
#[case("FLOAT_PUNCTUATION")]
#[case("LIST_STRING_DELIMITER")]
#[case("VTPR_UNIFAC_PATH")]
fn set_config_string(#[case] key: impl AsRef<str>) {
    // Given
    let value = "something";

    // When
    let res = CoolProp::set_config(key, value);

    // Then
    assert!(res.is_ok());
}

#[rstest]
fn set_config_nonexistent_key(
    #[values("THIS_KEY_DOES_NOT_EXIST", "THIS_ONE_TOO", "AND_THIS_ONE_TOO")] key: &str,
    #[values(true, false, 42.0, "something")] value: impl Into<ConfigValue>,
) {
    // When
    let res = CoolProp::set_config(key, value);

    // Then
    assert!(res.is_ok()); // CoolProp ignores nonexistent keys
}
