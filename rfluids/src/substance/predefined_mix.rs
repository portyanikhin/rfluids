// cSpell:disable

#[cfg(test)]
use strum_macros::EnumIter;
use strum_macros::{AsRefStr, EnumString};

/// `CoolProp` predefined mixtures.
///
/// # Examples
///
/// Conversion between [`&str`](str):
///
/// ```
/// use std::str::FromStr;
/// use rfluids::substance::PredefinedMix;
///
/// assert_eq!(PredefinedMix::R444A.as_ref(), "R444A.mix");
/// assert_eq!(PredefinedMix::from_str("R444A.mix"), Ok(PredefinedMix::R444A));
/// assert_eq!(PredefinedMix::from_str("R444A"), Ok(PredefinedMix::R444A));
/// ```
///
/// # See also
///
/// - [Predefined mixtures](https://coolprop.github.io/CoolProp/coolprop/HighLevelAPI.html#predefined-mixtures)
/// - [List of REFPROP-only refrigerant mixes which are not available in CoolProp yet](https://github.com/portyanikhin/rfluids/blob/main/rfluids/src/substance/refprop_refrigerants.txt)
#[derive(AsRefStr, EnumString, Debug, Copy, Clone, Eq, PartialEq)]
#[strum(ascii_case_insensitive)]
#[cfg_attr(test, derive(EnumIter))]
pub enum PredefinedMix {
    #[strum(to_string = "Air.mix", serialize = "Air")]
    Air,

    #[strum(to_string = "Amarillo.mix", serialize = "Amarillo")]
    Amarillo,

    #[strum(to_string = "Ekofisk.mix", serialize = "Ekofisk")]
    Ekofisk,

    #[strum(to_string = "GulfCoast.mix", serialize = "GulfCoast")]
    GulfCoast,

    #[strum(to_string = "GulfCoastGas(NIST1).mix", serialize = "GulfCoastGasNIST")]
    GulfCoastGasNIST,

    #[strum(to_string = "HighCO2.mix", serialize = "HighCO2")]
    HighCO2,

    #[strum(to_string = "HighN2.mix", serialize = "HighN2")]
    HighN2,

    #[strum(to_string = "NaturalGasSample.mix", serialize = "NaturalGasSample")]
    NaturalGasSample,

    #[strum(to_string = "R404A.mix", serialize = "R404A")]
    R404A,

    #[strum(to_string = "R407A.mix", serialize = "R407A")]
    R407A,

    #[strum(to_string = "R407B.mix", serialize = "R407B")]
    R407B,

    #[strum(to_string = "R407C.mix", serialize = "R407C")]
    R407C,

    #[strum(to_string = "R407D.mix", serialize = "R407D")]
    R407D,

    #[strum(to_string = "R407E.mix", serialize = "R407E")]
    R407E,

    #[strum(to_string = "R407F.mix", serialize = "R407F")]
    R407F,

    #[strum(to_string = "R410A.mix", serialize = "R410A")]
    R410A,

    #[strum(to_string = "R410B.mix", serialize = "R410B")]
    R410B,

    #[strum(to_string = "R411A.mix", serialize = "R411A")]
    R411A,

    #[strum(to_string = "R411B.mix", serialize = "R411B")]
    R411B,

    #[strum(to_string = "R415A.mix", serialize = "R415A")]
    R415A,

    #[strum(to_string = "R415B.mix", serialize = "R415B")]
    R415B,

    #[strum(to_string = "R417A.mix", serialize = "R417A")]
    R417A,

    #[strum(to_string = "R417B.mix", serialize = "R417B")]
    R417B,

    #[strum(to_string = "R417C.mix", serialize = "R417C")]
    R417C,

    #[strum(to_string = "R419A.mix", serialize = "R419A")]
    R419A,

    #[strum(to_string = "R419B.mix", serialize = "R419B")]
    R419B,

    #[strum(to_string = "R420A.mix", serialize = "R420A")]
    R420A,

    #[strum(to_string = "R421A.mix", serialize = "R421A")]
    R421A,

    #[strum(to_string = "R421B.mix", serialize = "R421B")]
    R421B,

    #[strum(to_string = "R422A.mix", serialize = "R422A")]
    R422A,

    #[strum(to_string = "R422B.mix", serialize = "R422B")]
    R422B,

    #[strum(to_string = "R422C.mix", serialize = "R422C")]
    R422C,

    #[strum(to_string = "R422D.mix", serialize = "R422D")]
    R422D,

    #[strum(to_string = "R422E.mix", serialize = "R422E")]
    R422E,

    #[strum(to_string = "R423A.mix", serialize = "R423A")]
    R423A,

    #[strum(to_string = "R425A.mix", serialize = "R425A")]
    R425A,

    #[strum(to_string = "R427A.mix", serialize = "R427A")]
    R427A,

    #[strum(to_string = "R428A.mix", serialize = "R428A")]
    R428A,

    #[strum(to_string = "R430A.mix", serialize = "R430A")]
    R430A,

    #[strum(to_string = "R431A.mix", serialize = "R431A")]
    R431A,

    #[strum(to_string = "R432A.mix", serialize = "R432A")]
    R432A,

    #[strum(to_string = "R433A.mix", serialize = "R433A")]
    R433A,

    #[strum(to_string = "R433B.mix", serialize = "R433B")]
    R433B,

    #[strum(to_string = "R433C.mix", serialize = "R433C")]
    R433C,

    #[strum(to_string = "R434A.mix", serialize = "R434A")]
    R434A,

    #[strum(to_string = "R436A.mix", serialize = "R436A")]
    R436A,

    #[strum(to_string = "R436B.mix", serialize = "R436B")]
    R436B,

    #[strum(to_string = "R439A.mix", serialize = "R439A")]
    R439A,

    #[strum(to_string = "R440A.mix", serialize = "R440A")]
    R440A,

    #[strum(to_string = "R441A.mix", serialize = "R441A")]
    R441A,

    #[strum(to_string = "R442A.mix", serialize = "R442A")]
    R442A,

    #[strum(to_string = "R443A.mix", serialize = "R443A")]
    R443A,

    #[strum(to_string = "R444A.mix", serialize = "R444A")]
    R444A,

    #[strum(to_string = "R444B.mix", serialize = "R444B")]
    R444B,

    #[strum(to_string = "R449A.mix", serialize = "R449A")]
    R449A,

    #[strum(to_string = "R449B.mix", serialize = "R449B")]
    R449B,

    #[strum(to_string = "R451A.mix", serialize = "R451A")]
    R451A,

    #[strum(to_string = "R451B.mix", serialize = "R451B")]
    R451B,

    #[strum(to_string = "R452A.mix", serialize = "R452A")]
    R452A,

    #[strum(to_string = "R454A.mix", serialize = "R454A")]
    R454A,

    #[strum(to_string = "R454B.mix", serialize = "R454B")]
    R454B,

    #[strum(to_string = "R500.mix", serialize = "R500")]
    R500,

    #[strum(to_string = "R501.mix", serialize = "R501")]
    R501,

    #[strum(to_string = "R502.mix", serialize = "R502")]
    R502,

    #[strum(to_string = "R503.mix", serialize = "R503")]
    R503,

    #[strum(to_string = "R507A.mix", serialize = "R507A")]
    R507A,

    #[strum(to_string = "R508A.mix", serialize = "R508A")]
    R508A,

    #[strum(to_string = "R508B.mix", serialize = "R508B")]
    R508B,

    #[strum(to_string = "R509A.mix", serialize = "R509A")]
    R509A,

    #[strum(to_string = "R510A.mix", serialize = "R510A")]
    R510A,

    #[strum(to_string = "R511A.mix", serialize = "R511A")]
    R511A,

    #[strum(to_string = "R512A.mix", serialize = "R512A")]
    R512A,

    #[strum(to_string = "R513A.mix", serialize = "R513A")]
    R513A,

    #[strum(
        to_string = "TypicalNaturalGas.mix",
        serialize = "TypicalNaturalGas",
        serialize = "NaturalGas"
    )]
    TypicalNaturalGas,
}

#[cfg(test)]
mod tests {
    use super::PredefinedMix::*;
    use super::*;
    use rstest::*;
    use std::str::FromStr;

    #[rstest]
    #[case(Air, "Air.mix")]
    #[case(Amarillo, "Amarillo.mix")]
    #[case(Ekofisk, "Ekofisk.mix")]
    #[case(GulfCoast, "GulfCoast.mix")]
    #[case(GulfCoastGasNIST, "GulfCoastGas(NIST1).mix")]
    #[case(HighCO2, "HighCO2.mix")]
    #[case(HighN2, "HighN2.mix")]
    #[case(NaturalGasSample, "NaturalGasSample.mix")]
    #[case(R404A, "R404A.mix")]
    #[case(R407A, "R407A.mix")]
    #[case(R407B, "R407B.mix")]
    #[case(R407C, "R407C.mix")]
    #[case(R407D, "R407D.mix")]
    #[case(R407E, "R407E.mix")]
    #[case(R407F, "R407F.mix")]
    #[case(R410A, "R410A.mix")]
    #[case(R410B, "R410B.mix")]
    #[case(R411A, "R411A.mix")]
    #[case(R411B, "R411B.mix")]
    #[case(R415A, "R415A.mix")]
    #[case(R415B, "R415B.mix")]
    #[case(R417A, "R417A.mix")]
    #[case(R417B, "R417B.mix")]
    #[case(R417C, "R417C.mix")]
    #[case(R419A, "R419A.mix")]
    #[case(R419B, "R419B.mix")]
    #[case(R420A, "R420A.mix")]
    #[case(R421A, "R421A.mix")]
    #[case(R421B, "R421B.mix")]
    #[case(R422A, "R422A.mix")]
    #[case(R422B, "R422B.mix")]
    #[case(R422C, "R422C.mix")]
    #[case(R422D, "R422D.mix")]
    #[case(R422E, "R422E.mix")]
    #[case(R423A, "R423A.mix")]
    #[case(R425A, "R425A.mix")]
    #[case(R427A, "R427A.mix")]
    #[case(R428A, "R428A.mix")]
    #[case(R430A, "R430A.mix")]
    #[case(R431A, "R431A.mix")]
    #[case(R432A, "R432A.mix")]
    #[case(R433A, "R433A.mix")]
    #[case(R433B, "R433B.mix")]
    #[case(R433C, "R433C.mix")]
    #[case(R434A, "R434A.mix")]
    #[case(R436A, "R436A.mix")]
    #[case(R436B, "R436B.mix")]
    #[case(R439A, "R439A.mix")]
    #[case(R440A, "R440A.mix")]
    #[case(R441A, "R441A.mix")]
    #[case(R442A, "R442A.mix")]
    #[case(R443A, "R443A.mix")]
    #[case(R444A, "R444A.mix")]
    #[case(R444B, "R444B.mix")]
    #[case(R449A, "R449A.mix")]
    #[case(R449B, "R449B.mix")]
    #[case(R451A, "R451A.mix")]
    #[case(R451B, "R451B.mix")]
    #[case(R452A, "R452A.mix")]
    #[case(R454A, "R454A.mix")]
    #[case(R454B, "R454B.mix")]
    #[case(R500, "R500.mix")]
    #[case(R501, "R501.mix")]
    #[case(R502, "R502.mix")]
    #[case(R503, "R503.mix")]
    #[case(R507A, "R507A.mix")]
    #[case(R508A, "R508A.mix")]
    #[case(R508B, "R508B.mix")]
    #[case(R509A, "R509A.mix")]
    #[case(R510A, "R510A.mix")]
    #[case(R511A, "R511A.mix")]
    #[case(R512A, "R512A.mix")]
    #[case(R513A, "R513A.mix")]
    #[case(TypicalNaturalGas, "TypicalNaturalGas.mix")]
    fn as_ref_returns_expected_str(#[case] substance: PredefinedMix, #[case] expected: &str) {
        assert_eq!(substance.as_ref(), expected);
    }

    #[rstest]
    #[case(vec!["Air.mix", "Air"], Air)]
    #[case(vec!["Amarillo.mix", "Amarillo"], Amarillo)]
    #[case(vec!["Ekofisk.mix", "Ekofisk"], Ekofisk)]
    #[case(vec!["GulfCoast.mix", "GulfCoast"], GulfCoast)]
    #[case(vec!["GulfCoastGas(NIST1).mix", "GulfCoastGasNIST"], GulfCoastGasNIST)]
    #[case(vec!["HighCO2.mix", "HighCO2"], HighCO2)]
    #[case(vec!["HighN2.mix", "HighN2"], HighN2)]
    #[case(vec!["NaturalGasSample.mix", "NaturalGasSample"], NaturalGasSample)]
    #[case(vec!["R404A.mix", "R404A"], R404A)]
    #[case(vec!["R407A.mix", "R407A"], R407A)]
    #[case(vec!["R407B.mix", "R407B"], R407B)]
    #[case(vec!["R407C.mix", "R407C"], R407C)]
    #[case(vec!["R407D.mix", "R407D"], R407D)]
    #[case(vec!["R407E.mix", "R407E"], R407E)]
    #[case(vec!["R407F.mix", "R407F"], R407F)]
    #[case(vec!["R410A.mix", "R410A"], R410A)]
    #[case(vec!["R410B.mix", "R410B"], R410B)]
    #[case(vec!["R411A.mix", "R411A"], R411A)]
    #[case(vec!["R411B.mix", "R411B"], R411B)]
    #[case(vec!["R415A.mix", "R415A"], R415A)]
    #[case(vec!["R415B.mix", "R415B"], R415B)]
    #[case(vec!["R417A.mix", "R417A"], R417A)]
    #[case(vec!["R417B.mix", "R417B"], R417B)]
    #[case(vec!["R417C.mix", "R417C"], R417C)]
    #[case(vec!["R419A.mix", "R419A"], R419A)]
    #[case(vec!["R419B.mix", "R419B"], R419B)]
    #[case(vec!["R420A.mix", "R420A"], R420A)]
    #[case(vec!["R421A.mix", "R421A"], R421A)]
    #[case(vec!["R421B.mix", "R421B"], R421B)]
    #[case(vec!["R422A.mix", "R422A"], R422A)]
    #[case(vec!["R422B.mix", "R422B"], R422B)]
    #[case(vec!["R422C.mix", "R422C"], R422C)]
    #[case(vec!["R422D.mix", "R422D"], R422D)]
    #[case(vec!["R422E.mix", "R422E"], R422E)]
    #[case(vec!["R423A.mix", "R423A"], R423A)]
    #[case(vec!["R425A.mix", "R425A"], R425A)]
    #[case(vec!["R427A.mix", "R427A"], R427A)]
    #[case(vec!["R428A.mix", "R428A"], R428A)]
    #[case(vec!["R430A.mix", "R430A"], R430A)]
    #[case(vec!["R431A.mix", "R431A"], R431A)]
    #[case(vec!["R432A.mix", "R432A"], R432A)]
    #[case(vec!["R433A.mix", "R433A"], R433A)]
    #[case(vec!["R433B.mix", "R433B"], R433B)]
    #[case(vec!["R433C.mix", "R433C"], R433C)]
    #[case(vec!["R434A.mix", "R434A"], R434A)]
    #[case(vec!["R436A.mix", "R436A"], R436A)]
    #[case(vec!["R436B.mix", "R436B"], R436B)]
    #[case(vec!["R439A.mix", "R439A"], R439A)]
    #[case(vec!["R440A.mix", "R440A"], R440A)]
    #[case(vec!["R441A.mix", "R441A"], R441A)]
    #[case(vec!["R442A.mix", "R442A"], R442A)]
    #[case(vec!["R443A.mix", "R443A"], R443A)]
    #[case(vec!["R444A.mix", "R444A"], R444A)]
    #[case(vec!["R444B.mix", "R444B"], R444B)]
    #[case(vec!["R449A.mix", "R449A"], R449A)]
    #[case(vec!["R449B.mix", "R449B"], R449B)]
    #[case(vec!["R451A.mix", "R451A"], R451A)]
    #[case(vec!["R451B.mix", "R451B"], R451B)]
    #[case(vec!["R452A.mix", "R452A"], R452A)]
    #[case(vec!["R454A.mix", "R454A"], R454A)]
    #[case(vec!["R454B.mix", "R454B"], R454B)]
    #[case(vec!["R500.mix", "R500"], R500)]
    #[case(vec!["R501.mix", "R501"], R501)]
    #[case(vec!["R502.mix", "R502"], R502)]
    #[case(vec!["R503.mix", "R503"], R503)]
    #[case(vec!["R507A.mix", "R507A"], R507A)]
    #[case(vec!["R508A.mix", "R508A"], R508A)]
    #[case(vec!["R508B.mix", "R508B"], R508B)]
    #[case(vec!["R509A.mix", "R509A"], R509A)]
    #[case(vec!["R510A.mix", "R510A"], R510A)]
    #[case(vec!["R511A.mix", "R511A"], R511A)]
    #[case(vec!["R512A.mix", "R512A"], R512A)]
    #[case(vec!["R513A.mix", "R513A"], R513A)]
    #[case(vec!["TypicalNaturalGas.mix", "TypicalNaturalGas", "NaturalGas"], TypicalNaturalGas)]
    fn from_valid_str_returns_ok(#[case] valid_values: Vec<&str>, #[case] expected: PredefinedMix) {
        for s in valid_values {
            assert_eq!(PredefinedMix::from_str(s), Ok(expected));
            assert_eq!(PredefinedMix::try_from(s), Ok(expected));
        }
    }

    #[rstest]
    #[case("")]
    #[case("Hello, World!")]
    fn from_invalid_str_returns_err(#[case] invalid_value: &str) {
        assert!(PredefinedMix::from_str(invalid_value).is_err());
        assert!(PredefinedMix::try_from(invalid_value).is_err());
    }
}
