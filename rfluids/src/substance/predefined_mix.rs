// cSpell:disable

/// `CoolProp` predefined mixtures.
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
/// assert_eq!(PredefinedMix::R444A.as_ref(), "R444A.mix");
/// assert_eq!(PredefinedMix::from_str("R444A.mix"), Ok(PredefinedMix::R444A));
/// assert_eq!(PredefinedMix::from_str("R444A"), Ok(PredefinedMix::R444A));
/// ```
///
/// # See Also
///
/// - [Predefined Mixtures](https://coolprop.org/coolprop/HighLevelAPI.html#predefined-mixtures)
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    strum_macros::AsRefStr,
    strum_macros::EnumString,
    strum_macros::IntoStaticStr,
)]
#[strum(ascii_case_insensitive)]
#[cfg_attr(test, derive(strum_macros::EnumIter))]
pub enum PredefinedMix {
    /// Air mixture.
    #[strum(to_string = "Air.mix", serialize = "Air")]
    Air,

    /// Amarillo natural gas mixture.
    #[strum(to_string = "Amarillo.mix", serialize = "Amarillo")]
    Amarillo,

    /// Ekofisk natural gas mixture.
    #[strum(to_string = "Ekofisk.mix", serialize = "Ekofisk")]
    Ekofisk,

    /// Gulf Coast natural gas mixture.
    #[strum(to_string = "GulfCoast.mix", serialize = "GulfCoast")]
    GulfCoast,

    /// NIST Gulf Coast gas mixture.
    #[strum(to_string = "GulfCoastGas(NIST1).mix", serialize = "GulfCoastGasNIST")]
    GulfCoastGasNIST,

    /// High-carbon-dioxide natural gas mixture.
    #[strum(to_string = "HighCO2.mix", serialize = "HighCO2")]
    HighCO2,

    /// High-nitrogen natural gas mixture.
    #[strum(to_string = "HighN2.mix", serialize = "HighN2")]
    HighN2,

    /// Natural gas sample mixture.
    #[strum(to_string = "NaturalGasSample.mix", serialize = "NaturalGasSample")]
    NaturalGasSample,

    /// Refrigerant blend R401A.
    #[strum(to_string = "R401A.mix", serialize = "R401A")]
    R401A,

    /// Refrigerant blend R401B.
    #[strum(to_string = "R401B.mix", serialize = "R401B")]
    R401B,

    /// Refrigerant blend R401C.
    #[strum(to_string = "R401C.mix", serialize = "R401C")]
    R401C,

    /// Refrigerant blend R402A.
    #[strum(to_string = "R402A.mix", serialize = "R402A")]
    R402A,

    /// Refrigerant blend R402B.
    #[strum(to_string = "R402B.mix", serialize = "R402B")]
    R402B,

    /// Refrigerant blend R403A.
    #[strum(to_string = "R403A.mix", serialize = "R403A")]
    R403A,

    /// Refrigerant blend R403B.
    #[strum(to_string = "R403B.mix", serialize = "R403B")]
    R403B,

    /// Refrigerant blend R404A.
    #[strum(to_string = "R404A.mix", serialize = "R404A")]
    R404A,

    /// Refrigerant blend R405A.
    #[strum(to_string = "R405A.mix", serialize = "R405A")]
    R405A,

    /// Refrigerant blend R406A.
    #[strum(to_string = "R406A.mix", serialize = "R406A")]
    R406A,

    /// Refrigerant blend R407A.
    #[strum(to_string = "R407A.mix", serialize = "R407A")]
    R407A,

    /// Refrigerant blend R407B.
    #[strum(to_string = "R407B.mix", serialize = "R407B")]
    R407B,

    /// Refrigerant blend R407C.
    #[strum(to_string = "R407C.mix", serialize = "R407C")]
    R407C,

    /// Refrigerant blend R407D.
    #[strum(to_string = "R407D.mix", serialize = "R407D")]
    R407D,

    /// Refrigerant blend R407E.
    #[strum(to_string = "R407E.mix", serialize = "R407E")]
    R407E,

    /// Refrigerant blend R407F.
    #[strum(to_string = "R407F.mix", serialize = "R407F")]
    R407F,

    /// Refrigerant blend R407G.
    #[strum(to_string = "R407G.mix", serialize = "R407G")]
    R407G,

    /// Refrigerant blend R407H.
    #[strum(to_string = "R407H.mix", serialize = "R407H")]
    R407H,

    /// Refrigerant blend R407I.
    #[strum(to_string = "R407I.mix", serialize = "R407I")]
    R407I,

    /// Refrigerant blend R408A.
    #[strum(to_string = "R408A.mix", serialize = "R408A")]
    R408A,

    /// Refrigerant blend R409A.
    #[strum(to_string = "R409A.mix", serialize = "R409A")]
    R409A,

    /// Refrigerant blend R409B.
    #[strum(to_string = "R409B.mix", serialize = "R409B")]
    R409B,

    /// Refrigerant blend R410A.
    #[strum(to_string = "R410A.mix", serialize = "R410A")]
    R410A,

    /// Refrigerant blend R410B.
    #[strum(to_string = "R410B.mix", serialize = "R410B")]
    R410B,

    /// Refrigerant blend R411A.
    #[strum(to_string = "R411A.mix", serialize = "R411A")]
    R411A,

    /// Refrigerant blend R411B.
    #[strum(to_string = "R411B.mix", serialize = "R411B")]
    R411B,

    /// Refrigerant blend R412A.
    #[strum(to_string = "R412A.mix", serialize = "R412A")]
    R412A,

    /// Refrigerant blend R413A.
    #[strum(to_string = "R413A.mix", serialize = "R413A")]
    R413A,

    /// Refrigerant blend R414A.
    #[strum(to_string = "R414A.mix", serialize = "R414A")]
    R414A,

    /// Refrigerant blend R414B.
    #[strum(to_string = "R414B.mix", serialize = "R414B")]
    R414B,

    /// Refrigerant blend R415A.
    #[strum(to_string = "R415A.mix", serialize = "R415A")]
    R415A,

    /// Refrigerant blend R415B.
    #[strum(to_string = "R415B.mix", serialize = "R415B")]
    R415B,

    /// Refrigerant blend R416A.
    #[strum(to_string = "R416A.mix", serialize = "R416A")]
    R416A,

    /// Refrigerant blend R417A.
    #[strum(to_string = "R417A.mix", serialize = "R417A")]
    R417A,

    /// Refrigerant blend R417B.
    #[strum(to_string = "R417B.mix", serialize = "R417B")]
    R417B,

    /// Refrigerant blend R417C.
    #[strum(to_string = "R417C.mix", serialize = "R417C")]
    R417C,

    /// Refrigerant blend R418A.
    #[strum(to_string = "R418A.mix", serialize = "R418A")]
    R418A,

    /// Refrigerant blend R419A.
    #[strum(to_string = "R419A.mix", serialize = "R419A")]
    R419A,

    /// Refrigerant blend R419B.
    #[strum(to_string = "R419B.mix", serialize = "R419B")]
    R419B,

    /// Refrigerant blend R420A.
    #[strum(to_string = "R420A.mix", serialize = "R420A")]
    R420A,

    /// Refrigerant blend R421A.
    #[strum(to_string = "R421A.mix", serialize = "R421A")]
    R421A,

    /// Refrigerant blend R421B.
    #[strum(to_string = "R421B.mix", serialize = "R421B")]
    R421B,

    /// Refrigerant blend R422A.
    #[strum(to_string = "R422A.mix", serialize = "R422A")]
    R422A,

    /// Refrigerant blend R422B.
    #[strum(to_string = "R422B.mix", serialize = "R422B")]
    R422B,

    /// Refrigerant blend R422C.
    #[strum(to_string = "R422C.mix", serialize = "R422C")]
    R422C,

    /// Refrigerant blend R422D.
    #[strum(to_string = "R422D.mix", serialize = "R422D")]
    R422D,

    /// Refrigerant blend R422E.
    #[strum(to_string = "R422E.mix", serialize = "R422E")]
    R422E,

    /// Refrigerant blend R423A.
    #[strum(to_string = "R423A.mix", serialize = "R423A")]
    R423A,

    /// Refrigerant blend R424A.
    #[strum(to_string = "R424A.mix", serialize = "R424A")]
    R424A,

    /// Refrigerant blend R425A.
    #[strum(to_string = "R425A.mix", serialize = "R425A")]
    R425A,

    /// Refrigerant blend R426A.
    #[strum(to_string = "R426A.mix", serialize = "R426A")]
    R426A,

    /// Refrigerant blend R427A.
    #[strum(to_string = "R427A.mix", serialize = "R427A")]
    R427A,

    /// Refrigerant blend R427C.
    #[strum(to_string = "R427C.mix", serialize = "R427C")]
    R427C,

    /// Refrigerant blend R428A.
    #[strum(to_string = "R428A.mix", serialize = "R428A")]
    R428A,

    /// Refrigerant blend R429A.
    #[strum(to_string = "R429A.mix", serialize = "R429A")]
    R429A,

    /// Refrigerant blend R430A.
    #[strum(to_string = "R430A.mix", serialize = "R430A")]
    R430A,

    /// Refrigerant blend R431A.
    #[strum(to_string = "R431A.mix", serialize = "R431A")]
    R431A,

    /// Refrigerant blend R432A.
    #[strum(to_string = "R432A.mix", serialize = "R432A")]
    R432A,

    /// Refrigerant blend R433A.
    #[strum(to_string = "R433A.mix", serialize = "R433A")]
    R433A,

    /// Refrigerant blend R433B.
    #[strum(to_string = "R433B.mix", serialize = "R433B")]
    R433B,

    /// Refrigerant blend R433C.
    #[strum(to_string = "R433C.mix", serialize = "R433C")]
    R433C,

    /// Refrigerant blend R434A.
    #[strum(to_string = "R434A.mix", serialize = "R434A")]
    R434A,

    /// Refrigerant blend R435A.
    #[strum(to_string = "R435A.mix", serialize = "R435A")]
    R435A,

    /// Refrigerant blend R436A.
    #[strum(to_string = "R436A.mix", serialize = "R436A")]
    R436A,

    /// Refrigerant blend R436B.
    #[strum(to_string = "R436B.mix", serialize = "R436B")]
    R436B,

    /// Refrigerant blend R436C.
    #[strum(to_string = "R436C.mix", serialize = "R436C")]
    R436C,

    /// Refrigerant blend R437A.
    #[strum(to_string = "R437A.mix", serialize = "R437A")]
    R437A,

    /// Refrigerant blend R438A.
    #[strum(to_string = "R438A.mix", serialize = "R438A")]
    R438A,

    /// Refrigerant blend R439A.
    #[strum(to_string = "R439A.mix", serialize = "R439A")]
    R439A,

    /// Refrigerant blend R440A.
    #[strum(to_string = "R440A.mix", serialize = "R440A")]
    R440A,

    /// Refrigerant blend R441A.
    #[strum(to_string = "R441A.mix", serialize = "R441A")]
    R441A,

    /// Refrigerant blend R442A.
    #[strum(to_string = "R442A.mix", serialize = "R442A")]
    R442A,

    /// Refrigerant blend R443A.
    #[strum(to_string = "R443A.mix", serialize = "R443A")]
    R443A,

    /// Refrigerant blend R444A.
    #[strum(to_string = "R444A.mix", serialize = "R444A")]
    R444A,

    /// Refrigerant blend R444B.
    #[strum(to_string = "R444B.mix", serialize = "R444B")]
    R444B,

    /// Refrigerant blend R445A.
    #[strum(to_string = "R445A.mix", serialize = "R445A")]
    R445A,

    /// Refrigerant blend R446A.
    #[strum(to_string = "R446A.mix", serialize = "R446A")]
    R446A,

    /// Refrigerant blend R447A.
    #[strum(to_string = "R447A.mix", serialize = "R447A")]
    R447A,

    /// Refrigerant blend R447B.
    #[strum(to_string = "R447B.mix", serialize = "R447B")]
    R447B,

    /// Refrigerant blend R448A.
    #[strum(to_string = "R448A.mix", serialize = "R448A")]
    R448A,

    /// Refrigerant blend R448B.
    #[strum(to_string = "R448B.mix", serialize = "R448B")]
    R448B,

    /// Refrigerant blend R449A.
    #[strum(to_string = "R449A.mix", serialize = "R449A")]
    R449A,

    /// Refrigerant blend R449B.
    #[strum(to_string = "R449B.mix", serialize = "R449B")]
    R449B,

    /// Refrigerant blend R449C.
    #[strum(to_string = "R449C.mix", serialize = "R449C")]
    R449C,

    /// Refrigerant blend R450A.
    #[strum(to_string = "R450A.mix", serialize = "R450A")]
    R450A,

    /// Refrigerant blend R451A.
    #[strum(to_string = "R451A.mix", serialize = "R451A")]
    R451A,

    /// Refrigerant blend R451B.
    #[strum(to_string = "R451B.mix", serialize = "R451B")]
    R451B,

    /// Refrigerant blend R452A.
    #[strum(to_string = "R452A.mix", serialize = "R452A")]
    R452A,

    /// Refrigerant blend R452B.
    #[strum(to_string = "R452B.mix", serialize = "R452B")]
    R452B,

    /// Refrigerant blend R452C.
    #[strum(to_string = "R452C.mix", serialize = "R452C")]
    R452C,

    /// Refrigerant blend R453A.
    #[strum(to_string = "R453A.mix", serialize = "R453A")]
    R453A,

    /// Refrigerant blend R454A.
    #[strum(to_string = "R454A.mix", serialize = "R454A")]
    R454A,

    /// Refrigerant blend R454B.
    #[strum(to_string = "R454B.mix", serialize = "R454B")]
    R454B,

    /// Refrigerant blend R454C.
    #[strum(to_string = "R454C.mix", serialize = "R454C")]
    R454C,

    /// Refrigerant blend R455A.
    #[strum(to_string = "R455A.mix", serialize = "R455A")]
    R455A,

    /// Refrigerant blend R456A.
    #[strum(to_string = "R456A.mix", serialize = "R456A")]
    R456A,

    /// Refrigerant blend R457A.
    #[strum(to_string = "R457A.mix", serialize = "R457A")]
    R457A,

    /// Refrigerant blend R457B.
    #[strum(to_string = "R457B.mix", serialize = "R457B")]
    R457B,

    /// Refrigerant blend R457C.
    #[strum(to_string = "R457C.mix", serialize = "R457C")]
    R457C,

    /// Refrigerant blend R458A.
    #[strum(to_string = "R458A.mix", serialize = "R458A")]
    R458A,

    /// Refrigerant blend R459A.
    #[strum(to_string = "R459A.mix", serialize = "R459A")]
    R459A,

    /// Refrigerant blend R459B.
    #[strum(to_string = "R459B.mix", serialize = "R459B")]
    R459B,

    /// Refrigerant blend R460A.
    #[strum(to_string = "R460A.mix", serialize = "R460A")]
    R460A,

    /// Refrigerant blend R460B.
    #[strum(to_string = "R460B.mix", serialize = "R460B")]
    R460B,

    /// Refrigerant blend R460C.
    #[strum(to_string = "R460C.mix", serialize = "R460C")]
    R460C,

    /// Refrigerant blend R461A.
    #[strum(to_string = "R461A.mix", serialize = "R461A")]
    R461A,

    /// Refrigerant blend R462A.
    #[strum(to_string = "R462A.mix", serialize = "R462A")]
    R462A,

    /// Refrigerant blend R463A.
    #[strum(to_string = "R463A.mix", serialize = "R463A")]
    R463A,

    /// Refrigerant blend R464A.
    #[strum(to_string = "R464A.mix", serialize = "R464A")]
    R464A,

    /// Refrigerant blend R465A.
    #[strum(to_string = "R465A.mix", serialize = "R465A")]
    R465A,

    /// Refrigerant blend R466A.
    #[strum(to_string = "R466A.mix", serialize = "R466A")]
    R466A,

    /// Refrigerant blend R467A.
    #[strum(to_string = "R467A.mix", serialize = "R467A")]
    R467A,

    /// Refrigerant blend R468A.
    #[strum(to_string = "R468A.mix", serialize = "R468A")]
    R468A,

    /// Refrigerant blend R468B.
    #[strum(to_string = "R468B.mix", serialize = "R468B")]
    R468B,

    /// Refrigerant blend R468C.
    #[strum(to_string = "R468C.mix", serialize = "R468C")]
    R468C,

    /// Refrigerant blend R469A.
    #[strum(to_string = "R469A.mix", serialize = "R469A")]
    R469A,

    /// Refrigerant blend R470A.
    #[strum(to_string = "R470A.mix", serialize = "R470A")]
    R470A,

    /// Refrigerant blend R470B.
    #[strum(to_string = "R470B.mix", serialize = "R470B")]
    R470B,

    /// Refrigerant blend R471A.
    #[strum(to_string = "R471A.mix", serialize = "R471A")]
    R471A,

    /// Refrigerant blend R472A.
    #[strum(to_string = "R472A.mix", serialize = "R472A")]
    R472A,

    /// Refrigerant blend R472B.
    #[strum(to_string = "R472B.mix", serialize = "R472B")]
    R472B,

    /// Refrigerant blend R473A.
    #[strum(to_string = "R473A.mix", serialize = "R473A")]
    R473A,

    /// Refrigerant blend R474A.
    #[strum(to_string = "R474A.mix", serialize = "R474A")]
    R474A,

    /// Refrigerant blend R475A.
    #[strum(to_string = "R475A.mix", serialize = "R475A")]
    R475A,

    /// Refrigerant blend R476A.
    #[strum(to_string = "R476A.mix", serialize = "R476A")]
    R476A,

    /// Refrigerant blend R500.
    #[strum(to_string = "R500.mix", serialize = "R500")]
    R500,

    /// Refrigerant blend R501.
    #[strum(to_string = "R501.mix", serialize = "R501")]
    R501,

    /// Refrigerant blend R502.
    #[strum(to_string = "R502.mix", serialize = "R502")]
    R502,

    /// Refrigerant blend R503.
    #[strum(to_string = "R503.mix", serialize = "R503")]
    R503,

    /// Refrigerant blend R504.
    #[strum(to_string = "R504.mix", serialize = "R504")]
    R504,

    /// Refrigerant blend R507A.
    #[strum(to_string = "R507A.mix", serialize = "R507A")]
    R507A,

    /// Refrigerant blend R508A.
    #[strum(to_string = "R508A.mix", serialize = "R508A")]
    R508A,

    /// Refrigerant blend R508B.
    #[strum(to_string = "R508B.mix", serialize = "R508B")]
    R508B,

    /// Refrigerant blend R509A.
    #[strum(to_string = "R509A.mix", serialize = "R509A")]
    R509A,

    /// Refrigerant blend R510A.
    #[strum(to_string = "R510A.mix", serialize = "R510A")]
    R510A,

    /// Refrigerant blend R511A.
    #[strum(to_string = "R511A.mix", serialize = "R511A")]
    R511A,

    /// Refrigerant blend R512A.
    #[strum(to_string = "R512A.mix", serialize = "R512A")]
    R512A,

    /// Refrigerant blend R513A.
    #[strum(to_string = "R513A.mix", serialize = "R513A")]
    R513A,

    /// Typical natural gas mixture.
    #[strum(
        to_string = "TypicalNaturalGas.mix",
        serialize = "TypicalNaturalGas",
        serialize = "NaturalGas"
    )]
    TypicalNaturalGas,
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use rstest::*;

    use super::{PredefinedMix::*, *};

    #[rstest]
    #[case(Air, "Air.mix")]
    #[case(Amarillo, "Amarillo.mix")]
    #[case(Ekofisk, "Ekofisk.mix")]
    #[case(GulfCoast, "GulfCoast.mix")]
    #[case(GulfCoastGasNIST, "GulfCoastGas(NIST1).mix")]
    #[case(HighCO2, "HighCO2.mix")]
    #[case(HighN2, "HighN2.mix")]
    #[case(NaturalGasSample, "NaturalGasSample.mix")]
    #[case(R401A, "R401A.mix")]
    #[case(R401B, "R401B.mix")]
    #[case(R401C, "R401C.mix")]
    #[case(R402A, "R402A.mix")]
    #[case(R402B, "R402B.mix")]
    #[case(R403A, "R403A.mix")]
    #[case(R403B, "R403B.mix")]
    #[case(R404A, "R404A.mix")]
    #[case(R405A, "R405A.mix")]
    #[case(R406A, "R406A.mix")]
    #[case(R407A, "R407A.mix")]
    #[case(R407B, "R407B.mix")]
    #[case(R407C, "R407C.mix")]
    #[case(R407D, "R407D.mix")]
    #[case(R407E, "R407E.mix")]
    #[case(R407F, "R407F.mix")]
    #[case(R407G, "R407G.mix")]
    #[case(R407H, "R407H.mix")]
    #[case(R407I, "R407I.mix")]
    #[case(R408A, "R408A.mix")]
    #[case(R409A, "R409A.mix")]
    #[case(R409B, "R409B.mix")]
    #[case(R410A, "R410A.mix")]
    #[case(R410B, "R410B.mix")]
    #[case(R411A, "R411A.mix")]
    #[case(R411B, "R411B.mix")]
    #[case(R412A, "R412A.mix")]
    #[case(R413A, "R413A.mix")]
    #[case(R414A, "R414A.mix")]
    #[case(R414B, "R414B.mix")]
    #[case(R415A, "R415A.mix")]
    #[case(R415B, "R415B.mix")]
    #[case(R416A, "R416A.mix")]
    #[case(R417A, "R417A.mix")]
    #[case(R417B, "R417B.mix")]
    #[case(R417C, "R417C.mix")]
    #[case(R418A, "R418A.mix")]
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
    #[case(R424A, "R424A.mix")]
    #[case(R425A, "R425A.mix")]
    #[case(R426A, "R426A.mix")]
    #[case(R427A, "R427A.mix")]
    #[case(R427C, "R427C.mix")]
    #[case(R428A, "R428A.mix")]
    #[case(R429A, "R429A.mix")]
    #[case(R430A, "R430A.mix")]
    #[case(R431A, "R431A.mix")]
    #[case(R432A, "R432A.mix")]
    #[case(R433A, "R433A.mix")]
    #[case(R433B, "R433B.mix")]
    #[case(R433C, "R433C.mix")]
    #[case(R434A, "R434A.mix")]
    #[case(R435A, "R435A.mix")]
    #[case(R436A, "R436A.mix")]
    #[case(R436B, "R436B.mix")]
    #[case(R436C, "R436C.mix")]
    #[case(R437A, "R437A.mix")]
    #[case(R438A, "R438A.mix")]
    #[case(R439A, "R439A.mix")]
    #[case(R440A, "R440A.mix")]
    #[case(R441A, "R441A.mix")]
    #[case(R442A, "R442A.mix")]
    #[case(R443A, "R443A.mix")]
    #[case(R444A, "R444A.mix")]
    #[case(R444B, "R444B.mix")]
    #[case(R445A, "R445A.mix")]
    #[case(R446A, "R446A.mix")]
    #[case(R447A, "R447A.mix")]
    #[case(R447B, "R447B.mix")]
    #[case(R448A, "R448A.mix")]
    #[case(R448B, "R448B.mix")]
    #[case(R449A, "R449A.mix")]
    #[case(R449B, "R449B.mix")]
    #[case(R449C, "R449C.mix")]
    #[case(R450A, "R450A.mix")]
    #[case(R451A, "R451A.mix")]
    #[case(R451B, "R451B.mix")]
    #[case(R452A, "R452A.mix")]
    #[case(R452B, "R452B.mix")]
    #[case(R452C, "R452C.mix")]
    #[case(R453A, "R453A.mix")]
    #[case(R454A, "R454A.mix")]
    #[case(R454B, "R454B.mix")]
    #[case(R454C, "R454C.mix")]
    #[case(R455A, "R455A.mix")]
    #[case(R456A, "R456A.mix")]
    #[case(R457A, "R457A.mix")]
    #[case(R457B, "R457B.mix")]
    #[case(R457C, "R457C.mix")]
    #[case(R458A, "R458A.mix")]
    #[case(R459A, "R459A.mix")]
    #[case(R459B, "R459B.mix")]
    #[case(R460A, "R460A.mix")]
    #[case(R460B, "R460B.mix")]
    #[case(R460C, "R460C.mix")]
    #[case(R461A, "R461A.mix")]
    #[case(R462A, "R462A.mix")]
    #[case(R463A, "R463A.mix")]
    #[case(R464A, "R464A.mix")]
    #[case(R465A, "R465A.mix")]
    #[case(R466A, "R466A.mix")]
    #[case(R467A, "R467A.mix")]
    #[case(R468A, "R468A.mix")]
    #[case(R468B, "R468B.mix")]
    #[case(R468C, "R468C.mix")]
    #[case(R469A, "R469A.mix")]
    #[case(R470A, "R470A.mix")]
    #[case(R470B, "R470B.mix")]
    #[case(R471A, "R471A.mix")]
    #[case(R472A, "R472A.mix")]
    #[case(R472B, "R472B.mix")]
    #[case(R473A, "R473A.mix")]
    #[case(R474A, "R474A.mix")]
    #[case(R475A, "R475A.mix")]
    #[case(R476A, "R476A.mix")]
    #[case(R500, "R500.mix")]
    #[case(R501, "R501.mix")]
    #[case(R502, "R502.mix")]
    #[case(R503, "R503.mix")]
    #[case(R504, "R504.mix")]
    #[case(R507A, "R507A.mix")]
    #[case(R508A, "R508A.mix")]
    #[case(R508B, "R508B.mix")]
    #[case(R509A, "R509A.mix")]
    #[case(R510A, "R510A.mix")]
    #[case(R511A, "R511A.mix")]
    #[case(R512A, "R512A.mix")]
    #[case(R513A, "R513A.mix")]
    #[case(TypicalNaturalGas, "TypicalNaturalGas.mix")]
    fn as_str(#[case] sut: PredefinedMix, #[case] expected: &str) {
        // When
        let str = sut.as_ref();
        let static_str: &'static str = sut.into();

        // Then
        assert_eq!(str, expected);
        assert_eq!(static_str, expected);
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
    #[case(vec!["R401A.mix", "R401A"], R401A)]
    #[case(vec!["R401B.mix", "R401B"], R401B)]
    #[case(vec!["R401C.mix", "R401C"], R401C)]
    #[case(vec!["R402A.mix", "R402A"], R402A)]
    #[case(vec!["R402B.mix", "R402B"], R402B)]
    #[case(vec!["R403A.mix", "R403A"], R403A)]
    #[case(vec!["R403B.mix", "R403B"], R403B)]
    #[case(vec!["R404A.mix", "R404A"], R404A)]
    #[case(vec!["R405A.mix", "R405A"], R405A)]
    #[case(vec!["R406A.mix", "R406A"], R406A)]
    #[case(vec!["R407A.mix", "R407A"], R407A)]
    #[case(vec!["R407B.mix", "R407B"], R407B)]
    #[case(vec!["R407C.mix", "R407C"], R407C)]
    #[case(vec!["R407D.mix", "R407D"], R407D)]
    #[case(vec!["R407E.mix", "R407E"], R407E)]
    #[case(vec!["R407F.mix", "R407F"], R407F)]
    #[case(vec!["R407G.mix", "R407G"], R407G)]
    #[case(vec!["R407H.mix", "R407H"], R407H)]
    #[case(vec!["R407I.mix", "R407I"], R407I)]
    #[case(vec!["R408A.mix", "R408A"], R408A)]
    #[case(vec!["R409A.mix", "R409A"], R409A)]
    #[case(vec!["R409B.mix", "R409B"], R409B)]
    #[case(vec!["R410A.mix", "R410A"], R410A)]
    #[case(vec!["R410B.mix", "R410B"], R410B)]
    #[case(vec!["R411A.mix", "R411A"], R411A)]
    #[case(vec!["R411B.mix", "R411B"], R411B)]
    #[case(vec!["R412A.mix", "R412A"], R412A)]
    #[case(vec!["R413A.mix", "R413A"], R413A)]
    #[case(vec!["R414A.mix", "R414A"], R414A)]
    #[case(vec!["R414B.mix", "R414B"], R414B)]
    #[case(vec!["R415A.mix", "R415A"], R415A)]
    #[case(vec!["R415B.mix", "R415B"], R415B)]
    #[case(vec!["R416A.mix", "R416A"], R416A)]
    #[case(vec!["R417A.mix", "R417A"], R417A)]
    #[case(vec!["R417B.mix", "R417B"], R417B)]
    #[case(vec!["R417C.mix", "R417C"], R417C)]
    #[case(vec!["R418A.mix", "R418A"], R418A)]
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
    #[case(vec!["R424A.mix", "R424A"], R424A)]
    #[case(vec!["R425A.mix", "R425A"], R425A)]
    #[case(vec!["R426A.mix", "R426A"], R426A)]
    #[case(vec!["R427A.mix", "R427A"], R427A)]
    #[case(vec!["R427C.mix", "R427C"], R427C)]
    #[case(vec!["R428A.mix", "R428A"], R428A)]
    #[case(vec!["R429A.mix", "R429A"], R429A)]
    #[case(vec!["R430A.mix", "R430A"], R430A)]
    #[case(vec!["R431A.mix", "R431A"], R431A)]
    #[case(vec!["R432A.mix", "R432A"], R432A)]
    #[case(vec!["R433A.mix", "R433A"], R433A)]
    #[case(vec!["R433B.mix", "R433B"], R433B)]
    #[case(vec!["R433C.mix", "R433C"], R433C)]
    #[case(vec!["R434A.mix", "R434A"], R434A)]
    #[case(vec!["R435A.mix", "R435A"], R435A)]
    #[case(vec!["R436A.mix", "R436A"], R436A)]
    #[case(vec!["R436B.mix", "R436B"], R436B)]
    #[case(vec!["R436C.mix", "R436C"], R436C)]
    #[case(vec!["R437A.mix", "R437A"], R437A)]
    #[case(vec!["R438A.mix", "R438A"], R438A)]
    #[case(vec!["R439A.mix", "R439A"], R439A)]
    #[case(vec!["R440A.mix", "R440A"], R440A)]
    #[case(vec!["R441A.mix", "R441A"], R441A)]
    #[case(vec!["R442A.mix", "R442A"], R442A)]
    #[case(vec!["R443A.mix", "R443A"], R443A)]
    #[case(vec!["R444A.mix", "R444A"], R444A)]
    #[case(vec!["R444B.mix", "R444B"], R444B)]
    #[case(vec!["R445A.mix", "R445A"], R445A)]
    #[case(vec!["R446A.mix", "R446A"], R446A)]
    #[case(vec!["R447A.mix", "R447A"], R447A)]
    #[case(vec!["R447B.mix", "R447B"], R447B)]
    #[case(vec!["R448A.mix", "R448A"], R448A)]
    #[case(vec!["R448B.mix", "R448B"], R448B)]
    #[case(vec!["R449A.mix", "R449A"], R449A)]
    #[case(vec!["R449B.mix", "R449B"], R449B)]
    #[case(vec!["R449C.mix", "R449C"], R449C)]
    #[case(vec!["R450A.mix", "R450A"], R450A)]
    #[case(vec!["R451A.mix", "R451A"], R451A)]
    #[case(vec!["R451B.mix", "R451B"], R451B)]
    #[case(vec!["R452A.mix", "R452A"], R452A)]
    #[case(vec!["R452B.mix", "R452B"], R452B)]
    #[case(vec!["R452C.mix", "R452C"], R452C)]
    #[case(vec!["R453A.mix", "R453A"], R453A)]
    #[case(vec!["R454A.mix", "R454A"], R454A)]
    #[case(vec!["R454B.mix", "R454B"], R454B)]
    #[case(vec!["R454C.mix", "R454C"], R454C)]
    #[case(vec!["R455A.mix", "R455A"], R455A)]
    #[case(vec!["R456A.mix", "R456A"], R456A)]
    #[case(vec!["R457A.mix", "R457A"], R457A)]
    #[case(vec!["R457B.mix", "R457B"], R457B)]
    #[case(vec!["R457C.mix", "R457C"], R457C)]
    #[case(vec!["R458A.mix", "R458A"], R458A)]
    #[case(vec!["R459A.mix", "R459A"], R459A)]
    #[case(vec!["R459B.mix", "R459B"], R459B)]
    #[case(vec!["R460A.mix", "R460A"], R460A)]
    #[case(vec!["R460B.mix", "R460B"], R460B)]
    #[case(vec!["R460C.mix", "R460C"], R460C)]
    #[case(vec!["R461A.mix", "R461A"], R461A)]
    #[case(vec!["R462A.mix", "R462A"], R462A)]
    #[case(vec!["R463A.mix", "R463A"], R463A)]
    #[case(vec!["R464A.mix", "R464A"], R464A)]
    #[case(vec!["R465A.mix", "R465A"], R465A)]
    #[case(vec!["R466A.mix", "R466A"], R466A)]
    #[case(vec!["R467A.mix", "R467A"], R467A)]
    #[case(vec!["R468A.mix", "R468A"], R468A)]
    #[case(vec!["R468B.mix", "R468B"], R468B)]
    #[case(vec!["R468C.mix", "R468C"], R468C)]
    #[case(vec!["R469A.mix", "R469A"], R469A)]
    #[case(vec!["R470A.mix", "R470A"], R470A)]
    #[case(vec!["R470B.mix", "R470B"], R470B)]
    #[case(vec!["R471A.mix", "R471A"], R471A)]
    #[case(vec!["R472A.mix", "R472A"], R472A)]
    #[case(vec!["R472B.mix", "R472B"], R472B)]
    #[case(vec!["R473A.mix", "R473A"], R473A)]
    #[case(vec!["R474A.mix", "R474A"], R474A)]
    #[case(vec!["R475A.mix", "R475A"], R475A)]
    #[case(vec!["R476A.mix", "R476A"], R476A)]
    #[case(vec!["R500.mix", "R500"], R500)]
    #[case(vec!["R501.mix", "R501"], R501)]
    #[case(vec!["R502.mix", "R502"], R502)]
    #[case(vec!["R503.mix", "R503"], R503)]
    #[case(vec!["R504.mix", "R504"], R504)]
    #[case(vec!["R507A.mix", "R507A"], R507A)]
    #[case(vec!["R508A.mix", "R508A"], R508A)]
    #[case(vec!["R508B.mix", "R508B"], R508B)]
    #[case(vec!["R509A.mix", "R509A"], R509A)]
    #[case(vec!["R510A.mix", "R510A"], R510A)]
    #[case(vec!["R511A.mix", "R511A"], R511A)]
    #[case(vec!["R512A.mix", "R512A"], R512A)]
    #[case(vec!["R513A.mix", "R513A"], R513A)]
    #[case(vec!["TypicalNaturalGas.mix", "TypicalNaturalGas", "NaturalGas"], TypicalNaturalGas)]
    fn from_valid_str(#[case] valid: Vec<&str>, #[case] expected: PredefinedMix) {
        for s in valid {
            // When
            let res1 = PredefinedMix::from_str(s).unwrap();
            let res2 = PredefinedMix::try_from(s).unwrap();

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
        let res1 = PredefinedMix::from_str(invalid);
        let res2 = PredefinedMix::try_from(invalid);

        // Then
        assert!(res1.is_err());
        assert!(res2.is_err());
    }
}
