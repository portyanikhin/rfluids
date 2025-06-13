// cSpell:disable

#[cfg(test)]
use strum_macros::EnumIter;
use strum_macros::{AsRefStr, EnumString};

/// `CoolProp` incompressible pure substances.
///
/// # Examples
///
/// Conversion between [`&str`](str):
///
/// ```
/// use std::str::FromStr;
/// use rfluids::prelude::*;
///
/// assert_eq!(IncompPure::Water.as_ref(), "Water");
/// assert_eq!(IncompPure::from_str("Water"), Ok(IncompPure::Water));
/// assert_eq!(IncompPure::try_from("H2O"), Ok(IncompPure::Water));
/// ```
///
/// # See also
///
/// - [Incompressible substances](https://coolprop.github.io/CoolProp/fluid_properties/Incompressibles.html)
#[derive(AsRefStr, Clone, Copy, Debug, EnumString, Eq, PartialEq)]
#[strum(ascii_case_insensitive)]
#[cfg_attr(test, derive(EnumIter))]
pub enum IncompPure {
    #[strum(to_string = "AS10")]
    AS10,

    #[strum(to_string = "AS20")]
    AS20,

    #[strum(to_string = "AS30")]
    AS30,

    #[strum(to_string = "AS40")]
    AS40,

    #[strum(to_string = "AS55")]
    AS55,

    #[strum(to_string = "DEB")]
    DEB,

    #[strum(to_string = "DowJ")]
    DowJ,

    #[strum(to_string = "DowJ2")]
    DowJ2,

    #[strum(to_string = "DowQ")]
    DowQ,

    #[strum(to_string = "DowQ2")]
    DowQ2,

    #[strum(to_string = "DSF")]
    DSF,

    #[strum(to_string = "HC10")]
    HC10,

    #[strum(to_string = "HC20")]
    HC20,

    #[strum(to_string = "HC30")]
    HC30,

    #[strum(to_string = "HC40")]
    HC40,

    #[strum(to_string = "HC50")]
    HC50,

    #[strum(to_string = "HCB")]
    HCB,

    #[strum(to_string = "HCM")]
    HCM,

    #[strum(to_string = "HFE")]
    HFE,

    #[strum(to_string = "HFE2")]
    HFE2,

    #[strum(to_string = "HY20")]
    HY20,

    #[strum(to_string = "HY30")]
    HY30,

    #[strum(to_string = "HY40")]
    HY40,

    #[strum(to_string = "HY45")]
    HY45,

    #[strum(to_string = "HY50")]
    HY50,

    #[strum(to_string = "NaK")]
    NaK,

    #[strum(to_string = "NBS")]
    NBS,

    #[strum(to_string = "PBB")]
    PBB,

    #[strum(to_string = "PCL")]
    PCL,

    #[strum(to_string = "PCR")]
    PCR,

    #[strum(to_string = "PGLT")]
    PGLT,

    #[strum(to_string = "PHE")]
    PHE,

    #[strum(to_string = "PHR")]
    PHR,

    #[strum(to_string = "PLR")]
    PLR,

    #[strum(to_string = "PMR")]
    PMR,

    #[strum(to_string = "PMS1")]
    PMS1,

    #[strum(to_string = "PMS2")]
    PMS2,

    #[strum(to_string = "PNF")]
    PNF,

    #[strum(to_string = "PNF2")]
    PNF2,

    #[strum(to_string = "S800")]
    S800,

    #[strum(to_string = "SAB")]
    SAB,

    #[strum(to_string = "T66")]
    T66,

    #[strum(to_string = "T72")]
    T72,

    #[strum(to_string = "TCO")]
    TCO,

    #[strum(to_string = "TD12")]
    TD12,

    #[strum(to_string = "TVP1")]
    TVP1,

    #[strum(to_string = "TVP1869")]
    TVP1869,

    #[strum(to_string = "TX22")]
    TX22,

    #[strum(to_string = "TY10")]
    TY10,

    #[strum(to_string = "TY15")]
    TY15,

    #[strum(to_string = "TY20")]
    TY20,

    #[strum(to_string = "TY24")]
    TY24,

    #[strum(to_string = "Water", serialize = "H2O")]
    Water,

    #[strum(to_string = "XLT")]
    XLT,

    #[strum(to_string = "XLT2")]
    XLT2,

    #[strum(to_string = "ZS10")]
    ZS10,

    #[strum(to_string = "ZS25")]
    ZS25,

    #[strum(to_string = "ZS40")]
    ZS40,

    #[strum(to_string = "ZS45")]
    ZS45,

    #[strum(to_string = "ZS55")]
    ZS55,
}

#[cfg(test)]
mod tests {
    use super::{IncompPure::*, *};
    use rstest::*;
    use std::str::FromStr;

    #[rstest]
    #[case(AS10, "AS10")]
    #[case(AS20, "AS20")]
    #[case(AS30, "AS30")]
    #[case(AS40, "AS40")]
    #[case(AS55, "AS55")]
    #[case(DEB, "DEB")]
    #[case(DowJ, "DowJ")]
    #[case(DowJ2, "DowJ2")]
    #[case(DowQ, "DowQ")]
    #[case(DowQ2, "DowQ2")]
    #[case(DSF, "DSF")]
    #[case(HC10, "HC10")]
    #[case(HC20, "HC20")]
    #[case(HC30, "HC30")]
    #[case(HC40, "HC40")]
    #[case(HC50, "HC50")]
    #[case(HCB, "HCB")]
    #[case(HCM, "HCM")]
    #[case(HFE, "HFE")]
    #[case(HFE2, "HFE2")]
    #[case(HY20, "HY20")]
    #[case(HY30, "HY30")]
    #[case(HY40, "HY40")]
    #[case(HY45, "HY45")]
    #[case(HY50, "HY50")]
    #[case(NaK, "NaK")]
    #[case(NBS, "NBS")]
    #[case(PBB, "PBB")]
    #[case(PCL, "PCL")]
    #[case(PCR, "PCR")]
    #[case(PGLT, "PGLT")]
    #[case(PHE, "PHE")]
    #[case(PHR, "PHR")]
    #[case(PLR, "PLR")]
    #[case(PMR, "PMR")]
    #[case(PMS1, "PMS1")]
    #[case(PMS2, "PMS2")]
    #[case(PNF, "PNF")]
    #[case(PNF2, "PNF2")]
    #[case(S800, "S800")]
    #[case(SAB, "SAB")]
    #[case(T66, "T66")]
    #[case(T72, "T72")]
    #[case(TCO, "TCO")]
    #[case(TD12, "TD12")]
    #[case(TVP1, "TVP1")]
    #[case(TVP1869, "TVP1869")]
    #[case(TX22, "TX22")]
    #[case(TY10, "TY10")]
    #[case(TY15, "TY15")]
    #[case(TY20, "TY20")]
    #[case(TY24, "TY24")]
    #[case(Water, "Water")]
    #[case(XLT, "XLT")]
    #[case(XLT2, "XLT2")]
    #[case(ZS10, "ZS10")]
    #[case(ZS25, "ZS25")]
    #[case(ZS40, "ZS40")]
    #[case(ZS45, "ZS45")]
    #[case(ZS55, "ZS55")]
    fn as_ref(#[case] sut: IncompPure, #[case] expected: &str) {
        // When
        let res = sut.as_ref();

        // Then
        assert_eq!(res, expected);
    }

    #[rstest]
    #[case(vec!["AS10"], AS10)]
    #[case(vec!["AS20"], AS20)]
    #[case(vec!["AS30"], AS30)]
    #[case(vec!["AS40"], AS40)]
    #[case(vec!["AS55"], AS55)]
    #[case(vec!["DEB"], DEB)]
    #[case(vec!["DowJ"], DowJ)]
    #[case(vec!["DowJ2"], DowJ2)]
    #[case(vec!["DowQ"], DowQ)]
    #[case(vec!["DowQ2"], DowQ2)]
    #[case(vec!["DSF"], DSF)]
    #[case(vec!["HC10"], HC10)]
    #[case(vec!["HC20"], HC20)]
    #[case(vec!["HC30"], HC30)]
    #[case(vec!["HC40"], HC40)]
    #[case(vec!["HC50"], HC50)]
    #[case(vec!["HCB"], HCB)]
    #[case(vec!["HCM"], HCM)]
    #[case(vec!["HFE"], HFE)]
    #[case(vec!["HFE2"], HFE2)]
    #[case(vec!["HY20"], HY20)]
    #[case(vec!["HY30"], HY30)]
    #[case(vec!["HY40"], HY40)]
    #[case(vec!["HY45"], HY45)]
    #[case(vec!["HY50"], HY50)]
    #[case(vec!["NaK"], NaK)]
    #[case(vec!["NBS"], NBS)]
    #[case(vec!["PBB"], PBB)]
    #[case(vec!["PCL"], PCL)]
    #[case(vec!["PCR"], PCR)]
    #[case(vec!["PGLT"], PGLT)]
    #[case(vec!["PHE"], PHE)]
    #[case(vec!["PHR"], PHR)]
    #[case(vec!["PLR"], PLR)]
    #[case(vec!["PMR"], PMR)]
    #[case(vec!["PMS1"], PMS1)]
    #[case(vec!["PMS2"], PMS2)]
    #[case(vec!["PNF"], PNF)]
    #[case(vec!["PNF2"], PNF2)]
    #[case(vec!["S800"], S800)]
    #[case(vec!["SAB"], SAB)]
    #[case(vec!["T66"], T66)]
    #[case(vec!["T72"], T72)]
    #[case(vec!["TCO"], TCO)]
    #[case(vec!["TD12"], TD12)]
    #[case(vec!["TVP1"], TVP1)]
    #[case(vec!["TVP1869"], TVP1869)]
    #[case(vec!["TX22"], TX22)]
    #[case(vec!["TY10"], TY10)]
    #[case(vec!["TY15"], TY15)]
    #[case(vec!["TY20"], TY20)]
    #[case(vec!["TY24"], TY24)]
    #[case(vec!["Water", "H2O"], Water)]
    #[case(vec!["XLT"], XLT)]
    #[case(vec!["XLT2"], XLT2)]
    #[case(vec!["ZS10"], ZS10)]
    #[case(vec!["ZS25"], ZS25)]
    #[case(vec!["ZS40"], ZS40)]
    #[case(vec!["ZS45"], ZS45)]
    #[case(vec!["ZS55"], ZS55)]
    fn from_valid_str(#[case] valid: Vec<&str>, #[case] expected: IncompPure) {
        for s in valid {
            // When
            let res1 = IncompPure::from_str(s).unwrap();
            let res2 = IncompPure::try_from(s).unwrap();

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
        let res1 = IncompPure::from_str(invalid);
        let res2 = IncompPure::try_from(invalid);

        // Then
        assert!(res1.is_err());
        assert!(res2.is_err());
    }
}
