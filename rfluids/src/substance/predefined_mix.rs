// cSpell:disable

#[cfg(test)]
use strum_macros::EnumIter;
use strum_macros::{AsRefStr, EnumString};

/// CoolProp predefined mixtures.
///
/// # Examples
///
/// Conversion between [`&str`](str):
///
/// ```
/// use std::str::FromStr;
/// use rfluids::substance::PredefinedMix;
///
/// assert_eq!(PredefinedMix::TypicalNaturalGas.as_ref(), "TypicalNaturalGas.mix");
/// assert_eq!(
///     PredefinedMix::from_str("TypicalNaturalGas.mix"),
///     Ok(PredefinedMix::TypicalNaturalGas)
/// );
/// assert_eq!(
///     PredefinedMix::from_str("NaturalGas"),
///     Ok(PredefinedMix::TypicalNaturalGas)
/// );
/// ```
///
/// # See also
///
/// - [Predefined mixtures](https://coolprop.github.io/CoolProp/coolprop/HighLevelAPI.html#predefined-mixtures)
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
