/// `CoolProp` global parameters.
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
/// assert_eq!(GlobalParam::Version.as_ref(), "version");
/// assert_eq!(GlobalParam::from_str("version"), Ok(GlobalParam::Version));
/// assert_eq!(GlobalParam::try_from("VERSION"), Ok(GlobalParam::Version));
/// ```
///
/// # See Also
///
/// - [CoolProp High-Level API](https://coolprop.org/coolprop/HighLevelAPI.html)
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
#[non_exhaustive]
pub enum GlobalParam {
    /// `CoolProp` library version.
    #[strum(to_string = "version")]
    Version,

    /// `git` commit hash of the `CoolProp` build.
    #[strum(to_string = "gitrevision")]
    GitRevision,

    /// `CoolProp` home directory path.
    #[strum(to_string = "HOME", serialize = "HomePath")]
    HomePath,

    /// `REFPROP` version, if available.
    #[strum(to_string = "REFPROP_version", serialize = "RefpropVersion")]
    RefpropVersion,

    /// Pending error message.
    #[strum(to_string = "errstring", serialize = "PendingError")]
    PendingError,

    /// Pending warning message.
    #[strum(to_string = "warnstring", serialize = "PendingWarning")]
    PendingWarning,

    /// List of all available pure and pseudo-pure substances _(comma-separated)_.
    #[strum(to_string = "fluids_list", serialize = "FluidsList", serialize = "PureList")]
    PureList,

    /// List of all available pure incompressible substances _(comma-separated)_.
    #[strum(to_string = "incompressible_list_pure", serialize = "IncompPureList")]
    IncompPureList,

    /// List of all available incompressible binary mixtures _(comma-separated)_.
    #[strum(to_string = "incompressible_list_solution", serialize = "BinaryMixList")]
    BinaryMixList,

    /// List of all available predefined mixtures _(comma-separated)_.
    #[strum(to_string = "predefined_mixtures", serialize = "PredefinedMixList")]
    PredefinedMixList,

    /// List of all substances for which a cubic EOS is applicable _(comma-separated)_.
    #[strum(to_string = "cubic_fluids_list", serialize = "CubicList")]
    CubicList,

    /// List of binary pairs for mixtures _(comma-separated)_.
    #[strum(to_string = "mixture_binary_pairs_list", serialize = "MixBinaryPairsList")]
    MixBinaryPairsList,

    /// List of all available fluid parameters _(comma-separated)_.
    #[strum(to_string = "parameter_list", serialize = "ParamList")]
    ParamList,

    /// JSON schema for substances with cubic EOS
    /// ([`Srk`](crate::fluid::backend::BaseBackend::Srk),
    /// [`Pr`](crate::fluid::backend::BaseBackend::Pr), or
    /// [`VtPr`](crate::fluid::backend::BaseBackend::VtPr)).
    #[strum(to_string = "cubic_fluids_schema", serialize = "CubicJsonSchema")]
    CubicJsonSchema,

    /// JSON schema for substances with PC-SAFT EOS
    /// ([`PcSaft`](crate::fluid::backend::BaseBackend::PcSaft)).
    #[strum(to_string = "pcsaft_fluids_schema", serialize = "PcSaftJsonSchema")]
    PcSaftJsonSchema,
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use rstest::*;

    use super::{GlobalParam::*, *};

    #[rstest]
    #[case(Version, "version")]
    #[case(GitRevision, "gitrevision")]
    #[case(HomePath, "HOME")]
    #[case(RefpropVersion, "REFPROP_version")]
    #[case(PendingError, "errstring")]
    #[case(PendingWarning, "warnstring")]
    #[case(PureList, "fluids_list")]
    #[case(IncompPureList, "incompressible_list_pure")]
    #[case(BinaryMixList, "incompressible_list_solution")]
    #[case(PredefinedMixList, "predefined_mixtures")]
    #[case(CubicList, "cubic_fluids_list")]
    #[case(MixBinaryPairsList, "mixture_binary_pairs_list")]
    #[case(ParamList, "parameter_list")]
    #[case(CubicJsonSchema, "cubic_fluids_schema")]
    #[case(PcSaftJsonSchema, "pcsaft_fluids_schema")]
    fn as_str(#[case] sut: GlobalParam, #[case] expected: &str) {
        // When
        let str = sut.as_ref();
        let static_str: &'static str = sut.into();

        // Then
        assert_eq!(str, expected);
        assert_eq!(static_str, expected);
    }

    #[rstest]
    #[case(vec!["version", "Version"], Version)]
    #[case(vec!["gitrevision", "GitRevision"], GitRevision)]
    #[case(vec!["HOME", "HomePath"], HomePath)]
    #[case(vec!["REFPROP_version", "RefpropVersion"], RefpropVersion)]
    #[case(vec!["errstring", "PendingError"], PendingError)]
    #[case(vec!["warnstring", "PendingWarning"], PendingWarning)]
    #[case(vec!["fluids_list", "FluidsList", "PureList"], PureList)]
    #[case(vec!["incompressible_list_pure", "IncompPureList"], IncompPureList)]
    #[case(vec!["incompressible_list_solution", "BinaryMixList"], BinaryMixList)]
    #[case(vec!["predefined_mixtures", "PredefinedMixList"], PredefinedMixList)]
    #[case(vec!["cubic_fluids_list", "CubicList"], CubicList)]
    #[case(vec!["mixture_binary_pairs_list", "MixBinaryPairsList"], MixBinaryPairsList)]
    #[case(vec!["parameter_list", "ParamList"], ParamList)]
    #[case(vec!["cubic_fluids_schema", "CubicJsonSchema"], CubicJsonSchema)]
    #[case(vec!["pcsaft_fluids_schema", "PcSaftJsonSchema"], PcSaftJsonSchema)]
    fn from_valid_str<'a>(#[case] valid: Vec<&'a str>, #[case] expected: GlobalParam) {
        for s in valid {
            // When
            let res1 = GlobalParam::from_str(s).unwrap();
            let res2 = GlobalParam::try_from(s).unwrap();

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
        let res1 = GlobalParam::from_str(invalid);
        let res2 = GlobalParam::try_from(invalid);

        // Then
        assert!(res1.is_err());
        assert!(res2.is_err());
    }
}
