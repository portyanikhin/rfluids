use regex::Regex;
#[cfg(test)]
use strum_macros::EnumIter;
use strum_macros::{AsRefStr, EnumString};

/// CoolProp refrigerants.
///
/// # Examples
///
/// Conversion between [`&str`](str):
///
/// ```
/// use std::str::FromStr;
/// use rfluids::substance::Refrigerant;
///
/// assert_eq!(Refrigerant::R32.as_ref(), "R32");
/// assert_eq!(Refrigerant::from_str("R32"), Ok(Refrigerant::R32));
/// assert_eq!(Refrigerant::try_from("R32"), Ok(Refrigerant::R32));
/// ```
///
/// # See also
///
/// - [Pure and pseudo-pure substances](https://coolprop.github.io/CoolProp/fluid_properties/PurePseudoPure.html)
/// - [List of REFPROP-only refrigerants which are not available in CoolProp yet](https://github.com/portyanikhin/rfluids/blob/main/rfluids/src/substance/refprop_refrigerants.txt)
//noinspection SpellCheckingInspection
#[derive(AsRefStr, EnumString, Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[strum(ascii_case_insensitive)]
#[cfg_attr(test, derive(EnumIter))]
pub enum Refrigerant {
    #[strum(to_string = "R11")]
    R11,

    #[strum(to_string = "R12")]
    R12,

    #[strum(to_string = "R13")]
    R13,

    #[strum(to_string = "R13I1", serialize = "CF3I")]
    R13I1,

    #[strum(to_string = "R14")]
    R14,

    #[strum(to_string = "R21")]
    R21,

    #[strum(to_string = "R22")]
    R22,

    #[strum(to_string = "R23")]
    R23,

    #[strum(to_string = "R32")]
    R32,

    #[strum(to_string = "R40")]
    R40,

    #[strum(to_string = "R41")]
    R41,

    #[strum(to_string = "R50")]
    R50,

    #[strum(to_string = "R113")]
    R113,

    #[strum(to_string = "R114")]
    R114,

    #[strum(to_string = "R115")]
    R115,

    #[strum(to_string = "R116")]
    R116,

    #[strum(to_string = "R123")]
    R123,

    #[strum(to_string = "R124")]
    R124,

    #[strum(to_string = "R125")]
    R125,

    #[strum(to_string = "R134a")]
    R134a,

    #[strum(to_string = "R141b")]
    R141b,

    #[strum(to_string = "R142b")]
    R142b,

    #[strum(to_string = "R143a")]
    R143a,

    #[strum(to_string = "RE143a")]
    RE143a,

    #[strum(to_string = "R152a")]
    R152a,

    #[strum(to_string = "R161")]
    R161,

    #[strum(to_string = "R170")]
    R170,

    #[strum(to_string = "R218")]
    R218,

    #[strum(to_string = "R227ea")]
    R227ea,

    #[strum(to_string = "R236ea")]
    R236ea,

    #[strum(to_string = "R236fa")]
    R236fa,

    #[strum(to_string = "R245ca")]
    R245ca,

    #[strum(to_string = "R245fa")]
    R245fa,

    #[strum(to_string = "R290")]
    R290,

    #[strum(to_string = "RC318")]
    RC318,

    #[strum(to_string = "R365mfc")]
    R365mfc,

    #[strum(to_string = "R404A")]
    R404A,

    #[strum(
        to_string = "R404A.mix",
        serialize = "R404AMix",
        serialize = "R404A-mix"
    )]
    R404AMix,

    #[strum(to_string = "R407A.mix", serialize = "R407A")]
    R407A,

    #[strum(to_string = "R407B.mix", serialize = "R407B")]
    R407B,

    #[strum(to_string = "R407C")]
    R407C,

    #[strum(
        to_string = "R407C.mix",
        serialize = "R407CMix",
        serialize = "R407C-mix"
    )]
    R407CMix,

    #[strum(to_string = "R407D.mix", serialize = "R407D")]
    R407D,

    #[strum(to_string = "R407E.mix", serialize = "R407E")]
    R407E,

    #[strum(to_string = "R407F.mix", serialize = "R407F")]
    R407F,

    #[strum(to_string = "R410A")]
    R410A,

    #[strum(
        to_string = "R410A.mix",
        serialize = "R410AMix",
        serialize = "R410A-mix"
    )]
    R410AMix,

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

    #[strum(to_string = "R507A")]
    R507A,

    #[strum(
        to_string = "R507A.mix",
        serialize = "R507AMix",
        serialize = "R507A-mix"
    )]
    R507AMix,

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

    #[strum(to_string = "R600")]
    R600,

    #[strum(to_string = "R600a")]
    R600a,

    #[strum(to_string = "R601")]
    R601,

    #[strum(to_string = "R601a")]
    R601a,

    #[strum(to_string = "R702")]
    R702,

    #[strum(to_string = "R704")]
    R704,

    #[strum(to_string = "R717")]
    R717,

    #[strum(to_string = "R718")]
    R718,

    #[strum(to_string = "R720")]
    R720,

    #[strum(to_string = "R728")]
    R728,

    #[strum(to_string = "R729")]
    R729,

    #[strum(to_string = "R732")]
    R732,

    #[strum(to_string = "R740")]
    R740,

    #[strum(to_string = "R744")]
    R744,

    #[strum(to_string = "SulfurDioxide", serialize = "R764")]
    R764,

    #[strum(to_string = "SulfurHexafluoride", serialize = "R846")]
    R846,

    #[strum(to_string = "R1150")]
    R1150,

    #[strum(to_string = "R1233zd(E)", serialize = "R1233zdE")]
    R1233zdE,

    #[strum(to_string = "R1234yf")]
    R1234yf,

    #[strum(to_string = "R1234ze(E)", serialize = "R1234zeE")]
    R1234zeE,

    #[strum(to_string = "R1234ze(Z)", serialize = "R1234zeZ")]
    R1234zeZ,

    #[strum(to_string = "R1243zf")]
    R1243zf,

    #[strum(to_string = "R1270")]
    R1270,
}

impl Refrigerant {
    /// Category.
    ///
    /// # Examples
    ///
    /// ```
    /// use rfluids::substance::{Refrigerant, RefrigerantCategory};
    ///
    /// assert_eq!(Refrigerant::R32.category(), RefrigerantCategory::Pure);
    /// assert_eq!(Refrigerant::R407C.category(), RefrigerantCategory::ZeotropicMix);
    /// assert_eq!(Refrigerant::R507A.category(), RefrigerantCategory::AzeotropicMix);
    /// ```
    pub fn category(&self) -> RefrigerantCategory {
        let zeotropic = Regex::new(r"^R4\d{2}").unwrap();
        let azeotropic = Regex::new(r"^R5\d{2}").unwrap();
        match self.as_ref() {
            s if zeotropic.is_match(s) => RefrigerantCategory::ZeotropicMix,
            s if azeotropic.is_match(s) => RefrigerantCategory::AzeotropicMix,
            _ => RefrigerantCategory::Pure,
        }
    }
}

/// [`Refrigerant`]s categories.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum RefrigerantCategory {
    /// Pure substance.
    Pure,

    /// Zeotropic mixture _(with a temperature glide)_.
    ZeotropicMix,

    /// Azeotropic mixture _(without a temperature glide)_.
    AzeotropicMix,
}

#[cfg(test)]
mod tests {
    use super::Refrigerant::*;
    use super::RefrigerantCategory::*;
    use super::*;
    use rstest::*;
    use std::str::FromStr;

    #[rstest]
    #[case(R11, Pure)]
    #[case(R12, Pure)]
    #[case(R13, Pure)]
    #[case(R13I1, Pure)]
    #[case(R14, Pure)]
    #[case(R21, Pure)]
    #[case(R22, Pure)]
    #[case(R23, Pure)]
    #[case(R32, Pure)]
    #[case(R40, Pure)]
    #[case(R41, Pure)]
    #[case(R50, Pure)]
    #[case(R113, Pure)]
    #[case(R114, Pure)]
    #[case(R115, Pure)]
    #[case(R116, Pure)]
    #[case(R123, Pure)]
    #[case(R124, Pure)]
    #[case(R125, Pure)]
    #[case(R134a, Pure)]
    #[case(R141b, Pure)]
    #[case(R142b, Pure)]
    #[case(R143a, Pure)]
    #[case(RE143a, Pure)]
    #[case(R152a, Pure)]
    #[case(R161, Pure)]
    #[case(R170, Pure)]
    #[case(R218, Pure)]
    #[case(R227ea, Pure)]
    #[case(R236ea, Pure)]
    #[case(R236fa, Pure)]
    #[case(R245ca, Pure)]
    #[case(R245fa, Pure)]
    #[case(R290, Pure)]
    #[case(RC318, Pure)]
    #[case(R365mfc, Pure)]
    #[case(R404A, ZeotropicMix)]
    #[case(R404AMix, ZeotropicMix)]
    #[case(R407A, ZeotropicMix)]
    #[case(R407B, ZeotropicMix)]
    #[case(R407C, ZeotropicMix)]
    #[case(R407CMix, ZeotropicMix)]
    #[case(R407D, ZeotropicMix)]
    #[case(R407E, ZeotropicMix)]
    #[case(R407F, ZeotropicMix)]
    #[case(R410A, ZeotropicMix)]
    #[case(R410AMix, ZeotropicMix)]
    #[case(R410B, ZeotropicMix)]
    #[case(R411A, ZeotropicMix)]
    #[case(R411B, ZeotropicMix)]
    #[case(R415A, ZeotropicMix)]
    #[case(R415B, ZeotropicMix)]
    #[case(R417A, ZeotropicMix)]
    #[case(R417B, ZeotropicMix)]
    #[case(R417C, ZeotropicMix)]
    #[case(R419A, ZeotropicMix)]
    #[case(R419B, ZeotropicMix)]
    #[case(R420A, ZeotropicMix)]
    #[case(R421A, ZeotropicMix)]
    #[case(R421B, ZeotropicMix)]
    #[case(R422A, ZeotropicMix)]
    #[case(R422B, ZeotropicMix)]
    #[case(R422C, ZeotropicMix)]
    #[case(R422D, ZeotropicMix)]
    #[case(R422E, ZeotropicMix)]
    #[case(R423A, ZeotropicMix)]
    #[case(R425A, ZeotropicMix)]
    #[case(R427A, ZeotropicMix)]
    #[case(R428A, ZeotropicMix)]
    #[case(R430A, ZeotropicMix)]
    #[case(R431A, ZeotropicMix)]
    #[case(R432A, ZeotropicMix)]
    #[case(R433A, ZeotropicMix)]
    #[case(R433B, ZeotropicMix)]
    #[case(R433C, ZeotropicMix)]
    #[case(R434A, ZeotropicMix)]
    #[case(R436A, ZeotropicMix)]
    #[case(R436B, ZeotropicMix)]
    #[case(R439A, ZeotropicMix)]
    #[case(R440A, ZeotropicMix)]
    #[case(R441A, ZeotropicMix)]
    #[case(R442A, ZeotropicMix)]
    #[case(R443A, ZeotropicMix)]
    #[case(R444A, ZeotropicMix)]
    #[case(R444B, ZeotropicMix)]
    #[case(R449A, ZeotropicMix)]
    #[case(R449B, ZeotropicMix)]
    #[case(R451A, ZeotropicMix)]
    #[case(R451B, ZeotropicMix)]
    #[case(R452A, ZeotropicMix)]
    #[case(R454A, ZeotropicMix)]
    #[case(R454B, ZeotropicMix)]
    #[case(R500, AzeotropicMix)]
    #[case(R501, AzeotropicMix)]
    #[case(R502, AzeotropicMix)]
    #[case(R503, AzeotropicMix)]
    #[case(R507A, AzeotropicMix)]
    #[case(R507AMix, AzeotropicMix)]
    #[case(R508A, AzeotropicMix)]
    #[case(R508B, AzeotropicMix)]
    #[case(R509A, AzeotropicMix)]
    #[case(R510A, AzeotropicMix)]
    #[case(R511A, AzeotropicMix)]
    #[case(R512A, AzeotropicMix)]
    #[case(R513A, AzeotropicMix)]
    #[case(R600, Pure)]
    #[case(R600a, Pure)]
    #[case(R601, Pure)]
    #[case(R601a, Pure)]
    #[case(R702, Pure)]
    #[case(R704, Pure)]
    #[case(R717, Pure)]
    #[case(R718, Pure)]
    #[case(R720, Pure)]
    #[case(R728, Pure)]
    #[case(R729, Pure)]
    #[case(R732, Pure)]
    #[case(R740, Pure)]
    #[case(R744, Pure)]
    #[case(R764, Pure)]
    #[case(R846, Pure)]
    #[case(R1150, Pure)]
    #[case(R1233zdE, Pure)]
    #[case(R1234yf, Pure)]
    #[case(R1234zeE, Pure)]
    #[case(R1234zeZ, Pure)]
    #[case(R1243zf, Pure)]
    #[case(R1270, Pure)]
    fn category_returns_expected_value(
        #[case] substance: Refrigerant,
        #[case] expected: RefrigerantCategory,
    ) {
        assert_eq!(substance.category(), expected);
    }

    //noinspection SpellCheckingInspection
    #[rstest]
    #[case(R11, "R11")]
    #[case(R12, "R12")]
    #[case(R13, "R13")]
    #[case(R13I1, "R13I1")]
    #[case(R14, "R14")]
    #[case(R21, "R21")]
    #[case(R22, "R22")]
    #[case(R23, "R23")]
    #[case(R32, "R32")]
    #[case(R40, "R40")]
    #[case(R41, "R41")]
    #[case(R50, "R50")]
    #[case(R113, "R113")]
    #[case(R114, "R114")]
    #[case(R115, "R115")]
    #[case(R116, "R116")]
    #[case(R123, "R123")]
    #[case(R124, "R124")]
    #[case(R125, "R125")]
    #[case(R134a, "R134a")]
    #[case(R141b, "R141b")]
    #[case(R142b, "R142b")]
    #[case(R143a, "R143a")]
    #[case(RE143a, "RE143a")]
    #[case(R152a, "R152a")]
    #[case(R161, "R161")]
    #[case(R170, "R170")]
    #[case(R218, "R218")]
    #[case(R227ea, "R227ea")]
    #[case(R236ea, "R236ea")]
    #[case(R236fa, "R236fa")]
    #[case(R245ca, "R245ca")]
    #[case(R245fa, "R245fa")]
    #[case(R290, "R290")]
    #[case(RC318, "RC318")]
    #[case(R365mfc, "R365mfc")]
    #[case(R404A, "R404A")]
    #[case(R404AMix, "R404A.mix")]
    #[case(R407A, "R407A.mix")]
    #[case(R407B, "R407B.mix")]
    #[case(R407C, "R407C")]
    #[case(R407CMix, "R407C.mix")]
    #[case(R407D, "R407D.mix")]
    #[case(R407E, "R407E.mix")]
    #[case(R407F, "R407F.mix")]
    #[case(R410A, "R410A")]
    #[case(R410AMix, "R410A.mix")]
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
    #[case(R507A, "R507A")]
    #[case(R507AMix, "R507A.mix")]
    #[case(R508A, "R508A.mix")]
    #[case(R508B, "R508B.mix")]
    #[case(R509A, "R509A.mix")]
    #[case(R510A, "R510A.mix")]
    #[case(R511A, "R511A.mix")]
    #[case(R512A, "R512A.mix")]
    #[case(R513A, "R513A.mix")]
    #[case(R600, "R600")]
    #[case(R600a, "R600a")]
    #[case(R601, "R601")]
    #[case(R601a, "R601a")]
    #[case(R702, "R702")]
    #[case(R704, "R704")]
    #[case(R717, "R717")]
    #[case(R718, "R718")]
    #[case(R720, "R720")]
    #[case(R728, "R728")]
    #[case(R729, "R729")]
    #[case(R732, "R732")]
    #[case(R740, "R740")]
    #[case(R744, "R744")]
    #[case(R764, "SulfurDioxide")]
    #[case(R846, "SulfurHexafluoride")]
    #[case(R1150, "R1150")]
    #[case(R1233zdE, "R1233zd(E)")]
    #[case(R1234yf, "R1234yf")]
    #[case(R1234zeE, "R1234ze(E)")]
    #[case(R1234zeZ, "R1234ze(Z)")]
    #[case(R1243zf, "R1243zf")]
    #[case(R1270, "R1270")]
    fn as_ref_returns_expected_str(#[case] substance: Refrigerant, #[case] expected: &str) {
        assert_eq!(substance.as_ref(), expected);
    }

    //noinspection SpellCheckingInspection
    #[rstest]
    #[case(vec!["R11"], R11)]
    #[case(vec!["R12"], R12)]
    #[case(vec!["R13"], R13)]
    #[case(vec!["R13I1", "CF3I"], R13I1)]
    #[case(vec!["R14"], R14)]
    #[case(vec!["R21"], R21)]
    #[case(vec!["R22"], R22)]
    #[case(vec!["R23"], R23)]
    #[case(vec!["R32"], R32)]
    #[case(vec!["R40"], R40)]
    #[case(vec!["R41"], R41)]
    #[case(vec!["R50"], R50)]
    #[case(vec!["R113"], R113)]
    #[case(vec!["R114"], R114)]
    #[case(vec!["R115"], R115)]
    #[case(vec!["R116"], R116)]
    #[case(vec!["R123"], R123)]
    #[case(vec!["R124"], R124)]
    #[case(vec!["R125"], R125)]
    #[case(vec!["R134a"], R134a)]
    #[case(vec!["R141b"], R141b)]
    #[case(vec!["R142b"], R142b)]
    #[case(vec!["R143a"], R143a)]
    #[case(vec!["RE143a"], RE143a)]
    #[case(vec!["R152a"], R152a)]
    #[case(vec!["R161"], R161)]
    #[case(vec!["R170"], R170)]
    #[case(vec!["R218"], R218)]
    #[case(vec!["R227ea"], R227ea)]
    #[case(vec!["R236ea"], R236ea)]
    #[case(vec!["R236fa"], R236fa)]
    #[case(vec!["R245ca"], R245ca)]
    #[case(vec!["R245fa"], R245fa)]
    #[case(vec!["R290"], R290)]
    #[case(vec!["RC318"], RC318)]
    #[case(vec!["R365mfc"], R365mfc)]
    #[case(vec!["R404A"], R404A)]
    #[case(vec!["R404A.mix", "R404AMix" , "R404A-mix"], R404AMix)]
    #[case(vec!["R407A.mix", "R407A"], R407A)]
    #[case(vec!["R407B.mix", "R407B"], R407B)]
    #[case(vec!["R407C"], R407C)]
    #[case(vec!["R407C.mix", "R407CMix" , "R407C-mix"], R407CMix)]
    #[case(vec!["R407D.mix", "R407D"], R407D)]
    #[case(vec!["R407E.mix", "R407E"], R407E)]
    #[case(vec!["R407F.mix", "R407F"], R407F)]
    #[case(vec!["R410A"], R410A)]
    #[case(vec!["R410A.mix", "R410AMix" ,  "R410A-mix"], R410AMix)]
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
    #[case(vec!["R507A"], R507A)]
    #[case(vec!["R507A.mix", "R507AMix" , "R507A-mix"], R507AMix)]
    #[case(vec!["R508A.mix", "R508A"], R508A)]
    #[case(vec!["R508B.mix", "R508B"], R508B)]
    #[case(vec!["R509A.mix", "R509A"], R509A)]
    #[case(vec!["R510A.mix", "R510A"], R510A)]
    #[case(vec!["R511A.mix", "R511A"], R511A)]
    #[case(vec!["R512A.mix", "R512A"], R512A)]
    #[case(vec!["R513A.mix", "R513A"], R513A)]
    #[case(vec!["R600a"], R600a)]
    #[case(vec!["R600"], R600)]
    #[case(vec!["R601a"], R601a)]
    #[case(vec!["R601"], R601)]
    #[case(vec!["R702"], R702)]
    #[case(vec!["R704"], R704)]
    #[case(vec!["R717"], R717)]
    #[case(vec!["R718"], R718)]
    #[case(vec!["R720"], R720)]
    #[case(vec!["R728"], R728)]
    #[case(vec!["R729"], R729)]
    #[case(vec!["R732"], R732)]
    #[case(vec!["R740"], R740)]
    #[case(vec!["R744"], R744)]
    #[case(vec!["R764", "SulfurDioxide"], R764)]
    #[case(vec!["R846", "SulfurHexafluoride"], R846)]
    #[case(vec!["R1150"], R1150)]
    #[case(vec!["R1233zd(E)", "R1233zdE"], R1233zdE)]
    #[case(vec!["R1234yf"], R1234yf)]
    #[case(vec!["R1234ze(E)", "R1234zeE"], R1234zeE)]
    #[case(vec!["R1234ze(Z)", "R1234zeZ"], R1234zeZ)]
    #[case(vec!["R1243zf"], R1243zf)]
    #[case(vec!["R1270"], R1270)]
    fn from_valid_str_returns_ok(#[case] valid_values: Vec<&str>, #[case] expected: Refrigerant) {
        for s in valid_values {
            assert_eq!(Refrigerant::from_str(s), Ok(expected));
            assert_eq!(Refrigerant::try_from(s), Ok(expected));
        }
    }

    #[rstest]
    #[case("")]
    #[case("Hello, World!")]
    fn from_invalid_str_returns_err(#[case] invalid_value: &str) {
        assert!(Refrigerant::from_str(invalid_value).is_err());
        assert!(Refrigerant::try_from(invalid_value).is_err());
    }
}
