// cSpell:disable

#[cfg(test)]
use strum_macros::EnumIter;
use strum_macros::{AsRefStr, EnumString};

/// `CoolProp` pure or pseudo-pure substances.
///
/// # Examples
///
/// Conversion between [`&str`](str):
///
/// ```
/// use std::str::FromStr;
/// use rfluids::substance::Pure;
///
/// assert_eq!(Pure::Water.as_ref(), "Water");
/// assert_eq!(Pure::from_str("Water"), Ok(Pure::Water));
/// assert_eq!(Pure::try_from("H2O"), Ok(Pure::Water));
/// ```
///
/// # See also
///
/// - [Pure and pseudo-pure substances](https://coolprop.github.io/CoolProp/fluid_properties/PurePseudoPure.html)
#[derive(AsRefStr, EnumString, Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[strum(ascii_case_insensitive)]
#[cfg_attr(test, derive(EnumIter))]
pub enum Pure {
    #[strum(to_string = "Acetone")]
    Acetone,

    #[strum(to_string = "Air")]
    Air,

    #[strum(to_string = "Ammonia", serialize = "NH3")]
    Ammonia,

    #[strum(to_string = "Argon", serialize = "Ar")]
    Argon,

    #[strum(to_string = "Benzene")]
    Benzene,

    #[strum(to_string = "1-Butene", serialize = "1Butene", serialize = "Butene")]
    Butene,

    #[strum(to_string = "CarbonDioxide", serialize = "CO2")]
    CarbonDioxide,

    #[strum(to_string = "CarbonMonoxide", serialize = "CO")]
    CarbonMonoxide,

    #[strum(to_string = "CarbonylSulfide", serialize = "COS")]
    CarbonylSulfide,

    #[strum(to_string = "cis-2-Butene", serialize = "C2BUTENE")]
    cis2Butene,

    #[strum(to_string = "Cyclohexane", serialize = "CYCLOHEX")]
    Cyclohexane,

    #[strum(to_string = "Cyclopentane", serialize = "CYCLOPEN")]
    Cyclopentane,

    #[strum(to_string = "Cyclopropane", serialize = "CYCLOPRO")]
    Cyclopropane,

    #[strum(to_string = "D4", serialize = "Octamethylcyclotetrasiloxane")]
    D4,

    #[strum(to_string = "D5", serialize = "Decamethylcyclopentasiloxane")]
    D5,

    #[strum(to_string = "D6", serialize = "Dodecamethylcyclohexasiloxane")]
    D6,

    #[strum(to_string = "Deuterium", serialize = "D2")]
    Deuterium,

    #[strum(to_string = "Dichloroethane", serialize = "1,2-dichloroethane")]
    Dichloroethane,

    #[strum(to_string = "DiethylEther", serialize = "DEE")]
    DiethylEther,

    #[strum(to_string = "DimethylCarbonate", serialize = "DMC")]
    DimethylCarbonate,

    #[strum(to_string = "DimethylEther", serialize = "DME")]
    DimethylEther,

    #[strum(to_string = "Ethane", serialize = "n-C2H6")]
    Ethane,

    #[strum(to_string = "Ethanol", serialize = "C2H6O")]
    Ethanol,

    #[strum(to_string = "EthylBenzene", serialize = "EBENZENE")]
    EthylBenzene,

    #[strum(to_string = "Ethylene")]
    Ethylene,

    #[strum(to_string = "EthyleneOxide")]
    EthyleneOxide,

    #[strum(to_string = "Fluorine")]
    Fluorine,

    #[strum(to_string = "HeavyWater", serialize = "D2O")]
    HeavyWater,

    #[strum(to_string = "Helium", serialize = "He")]
    Helium,

    #[strum(to_string = "HFE143m", serialize = "HFE-143m")]
    HFE143m,

    #[strum(to_string = "Hydrogen", serialize = "H2")]
    Hydrogen,

    #[strum(to_string = "HydrogenChloride", serialize = "HCl")]
    HydrogenChloride,

    #[strum(to_string = "HydrogenSulfide", serialize = "H2S")]
    HydrogenSulfide,

    #[strum(to_string = "Isobutane", serialize = "IBUTANE")]
    Isobutane,

    #[strum(to_string = "Isobutene", serialize = "IBUTENE")]
    Isobutene,

    #[strum(to_string = "Isohexane", serialize = "IHEXANE")]
    Isohexane,

    #[strum(to_string = "Isopentane", serialize = "IPENTANE")]
    Isopentane,

    #[strum(to_string = "Krypton")]
    Krypton,

    #[strum(to_string = "MD2M", serialize = "Decamethyltetrasiloxane")]
    MD2M,

    #[strum(to_string = "MD3M", serialize = "Dodecamethylpentasiloxane")]
    MD3M,

    #[strum(to_string = "MD4M", serialize = "Tetradecamethylhexasiloxane")]
    MD4M,

    #[strum(to_string = "MDM", serialize = "Octamethyltrisiloxane")]
    MDM,

    #[strum(to_string = "Methane", serialize = "CH4", serialize = "n-C1H4")]
    Methane,

    #[strum(to_string = "Methanol")]
    Methanol,

    #[strum(to_string = "MethylLinoleate", serialize = "MLINOLEA")]
    MethylLinoleate,

    #[strum(to_string = "MethylLinolenate", serialize = "MLINOLEN")]
    MethylLinolenate,

    #[strum(to_string = "MethylOleate", serialize = "MOLEATE")]
    MethylOleate,

    #[strum(to_string = "MethylPalmitate", serialize = "MPALMITA")]
    MethylPalmitate,

    #[strum(to_string = "MethylStearate", serialize = "MSTEARAT")]
    MethylStearate,

    #[strum(to_string = "MM", serialize = "Hexamethyldisiloxane")]
    MM,

    #[strum(to_string = "m-Xylene", serialize = "mXylene", serialize = "MC8H10")]
    mXylene,

    #[strum(
        to_string = "n-Butane",
        serialize = "nButane",
        serialize = "Butane",
        serialize = "NC4H10",
        serialize = "n-C4H10"
    )]
    nButane,

    #[strum(
        to_string = "n-Decane",
        serialize = "nDecane",
        serialize = "Decane",
        serialize = "NC10H22",
        serialize = "n-C10H22"
    )]
    nDecane,

    #[strum(
        to_string = "n-Dodecane",
        serialize = "nDodecane",
        serialize = "Dodecane",
        serialize = "NC12H26",
        serialize = "n-C12H26"
    )]
    nDodecane,

    #[strum(to_string = "Neon", serialize = "Ne")]
    Neon,

    #[strum(to_string = "Neopentane")]
    Neopentane,

    #[strum(
        to_string = "n-Heptane",
        serialize = "nHeptane",
        serialize = "Heptane",
        serialize = "NC7H16",
        serialize = "n-C7H16"
    )]
    nHeptane,

    #[strum(
        to_string = "n-Hexane",
        serialize = "nHexane",
        serialize = "Hexane",
        serialize = "NC6H14",
        serialize = "n-C6H14"
    )]
    nHexane,

    #[strum(to_string = "Nitrogen", serialize = "N2")]
    Nitrogen,

    #[strum(to_string = "NitrousOxide", serialize = "N2O")]
    NitrousOxide,

    #[strum(
        to_string = "n-Nonane",
        serialize = "nNonane",
        serialize = "Nonane",
        serialize = "NC9H20",
        serialize = "n-C9H20"
    )]
    nNonane,

    #[strum(
        to_string = "n-Octane",
        serialize = "nOctane",
        serialize = "Octane",
        serialize = "NC8H18",
        serialize = "n-C8H18"
    )]
    nOctane,

    #[strum(to_string = "Novec649", serialize = "Novec1230")]
    Novec649,

    #[strum(
        to_string = "n-Pentane",
        serialize = "nPentane",
        serialize = "Pentane",
        serialize = "NC5H12",
        serialize = "n-C5H12"
    )]
    nPentane,

    #[strum(
        to_string = "n-Propane",
        serialize = "nPropane",
        serialize = "Propane",
        serialize = "C3H8",
        serialize = "NC3H8",
        serialize = "n-C3H8"
    )]
    nPropane,

    #[strum(
        to_string = "n-Undecane",
        serialize = "nUndecane",
        serialize = "Undecane",
        serialize = "NC11H24",
        serialize = "n-C11H24"
    )]
    nUndecane,

    #[strum(to_string = "OrthoDeuterium", serialize = "o-D2")]
    Orthodeuterium,

    #[strum(to_string = "OrthoHydrogen", serialize = "o-H2")]
    Orthohydrogen,

    #[strum(to_string = "Oxygen", serialize = "O2")]
    Oxygen,

    #[strum(to_string = "o-Xylene", serialize = "oXylene", serialize = "OC8H10")]
    oXylene,

    #[strum(to_string = "ParaDeuterium", serialize = "p-D2")]
    Paradeuterium,

    #[strum(to_string = "ParaHydrogen", serialize = "p-H2")]
    Parahydrogen,

    #[strum(to_string = "Propylene")]
    Propylene,

    #[strum(to_string = "Propyne")]
    Propyne,

    #[strum(to_string = "p-Xylene", serialize = "pXylene", serialize = "PC8H10")]
    pXylene,

    #[strum(to_string = "SES36")]
    SES36,

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

    #[strum(to_string = "R407C")]
    R407C,

    #[strum(to_string = "R410A")]
    R410A,

    #[strum(to_string = "R507A")]
    R507A,

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

    #[strum(to_string = "R1336mzz(E)", serialize = "R1336mzzE")]
    R1336mzzE,

    #[strum(to_string = "SulfurDioxide", serialize = "SO2")]
    SulfurDioxide,

    #[strum(to_string = "SulfurHexafluoride", serialize = "SF6")]
    SulfurHexafluoride,

    #[strum(to_string = "Toluene")]
    Toluene,

    #[strum(to_string = "trans-2-Butene", serialize = "T2BUTENE")]
    trans2Butene,

    #[strum(to_string = "Water", serialize = "H2O")]
    Water,

    #[strum(to_string = "Xenon", serialize = "Xe")]
    Xenon,
}

#[cfg(test)]
mod tests {
    use super::{Pure::*, *};
    use rstest::*;
    use std::str::FromStr;

    #[rstest]
    #[case(Acetone, "Acetone")]
    #[case(Air, "Air")]
    #[case(Ammonia, "Ammonia")]
    #[case(Argon, "Argon")]
    #[case(Benzene, "Benzene")]
    #[case(Butene, "1-Butene")]
    #[case(CarbonDioxide, "CarbonDioxide")]
    #[case(CarbonMonoxide, "CarbonMonoxide")]
    #[case(CarbonylSulfide, "CarbonylSulfide")]
    #[case(cis2Butene, "cis-2-Butene")]
    #[case(Cyclohexane, "Cyclohexane")]
    #[case(Cyclopentane, "Cyclopentane")]
    #[case(Cyclopropane, "Cyclopropane")]
    #[case(D4, "D4")]
    #[case(D5, "D5")]
    #[case(D6, "D6")]
    #[case(Deuterium, "Deuterium")]
    #[case(Dichloroethane, "Dichloroethane")]
    #[case(DiethylEther, "DiethylEther")]
    #[case(DimethylCarbonate, "DimethylCarbonate")]
    #[case(DimethylEther, "DimethylEther")]
    #[case(Ethane, "Ethane")]
    #[case(Ethanol, "Ethanol")]
    #[case(EthylBenzene, "EthylBenzene")]
    #[case(Ethylene, "Ethylene")]
    #[case(EthyleneOxide, "EthyleneOxide")]
    #[case(Fluorine, "Fluorine")]
    #[case(HeavyWater, "HeavyWater")]
    #[case(Helium, "Helium")]
    #[case(HFE143m, "HFE143m")]
    #[case(Hydrogen, "Hydrogen")]
    #[case(HydrogenChloride, "HydrogenChloride")]
    #[case(HydrogenSulfide, "HydrogenSulfide")]
    #[case(Isobutane, "Isobutane")]
    #[case(Isobutene, "Isobutene")]
    #[case(Isohexane, "Isohexane")]
    #[case(Isopentane, "Isopentane")]
    #[case(Krypton, "Krypton")]
    #[case(MD2M, "MD2M")]
    #[case(MD3M, "MD3M")]
    #[case(MD4M, "MD4M")]
    #[case(MDM, "MDM")]
    #[case(Methane, "Methane")]
    #[case(Methanol, "Methanol")]
    #[case(MethylLinoleate, "MethylLinoleate")]
    #[case(MethylLinolenate, "MethylLinolenate")]
    #[case(MethylOleate, "MethylOleate")]
    #[case(MethylPalmitate, "MethylPalmitate")]
    #[case(MethylStearate, "MethylStearate")]
    #[case(MM, "MM")]
    #[case(mXylene, "m-Xylene")]
    #[case(nButane, "n-Butane")]
    #[case(nDecane, "n-Decane")]
    #[case(nDodecane, "n-Dodecane")]
    #[case(Neon, "Neon")]
    #[case(Neopentane, "Neopentane")]
    #[case(nHeptane, "n-Heptane")]
    #[case(nHexane, "n-Hexane")]
    #[case(Nitrogen, "Nitrogen")]
    #[case(NitrousOxide, "NitrousOxide")]
    #[case(nNonane, "n-Nonane")]
    #[case(nOctane, "n-Octane")]
    #[case(Novec649, "Novec649")]
    #[case(nPentane, "n-Pentane")]
    #[case(nPropane, "n-Propane")]
    #[case(nUndecane, "n-Undecane")]
    #[case(Orthodeuterium, "OrthoDeuterium")]
    #[case(Orthohydrogen, "OrthoHydrogen")]
    #[case(Oxygen, "Oxygen")]
    #[case(oXylene, "o-Xylene")]
    #[case(Paradeuterium, "ParaDeuterium")]
    #[case(Parahydrogen, "ParaHydrogen")]
    #[case(Propylene, "Propylene")]
    #[case(Propyne, "Propyne")]
    #[case(pXylene, "p-Xylene")]
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
    #[case(R407C, "R407C")]
    #[case(R410A, "R410A")]
    #[case(R507A, "R507A")]
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
    #[case(R1336mzzE, "R1336mzz(E)")]
    #[case(SES36, "SES36")]
    #[case(SulfurDioxide, "SulfurDioxide")]
    #[case(SulfurHexafluoride, "SulfurHexafluoride")]
    #[case(Toluene, "Toluene")]
    #[case(trans2Butene, "trans-2-Butene")]
    #[case(Water, "Water")]
    #[case(Xenon, "Xenon")]
    fn as_ref_returns_expected_str(#[case] substance: Pure, #[case] expected: &str) {
        assert_eq!(substance.as_ref(), expected);
    }

    #[rstest]
    #[case(vec!["Acetone"], Acetone)]
    #[case(vec!["Air"], Air)]
    #[case(vec!["Ammonia", "NH3"], Ammonia)]
    #[case(vec!["Argon", "Ar"], Argon)]
    #[case(vec!["Benzene"], Benzene)]
    #[case(vec!["1-Butene", "1Butene" ,  "Butene"], Butene)]
    #[case(vec!["CarbonDioxide", "CO2"], CarbonDioxide)]
    #[case(vec!["CarbonMonoxide", "CO"], CarbonMonoxide)]
    #[case(vec!["CarbonylSulfide", "COS"], CarbonylSulfide)]
    #[case(vec!["cis-2-Butene", "C2BUTENE"], cis2Butene)]
    #[case(vec!["Cyclohexane", "CYCLOHEX"], Cyclohexane)]
    #[case(vec!["Cyclopentane", "CYCLOPEN"], Cyclopentane)]
    #[case(vec!["Cyclopropane", "CYCLOPRO"], Cyclopropane)]
    #[case(vec!["D4", "Octamethylcyclotetrasiloxane"], D4)]
    #[case(vec!["D5", "Decamethylcyclopentasiloxane"], D5)]
    #[case(vec!["D6", "Dodecamethylcyclohexasiloxane"], D6)]
    #[case(vec!["Deuterium", "D2"], Deuterium)]
    #[case(vec!["Dichloroethane", "1,2-dichloroethane"], Dichloroethane)]
    #[case(vec!["DiethylEther", "DEE"], DiethylEther)]
    #[case(vec!["DimethylCarbonate", "DMC"], DimethylCarbonate)]
    #[case(vec!["DimethylEther", "DME"], DimethylEther)]
    #[case(vec!["Ethane", "n-C2H6"], Ethane)]
    #[case(vec!["Ethanol", "C2H6O"], Ethanol)]
    #[case(vec!["EthylBenzene", "EBENZENE"], EthylBenzene)]
    #[case(vec!["Ethylene"], Ethylene)]
    #[case(vec!["EthyleneOxide"], EthyleneOxide)]
    #[case(vec!["Fluorine"], Fluorine)]
    #[case(vec!["HeavyWater", "D2O"], HeavyWater)]
    #[case(vec!["Helium", "He"], Helium)]
    #[case(vec!["HFE143m", "HFE-143m"], HFE143m)]
    #[case(vec!["Hydrogen", "H2"], Hydrogen)]
    #[case(vec!["HydrogenChloride", "HCl"], HydrogenChloride)]
    #[case(vec!["HydrogenSulfide", "H2S"], HydrogenSulfide)]
    #[case(vec!["Isobutane", "IBUTANE"], Isobutane)]
    #[case(vec!["Isobutene", "IBUTENE"], Isobutene)]
    #[case(vec!["Isohexane", "IHEXANE"], Isohexane)]
    #[case(vec!["Isopentane", "IPENTANE"], Isopentane)]
    #[case(vec!["Krypton"], Krypton)]
    #[case(vec!["MD2M", "Decamethyltetrasiloxane"], MD2M)]
    #[case(vec!["MD3M", "Dodecamethylpentasiloxane"], MD3M)]
    #[case(vec!["MD4M", "Tetradecamethylhexasiloxane"], MD4M)]
    #[case(vec!["MDM", "Octamethyltrisiloxane"], MDM)]
    #[case(vec!["Methane", "CH4" ,  "n-C1H4"], Methane)]
    #[case(vec!["Methanol"], Methanol)]
    #[case(vec!["MethylLinoleate", "MLINOLEA"], MethylLinoleate)]
    #[case(vec!["MethylLinolenate", "MLINOLEN"], MethylLinolenate)]
    #[case(vec!["MethylOleate", "MOLEATE"], MethylOleate)]
    #[case(vec!["MethylPalmitate", "MPALMITA"], MethylPalmitate)]
    #[case(vec!["MethylStearate", "MSTEARAT"], MethylStearate)]
    #[case(vec!["MM", "Hexamethyldisiloxane"], MM)]
    #[case(vec!["m-Xylene", "mXylene" ,  "MC8H10"], mXylene)]
    #[case(vec!["n-Butane", "nButane" ,  "Butane" ,  "NC4H10" ,  "n-C4H10"], nButane)]
    #[case(vec!["n-Decane", "nDecane" ,  "Decane" ,  "NC10H22" ,  "n-C10H22"], nDecane)]
    #[case(vec!["n-Dodecane", "nDodecane" ,  "Dodecane" ,  "NC12H26" ,  "n-C12H26"], nDodecane)]
    #[case(vec!["Neon"], Neon)]
    #[case(vec!["Neopentane"], Neopentane)]
    #[case(vec!["n-Heptane", "nHeptane" ,  "Heptane" ,  "NC7H16" ,  "n-C7H16"], nHeptane)]
    #[case(vec!["n-Hexane", "nHexane" ,  "Hexane" ,  "NC6H14" ,  "n-C6H14"], nHexane)]
    #[case(vec!["Nitrogen", "N2"], Nitrogen)]
    #[case(vec!["NitrousOxide", "N2O"], NitrousOxide)]
    #[case(vec!["n-Nonane", "nNonane" ,  "Nonane" ,  "NC9H20" ,  "n-C9H20"], nNonane)]
    #[case(vec!["n-Octane", "nOctane" ,  "Octane" ,  "NC8H18" ,  "n-C8H18"], nOctane)]
    #[case(vec!["Novec649", "Novec1230"], Novec649)]
    #[case(vec!["n-Pentane", "nPentane" ,  "Pentane" ,  "NC5H12" ,  "n-C5H12"], nPentane)]
    #[case(vec!["n-Propane", "nPropane" ,  "Propane" ,  "C3H8" ,  "NC3H8" ,  "n-C3H8"], nPropane)]
    #[case(vec!["n-Undecane", "nUndecane" ,  "Undecane" ,  "NC11H24" ,  "n-C11H24"], nUndecane)]
    #[case(vec!["OrthoDeuterium"], Orthodeuterium)]
    #[case(vec!["OrthoHydrogen"], Orthohydrogen)]
    #[case(vec!["Oxygen", "O2"], Oxygen)]
    #[case(vec!["o-Xylene", "oXylene" ,  "OC8H10"], oXylene)]
    #[case(vec!["ParaDeuterium"], Paradeuterium)]
    #[case(vec!["ParaHydrogen"], Parahydrogen)]
    #[case(vec!["Propylene"], Propylene)]
    #[case(vec!["Propyne"], Propyne)]
    #[case(vec!["p-Xylene", "pXylene" ,  "PC8H10"], pXylene)]
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
    #[case(vec!["R407C"], R407C)]
    #[case(vec!["R410A"], R410A)]
    #[case(vec!["R507A"], R507A)]
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
    #[case(vec!["R1336mzz(E)", "R1336mzzE"], R1336mzzE)]
    #[case(vec!["SES36"], SES36)]
    #[case(vec!["SO2"], SulfurDioxide)]
    #[case(vec!["SF6"], SulfurHexafluoride)]
    #[case(vec!["Toluene"], Toluene)]
    #[case(vec!["trans-2-Butene", "T2BUTENE"], trans2Butene)]
    #[case(vec!["Water", "H2O"], Water)]
    #[case(vec!["Xenon", "Xe"], Xenon)]
    fn from_valid_str_returns_ok(#[case] valid_values: Vec<&str>, #[case] expected: Pure) {
        for s in valid_values {
            assert_eq!(Pure::from_str(s), Ok(expected));
            assert_eq!(Pure::try_from(s), Ok(expected));
        }
    }

    #[rstest]
    #[case("")]
    #[case("Hello, World!")]
    fn from_invalid_str_returns_err(#[case] invalid_value: &str) {
        assert!(Pure::from_str(invalid_value).is_err());
        assert!(Pure::try_from(invalid_value).is_err());
    }
}
