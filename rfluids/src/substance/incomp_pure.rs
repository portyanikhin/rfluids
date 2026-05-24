// cSpell:disable

/// `CoolProp` incompressible pure substances.
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
/// assert_eq!(IncompPure::Water.as_ref(), "Water");
/// assert_eq!(IncompPure::from_str("Water"), Ok(IncompPure::Water));
/// assert_eq!(IncompPure::try_from("H2O"), Ok(IncompPure::Water));
/// ```
///
/// # See Also
///
/// - [Incompressible Substances](https://coolprop.org/fluid_properties/Incompressibles.html)
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
pub enum IncompPure {
    /// Aspen Temper -10 potassium acetate/formate heat transfer fluid.
    ///
    /// # See Also
    ///
    /// - [AS10 Fitting Report](https://coolprop.org/_downloads/bf3cfac57033a728817aa0be5c2b94bc/AS10_fitreport.pdf)
    #[strum(to_string = "AS10")]
    AS10,

    /// Aspen Temper -20 potassium acetate/formate heat transfer fluid.
    ///
    /// # See Also
    ///
    /// - [AS20 Fitting Report](https://coolprop.org/_downloads/560bd04fc4219373969a78477cd8b09a/AS20_fitreport.pdf)
    #[strum(to_string = "AS20")]
    AS20,

    /// Aspen Temper -30 potassium acetate/formate heat transfer fluid.
    ///
    /// # See Also
    ///
    /// - [AS30 Fitting Report](https://coolprop.org/_downloads/7cf86b087f3f5530c3de5378a1b78ad0/AS30_fitreport.pdf)
    #[strum(to_string = "AS30")]
    AS30,

    /// Aspen Temper -40 potassium acetate/formate heat transfer fluid.
    ///
    /// # See Also
    ///
    /// - [AS40 Fitting Report](https://coolprop.org/_downloads/e708f39ef5683d81f41e34b556cab283/AS40_fitreport.pdf)
    #[strum(to_string = "AS40")]
    AS40,

    /// Aspen Temper -55 potassium acetate/formate heat transfer fluid.
    ///
    /// # See Also
    ///
    /// - [AS55 Fitting Report](https://coolprop.org/_downloads/86d5df8a051f134e4de3d288fc69a8e9/AS55_fitreport.pdf)
    #[strum(to_string = "AS55")]
    AS55,

    /// Dowtherm J diethylbenzene mixture.
    ///
    /// # See Also
    ///
    /// - [DEB Fitting Report](https://coolprop.org/_downloads/e2f245a429caa89d9d208645008f1c59/DEB_fitreport.pdf)
    #[strum(to_string = "DEB")]
    DEB,

    /// Dowtherm J heat transfer fluid.
    ///
    /// # See Also
    ///
    /// - [DowJ Fitting Report](https://coolprop.org/_downloads/e9311d9f6ab0dc25267ab3fa6c93c1b5/DowJ_fitreport.pdf)
    #[strum(to_string = "DowJ")]
    DowJ,

    /// Dowtherm J diethylbenzene mixture.
    ///
    /// # See Also
    ///
    /// - [DowJ2 Fitting Report](https://coolprop.org/_downloads/85d168acf09f9cddf7288adbcf70ac3e/DowJ2_fitreport.pdf)
    #[strum(to_string = "DowJ2")]
    DowJ2,

    /// Dowtherm Q heat transfer fluid.
    ///
    /// # See Also
    ///
    /// - [DowQ Fitting Report](https://coolprop.org/_downloads/6d077683e7753f9ab7ae382c1fff9fca/DowQ_fitreport.pdf)
    #[strum(to_string = "DowQ")]
    DowQ,

    /// Dowtherm Q diphenylethane and alkylated aromatics mixture.
    ///
    /// # See Also
    ///
    /// - [DowQ2 Fitting Report](https://coolprop.org/_downloads/21cefd93c12f2a73512e6da5fafcf891/DowQ2_fitreport.pdf)
    #[strum(to_string = "DowQ2")]
    DowQ2,

    /// Dynalene SF heat transfer fluid.
    ///
    /// # See Also
    ///
    /// - [DSF Fitting Report](https://coolprop.org/_downloads/f3fbf9f110af23d6fb1825fbd2180cf4/DSF_fitreport.pdf)
    #[strum(to_string = "DSF")]
    DSF,

    /// Dynalene HC10 heat transfer fluid.
    ///
    /// # See Also
    ///
    /// - [HC10 Fitting Report](https://coolprop.org/_downloads/dcc777c704e05fe4b3e6b16bb2711de7/HC10_fitreport.pdf)
    #[strum(to_string = "HC10")]
    HC10,

    /// Dynalene HC20 heat transfer fluid.
    ///
    /// # See Also
    ///
    /// - [HC20 Fitting Report](https://coolprop.org/_downloads/b0eaffcd52623f23de69a48496aa7a3e/HC20_fitreport.pdf)
    #[strum(to_string = "HC20")]
    HC20,

    /// Dynalene HC30 heat transfer fluid.
    ///
    /// # See Also
    ///
    /// - [HC30 Fitting Report](https://coolprop.org/_downloads/d2879736df866be13aa1f48a0cf9d4ee/HC30_fitreport.pdf)
    #[strum(to_string = "HC30")]
    HC30,

    /// Dynalene HC40 heat transfer fluid.
    ///
    /// # See Also
    ///
    /// - [HC40 Fitting Report](https://coolprop.org/_downloads/fa8d37c1a49884e5a3fd9767e15b3e92/HC40_fitreport.pdf)
    #[strum(to_string = "HC40")]
    HC40,

    /// Dynalene HC50 heat transfer fluid.
    ///
    /// # See Also
    ///
    /// - [HC50 Fitting Report](https://coolprop.org/_downloads/ba1a7ead7e2623ec646e10f4bb4ff4ca/HC50_fitreport.pdf)
    #[strum(to_string = "HC50")]
    HC50,

    /// Dynalene MV hydrocarbon blend.
    ///
    /// # See Also
    ///
    /// - [HCB Fitting Report](https://coolprop.org/_downloads/0dcf1b55843b045cdf24f21acfcaea23/HCB_fitreport.pdf)
    #[strum(to_string = "HCB")]
    HCB,

    /// Gilotherm D12 hydrocarbon mixture.
    ///
    /// # See Also
    ///
    /// - [HCM Fitting Report](https://coolprop.org/_downloads/25707f3eb146285ac101bcabbbfd921d/HCM_fitreport.pdf)
    #[strum(to_string = "HCM")]
    HCM,

    /// HFE-7100 hydrofluoroether heat transfer fluid.
    ///
    /// # See Also
    ///
    /// - [HFE Fitting Report](https://coolprop.org/_downloads/62b62f530d939bdf953b3bbfca1548e4/HFE_fitreport.pdf)
    #[strum(to_string = "HFE")]
    HFE,

    /// HFE-7100 hydrofluoroether heat transfer fluid.
    ///
    /// # See Also
    ///
    /// - [HFE2 Fitting Report](https://coolprop.org/_downloads/54cc451445aad3199aa01e642c87e26f/HFE2_fitreport.pdf)
    #[strum(to_string = "HFE2")]
    HFE2,

    /// HYCOOL 20 potassium formate heat transfer fluid.
    ///
    /// # See Also
    ///
    /// - [HY20 Fitting Report](https://coolprop.org/_downloads/31836c309533b7a6351188270e85bed5/HY20_fitreport.pdf)
    #[strum(to_string = "HY20")]
    HY20,

    /// HYCOOL 30 potassium formate heat transfer fluid.
    ///
    /// # See Also
    ///
    /// - [HY30 Fitting Report](https://coolprop.org/_downloads/205e79795d933f41391b1da45cd3ca9f/HY30_fitreport.pdf)
    #[strum(to_string = "HY30")]
    HY30,

    /// HYCOOL 40 potassium formate heat transfer fluid.
    ///
    /// # See Also
    ///
    /// - [HY40 Fitting Report](https://coolprop.org/_downloads/7b9a555ba8b228b6a723f04d24032431/HY40_fitreport.pdf)
    #[strum(to_string = "HY40")]
    HY40,

    /// HYCOOL 45 potassium formate heat transfer fluid.
    ///
    /// # See Also
    ///
    /// - [HY45 Fitting Report](https://coolprop.org/_downloads/7063db8d87514624b20a092e7d903815/HY45_fitreport.pdf)
    #[strum(to_string = "HY45")]
    HY45,

    /// HYCOOL 50 potassium formate heat transfer fluid.
    ///
    /// # See Also
    ///
    /// - [HY50 Fitting Report](https://coolprop.org/_downloads/5194ef7b0a987529ab6d8e0c7454e987/HY50_fitreport.pdf)
    #[strum(to_string = "HY50")]
    HY50,

    /// Nitrate salt mixture with 0.6 NaNO3 and 0.4 KNO3.
    ///
    /// # See Also
    ///
    /// - [NaK Fitting Report](https://coolprop.org/_downloads/4642e22475f3b0eae483cce1a554e08f/NaK_fitreport.pdf)
    #[strum(to_string = "NaK")]
    NaK,

    /// NBS water model.
    ///
    /// # See Also
    ///
    /// - [NBS Fitting Report](https://coolprop.org/_downloads/d4f5cfdc63720c0e9a030e5429c5b32d/NBS_fitreport.pdf)
    #[strum(to_string = "NBS")]
    NBS,

    /// Pirobloc HTF-BASIC heat transfer fluid.
    ///
    /// # See Also
    ///
    /// - [PBB Fitting Report](https://coolprop.org/_downloads/44c89ee563242455aa99c8c63aa3ce94/PBB_fitreport.pdf)
    #[strum(to_string = "PBB")]
    PBB,

    /// Paracryol aliphatic hydrocarbon heat transfer fluid.
    ///
    /// # See Also
    ///
    /// - [PCL Fitting Report](https://coolprop.org/_downloads/036ff01deb860280c7fb3c2e7a930eaa/PCL_fitreport.pdf)
    #[strum(to_string = "PCL")]
    PCL,

    /// Paratherm CR heat transfer fluid.
    ///
    /// # See Also
    ///
    /// - [PCR Fitting Report](https://coolprop.org/_downloads/2abd61a94d3317288e57d88535eb58cb/PCR_fitreport.pdf)
    #[strum(to_string = "PCR")]
    PCR,

    /// Paratherm GLT heat transfer fluid.
    ///
    /// # See Also
    ///
    /// - [PGLT Fitting Report](https://coolprop.org/_downloads/7a5f3f5390a98fa54eb2710b26bb4c93/PGLT_fitreport.pdf)
    #[strum(to_string = "PGLT")]
    PGLT,

    /// Paratherm HE heat transfer fluid.
    ///
    /// # See Also
    ///
    /// - [PHE Fitting Report](https://coolprop.org/_downloads/c2853ecb0ad9dedd18875317bb622a16/PHE_fitreport.pdf)
    #[strum(to_string = "PHE")]
    PHE,

    /// Paratherm HR heat transfer fluid.
    ///
    /// # See Also
    ///
    /// - [PHR Fitting Report](https://coolprop.org/_downloads/93d5cd091d665f9f6182f526b6cca9af/PHR_fitreport.pdf)
    #[strum(to_string = "PHR")]
    PHR,

    /// Paratherm LR heat transfer fluid.
    ///
    /// # See Also
    ///
    /// - [PLR Fitting Report](https://coolprop.org/_downloads/f00877323d154671af9aa1aedfac8de9/PLR_fitreport.pdf)
    #[strum(to_string = "PLR")]
    PLR,

    /// Paratherm MR heat transfer fluid.
    ///
    /// # See Also
    ///
    /// - [PMR Fitting Report](https://coolprop.org/_downloads/bb939155611228bac60bb6286fec46ca/PMR_fitreport.pdf)
    #[strum(to_string = "PMR")]
    PMR,

    /// Baysilone KT3 polydimethylsiloxane heat transfer fluid.
    ///
    /// # See Also
    ///
    /// - [PMS1 Fitting Report](https://coolprop.org/_downloads/d733e22cba3fc231f1fa5affc41da839/PMS1_fitreport.pdf)
    #[strum(to_string = "PMS1")]
    PMS1,

    /// Syltherm XLT polydimethylsiloxane heat transfer fluid.
    ///
    /// # See Also
    ///
    /// - [PMS2 Fitting Report](https://coolprop.org/_downloads/24245c5c849f334da86a67d0e1606e05/PMS2_fitreport.pdf)
    #[strum(to_string = "PMS2")]
    PMS2,

    /// Paratherm NF heat transfer fluid.
    ///
    /// # See Also
    ///
    /// - [PNF Fitting Report](https://coolprop.org/_downloads/7ef705bb0a4f36f931f44303bbad41a0/PNF_fitreport.pdf)
    #[strum(to_string = "PNF")]
    PNF,

    /// Paratherm NF hydrotreated mineral oil.
    ///
    /// # See Also
    ///
    /// - [PNF2 Fitting Report](https://coolprop.org/_downloads/041f94a1be9d7396a03ed319baffd445/PNF2_fitreport.pdf)
    #[strum(to_string = "PNF2")]
    PNF2,

    /// Syltherm 800 heat transfer fluid.
    ///
    /// # See Also
    ///
    /// - [S800 Fitting Report](https://coolprop.org/_downloads/1c0efed06de7b25d317dbe5b7045f43f/S800_fitreport.pdf)
    #[strum(to_string = "S800")]
    S800,

    /// Marlotherm X synthetic alkyl benzene heat transfer fluid.
    ///
    /// # See Also
    ///
    /// - [SAB Fitting Report](https://coolprop.org/_downloads/e36104335e8097381176186eb400daad/SAB_fitreport.pdf)
    #[strum(to_string = "SAB")]
    SAB,

    /// Therminol 66 heat transfer fluid.
    ///
    /// # See Also
    ///
    /// - [T66 Fitting Report](https://coolprop.org/_downloads/53342adaacb2ff768878b53874cc6944/T66_fitreport.pdf)
    #[strum(to_string = "T66")]
    T66,

    /// Therminol 72 heat transfer fluid.
    ///
    /// # See Also
    ///
    /// - [T72 Fitting Report](https://coolprop.org/_downloads/c4992f0129278f361763445d16821345/T72_fitreport.pdf)
    #[strum(to_string = "T72")]
    T72,

    /// d-limonene citrus oil terpene.
    ///
    /// # See Also
    ///
    /// - [TCO Fitting Report](https://coolprop.org/_downloads/c35858695304d84761cfbd57f6db950c/TCO_fitreport.pdf)
    #[strum(to_string = "TCO")]
    TCO,

    /// Therminol D12 heat transfer fluid.
    ///
    /// # See Also
    ///
    /// - [TD12 Fitting Report](https://coolprop.org/_downloads/f4c847468a8914858c509abeb3515fd9/TD12_fitreport.pdf)
    #[strum(to_string = "TD12")]
    TD12,

    /// Therminol VP1 heat transfer fluid.
    ///
    /// # See Also
    ///
    /// - [TVP1 Fitting Report](https://coolprop.org/_downloads/ebf9be26fc4c4fd4322e4fc11c358235/TVP1_fitreport.pdf)
    #[strum(to_string = "TVP1")]
    TVP1,

    /// Thermogen VP 1869 heat transfer fluid.
    ///
    /// # See Also
    ///
    /// - [TVP1869 Fitting Report](https://coolprop.org/_downloads/d27b5f268224fbcfef3b756bd5453480/TVP1869_fitreport.pdf)
    #[strum(to_string = "TVP1869")]
    TVP1869,

    /// Texatherm 22 heat transfer fluid.
    ///
    /// # See Also
    ///
    /// - [TX22 Fitting Report](https://coolprop.org/_downloads/b3db9a11a25e0757cbcdbc310231210a/TX22_fitreport.pdf)
    #[strum(to_string = "TX22")]
    TX22,

    /// Tyfoxit 1.10 potassium acetate heat transfer fluid.
    ///
    /// # See Also
    ///
    /// - [TY10 Fitting Report](https://coolprop.org/_downloads/8d46ab87b77876cc23f5059cb8b93b27/TY10_fitreport.pdf)
    #[strum(to_string = "TY10")]
    TY10,

    /// Tyfoxit 1.15 potassium acetate heat transfer fluid.
    ///
    /// # See Also
    ///
    /// - [TY15 Fitting Report](https://coolprop.org/_downloads/a63acbe410e8a0f39b6b95d00f6ba021/TY15_fitreport.pdf)
    #[strum(to_string = "TY15")]
    TY15,

    /// Tyfoxit 1.20 potassium acetate heat transfer fluid.
    ///
    /// # See Also
    ///
    /// - [TY20 Fitting Report](https://coolprop.org/_downloads/b28697731aa7c56ebe897f204a51facc/TY20_fitreport.pdf)
    #[strum(to_string = "TY20")]
    TY20,

    /// Tyfoxit 1.24 potassium acetate heat transfer fluid.
    ///
    /// # See Also
    ///
    /// - [TY24 Fitting Report](https://coolprop.org/_downloads/1030e8efe000dc19242bebf96ade250a/TY24_fitreport.pdf)
    #[strum(to_string = "TY24")]
    TY24,

    /// Water.
    ///
    /// # See Also
    ///
    /// - [Water Fitting Report](https://coolprop.org/_downloads/1797e60f6da7a28792efcd9f1f2e8506/Water_fitreport.pdf)
    #[strum(to_string = "Water", serialize = "H2O")]
    Water,

    /// Syltherm XLT heat transfer fluid.
    ///
    /// # See Also
    ///
    /// - [XLT Fitting Report](https://coolprop.org/_downloads/5f90a623120e34f550c12c3334232cde/XLT_fitreport.pdf)
    #[strum(to_string = "XLT")]
    XLT,

    /// Syltherm XLT polydimethylsiloxane heat transfer fluid.
    ///
    /// # See Also
    ///
    /// - [XLT2 Fitting Report](https://coolprop.org/_downloads/1b7060bc766294d4e1ba401337815b7f/XLT2_fitreport.pdf)
    #[strum(to_string = "XLT2")]
    XLT2,

    /// Zitrec S10 potassium formate/sodium propionate heat transfer fluid.
    ///
    /// # See Also
    ///
    /// - [ZS10 Fitting Report](https://coolprop.org/_downloads/403b92f11cc03d40ffa6a399a0ca9ad6/ZS10_fitreport.pdf)
    #[strum(to_string = "ZS10")]
    ZS10,

    /// Zitrec S25 potassium formate/sodium propionate heat transfer fluid.
    ///
    /// # See Also
    ///
    /// - [ZS25 Fitting Report](https://coolprop.org/_downloads/84d0302134c371182b92df36c424bd3d/ZS25_fitreport.pdf)
    #[strum(to_string = "ZS25")]
    ZS25,

    /// Zitrec S40 potassium formate/sodium propionate heat transfer fluid.
    ///
    /// # See Also
    ///
    /// - [ZS40 Fitting Report](https://coolprop.org/_downloads/03ebc8c497761bbbf60b18413e094ad8/ZS40_fitreport.pdf)
    #[strum(to_string = "ZS40")]
    ZS40,

    /// Zitrec S45 potassium formate/sodium propionate heat transfer fluid.
    ///
    /// # See Also
    ///
    /// - [ZS45 Fitting Report](https://coolprop.org/_downloads/a647d6d95d4a165e405ac248aae38148/ZS45_fitreport.pdf)
    #[strum(to_string = "ZS45")]
    ZS45,

    /// Zitrec S55 potassium formate/sodium propionate heat transfer fluid.
    ///
    /// # See Also
    ///
    /// - [ZS55 Fitting Report](https://coolprop.org/_downloads/3ffeb579fefbbbf4ba5ac154f6e8006d/ZS55_fitreport.pdf)
    #[strum(to_string = "ZS55")]
    ZS55,
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use rstest::*;

    use super::{IncompPure::*, *};

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
    fn as_str(#[case] sut: IncompPure, #[case] expected: &str) {
        // When
        let str = sut.as_ref();
        let static_str: &'static str = sut.into();

        // Then
        assert_eq!(str, expected);
        assert_eq!(static_str, expected);
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
