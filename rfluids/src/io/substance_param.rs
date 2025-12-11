/// `CoolProp` substance parameters.
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
/// assert_eq!(SubstanceParam::Aliases.as_ref(), "aliases");
/// assert_eq!(SubstanceParam::from_str("aliases"), Ok(SubstanceParam::Aliases));
/// assert_eq!(SubstanceParam::try_from("ALIASES"), Ok(SubstanceParam::Aliases));
/// ```
///
/// # See Also
///
/// - [CoolProp High-Level API](https://coolprop.org/coolprop/HighLevelAPI.html)
/// - [Substance Information](https://coolprop.org/coolprop/HighLevelAPI.html#fluid-information)
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
pub enum SubstanceParam {
    /// List of aliases _(comma-separated)_.
    #[strum(to_string = "aliases")]
    Aliases,

    /// CAS number.
    #[strum(to_string = "CAS", serialize = "CAS_number")]
    Cas,

    /// ASHRAE Standard 34 safety rating.
    #[strum(to_string = "ASHRAE34")]
    Ashrae34,

    /// Name used in `REFPROP`.
    #[strum(to_string = "REFPROP_name", serialize = "RefpropName")]
    RefpropName,

    /// Equation of state BibTeX key.
    #[strum(to_string = "BibTeX-EOS", serialize = "BibTeX_EOS", serialize = "BibtexEos")]
    BibtexEos,

    /// Ideal gas heat capacity equation BibTeX key.
    #[strum(to_string = "BibTeX-CP0", serialize = "BibTeX_CP0", serialize = "BibtexCp0")]
    BibtexCp0,

    /// Thermal conductivity equation BibTeX key.
    #[strum(
        to_string = "BibTeX-CONDUCTIVITY",
        serialize = "BibTeX_CONDUCTIVITY",
        serialize = "BibtexConductivity"
    )]
    BibtexConductivity,

    /// Melting line equation BibTeX key.
    #[strum(
        to_string = "BibTeX-MELTING_LINE",
        serialize = "BibTeX_MELTING_LINE",
        serialize = "BibtexMeltingLine"
    )]
    BibtexMeltingLine,

    /// Surface tension equation BibTeX key.
    #[strum(
        to_string = "BibTeX-SURFACE_TENSION",
        serialize = "BibTeX_SURFACE_TENSION",
        serialize = "BibtexSurfaceTension"
    )]
    BibtexSurfaceTension,

    /// Viscosity equation BibTeX key.
    #[strum(
        to_string = "BibTeX-VISCOSITY",
        serialize = "BibTeX_VISCOSITY",
        serialize = "BibtexViscosity"
    )]
    BibtexViscosity,

    /// `"true"` if is pure, `"false"` otherwise.
    #[strum(to_string = "pure", serialize = "is_pure", serialize = "IsPure")]
    IsPure,

    /// Chemical formula in LaTeX form _(if available)_.
    #[strum(to_string = "formula")]
    Formula,
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use rstest::*;

    use super::{SubstanceParam::*, *};

    #[rstest]
    #[case(Aliases, "aliases")]
    #[case(Cas, "CAS")]
    #[case(Ashrae34, "ASHRAE34")]
    #[case(RefpropName, "REFPROP_name")]
    #[case(BibtexEos, "BibTeX-EOS")]
    #[case(BibtexCp0, "BibTeX-CP0")]
    #[case(BibtexConductivity, "BibTeX-CONDUCTIVITY")]
    #[case(BibtexMeltingLine, "BibTeX-MELTING_LINE")]
    #[case(BibtexSurfaceTension, "BibTeX-SURFACE_TENSION")]
    #[case(BibtexViscosity, "BibTeX-VISCOSITY")]
    #[case(IsPure, "pure")]
    #[case(Formula, "formula")]
    fn as_str(#[case] sut: SubstanceParam, #[case] expected: &str) {
        // When
        let str = sut.as_ref();
        let static_str: &'static str = sut.into();

        // Then
        assert_eq!(str, expected);
        assert_eq!(static_str, expected);
    }

    #[rstest]
    #[case(vec!["aliases", "Aliases"], Aliases)]
    #[case(vec!["CAS", "CAS_number", "Cas"], Cas)]
    #[case(vec!["ASHRAE34", "Ashrae34"], Ashrae34)]
    #[case(vec!["REFPROP_name", "RefpropName"], RefpropName)]
    #[case(vec!["BibTeX-EOS", "BibTeX_EOS", "BibtexEos"], BibtexEos)]
    #[case(vec!["BibTeX-CP0", "BibTeX_CP0", "BibtexCp0"], BibtexCp0)]
    #[case(
        vec!["BibTeX-CONDUCTIVITY", "BibTeX_CONDUCTIVITY", "BibtexConductivity"],
        BibtexConductivity
    )]
    #[case(
        vec!["BibTeX-MELTING_LINE", "BibTeX_MELTING_LINE", "BibtexMeltingLine"],
        BibtexMeltingLine
    )]
    #[case(
        vec!["BibTeX-SURFACE_TENSION", "BibTeX_SURFACE_TENSION", "BibtexSurfaceTension"],
        BibtexSurfaceTension
    )]
    #[case(vec!["BibTeX-VISCOSITY", "BibTeX_VISCOSITY", "BibtexViscosity"], BibtexViscosity)]
    #[case(vec!["pure", "is_pure", "IsPure"], IsPure)]
    #[case(vec!["formula", "Formula"], Formula)]
    fn from_valid_str<'a>(#[case] valid: Vec<&'a str>, #[case] expected: SubstanceParam) {
        for s in valid {
            // When
            let res1 = SubstanceParam::from_str(s).unwrap();
            let res2 = SubstanceParam::try_from(s).unwrap();

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
        let res1 = SubstanceParam::from_str(invalid);
        let res2 = SubstanceParam::try_from(invalid);

        // Then
        assert!(res1.is_err());
        assert!(res2.is_err());
    }
}
