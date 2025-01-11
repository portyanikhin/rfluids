use std::str::FromStr;
use strum::EnumProperty;
use strum_macros::{AsRefStr, EnumProperty, EnumString};

/// CoolProp substances.
///
/// # Examples
///
/// How to convert [`Substance`] into [`&str`](str):
/// ```
/// use fluids_rs::enums::Substance;
///
/// assert_eq!(Substance::Water.as_ref(), "Water");
/// ```
///
/// How to parse [`Substance`] from [`&str`](str):
/// ```
/// use std::str::FromStr;
/// use fluids_rs::enums::Substance;
///
/// assert_eq!(Substance::from_str("Water").unwrap(), Substance::Water);
///
/// // or
///
/// assert_eq!(Substance::try_from("H2O").unwrap(), Substance::Water);
/// ```
/// # See also
///
/// - [Pure and pseudo-pure substances](https://coolprop.github.io/CoolProp/fluid_properties/PurePseudoPure.html)
/// - [Incompressible substances](https://coolprop.github.io/CoolProp/fluid_properties/Incomps.html)
/// - [Predefined mixtures](https://coolprop.github.io/CoolProp/coolprop/HighLevelAPI.html#predefined-mixtures)
//noinspection SpellCheckingInspection
#[allow(non_camel_case_types)]
#[derive(AsRefStr, EnumString, EnumProperty, Debug, Copy, Clone, Eq, PartialEq)]
#[strum(ascii_case_insensitive)]
pub enum Substance {
    #[strum(to_string = "Acetone", props(category = "Pure"))]
    Acetone,

    #[strum(to_string = "Air", props(category = "PseudoPure"))]
    Air,

    #[strum(to_string = "R729", props(category = "Pure"))]
    R729,

    #[strum(to_string = "Ammonia", serialize = "NH3", props(category = "Pure"))]
    Ammonia,

    #[strum(to_string = "R717", props(category = "Pure"))]
    R717,

    #[strum(to_string = "Argon", serialize = "Ar", props(category = "Pure"))]
    Argon,

    #[strum(to_string = "R740", props(category = "Pure"))]
    R740,

    #[strum(to_string = "Benzene", props(category = "Pure"))]
    Benzene,

    #[strum(
        to_string = "1-Butene",
        serialize = "1Butene",
        serialize = "Butene",
        props(category = "Pure")
    )]
    Butene,

    #[strum(
        to_string = "CarbonDioxide",
        serialize = "CO2",
        props(category = "Pure")
    )]
    CarbonDioxide,

    #[strum(to_string = "R744", props(category = "Pure"))]
    R744,

    #[strum(
        to_string = "CarbonMonoxide",
        serialize = "CO",
        props(category = "Pure")
    )]
    CarbonMonoxide,

    #[strum(
        to_string = "CarbonylSulfide",
        serialize = "COS",
        props(category = "Pure")
    )]
    CarbonylSulfide,

    #[strum(
        to_string = "cis-2-Butene",
        serialize = "C2BUTENE",
        props(category = "Pure")
    )]
    cis2Butene,

    #[strum(
        to_string = "Cyclohexane",
        serialize = "CYCLOHEX",
        props(category = "Pure")
    )]
    Cyclohexane,

    #[strum(
        to_string = "Cyclopentane",
        serialize = "CYCLOPEN",
        props(category = "Pure")
    )]
    Cyclopentane,

    #[strum(
        to_string = "Cyclopropane",
        serialize = "CYCLOPRO",
        props(category = "Pure")
    )]
    Cyclopropane,

    #[strum(
        to_string = "D4",
        serialize = "Octamethylcyclotetrasiloxane",
        props(category = "Pure")
    )]
    D4,

    #[strum(
        to_string = "D5",
        serialize = "Decamethylcyclopentasiloxane",
        props(category = "Pure")
    )]
    D5,

    #[strum(
        to_string = "D6",
        serialize = "Dodecamethylcyclohexasiloxane",
        props(category = "Pure")
    )]
    D6,

    #[strum(to_string = "Deuterium", serialize = "D2", props(category = "Pure"))]
    Deuterium,

    #[strum(
        to_string = "Dichloroethane",
        serialize = "1,2-dichloroethane",
        props(category = "Pure")
    )]
    Dichloroethane,

    #[strum(
        to_string = "DiethylEther",
        serialize = "DEE",
        props(category = "Pure")
    )]
    DiethylEther,

    #[strum(
        to_string = "DimethylCarbonate",
        serialize = "DMC",
        props(category = "Pure")
    )]
    DimethylCarbonate,

    #[strum(
        to_string = "DimethylEther",
        serialize = "DME",
        props(category = "Pure")
    )]
    DimethylEther,

    #[strum(to_string = "Ethane", serialize = "n-C2H6", props(category = "Pure"))]
    Ethane,

    #[strum(to_string = "R170", props(category = "Pure"))]
    R170,

    #[strum(to_string = "Ethanol", serialize = "C2H6O", props(category = "Pure"))]
    Ethanol,

    #[strum(
        to_string = "EthylBenzene",
        serialize = "EBENZENE",
        props(category = "Pure")
    )]
    EthylBenzene,

    #[strum(to_string = "Ethylene", props(category = "Pure"))]
    Ethylene,

    #[strum(to_string = "R1150", props(category = "Pure"))]
    R1150,

    #[strum(to_string = "EthyleneOxide", props(category = "Pure"))]
    EthyleneOxide,

    #[strum(to_string = "Fluorine", props(category = "Pure"))]
    Fluorine,

    #[strum(to_string = "HeavyWater", serialize = "D2O", props(category = "Pure"))]
    HeavyWater,

    #[strum(to_string = "Helium", serialize = "He", props(category = "Pure"))]
    Helium,

    #[strum(to_string = "R704", props(category = "Pure"))]
    R704,

    #[strum(
        to_string = "HFE143m",
        serialize = "HFE-143m",
        props(category = "Pure")
    )]
    HFE143m,

    #[strum(to_string = "RE143a", props(category = "Pure"))]
    RE143a,

    #[strum(to_string = "Hydrogen", serialize = "H2", props(category = "Pure"))]
    Hydrogen,

    #[strum(to_string = "R702", props(category = "Pure"))]
    R702,

    #[strum(
        to_string = "HydrogenChloride",
        serialize = "HCl",
        props(category = "Pure")
    )]
    HydrogenChloride,

    #[strum(
        to_string = "HydrogenSulfide",
        serialize = "H2S",
        props(category = "Pure")
    )]
    HydrogenSulfide,

    #[strum(
        to_string = "Isobutane",
        serialize = "IBUTANE",
        props(category = "Pure")
    )]
    Isobutane,

    #[strum(to_string = "R600a", props(category = "Pure"))]
    R600a,

    #[strum(
        to_string = "Isobutene",
        serialize = "IBUTENE",
        props(category = "Pure")
    )]
    Isobutene,

    #[strum(
        to_string = "Isohexane",
        serialize = "IHEXANE",
        props(category = "Pure")
    )]
    Isohexane,

    #[strum(
        to_string = "Isopentane",
        serialize = "IPENTANE",
        props(category = "Pure")
    )]
    Isopentane,

    #[strum(to_string = "R601a", props(category = "Pure"))]
    R601a,

    #[strum(to_string = "Krypton", props(category = "Pure"))]
    Krypton,

    #[strum(
        to_string = "MD2M",
        serialize = "Decamethyltetrasiloxane",
        props(category = "Pure")
    )]
    MD2M,

    #[strum(
        to_string = "MD3M",
        serialize = "Dodecamethylpentasiloxane",
        props(category = "Pure")
    )]
    MD3M,

    #[strum(
        to_string = "MD4M",
        serialize = "Tetradecamethylhexasiloxane",
        props(category = "Pure")
    )]
    MD4M,

    #[strum(
        to_string = "MDM",
        serialize = "Octamethyltrisiloxane",
        props(category = "Pure")
    )]
    MDM,

    #[strum(
        to_string = "Methane",
        serialize = "CH4",
        serialize = "n-C1H4",
        props(category = "Pure")
    )]
    Methane,

    #[strum(to_string = "R50", props(category = "Pure"))]
    R50,

    #[strum(to_string = "Methanol", props(category = "Pure"))]
    Methanol,

    #[strum(
        to_string = "MethylLinoleate",
        serialize = "MLINOLEA",
        props(category = "Pure")
    )]
    MethylLinoleate,

    #[strum(
        to_string = "MethylLinolenate",
        serialize = "MLINOLEN",
        props(category = "Pure")
    )]
    MethylLinolenate,

    #[strum(
        to_string = "MethylOleate",
        serialize = "MOLEATE",
        props(category = "Pure")
    )]
    MethylOleate,

    #[strum(
        to_string = "MethylPalmitate",
        serialize = "MPALMITA",
        props(category = "Pure")
    )]
    MethylPalmitate,

    #[strum(
        to_string = "MethylStearate",
        serialize = "MSTEARAT",
        props(category = "Pure")
    )]
    MethylStearate,

    #[strum(
        to_string = "MM",
        serialize = "Hexamethyldisiloxane",
        props(category = "Pure")
    )]
    MM,

    #[strum(
        to_string = "m-Xylene",
        serialize = "mXylene",
        serialize = "MC8H10",
        props(category = "Pure")
    )]
    mXylene,

    #[strum(
        to_string = "n-Butane",
        serialize = "nButane",
        serialize = "Butane",
        serialize = "NC4H10",
        serialize = "n-C4H10",
        props(category = "Pure")
    )]
    nButane,

    #[strum(to_string = "R600", props(category = "Pure"))]
    R600,

    #[strum(
        to_string = "n-Decane",
        serialize = "nDecane",
        serialize = "Decane",
        serialize = "NC10H22",
        serialize = "n-C10H22",
        props(category = "Pure")
    )]
    nDecane,

    #[strum(
        to_string = "n-Dodecane",
        serialize = "nDodecane",
        serialize = "Dodecane",
        serialize = "NC12H26",
        serialize = "n-C12H26",
        props(category = "Pure")
    )]
    nDodecane,

    #[strum(to_string = "Neon", serialize = "Ne", props(category = "Pure"))]
    Neon,

    #[strum(to_string = "R720", props(category = "Pure"))]
    R720,

    #[strum(to_string = "Neopentane", props(category = "Pure"))]
    Neopentane,

    #[strum(
        to_string = "n-Heptane",
        serialize = "nHeptane",
        serialize = "Heptane",
        serialize = "NC7H16",
        serialize = "n-C7H16",
        props(category = "Pure")
    )]
    nHeptane,

    #[strum(
        to_string = "n-Hexane",
        serialize = "nHexane",
        serialize = "Hexane",
        serialize = "NC6H14",
        serialize = "n-C6H14",
        props(category = "Pure")
    )]
    nHexane,

    #[strum(to_string = "Nitrogen", serialize = "N2", props(category = "Pure"))]
    Nitrogen,

    #[strum(to_string = "R728", props(category = "Pure"))]
    R728,

    #[strum(
        to_string = "NitrousOxide",
        serialize = "N2O",
        props(category = "Pure")
    )]
    NitrousOxide,

    #[strum(
        to_string = "n-Nonane",
        serialize = "nNonane",
        serialize = "Nonane",
        serialize = "NC9H20",
        serialize = "n-C9H20",
        props(category = "Pure")
    )]
    nNonane,

    #[strum(
        to_string = "n-Octane",
        serialize = "nOctane",
        serialize = "Octane",
        serialize = "NC8H18",
        serialize = "n-C8H18",
        props(category = "Pure")
    )]
    nOctane,

    #[strum(
        to_string = "Novec649",
        serialize = "Novec1230",
        props(category = "Pure")
    )]
    Novec649,

    #[strum(
        to_string = "n-Pentane",
        serialize = "nPentane",
        serialize = "Pentane",
        serialize = "NC5H12",
        serialize = "n-C5H12",
        props(category = "Pure")
    )]
    nPentane,

    #[strum(to_string = "R601", props(category = "Pure"))]
    R601,

    #[strum(
        to_string = "n-Propane",
        serialize = "nPropane",
        serialize = "Propane",
        serialize = "C3H8",
        serialize = "NC3H8",
        serialize = "n-C3H8",
        props(category = "Pure")
    )]
    nPropane,

    #[strum(to_string = "R290", props(category = "Pure"))]
    R290,

    #[strum(
        to_string = "n-Undecane",
        serialize = "nUndecane",
        serialize = "Undecane",
        serialize = "NC11H24",
        serialize = "n-C11H24",
        props(category = "Pure")
    )]
    nUndecane,

    #[strum(
        to_string = "OrthoDeuterium",
        serialize = "o-D2",
        props(category = "Pure")
    )]
    Orthodeuterium,

    #[strum(
        to_string = "OrthoHydrogen",
        serialize = "o-H2",
        props(category = "Pure")
    )]
    Orthohydrogen,

    #[strum(to_string = "Oxygen", serialize = "O2", props(category = "Pure"))]
    Oxygen,

    #[strum(to_string = "R732", props(category = "Pure"))]
    R732,

    #[strum(
        to_string = "o-Xylene",
        serialize = "oXylene",
        serialize = "OC8H10",
        props(category = "Pure")
    )]
    oXylene,

    #[strum(
        to_string = "ParaDeuterium",
        serialize = "p-D2",
        props(category = "Pure")
    )]
    Paradeuterium,

    #[strum(
        to_string = "ParaHydrogen",
        serialize = "p-H2",
        props(category = "Pure")
    )]
    Parahydrogen,

    #[strum(to_string = "Propylene", props(category = "Pure"))]
    Propylene,

    #[strum(to_string = "R1270", props(category = "Pure"))]
    R1270,

    #[strum(to_string = "Propyne", props(category = "Pure"))]
    Propyne,

    #[strum(
        to_string = "p-Xylene",
        serialize = "pXylene",
        serialize = "PC8H10",
        props(category = "Pure")
    )]
    pXylene,

    #[strum(to_string = "R11", props(category = "Pure"))]
    R11,

    #[strum(to_string = "R113", props(category = "Pure"))]
    R113,

    #[strum(to_string = "R114", props(category = "Pure"))]
    R114,

    #[strum(to_string = "R115", props(category = "Pure"))]
    R115,

    #[strum(to_string = "R116", props(category = "Pure"))]
    R116,

    #[strum(to_string = "R12", props(category = "Pure"))]
    R12,

    #[strum(to_string = "R123", props(category = "Pure"))]
    R123,

    #[strum(
        to_string = "R1233zd(E)",
        serialize = "R1233zdE",
        props(category = "Pure")
    )]
    R1233zdE,

    #[strum(to_string = "R1234yf", props(category = "Pure"))]
    R1234yf,

    #[strum(
        to_string = "R1234ze(E)",
        serialize = "R1234zeE",
        props(category = "Pure")
    )]
    R1234zeE,

    #[strum(
        to_string = "R1234ze(Z)",
        serialize = "R1234zeZ",
        props(category = "Pure")
    )]
    R1234zeZ,

    #[strum(to_string = "R124", props(category = "Pure"))]
    R124,

    #[strum(to_string = "R1243zf", props(category = "Pure"))]
    R1243zf,

    #[strum(to_string = "R125", props(category = "Pure"))]
    R125,

    #[strum(to_string = "R13", props(category = "Pure"))]
    R13,

    #[strum(to_string = "R134a", props(category = "Pure"))]
    R134a,

    #[strum(to_string = "R13I1", serialize = "CF3I", props(category = "Pure"))]
    R13I1,

    #[strum(to_string = "R14", props(category = "Pure"))]
    R14,

    #[strum(to_string = "R141b", props(category = "Pure"))]
    R141b,

    #[strum(to_string = "R142b", props(category = "Pure"))]
    R142b,

    #[strum(to_string = "R143a", props(category = "Pure"))]
    R143a,

    #[strum(to_string = "R152a", props(category = "Pure"))]
    R152a,

    #[strum(to_string = "R161", props(category = "Pure"))]
    R161,

    #[strum(to_string = "R21", props(category = "Pure"))]
    R21,

    #[strum(to_string = "R218", props(category = "Pure"))]
    R218,

    #[strum(to_string = "R22", props(category = "Pure"))]
    R22,

    #[strum(to_string = "R227ea", props(category = "Pure"))]
    R227ea,

    #[strum(to_string = "R23", props(category = "Pure"))]
    R23,

    #[strum(to_string = "R236ea", props(category = "Pure"))]
    R236ea,

    #[strum(to_string = "R236fa", props(category = "Pure"))]
    R236fa,

    #[strum(to_string = "R245ca", props(category = "Pure"))]
    R245ca,

    #[strum(to_string = "R245fa", props(category = "Pure"))]
    R245fa,

    #[strum(to_string = "R32", props(category = "Pure"))]
    R32,

    #[strum(to_string = "R365mfc", props(category = "Pure"))]
    R365mfc,

    #[strum(to_string = "R40", props(category = "Pure"))]
    R40,

    #[strum(to_string = "R404A", props(category = "PseudoPure"))]
    R404A,

    #[strum(to_string = "R407C", props(category = "PseudoPure"))]
    R407C,

    #[strum(to_string = "R41", props(category = "Pure"))]
    R41,

    #[strum(to_string = "R410A", props(category = "PseudoPure"))]
    R410A,

    #[strum(to_string = "R507A", props(category = "PseudoPure"))]
    R507A,

    #[strum(to_string = "RC318", props(category = "Pure"))]
    RC318,

    #[strum(to_string = "SES36", props(category = "Pure"))]
    SES36,

    #[strum(
        to_string = "SulfurDioxide",
        serialize = "SO2",
        props(category = "Pure")
    )]
    SulfurDioxide,

    #[strum(
        to_string = "SulfurDioxide",
        serialize = "R764",
        props(category = "Pure")
    )]
    R764,

    #[strum(
        to_string = "SulfurHexafluoride",
        serialize = "SF6",
        props(category = "Pure")
    )]
    SulfurHexafluoride,

    #[strum(
        to_string = "SulfurHexafluoride",
        serialize = "R846",
        props(category = "Pure")
    )]
    R846,

    #[strum(to_string = "Toluene", props(category = "Pure"))]
    Toluene,

    #[strum(
        to_string = "trans-2-Butene",
        serialize = "T2BUTENE",
        props(category = "Pure")
    )]
    trans2Butene,

    #[strum(to_string = "Water", serialize = "H2O", props(category = "Pure"))]
    Water,

    #[strum(to_string = "R718", props(category = "Pure"))]
    R718,

    #[strum(to_string = "Xenon", serialize = "Xe", props(category = "Pure"))]
    Xenon,

    #[strum(to_string = "AS10", props(category = "IncompPure"))]
    AS10,

    #[strum(to_string = "AS20", props(category = "IncompPure"))]
    AS20,

    #[strum(to_string = "AS30", props(category = "IncompPure"))]
    AS30,

    #[strum(to_string = "AS40", props(category = "IncompPure"))]
    AS40,

    #[strum(to_string = "AS55", props(category = "IncompPure"))]
    AS55,

    #[strum(to_string = "DEB", props(category = "IncompPure"))]
    DEB,

    #[strum(to_string = "DowJ", props(category = "IncompPure"))]
    DowJ,

    #[strum(to_string = "DowJ2", props(category = "IncompPure"))]
    DowJ2,

    #[strum(to_string = "DowQ", props(category = "IncompPure"))]
    DowQ,

    #[strum(to_string = "DowQ2", props(category = "IncompPure"))]
    DowQ2,

    #[strum(to_string = "DSF", props(category = "IncompPure"))]
    DSF,

    #[strum(to_string = "HC10", props(category = "IncompPure"))]
    HC10,

    #[strum(to_string = "HC20", props(category = "IncompPure"))]
    HC20,

    #[strum(to_string = "HC30", props(category = "IncompPure"))]
    HC30,

    #[strum(to_string = "HC40", props(category = "IncompPure"))]
    HC40,

    #[strum(to_string = "HC50", props(category = "IncompPure"))]
    HC50,

    #[strum(to_string = "HCB", props(category = "IncompPure"))]
    HCB,

    #[strum(to_string = "HCM", props(category = "IncompPure"))]
    HCM,

    #[strum(to_string = "HFE", props(category = "IncompPure"))]
    HFE,

    #[strum(to_string = "HFE2", props(category = "IncompPure"))]
    HFE2,

    #[strum(to_string = "HY20", props(category = "IncompPure"))]
    HY20,

    #[strum(to_string = "HY30", props(category = "IncompPure"))]
    HY30,

    #[strum(to_string = "HY40", props(category = "IncompPure"))]
    HY40,

    #[strum(to_string = "HY45", props(category = "IncompPure"))]
    HY45,

    #[strum(to_string = "HY50", props(category = "IncompPure"))]
    HY50,

    #[strum(to_string = "NaK", props(category = "IncompPure"))]
    NaK,

    #[strum(to_string = "NBS", props(category = "IncompPure"))]
    NBS,

    #[strum(to_string = "PBB", props(category = "IncompPure"))]
    PBB,

    #[strum(to_string = "PCL", props(category = "IncompPure"))]
    PCL,

    #[strum(to_string = "PCR", props(category = "IncompPure"))]
    PCR,

    #[strum(to_string = "PGLT", props(category = "IncompPure"))]
    PGLT,

    #[strum(to_string = "PHE", props(category = "IncompPure"))]
    PHE,

    #[strum(to_string = "PHR", props(category = "IncompPure"))]
    PHR,

    #[strum(to_string = "PLR", props(category = "IncompPure"))]
    PLR,

    #[strum(to_string = "PMR", props(category = "IncompPure"))]
    PMR,

    #[strum(to_string = "PMS1", props(category = "IncompPure"))]
    PMS1,

    #[strum(to_string = "PMS2", props(category = "IncompPure"))]
    PMS2,

    #[strum(to_string = "PNF", props(category = "IncompPure"))]
    PNF,

    #[strum(to_string = "PNF2", props(category = "IncompPure"))]
    PNF2,

    #[strum(to_string = "S800", props(category = "IncompPure"))]
    S800,

    #[strum(to_string = "SAB", props(category = "IncompPure"))]
    SAB,

    #[strum(to_string = "T66", props(category = "IncompPure"))]
    T66,

    #[strum(to_string = "T72", props(category = "IncompPure"))]
    T72,

    #[strum(to_string = "TCO", props(category = "IncompPure"))]
    TCO,

    #[strum(to_string = "TD12", props(category = "IncompPure"))]
    TD12,

    #[strum(to_string = "TVP1", props(category = "IncompPure"))]
    TVP1,

    #[strum(to_string = "TVP1869", props(category = "IncompPure"))]
    TVP1869,

    #[strum(to_string = "TX22", props(category = "IncompPure"))]
    TX22,

    #[strum(to_string = "TY10", props(category = "IncompPure"))]
    TY10,

    #[strum(to_string = "TY15", props(category = "IncompPure"))]
    TY15,

    #[strum(to_string = "TY20", props(category = "IncompPure"))]
    TY20,

    #[strum(to_string = "TY24", props(category = "IncompPure"))]
    TY24,

    #[strum(
        to_string = "Water",
        serialize = "WaterIncomp",
        serialize = "IncompWater",
        props(category = "IncompPure")
    )]
    WaterIncomp,

    #[strum(to_string = "XLT", props(category = "IncompPure"))]
    XLT,

    #[strum(to_string = "XLT2", props(category = "IncompPure"))]
    XLT2,

    #[strum(to_string = "ZS10", props(category = "IncompPure"))]
    ZS10,

    #[strum(to_string = "ZS25", props(category = "IncompPure"))]
    ZS25,

    #[strum(to_string = "ZS40", props(category = "IncompPure"))]
    ZS40,

    #[strum(to_string = "ZS45", props(category = "IncompPure"))]
    ZS45,

    #[strum(to_string = "ZS55", props(category = "IncompPure"))]
    ZS55,

    #[strum(
        to_string = "FRE",
        props(
            category = "IncompMassBasedBinaryMixture",
            min_fraction = "0.19",
            max_fraction = "0.5"
        )
    )]
    FRE,

    #[strum(
        to_string = "IceEA",
        props(
            category = "IncompMassBasedBinaryMixture",
            min_fraction = "0.05",
            max_fraction = "0.35"
        )
    )]
    IceEA,

    #[strum(
        to_string = "IceNA",
        props(
            category = "IncompMassBasedBinaryMixture",
            min_fraction = "0.05",
            max_fraction = "0.35"
        )
    )]
    IceNA,

    #[strum(
        to_string = "IcePG",
        props(
            category = "IncompMassBasedBinaryMixture",
            min_fraction = "0.05",
            max_fraction = "0.35"
        )
    )]
    IcePG,

    #[strum(
        to_string = "LiBr",
        props(
            category = "IncompMassBasedBinaryMixture",
            min_fraction = "0.0",
            max_fraction = "0.75"
        )
    )]
    LiBr,

    #[strum(
        to_string = "MAM",
        props(
            category = "IncompMassBasedBinaryMixture",
            min_fraction = "0.0",
            max_fraction = "0.3"
        )
    )]
    MAM,

    #[strum(
        to_string = "MAM2",
        props(
            category = "IncompMassBasedBinaryMixture",
            min_fraction = "0.078",
            max_fraction = "0.236"
        )
    )]
    MAM2,

    #[strum(
        to_string = "MCA",
        props(
            category = "IncompMassBasedBinaryMixture",
            min_fraction = "0.0",
            max_fraction = "0.3"
        )
    )]
    MCA,

    #[strum(
        to_string = "MCA2",
        props(
            category = "IncompMassBasedBinaryMixture",
            min_fraction = "0.09",
            max_fraction = "0.294"
        )
    )]
    MCA2,

    #[strum(
        to_string = "MEA",
        props(
            category = "IncompMassBasedBinaryMixture",
            min_fraction = "0.0",
            max_fraction = "0.6"
        )
    )]
    MEA,

    #[strum(
        to_string = "MEA2",
        props(
            category = "IncompMassBasedBinaryMixture",
            min_fraction = "0.11",
            max_fraction = "0.6"
        )
    )]
    MEA2,

    #[strum(
        to_string = "MEG",
        props(
            category = "IncompMassBasedBinaryMixture",
            min_fraction = "0.0",
            max_fraction = "0.6"
        )
    )]
    MEG,

    #[strum(
        to_string = "MEG2",
        props(
            category = "IncompMassBasedBinaryMixture",
            min_fraction = "0.0",
            max_fraction = "0.56"
        )
    )]
    MEG2,

    #[strum(
        to_string = "MGL",
        props(
            category = "IncompMassBasedBinaryMixture",
            min_fraction = "0.0",
            max_fraction = "0.6"
        )
    )]
    MGL,

    #[strum(
        to_string = "MGL2",
        props(
            category = "IncompMassBasedBinaryMixture",
            min_fraction = "0.195",
            max_fraction = "0.63"
        )
    )]
    MGL2,

    #[strum(
        to_string = "MITSW",
        props(
            category = "IncompMassBasedBinaryMixture",
            min_fraction = "0.0",
            max_fraction = "0.12"
        )
    )]
    MITSW,

    #[strum(
        to_string = "MKA",
        props(
            category = "IncompMassBasedBinaryMixture",
            min_fraction = "0.0",
            max_fraction = "0.45"
        )
    )]
    MKA,

    #[strum(
        to_string = "MKA2",
        props(
            category = "IncompMassBasedBinaryMixture",
            min_fraction = "0.11",
            max_fraction = "0.41"
        )
    )]
    MKA2,

    #[strum(
        to_string = "MKC",
        props(
            category = "IncompMassBasedBinaryMixture",
            min_fraction = "0.0",
            max_fraction = "0.4"
        )
    )]
    MKC,

    #[strum(
        to_string = "MKC2",
        props(
            category = "IncompMassBasedBinaryMixture",
            min_fraction = "0.0",
            max_fraction = "0.39"
        )
    )]
    MKC2,

    #[strum(
        to_string = "MKF",
        props(
            category = "IncompMassBasedBinaryMixture",
            min_fraction = "0.0",
            max_fraction = "0.48"
        )
    )]
    MKF,

    #[strum(
        to_string = "MLI",
        props(
            category = "IncompMassBasedBinaryMixture",
            min_fraction = "0.0",
            max_fraction = "0.24"
        )
    )]
    MLI,

    #[strum(
        to_string = "MMA",
        props(
            category = "IncompMassBasedBinaryMixture",
            min_fraction = "0.0",
            max_fraction = "0.6"
        )
    )]
    MMA,

    #[strum(
        to_string = "MMA2",
        props(
            category = "IncompMassBasedBinaryMixture",
            min_fraction = "0.078",
            max_fraction = "0.474"
        )
    )]
    MMA2,

    #[strum(
        to_string = "MMG",
        props(
            category = "IncompMassBasedBinaryMixture",
            min_fraction = "0.0",
            max_fraction = "0.3"
        )
    )]
    MMG,

    #[strum(
        to_string = "MMG2",
        props(
            category = "IncompMassBasedBinaryMixture",
            min_fraction = "0.0",
            max_fraction = "0.205"
        )
    )]
    MMG2,

    #[strum(
        to_string = "MNA",
        props(
            category = "IncompMassBasedBinaryMixture",
            min_fraction = "0.0",
            max_fraction = "0.23"
        )
    )]
    MNA,

    #[strum(
        to_string = "MNA2",
        props(
            category = "IncompMassBasedBinaryMixture",
            min_fraction = "0.0",
            max_fraction = "0.23"
        )
    )]
    MNA2,

    #[strum(
        to_string = "MPG",
        props(
            category = "IncompMassBasedBinaryMixture",
            min_fraction = "0.0",
            max_fraction = "0.6"
        )
    )]
    MPG,

    #[strum(
        to_string = "MPG2",
        props(
            category = "IncompMassBasedBinaryMixture",
            min_fraction = "0.15",
            max_fraction = "0.57"
        )
    )]
    MPG2,

    #[strum(
        to_string = "VCA",
        props(
            category = "IncompMassBasedBinaryMixture",
            min_fraction = "0.147",
            max_fraction = "0.299"
        )
    )]
    VCA,

    #[strum(
        to_string = "VKC",
        props(
            category = "IncompMassBasedBinaryMixture",
            min_fraction = "0.128",
            max_fraction = "0.389"
        )
    )]
    VKC,

    #[strum(
        to_string = "VMA",
        props(
            category = "IncompMassBasedBinaryMixture",
            min_fraction = "0.1",
            max_fraction = "0.9"
        )
    )]
    VMA,

    #[strum(
        to_string = "VMG",
        props(
            category = "IncompMassBasedBinaryMixture",
            min_fraction = "0.072",
            max_fraction = "0.206"
        )
    )]
    VMG,

    #[strum(
        to_string = "VNA",
        props(
            category = "IncompMassBasedBinaryMixture",
            min_fraction = "0.07",
            max_fraction = "0.231"
        )
    )]
    VNA,

    #[strum(
        to_string = "AEG",
        props(
            category = "IncompVolumeBasedBinaryMixture",
            min_fraction = "0.1",
            max_fraction = "0.6"
        )
    )]
    AEG,

    #[strum(
        to_string = "AKF",
        props(
            category = "IncompVolumeBasedBinaryMixture",
            min_fraction = "0.4",
            max_fraction = "1.0"
        )
    )]
    AKF,

    #[strum(
        to_string = "AL",
        props(
            category = "IncompVolumeBasedBinaryMixture",
            min_fraction = "0.1",
            max_fraction = "0.6"
        )
    )]
    AL,

    #[strum(
        to_string = "AN",
        props(
            category = "IncompVolumeBasedBinaryMixture",
            min_fraction = "0.1",
            max_fraction = "0.6"
        )
    )]
    AN,

    #[strum(
        to_string = "APG",
        props(
            category = "IncompVolumeBasedBinaryMixture",
            min_fraction = "0.1",
            max_fraction = "0.6"
        )
    )]
    APG,

    #[strum(
        to_string = "GKN",
        props(
            category = "IncompVolumeBasedBinaryMixture",
            min_fraction = "0.1",
            max_fraction = "0.6"
        )
    )]
    GKN,

    #[strum(
        to_string = "PK2",
        props(
            category = "IncompVolumeBasedBinaryMixture",
            min_fraction = "0.3",
            max_fraction = "1.0"
        )
    )]
    PK2,

    #[strum(
        to_string = "PKL",
        props(
            category = "IncompVolumeBasedBinaryMixture",
            min_fraction = "0.1",
            max_fraction = "0.6"
        )
    )]
    PKL,

    #[strum(
        to_string = "ZAC",
        props(
            category = "IncompVolumeBasedBinaryMixture",
            min_fraction = "0.06",
            max_fraction = "0.5"
        )
    )]
    ZAC,

    #[strum(
        to_string = "ZFC",
        props(
            category = "IncompVolumeBasedBinaryMixture",
            min_fraction = "0.3",
            max_fraction = "0.6"
        )
    )]
    ZFC,

    #[strum(
        to_string = "ZLC",
        props(
            category = "IncompVolumeBasedBinaryMixture",
            min_fraction = "0.3",
            max_fraction = "0.7"
        )
    )]
    ZLC,

    #[strum(
        to_string = "ZM",
        props(
            category = "IncompVolumeBasedBinaryMixture",
            min_fraction = "0.0",
            max_fraction = "1.0"
        )
    )]
    ZM,

    #[strum(
        to_string = "ZMC",
        props(
            category = "IncompVolumeBasedBinaryMixture",
            min_fraction = "0.3",
            max_fraction = "0.7"
        )
    )]
    ZMC,

    #[strum(
        to_string = "Air.mix",
        serialize = "AirMix",
        serialize = "Air-mix",
        props(category = "PredefinedMixture")
    )]
    AirMix,

    #[strum(
        to_string = "Amarillo.mix",
        serialize = "Amarillo",
        props(category = "PredefinedMixture")
    )]
    Amarillo,

    #[strum(
        to_string = "Ekofisk.mix",
        serialize = "Ekofisk",
        props(category = "PredefinedMixture")
    )]
    Ekofisk,

    #[strum(
        to_string = "GulfCoast.mix",
        serialize = "GulfCoast",
        props(category = "PredefinedMixture")
    )]
    GulfCoast,

    #[strum(
        to_string = "GulfCoastGas(NIST1).mix",
        serialize = "GulfCoastGasNIST",
        props(category = "PredefinedMixture")
    )]
    GulfCoastGasNIST,

    #[strum(
        to_string = "HighCO2.mix",
        serialize = "HighCO2",
        props(category = "PredefinedMixture")
    )]
    HighCO2,

    #[strum(
        to_string = "HighN2.mix",
        serialize = "HighN2",
        props(category = "PredefinedMixture")
    )]
    HighN2,

    #[strum(
        to_string = "NaturalGasSample.mix",
        serialize = "NaturalGasSample",
        props(category = "PredefinedMixture")
    )]
    NaturalGasSample,

    #[strum(
        to_string = "R401A.mix",
        serialize = "R401A",
        props(category = "PredefinedMixture")
    )]
    R401A,

    #[strum(
        to_string = "R401B.mix",
        serialize = "R401B",
        props(category = "PredefinedMixture")
    )]
    R401B,

    #[strum(
        to_string = "R401C.mix",
        serialize = "R401C",
        props(category = "PredefinedMixture")
    )]
    R401C,

    #[strum(
        to_string = "R402A.mix",
        serialize = "R402A",
        props(category = "PredefinedMixture")
    )]
    R402A,

    #[strum(
        to_string = "R402B.mix",
        serialize = "R402B",
        props(category = "PredefinedMixture")
    )]
    R402B,

    #[strum(
        to_string = "R403A.mix",
        serialize = "R403A",
        props(category = "PredefinedMixture")
    )]
    R403A,

    #[strum(
        to_string = "R403B.mix",
        serialize = "R403B",
        props(category = "PredefinedMixture")
    )]
    R403B,

    #[strum(
        to_string = "R404A.mix",
        serialize = "R404AMix",
        serialize = "R404A-mix",
        props(category = "PredefinedMixture")
    )]
    R404AMix,

    #[strum(
        to_string = "R405A.mix",
        serialize = "R405A",
        props(category = "PredefinedMixture")
    )]
    R405A,

    #[strum(
        to_string = "R406A.mix",
        serialize = "R406A",
        props(category = "PredefinedMixture")
    )]
    R406A,

    #[strum(
        to_string = "R407A.mix",
        serialize = "R407A",
        props(category = "PredefinedMixture")
    )]
    R407A,

    #[strum(
        to_string = "R407B.mix",
        serialize = "R407B",
        props(category = "PredefinedMixture")
    )]
    R407B,

    #[strum(
        to_string = "R407C.mix",
        serialize = "R407CMix",
        serialize = "R407C-mix",
        props(category = "PredefinedMixture")
    )]
    R407CMix,

    #[strum(
        to_string = "R407D.mix",
        serialize = "R407D",
        props(category = "PredefinedMixture")
    )]
    R407D,

    #[strum(
        to_string = "R407E.mix",
        serialize = "R407E",
        props(category = "PredefinedMixture")
    )]
    R407E,

    #[strum(
        to_string = "R407F.mix",
        serialize = "R407F",
        props(category = "PredefinedMixture")
    )]
    R407F,

    #[strum(
        to_string = "R408A.mix",
        serialize = "R408A",
        props(category = "PredefinedMixture")
    )]
    R408A,

    #[strum(
        to_string = "R409A.mix",
        serialize = "R409A",
        props(category = "PredefinedMixture")
    )]
    R409A,

    #[strum(
        to_string = "R409B.mix",
        serialize = "R409B",
        props(category = "PredefinedMixture")
    )]
    R409B,

    #[strum(
        to_string = "R410A.mix",
        serialize = "R410AMix",
        serialize = "R410A-mix",
        props(category = "PredefinedMixture")
    )]
    R410AMix,

    #[strum(
        to_string = "R410B.mix",
        serialize = "R410B",
        props(category = "PredefinedMixture")
    )]
    R410B,

    #[strum(
        to_string = "R411A.mix",
        serialize = "R411A",
        props(category = "PredefinedMixture")
    )]
    R411A,

    #[strum(
        to_string = "R411B.mix",
        serialize = "R411B",
        props(category = "PredefinedMixture")
    )]
    R411B,

    #[strum(
        to_string = "R412A.mix",
        serialize = "R412A",
        props(category = "PredefinedMixture")
    )]
    R412A,

    #[strum(
        to_string = "R413A.mix",
        serialize = "R413A",
        props(category = "PredefinedMixture")
    )]
    R413A,

    #[strum(
        to_string = "R414A.mix",
        serialize = "R414A",
        props(category = "PredefinedMixture")
    )]
    R414A,

    #[strum(
        to_string = "R414B.mix",
        serialize = "R414B",
        props(category = "PredefinedMixture")
    )]
    R414B,

    #[strum(
        to_string = "R415A.mix",
        serialize = "R415A",
        props(category = "PredefinedMixture")
    )]
    R415A,

    #[strum(
        to_string = "R415B.mix",
        serialize = "R415B",
        props(category = "PredefinedMixture")
    )]
    R415B,

    #[strum(
        to_string = "R416A.mix",
        serialize = "R416A",
        props(category = "PredefinedMixture")
    )]
    R416A,

    #[strum(
        to_string = "R417A.mix",
        serialize = "R417A",
        props(category = "PredefinedMixture")
    )]
    R417A,

    #[strum(
        to_string = "R417B.mix",
        serialize = "R417B",
        props(category = "PredefinedMixture")
    )]
    R417B,

    #[strum(
        to_string = "R417C.mix",
        serialize = "R417C",
        props(category = "PredefinedMixture")
    )]
    R417C,

    #[strum(
        to_string = "R418A.mix",
        serialize = "R418A",
        props(category = "PredefinedMixture")
    )]
    R418A,

    #[strum(
        to_string = "R419A.mix",
        serialize = "R419A",
        props(category = "PredefinedMixture")
    )]
    R419A,

    #[strum(
        to_string = "R419B.mix",
        serialize = "R419B",
        props(category = "PredefinedMixture")
    )]
    R419B,

    #[strum(
        to_string = "R420A.mix",
        serialize = "R420A",
        props(category = "PredefinedMixture")
    )]
    R420A,

    #[strum(
        to_string = "R421A.mix",
        serialize = "R421A",
        props(category = "PredefinedMixture")
    )]
    R421A,

    #[strum(
        to_string = "R421B.mix",
        serialize = "R421B",
        props(category = "PredefinedMixture")
    )]
    R421B,

    #[strum(
        to_string = "R422A.mix",
        serialize = "R422A",
        props(category = "PredefinedMixture")
    )]
    R422A,

    #[strum(
        to_string = "R422B.mix",
        serialize = "R422B",
        props(category = "PredefinedMixture")
    )]
    R422B,

    #[strum(
        to_string = "R422C.mix",
        serialize = "R422C",
        props(category = "PredefinedMixture")
    )]
    R422C,

    #[strum(
        to_string = "R422D.mix",
        serialize = "R422D",
        props(category = "PredefinedMixture")
    )]
    R422D,

    #[strum(
        to_string = "R422E.mix",
        serialize = "R422E",
        props(category = "PredefinedMixture")
    )]
    R422E,

    #[strum(
        to_string = "R423A.mix",
        serialize = "R423A",
        props(category = "PredefinedMixture")
    )]
    R423A,

    #[strum(
        to_string = "R424A.mix",
        serialize = "R424A",
        props(category = "PredefinedMixture")
    )]
    R424A,

    #[strum(
        to_string = "R425A.mix",
        serialize = "R425A",
        props(category = "PredefinedMixture")
    )]
    R425A,

    #[strum(
        to_string = "R426A.mix",
        serialize = "R426A",
        props(category = "PredefinedMixture")
    )]
    R426A,

    #[strum(
        to_string = "R427A.mix",
        serialize = "R427A",
        props(category = "PredefinedMixture")
    )]
    R427A,

    #[strum(
        to_string = "R428A.mix",
        serialize = "R428A",
        props(category = "PredefinedMixture")
    )]
    R428A,

    #[strum(
        to_string = "R429A.mix",
        serialize = "R429A",
        props(category = "PredefinedMixture")
    )]
    R429A,

    #[strum(
        to_string = "R430A.mix",
        serialize = "R430A",
        props(category = "PredefinedMixture")
    )]
    R430A,

    #[strum(
        to_string = "R431A.mix",
        serialize = "R431A",
        props(category = "PredefinedMixture")
    )]
    R431A,

    #[strum(
        to_string = "R432A.mix",
        serialize = "R432A",
        props(category = "PredefinedMixture")
    )]
    R432A,

    #[strum(
        to_string = "R433A.mix",
        serialize = "R433A",
        props(category = "PredefinedMixture")
    )]
    R433A,

    #[strum(
        to_string = "R433B.mix",
        serialize = "R433B",
        props(category = "PredefinedMixture")
    )]
    R433B,

    #[strum(
        to_string = "R433C.mix",
        serialize = "R433C",
        props(category = "PredefinedMixture")
    )]
    R433C,

    #[strum(
        to_string = "R434A.mix",
        serialize = "R434A",
        props(category = "PredefinedMixture")
    )]
    R434A,

    #[strum(
        to_string = "R435A.mix",
        serialize = "R435A",
        props(category = "PredefinedMixture")
    )]
    R435A,

    #[strum(
        to_string = "R436A.mix",
        serialize = "R436A",
        props(category = "PredefinedMixture")
    )]
    R436A,

    #[strum(
        to_string = "R436B.mix",
        serialize = "R436B",
        props(category = "PredefinedMixture")
    )]
    R436B,

    #[strum(
        to_string = "R437A.mix",
        serialize = "R437A",
        props(category = "PredefinedMixture")
    )]
    R437A,

    #[strum(
        to_string = "R438A.mix",
        serialize = "R438A",
        props(category = "PredefinedMixture")
    )]
    R438A,

    #[strum(
        to_string = "R439A.mix",
        serialize = "R439A",
        props(category = "PredefinedMixture")
    )]
    R439A,

    #[strum(
        to_string = "R440A.mix",
        serialize = "R440A",
        props(category = "PredefinedMixture")
    )]
    R440A,

    #[strum(
        to_string = "R441A.mix",
        serialize = "R441A",
        props(category = "PredefinedMixture")
    )]
    R441A,

    #[strum(
        to_string = "R442A.mix",
        serialize = "R442A",
        props(category = "PredefinedMixture")
    )]
    R442A,

    #[strum(
        to_string = "R443A.mix",
        serialize = "R443A",
        props(category = "PredefinedMixture")
    )]
    R443A,

    #[strum(
        to_string = "R444A.mix",
        serialize = "R444A",
        props(category = "PredefinedMixture")
    )]
    R444A,

    #[strum(
        to_string = "R444B.mix",
        serialize = "R444B",
        props(category = "PredefinedMixture")
    )]
    R444B,

    #[strum(
        to_string = "R445A.mix",
        serialize = "R445A",
        props(category = "PredefinedMixture")
    )]
    R445A,

    #[strum(
        to_string = "R446A.mix",
        serialize = "R446A",
        props(category = "PredefinedMixture")
    )]
    R446A,

    #[strum(
        to_string = "R447A.mix",
        serialize = "R447A",
        props(category = "PredefinedMixture")
    )]
    R447A,

    #[strum(
        to_string = "R448A.mix",
        serialize = "R448A",
        props(category = "PredefinedMixture")
    )]
    R448A,

    #[strum(
        to_string = "R449A.mix",
        serialize = "R449A",
        props(category = "PredefinedMixture")
    )]
    R449A,

    #[strum(
        to_string = "R449B.mix",
        serialize = "R449B",
        props(category = "PredefinedMixture")
    )]
    R449B,

    #[strum(
        to_string = "R450A.mix",
        serialize = "R450A",
        props(category = "PredefinedMixture")
    )]
    R450A,

    #[strum(
        to_string = "R451A.mix",
        serialize = "R451A",
        props(category = "PredefinedMixture")
    )]
    R451A,

    #[strum(
        to_string = "R451B.mix",
        serialize = "R451B",
        props(category = "PredefinedMixture")
    )]
    R451B,

    #[strum(
        to_string = "R452A.mix",
        serialize = "R452A",
        props(category = "PredefinedMixture")
    )]
    R452A,

    #[strum(
        to_string = "R453A.mix",
        serialize = "R453A",
        props(category = "PredefinedMixture")
    )]
    R453A,

    #[strum(
        to_string = "R454A.mix",
        serialize = "R454A",
        props(category = "PredefinedMixture")
    )]
    R454A,

    #[strum(
        to_string = "R454B.mix",
        serialize = "R454B",
        props(category = "PredefinedMixture")
    )]
    R454B,

    #[strum(
        to_string = "R500.mix",
        serialize = "R500",
        props(category = "PredefinedMixture")
    )]
    R500,

    #[strum(
        to_string = "R501.mix",
        serialize = "R501",
        props(category = "PredefinedMixture")
    )]
    R501,

    #[strum(
        to_string = "R502.mix",
        serialize = "R502",
        props(category = "PredefinedMixture")
    )]
    R502,

    #[strum(
        to_string = "R503.mix",
        serialize = "R503",
        props(category = "PredefinedMixture")
    )]
    R503,

    #[strum(
        to_string = "R504.mix",
        serialize = "R504",
        props(category = "PredefinedMixture")
    )]
    R504,

    #[strum(
        to_string = "R507A.mix",
        serialize = "R507AMix",
        serialize = "R507A-mix",
        props(category = "PredefinedMixture")
    )]
    R507AMix,

    #[strum(
        to_string = "R508A.mix",
        serialize = "R508A",
        props(category = "PredefinedMixture")
    )]
    R508A,

    #[strum(
        to_string = "R508B.mix",
        serialize = "R508B",
        props(category = "PredefinedMixture")
    )]
    R508B,

    #[strum(
        to_string = "R509A.mix",
        serialize = "R509A",
        props(category = "PredefinedMixture")
    )]
    R509A,

    #[strum(
        to_string = "R510A.mix",
        serialize = "R510A",
        props(category = "PredefinedMixture")
    )]
    R510A,

    #[strum(
        to_string = "R511A.mix",
        serialize = "R511A",
        props(category = "PredefinedMixture")
    )]
    R511A,

    #[strum(
        to_string = "R512A.mix",
        serialize = "R512A",
        props(category = "PredefinedMixture")
    )]
    R512A,

    #[strum(
        to_string = "R513A.mix",
        serialize = "R513A",
        props(category = "PredefinedMixture")
    )]
    R513A,

    #[strum(
        to_string = "TypicalNaturalGas.mix",
        serialize = "TypicalNaturalGas",
        serialize = "NaturalGas",
        props(category = "PredefinedMixture")
    )]
    TypicalNaturalGas,
}

impl Substance {
    /// Substance category.
    ///
    /// # Examples
    ///
    /// ```
    /// use fluids_rs::enums::{Substance, SubstanceCategory};
    ///
    /// assert_eq!(Substance::Water.category(), SubstanceCategory::Pure);
    /// assert_eq!(Substance::Air.category(), SubstanceCategory::PseudoPure);
    /// assert_eq!(Substance::WaterIncomp.category(), SubstanceCategory::IncompPure);
    /// assert_eq!(Substance::MPG.category(), SubstanceCategory::IncompMassBasedBinaryMixture);
    /// assert_eq!(Substance::AEG.category(), SubstanceCategory::IncompVolumeBasedBinaryMixture);
    /// assert_eq!(Substance::AEG.category(), SubstanceCategory::IncompVolumeBasedBinaryMixture);
    /// assert_eq!(Substance::TypicalNaturalGas.category(), SubstanceCategory::PredefinedMixture);
    /// ```
    pub fn category(&self) -> SubstanceCategory {
        SubstanceCategory::from_str(self.get_str("category").unwrap()).unwrap()
    }

    /// CoolProp backend name.
    ///
    /// # Examples
    ///
    /// ```
    /// use fluids_rs::enums::Substance;
    ///
    /// assert_eq!(Substance::Water.backend(), "HEOS");
    /// assert_eq!(Substance::MPG.backend(), "INCOMP");
    /// ```
    pub fn backend(&self) -> &'static str {
        if matches!(
            self.category(),
            SubstanceCategory::IncompPure
                | SubstanceCategory::IncompMassBasedBinaryMixture
                | SubstanceCategory::IncompVolumeBasedBinaryMixture
        ) {
            "INCOMP"
        } else {
            "HEOS"
        }
    }

    /// Minimum possible fraction of binary mixture _(dimensionless, from 0 to 1)_.
    ///
    /// # Examples
    ///
    /// ```
    /// use fluids_rs::enums::Substance;
    ///
    /// assert_eq!(Substance::MPG.min_fraction(), Some(0.0));
    /// assert_eq!(Substance::Water.min_fraction(), None);
    /// ```
    pub fn min_fraction(&self) -> Option<f64> {
        f64::from_str(self.get_str("min_fraction")?).ok()
    }

    /// Maximum possible fraction of binary mixture _(dimensionless, from 0 to 1)_.
    ///
    /// # Examples
    ///
    /// ```
    /// use fluids_rs::enums::Substance;
    ///
    /// assert_eq!(Substance::MPG.max_fraction(), Some(0.6));
    /// assert_eq!(Substance::Water.max_fraction(), None);
    /// ```
    pub fn max_fraction(&self) -> Option<f64> {
        f64::from_str(self.get_str("max_fraction")?).ok()
    }
}

/// CoolProp substance categories.
#[derive(AsRefStr, EnumString, Debug, Copy, Clone, Eq, PartialEq)]
#[strum(ascii_case_insensitive)]
pub enum SubstanceCategory {
    Pure,
    PseudoPure,
    IncompPure,
    IncompMassBasedBinaryMixture,
    IncompVolumeBasedBinaryMixture,
    PredefinedMixture,
}

#[cfg(test)]
mod tests {
    use super::Substance::*;
    use super::SubstanceCategory::*;
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(Acetone, "HEOS", Pure, None, None)]
    #[case(Air, "HEOS", PseudoPure, None, None)]
    #[case(R729, "HEOS", Pure, None, None)]
    #[case(Ammonia, "HEOS", Pure, None, None)]
    #[case(R717, "HEOS", Pure, None, None)]
    #[case(Argon, "HEOS", Pure, None, None)]
    #[case(R740, "HEOS", Pure, None, None)]
    #[case(Benzene, "HEOS", Pure, None, None)]
    #[case(Butene, "HEOS", Pure, None, None)]
    #[case(CarbonDioxide, "HEOS", Pure, None, None)]
    #[case(R744, "HEOS", Pure, None, None)]
    #[case(CarbonMonoxide, "HEOS", Pure, None, None)]
    #[case(CarbonylSulfide, "HEOS", Pure, None, None)]
    #[case(cis2Butene, "HEOS", Pure, None, None)]
    #[case(Cyclohexane, "HEOS", Pure, None, None)]
    #[case(Cyclopentane, "HEOS", Pure, None, None)]
    #[case(Cyclopropane, "HEOS", Pure, None, None)]
    #[case(D4, "HEOS", Pure, None, None)]
    #[case(D5, "HEOS", Pure, None, None)]
    #[case(D6, "HEOS", Pure, None, None)]
    #[case(Deuterium, "HEOS", Pure, None, None)]
    #[case(Dichloroethane, "HEOS", Pure, None, None)]
    #[case(DiethylEther, "HEOS", Pure, None, None)]
    #[case(DimethylCarbonate, "HEOS", Pure, None, None)]
    #[case(DimethylEther, "HEOS", Pure, None, None)]
    #[case(Ethane, "HEOS", Pure, None, None)]
    #[case(R170, "HEOS", Pure, None, None)]
    #[case(Ethanol, "HEOS", Pure, None, None)]
    #[case(EthylBenzene, "HEOS", Pure, None, None)]
    #[case(Ethylene, "HEOS", Pure, None, None)]
    #[case(R1150, "HEOS", Pure, None, None)]
    #[case(EthyleneOxide, "HEOS", Pure, None, None)]
    #[case(Fluorine, "HEOS", Pure, None, None)]
    #[case(HeavyWater, "HEOS", Pure, None, None)]
    #[case(Helium, "HEOS", Pure, None, None)]
    #[case(R704, "HEOS", Pure, None, None)]
    #[case(HFE143m, "HEOS", Pure, None, None)]
    #[case(RE143a, "HEOS", Pure, None, None)]
    #[case(Hydrogen, "HEOS", Pure, None, None)]
    #[case(R702, "HEOS", Pure, None, None)]
    #[case(HydrogenChloride, "HEOS", Pure, None, None)]
    #[case(HydrogenSulfide, "HEOS", Pure, None, None)]
    #[case(Isobutane, "HEOS", Pure, None, None)]
    #[case(R600a, "HEOS", Pure, None, None)]
    #[case(Isobutene, "HEOS", Pure, None, None)]
    #[case(Isohexane, "HEOS", Pure, None, None)]
    #[case(Isopentane, "HEOS", Pure, None, None)]
    #[case(R601a, "HEOS", Pure, None, None)]
    #[case(Krypton, "HEOS", Pure, None, None)]
    #[case(MD2M, "HEOS", Pure, None, None)]
    #[case(MD3M, "HEOS", Pure, None, None)]
    #[case(MD4M, "HEOS", Pure, None, None)]
    #[case(MDM, "HEOS", Pure, None, None)]
    #[case(Methane, "HEOS", Pure, None, None)]
    #[case(R50, "HEOS", Pure, None, None)]
    #[case(Methanol, "HEOS", Pure, None, None)]
    #[case(MethylLinoleate, "HEOS", Pure, None, None)]
    #[case(MethylLinolenate, "HEOS", Pure, None, None)]
    #[case(MethylOleate, "HEOS", Pure, None, None)]
    #[case(MethylPalmitate, "HEOS", Pure, None, None)]
    #[case(MethylStearate, "HEOS", Pure, None, None)]
    #[case(MM, "HEOS", Pure, None, None)]
    #[case(mXylene, "HEOS", Pure, None, None)]
    #[case(nButane, "HEOS", Pure, None, None)]
    #[case(R600, "HEOS", Pure, None, None)]
    #[case(nDecane, "HEOS", Pure, None, None)]
    #[case(nDodecane, "HEOS", Pure, None, None)]
    #[case(Neon, "HEOS", Pure, None, None)]
    #[case(R720, "HEOS", Pure, None, None)]
    #[case(Neopentane, "HEOS", Pure, None, None)]
    #[case(nHeptane, "HEOS", Pure, None, None)]
    #[case(nHexane, "HEOS", Pure, None, None)]
    #[case(Nitrogen, "HEOS", Pure, None, None)]
    #[case(R728, "HEOS", Pure, None, None)]
    #[case(NitrousOxide, "HEOS", Pure, None, None)]
    #[case(nNonane, "HEOS", Pure, None, None)]
    #[case(nOctane, "HEOS", Pure, None, None)]
    #[case(Novec649, "HEOS", Pure, None, None)]
    #[case(nPentane, "HEOS", Pure, None, None)]
    #[case(R601, "HEOS", Pure, None, None)]
    #[case(nPropane, "HEOS", Pure, None, None)]
    #[case(R290, "HEOS", Pure, None, None)]
    #[case(nUndecane, "HEOS", Pure, None, None)]
    #[case(Orthodeuterium, "HEOS", Pure, None, None)]
    #[case(Orthohydrogen, "HEOS", Pure, None, None)]
    #[case(Oxygen, "HEOS", Pure, None, None)]
    #[case(R732, "HEOS", Pure, None, None)]
    #[case(oXylene, "HEOS", Pure, None, None)]
    #[case(Paradeuterium, "HEOS", Pure, None, None)]
    #[case(Parahydrogen, "HEOS", Pure, None, None)]
    #[case(Propylene, "HEOS", Pure, None, None)]
    #[case(R1270, "HEOS", Pure, None, None)]
    #[case(Propyne, "HEOS", Pure, None, None)]
    #[case(pXylene, "HEOS", Pure, None, None)]
    #[case(R11, "HEOS", Pure, None, None)]
    #[case(R113, "HEOS", Pure, None, None)]
    #[case(R114, "HEOS", Pure, None, None)]
    #[case(R115, "HEOS", Pure, None, None)]
    #[case(R116, "HEOS", Pure, None, None)]
    #[case(R12, "HEOS", Pure, None, None)]
    #[case(R123, "HEOS", Pure, None, None)]
    #[case(R1233zdE, "HEOS", Pure, None, None)]
    #[case(R1234yf, "HEOS", Pure, None, None)]
    #[case(R1234zeE, "HEOS", Pure, None, None)]
    #[case(R1234zeZ, "HEOS", Pure, None, None)]
    #[case(R124, "HEOS", Pure, None, None)]
    #[case(R1243zf, "HEOS", Pure, None, None)]
    #[case(R125, "HEOS", Pure, None, None)]
    #[case(R13, "HEOS", Pure, None, None)]
    #[case(R134a, "HEOS", Pure, None, None)]
    #[case(R13I1, "HEOS", Pure, None, None)]
    #[case(R14, "HEOS", Pure, None, None)]
    #[case(R141b, "HEOS", Pure, None, None)]
    #[case(R142b, "HEOS", Pure, None, None)]
    #[case(R143a, "HEOS", Pure, None, None)]
    #[case(R152a, "HEOS", Pure, None, None)]
    #[case(R161, "HEOS", Pure, None, None)]
    #[case(R21, "HEOS", Pure, None, None)]
    #[case(R218, "HEOS", Pure, None, None)]
    #[case(R22, "HEOS", Pure, None, None)]
    #[case(R227ea, "HEOS", Pure, None, None)]
    #[case(R23, "HEOS", Pure, None, None)]
    #[case(R236ea, "HEOS", Pure, None, None)]
    #[case(R236fa, "HEOS", Pure, None, None)]
    #[case(R245ca, "HEOS", Pure, None, None)]
    #[case(R245fa, "HEOS", Pure, None, None)]
    #[case(R32, "HEOS", Pure, None, None)]
    #[case(R365mfc, "HEOS", Pure, None, None)]
    #[case(R40, "HEOS", Pure, None, None)]
    #[case(R404A, "HEOS", PseudoPure, None, None)]
    #[case(R407C, "HEOS", PseudoPure, None, None)]
    #[case(R41, "HEOS", Pure, None, None)]
    #[case(R410A, "HEOS", PseudoPure, None, None)]
    #[case(R507A, "HEOS", PseudoPure, None, None)]
    #[case(RC318, "HEOS", Pure, None, None)]
    #[case(SES36, "HEOS", Pure, None, None)]
    #[case(SulfurDioxide, "HEOS", Pure, None, None)]
    #[case(R764, "HEOS", Pure, None, None)]
    #[case(SulfurHexafluoride, "HEOS", Pure, None, None)]
    #[case(R846, "HEOS", Pure, None, None)]
    #[case(Toluene, "HEOS", Pure, None, None)]
    #[case(trans2Butene, "HEOS", Pure, None, None)]
    #[case(Water, "HEOS", Pure, None, None)]
    #[case(R718, "HEOS", Pure, None, None)]
    #[case(Xenon, "HEOS", Pure, None, None)]
    #[case(AS10, "INCOMP", IncompPure, None, None)]
    #[case(AS20, "INCOMP", IncompPure, None, None)]
    #[case(AS30, "INCOMP", IncompPure, None, None)]
    #[case(AS40, "INCOMP", IncompPure, None, None)]
    #[case(AS55, "INCOMP", IncompPure, None, None)]
    #[case(DEB, "INCOMP", IncompPure, None, None)]
    #[case(DowJ, "INCOMP", IncompPure, None, None)]
    #[case(DowJ2, "INCOMP", IncompPure, None, None)]
    #[case(DowQ, "INCOMP", IncompPure, None, None)]
    #[case(DowQ2, "INCOMP", IncompPure, None, None)]
    #[case(DSF, "INCOMP", IncompPure, None, None)]
    #[case(HC10, "INCOMP", IncompPure, None, None)]
    #[case(HC20, "INCOMP", IncompPure, None, None)]
    #[case(HC30, "INCOMP", IncompPure, None, None)]
    #[case(HC40, "INCOMP", IncompPure, None, None)]
    #[case(HC50, "INCOMP", IncompPure, None, None)]
    #[case(HCB, "INCOMP", IncompPure, None, None)]
    #[case(HCM, "INCOMP", IncompPure, None, None)]
    #[case(HFE, "INCOMP", IncompPure, None, None)]
    #[case(HFE2, "INCOMP", IncompPure, None, None)]
    #[case(HY20, "INCOMP", IncompPure, None, None)]
    #[case(HY30, "INCOMP", IncompPure, None, None)]
    #[case(HY40, "INCOMP", IncompPure, None, None)]
    #[case(HY45, "INCOMP", IncompPure, None, None)]
    #[case(HY50, "INCOMP", IncompPure, None, None)]
    #[case(NaK, "INCOMP", IncompPure, None, None)]
    #[case(NBS, "INCOMP", IncompPure, None, None)]
    #[case(PBB, "INCOMP", IncompPure, None, None)]
    #[case(PCL, "INCOMP", IncompPure, None, None)]
    #[case(PCR, "INCOMP", IncompPure, None, None)]
    #[case(PGLT, "INCOMP", IncompPure, None, None)]
    #[case(PHE, "INCOMP", IncompPure, None, None)]
    #[case(PHR, "INCOMP", IncompPure, None, None)]
    #[case(PLR, "INCOMP", IncompPure, None, None)]
    #[case(PMR, "INCOMP", IncompPure, None, None)]
    #[case(PMS1, "INCOMP", IncompPure, None, None)]
    #[case(PMS2, "INCOMP", IncompPure, None, None)]
    #[case(PNF, "INCOMP", IncompPure, None, None)]
    #[case(PNF2, "INCOMP", IncompPure, None, None)]
    #[case(S800, "INCOMP", IncompPure, None, None)]
    #[case(SAB, "INCOMP", IncompPure, None, None)]
    #[case(T66, "INCOMP", IncompPure, None, None)]
    #[case(T72, "INCOMP", IncompPure, None, None)]
    #[case(TCO, "INCOMP", IncompPure, None, None)]
    #[case(TD12, "INCOMP", IncompPure, None, None)]
    #[case(TVP1, "INCOMP", IncompPure, None, None)]
    #[case(TVP1869, "INCOMP", IncompPure, None, None)]
    #[case(TX22, "INCOMP", IncompPure, None, None)]
    #[case(TY10, "INCOMP", IncompPure, None, None)]
    #[case(TY15, "INCOMP", IncompPure, None, None)]
    #[case(TY20, "INCOMP", IncompPure, None, None)]
    #[case(TY24, "INCOMP", IncompPure, None, None)]
    #[case(WaterIncomp, "INCOMP", IncompPure, None, None)]
    #[case(XLT, "INCOMP", IncompPure, None, None)]
    #[case(XLT2, "INCOMP", IncompPure, None, None)]
    #[case(ZS10, "INCOMP", IncompPure, None, None)]
    #[case(ZS25, "INCOMP", IncompPure, None, None)]
    #[case(ZS40, "INCOMP", IncompPure, None, None)]
    #[case(ZS45, "INCOMP", IncompPure, None, None)]
    #[case(ZS55, "INCOMP", IncompPure, None, None)]
    #[case(FRE, "INCOMP", IncompMassBasedBinaryMixture, Some(0.19), Some(0.5))]
    #[case(IceEA, "INCOMP", IncompMassBasedBinaryMixture, Some(0.05), Some(0.35))]
    #[case(IceNA, "INCOMP", IncompMassBasedBinaryMixture, Some(0.05), Some(0.35))]
    #[case(IcePG, "INCOMP", IncompMassBasedBinaryMixture, Some(0.05), Some(0.35))]
    #[case(LiBr, "INCOMP", IncompMassBasedBinaryMixture, Some(0.0), Some(0.75))]
    #[case(MAM, "INCOMP", IncompMassBasedBinaryMixture, Some(0.0), Some(0.3))]
    #[case(MAM2, "INCOMP", IncompMassBasedBinaryMixture, Some(0.078), Some(0.236))]
    #[case(MCA, "INCOMP", IncompMassBasedBinaryMixture, Some(0.0), Some(0.3))]
    #[case(MCA2, "INCOMP", IncompMassBasedBinaryMixture, Some(0.09), Some(0.294))]
    #[case(MEA, "INCOMP", IncompMassBasedBinaryMixture, Some(0.0), Some(0.6))]
    #[case(MEA2, "INCOMP", IncompMassBasedBinaryMixture, Some(0.11), Some(0.6))]
    #[case(MEG, "INCOMP", IncompMassBasedBinaryMixture, Some(0.0), Some(0.6))]
    #[case(MEG2, "INCOMP", IncompMassBasedBinaryMixture, Some(0.0), Some(0.56))]
    #[case(MGL, "INCOMP", IncompMassBasedBinaryMixture, Some(0.0), Some(0.6))]
    #[case(MGL2, "INCOMP", IncompMassBasedBinaryMixture, Some(0.195), Some(0.63))]
    #[case(MITSW, "INCOMP", IncompMassBasedBinaryMixture, Some(0.0), Some(0.12))]
    #[case(MKA, "INCOMP", IncompMassBasedBinaryMixture, Some(0.0), Some(0.45))]
    #[case(MKA2, "INCOMP", IncompMassBasedBinaryMixture, Some(0.11), Some(0.41))]
    #[case(MKC, "INCOMP", IncompMassBasedBinaryMixture, Some(0.0), Some(0.4))]
    #[case(MKC2, "INCOMP", IncompMassBasedBinaryMixture, Some(0.0), Some(0.39))]
    #[case(MKF, "INCOMP", IncompMassBasedBinaryMixture, Some(0.0), Some(0.48))]
    #[case(MLI, "INCOMP", IncompMassBasedBinaryMixture, Some(0.0), Some(0.24))]
    #[case(MMA, "INCOMP", IncompMassBasedBinaryMixture, Some(0.0), Some(0.6))]
    #[case(MMA2, "INCOMP", IncompMassBasedBinaryMixture, Some(0.078), Some(0.474))]
    #[case(MMG, "INCOMP", IncompMassBasedBinaryMixture, Some(0.0), Some(0.3))]
    #[case(MMG2, "INCOMP", IncompMassBasedBinaryMixture, Some(0.0), Some(0.205))]
    #[case(MNA, "INCOMP", IncompMassBasedBinaryMixture, Some(0.0), Some(0.23))]
    #[case(MNA2, "INCOMP", IncompMassBasedBinaryMixture, Some(0.0), Some(0.23))]
    #[case(MPG, "INCOMP", IncompMassBasedBinaryMixture, Some(0.0), Some(0.6))]
    #[case(MPG2, "INCOMP", IncompMassBasedBinaryMixture, Some(0.15), Some(0.57))]
    #[case(VCA, "INCOMP", IncompMassBasedBinaryMixture, Some(0.147), Some(0.299))]
    #[case(VKC, "INCOMP", IncompMassBasedBinaryMixture, Some(0.128), Some(0.389))]
    #[case(VMA, "INCOMP", IncompMassBasedBinaryMixture, Some(0.1), Some(0.9))]
    #[case(VMG, "INCOMP", IncompMassBasedBinaryMixture, Some(0.072), Some(0.206))]
    #[case(VNA, "INCOMP", IncompMassBasedBinaryMixture, Some(0.07), Some(0.231))]
    #[case(AEG, "INCOMP", IncompVolumeBasedBinaryMixture, Some(0.1), Some(0.6))]
    #[case(AKF, "INCOMP", IncompVolumeBasedBinaryMixture, Some(0.4), Some(1.0))]
    #[case(AL, "INCOMP", IncompVolumeBasedBinaryMixture, Some(0.1), Some(0.6))]
    #[case(AN, "INCOMP", IncompVolumeBasedBinaryMixture, Some(0.1), Some(0.6))]
    #[case(APG, "INCOMP", IncompVolumeBasedBinaryMixture, Some(0.1), Some(0.6))]
    #[case(GKN, "INCOMP", IncompVolumeBasedBinaryMixture, Some(0.1), Some(0.6))]
    #[case(PK2, "INCOMP", IncompVolumeBasedBinaryMixture, Some(0.3), Some(1.0))]
    #[case(PKL, "INCOMP", IncompVolumeBasedBinaryMixture, Some(0.1), Some(0.6))]
    #[case(ZAC, "INCOMP", IncompVolumeBasedBinaryMixture, Some(0.06), Some(0.5))]
    #[case(ZFC, "INCOMP", IncompVolumeBasedBinaryMixture, Some(0.3), Some(0.6))]
    #[case(ZLC, "INCOMP", IncompVolumeBasedBinaryMixture, Some(0.3), Some(0.7))]
    #[case(ZM, "INCOMP", IncompVolumeBasedBinaryMixture, Some(0.0), Some(1.0))]
    #[case(ZMC, "INCOMP", IncompVolumeBasedBinaryMixture, Some(0.3), Some(0.7))]
    #[case(AirMix, "HEOS", PredefinedMixture, None, None)]
    #[case(Amarillo, "HEOS", PredefinedMixture, None, None)]
    #[case(Ekofisk, "HEOS", PredefinedMixture, None, None)]
    #[case(GulfCoast, "HEOS", PredefinedMixture, None, None)]
    #[case(GulfCoastGasNIST, "HEOS", PredefinedMixture, None, None)]
    #[case(HighCO2, "HEOS", PredefinedMixture, None, None)]
    #[case(HighN2, "HEOS", PredefinedMixture, None, None)]
    #[case(NaturalGasSample, "HEOS", PredefinedMixture, None, None)]
    #[case(R401A, "HEOS", PredefinedMixture, None, None)]
    #[case(R401B, "HEOS", PredefinedMixture, None, None)]
    #[case(R401C, "HEOS", PredefinedMixture, None, None)]
    #[case(R402A, "HEOS", PredefinedMixture, None, None)]
    #[case(R402B, "HEOS", PredefinedMixture, None, None)]
    #[case(R403A, "HEOS", PredefinedMixture, None, None)]
    #[case(R403B, "HEOS", PredefinedMixture, None, None)]
    #[case(R404AMix, "HEOS", PredefinedMixture, None, None)]
    #[case(R405A, "HEOS", PredefinedMixture, None, None)]
    #[case(R406A, "HEOS", PredefinedMixture, None, None)]
    #[case(R407A, "HEOS", PredefinedMixture, None, None)]
    #[case(R407B, "HEOS", PredefinedMixture, None, None)]
    #[case(R407CMix, "HEOS", PredefinedMixture, None, None)]
    #[case(R407D, "HEOS", PredefinedMixture, None, None)]
    #[case(R407E, "HEOS", PredefinedMixture, None, None)]
    #[case(R407F, "HEOS", PredefinedMixture, None, None)]
    #[case(R408A, "HEOS", PredefinedMixture, None, None)]
    #[case(R409A, "HEOS", PredefinedMixture, None, None)]
    #[case(R409B, "HEOS", PredefinedMixture, None, None)]
    #[case(R410AMix, "HEOS", PredefinedMixture, None, None)]
    #[case(R410B, "HEOS", PredefinedMixture, None, None)]
    #[case(R411A, "HEOS", PredefinedMixture, None, None)]
    #[case(R411B, "HEOS", PredefinedMixture, None, None)]
    #[case(R412A, "HEOS", PredefinedMixture, None, None)]
    #[case(R413A, "HEOS", PredefinedMixture, None, None)]
    #[case(R414A, "HEOS", PredefinedMixture, None, None)]
    #[case(R414B, "HEOS", PredefinedMixture, None, None)]
    #[case(R415A, "HEOS", PredefinedMixture, None, None)]
    #[case(R415B, "HEOS", PredefinedMixture, None, None)]
    #[case(R416A, "HEOS", PredefinedMixture, None, None)]
    #[case(R417A, "HEOS", PredefinedMixture, None, None)]
    #[case(R417B, "HEOS", PredefinedMixture, None, None)]
    #[case(R417C, "HEOS", PredefinedMixture, None, None)]
    #[case(R418A, "HEOS", PredefinedMixture, None, None)]
    #[case(R419A, "HEOS", PredefinedMixture, None, None)]
    #[case(R419B, "HEOS", PredefinedMixture, None, None)]
    #[case(R420A, "HEOS", PredefinedMixture, None, None)]
    #[case(R421A, "HEOS", PredefinedMixture, None, None)]
    #[case(R421B, "HEOS", PredefinedMixture, None, None)]
    #[case(R422A, "HEOS", PredefinedMixture, None, None)]
    #[case(R422B, "HEOS", PredefinedMixture, None, None)]
    #[case(R422C, "HEOS", PredefinedMixture, None, None)]
    #[case(R422D, "HEOS", PredefinedMixture, None, None)]
    #[case(R422E, "HEOS", PredefinedMixture, None, None)]
    #[case(R423A, "HEOS", PredefinedMixture, None, None)]
    #[case(R424A, "HEOS", PredefinedMixture, None, None)]
    #[case(R425A, "HEOS", PredefinedMixture, None, None)]
    #[case(R426A, "HEOS", PredefinedMixture, None, None)]
    #[case(R427A, "HEOS", PredefinedMixture, None, None)]
    #[case(R428A, "HEOS", PredefinedMixture, None, None)]
    #[case(R429A, "HEOS", PredefinedMixture, None, None)]
    #[case(R430A, "HEOS", PredefinedMixture, None, None)]
    #[case(R431A, "HEOS", PredefinedMixture, None, None)]
    #[case(R432A, "HEOS", PredefinedMixture, None, None)]
    #[case(R433A, "HEOS", PredefinedMixture, None, None)]
    #[case(R433B, "HEOS", PredefinedMixture, None, None)]
    #[case(R433C, "HEOS", PredefinedMixture, None, None)]
    #[case(R434A, "HEOS", PredefinedMixture, None, None)]
    #[case(R435A, "HEOS", PredefinedMixture, None, None)]
    #[case(R436A, "HEOS", PredefinedMixture, None, None)]
    #[case(R436B, "HEOS", PredefinedMixture, None, None)]
    #[case(R437A, "HEOS", PredefinedMixture, None, None)]
    #[case(R438A, "HEOS", PredefinedMixture, None, None)]
    #[case(R439A, "HEOS", PredefinedMixture, None, None)]
    #[case(R440A, "HEOS", PredefinedMixture, None, None)]
    #[case(R441A, "HEOS", PredefinedMixture, None, None)]
    #[case(R442A, "HEOS", PredefinedMixture, None, None)]
    #[case(R443A, "HEOS", PredefinedMixture, None, None)]
    #[case(R444A, "HEOS", PredefinedMixture, None, None)]
    #[case(R444B, "HEOS", PredefinedMixture, None, None)]
    #[case(R445A, "HEOS", PredefinedMixture, None, None)]
    #[case(R446A, "HEOS", PredefinedMixture, None, None)]
    #[case(R447A, "HEOS", PredefinedMixture, None, None)]
    #[case(R448A, "HEOS", PredefinedMixture, None, None)]
    #[case(R449A, "HEOS", PredefinedMixture, None, None)]
    #[case(R449B, "HEOS", PredefinedMixture, None, None)]
    #[case(R450A, "HEOS", PredefinedMixture, None, None)]
    #[case(R451A, "HEOS", PredefinedMixture, None, None)]
    #[case(R451B, "HEOS", PredefinedMixture, None, None)]
    #[case(R452A, "HEOS", PredefinedMixture, None, None)]
    #[case(R453A, "HEOS", PredefinedMixture, None, None)]
    #[case(R454A, "HEOS", PredefinedMixture, None, None)]
    #[case(R454B, "HEOS", PredefinedMixture, None, None)]
    #[case(R500, "HEOS", PredefinedMixture, None, None)]
    #[case(R501, "HEOS", PredefinedMixture, None, None)]
    #[case(R502, "HEOS", PredefinedMixture, None, None)]
    #[case(R503, "HEOS", PredefinedMixture, None, None)]
    #[case(R504, "HEOS", PredefinedMixture, None, None)]
    #[case(R507AMix, "HEOS", PredefinedMixture, None, None)]
    #[case(R508A, "HEOS", PredefinedMixture, None, None)]
    #[case(R508B, "HEOS", PredefinedMixture, None, None)]
    #[case(R509A, "HEOS", PredefinedMixture, None, None)]
    #[case(R510A, "HEOS", PredefinedMixture, None, None)]
    #[case(R511A, "HEOS", PredefinedMixture, None, None)]
    #[case(R512A, "HEOS", PredefinedMixture, None, None)]
    #[case(R513A, "HEOS", PredefinedMixture, None, None)]
    #[case(TypicalNaturalGas, "HEOS", PredefinedMixture, None, None)]
    fn substances_has_expected_props(
        #[case] substance: Substance,
        #[case] expected_backend: &'static str,
        #[case] expected_category: SubstanceCategory,
        #[case] expected_min_fraction: Option<f64>,
        #[case] expected_max_fraction: Option<f64>,
    ) {
        assert_eq!(substance.backend(), expected_backend);
        assert_eq!(substance.category(), expected_category);
        assert_eq!(substance.min_fraction(), expected_min_fraction);
        assert_eq!(substance.max_fraction(), expected_max_fraction);
    }

    //noinspection SpellCheckingInspection
    #[rstest]
    #[case(Acetone, "Acetone")]
    #[case(Air, "Air")]
    #[case(R729, "R729")]
    #[case(Ammonia, "Ammonia")]
    #[case(R717, "R717")]
    #[case(Argon, "Argon")]
    #[case(R740, "R740")]
    #[case(Benzene, "Benzene")]
    #[case(Butene, "1-Butene")]
    #[case(CarbonDioxide, "CarbonDioxide")]
    #[case(R744, "R744")]
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
    #[case(R170, "R170")]
    #[case(Ethanol, "Ethanol")]
    #[case(EthylBenzene, "EthylBenzene")]
    #[case(Ethylene, "Ethylene")]
    #[case(R1150, "R1150")]
    #[case(EthyleneOxide, "EthyleneOxide")]
    #[case(Fluorine, "Fluorine")]
    #[case(HeavyWater, "HeavyWater")]
    #[case(Helium, "Helium")]
    #[case(R704, "R704")]
    #[case(HFE143m, "HFE143m")]
    #[case(RE143a, "RE143a")]
    #[case(Hydrogen, "Hydrogen")]
    #[case(R702, "R702")]
    #[case(HydrogenChloride, "HydrogenChloride")]
    #[case(HydrogenSulfide, "HydrogenSulfide")]
    #[case(Isobutane, "Isobutane")]
    #[case(R600a, "R600a")]
    #[case(Isobutene, "Isobutene")]
    #[case(Isohexane, "Isohexane")]
    #[case(Isopentane, "Isopentane")]
    #[case(R601a, "R601a")]
    #[case(Krypton, "Krypton")]
    #[case(MD2M, "MD2M")]
    #[case(MD3M, "MD3M")]
    #[case(MD4M, "MD4M")]
    #[case(MDM, "MDM")]
    #[case(Methane, "Methane")]
    #[case(R50, "R50")]
    #[case(Methanol, "Methanol")]
    #[case(MethylLinoleate, "MethylLinoleate")]
    #[case(MethylLinolenate, "MethylLinolenate")]
    #[case(MethylOleate, "MethylOleate")]
    #[case(MethylPalmitate, "MethylPalmitate")]
    #[case(MethylStearate, "MethylStearate")]
    #[case(MM, "MM")]
    #[case(mXylene, "m-Xylene")]
    #[case(nButane, "n-Butane")]
    #[case(R600, "R600")]
    #[case(nDecane, "n-Decane")]
    #[case(nDodecane, "n-Dodecane")]
    #[case(Neon, "Neon")]
    #[case(R720, "R720")]
    #[case(Neopentane, "Neopentane")]
    #[case(nHeptane, "n-Heptane")]
    #[case(nHexane, "n-Hexane")]
    #[case(Nitrogen, "Nitrogen")]
    #[case(R728, "R728")]
    #[case(NitrousOxide, "NitrousOxide")]
    #[case(nNonane, "n-Nonane")]
    #[case(nOctane, "n-Octane")]
    #[case(Novec649, "Novec649")]
    #[case(nPentane, "n-Pentane")]
    #[case(R601, "R601")]
    #[case(nPropane, "n-Propane")]
    #[case(R290, "R290")]
    #[case(nUndecane, "n-Undecane")]
    #[case(Orthodeuterium, "OrthoDeuterium")]
    #[case(Orthohydrogen, "OrthoHydrogen")]
    #[case(Oxygen, "Oxygen")]
    #[case(R732, "R732")]
    #[case(oXylene, "o-Xylene")]
    #[case(Paradeuterium, "ParaDeuterium")]
    #[case(Parahydrogen, "ParaHydrogen")]
    #[case(Propylene, "Propylene")]
    #[case(R1270, "R1270")]
    #[case(Propyne, "Propyne")]
    #[case(pXylene, "p-Xylene")]
    #[case(R11, "R11")]
    #[case(R113, "R113")]
    #[case(R114, "R114")]
    #[case(R115, "R115")]
    #[case(R116, "R116")]
    #[case(R12, "R12")]
    #[case(R123, "R123")]
    #[case(R1233zdE, "R1233zd(E)")]
    #[case(R1234yf, "R1234yf")]
    #[case(R1234zeE, "R1234ze(E)")]
    #[case(R1234zeZ, "R1234ze(Z)")]
    #[case(R124, "R124")]
    #[case(R1243zf, "R1243zf")]
    #[case(R125, "R125")]
    #[case(R13, "R13")]
    #[case(R134a, "R134a")]
    #[case(R13I1, "R13I1")]
    #[case(R14, "R14")]
    #[case(R141b, "R141b")]
    #[case(R142b, "R142b")]
    #[case(R143a, "R143a")]
    #[case(R152a, "R152a")]
    #[case(R161, "R161")]
    #[case(R21, "R21")]
    #[case(R218, "R218")]
    #[case(R22, "R22")]
    #[case(R227ea, "R227ea")]
    #[case(R23, "R23")]
    #[case(R236ea, "R236ea")]
    #[case(R236fa, "R236fa")]
    #[case(R245ca, "R245ca")]
    #[case(R245fa, "R245fa")]
    #[case(R32, "R32")]
    #[case(R365mfc, "R365mfc")]
    #[case(R40, "R40")]
    #[case(R404A, "R404A")]
    #[case(R407C, "R407C")]
    #[case(R41, "R41")]
    #[case(R410A, "R410A")]
    #[case(R507A, "R507A")]
    #[case(RC318, "RC318")]
    #[case(SES36, "SES36")]
    #[case(SulfurDioxide, "SulfurDioxide")]
    #[case(R764, "SulfurDioxide")]
    #[case(SulfurHexafluoride, "SulfurHexafluoride")]
    #[case(R846, "SulfurHexafluoride")]
    #[case(Toluene, "Toluene")]
    #[case(trans2Butene, "trans-2-Butene")]
    #[case(Water, "Water")]
    #[case(R718, "R718")]
    #[case(Xenon, "Xenon")]
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
    #[case(WaterIncomp, "Water")]
    #[case(XLT, "XLT")]
    #[case(XLT2, "XLT2")]
    #[case(ZS10, "ZS10")]
    #[case(ZS25, "ZS25")]
    #[case(ZS40, "ZS40")]
    #[case(ZS45, "ZS45")]
    #[case(ZS55, "ZS55")]
    #[case(FRE, "FRE")]
    #[case(IceEA, "IceEA")]
    #[case(IceNA, "IceNA")]
    #[case(IcePG, "IcePG")]
    #[case(LiBr, "LiBr")]
    #[case(MAM, "MAM")]
    #[case(MAM2, "MAM2")]
    #[case(MCA, "MCA")]
    #[case(MCA2, "MCA2")]
    #[case(MEA, "MEA")]
    #[case(MEA2, "MEA2")]
    #[case(MEG, "MEG")]
    #[case(MEG2, "MEG2")]
    #[case(MGL, "MGL")]
    #[case(MGL2, "MGL2")]
    #[case(MITSW, "MITSW")]
    #[case(MKA, "MKA")]
    #[case(MKA2, "MKA2")]
    #[case(MKC, "MKC")]
    #[case(MKC2, "MKC2")]
    #[case(MKF, "MKF")]
    #[case(MLI, "MLI")]
    #[case(MMA, "MMA")]
    #[case(MMA2, "MMA2")]
    #[case(MMG, "MMG")]
    #[case(MMG2, "MMG2")]
    #[case(MNA, "MNA")]
    #[case(MNA2, "MNA2")]
    #[case(MPG, "MPG")]
    #[case(MPG2, "MPG2")]
    #[case(VCA, "VCA")]
    #[case(VKC, "VKC")]
    #[case(VMA, "VMA")]
    #[case(VMG, "VMG")]
    #[case(VNA, "VNA")]
    #[case(AEG, "AEG")]
    #[case(AKF, "AKF")]
    #[case(AL, "AL")]
    #[case(AN, "AN")]
    #[case(APG, "APG")]
    #[case(GKN, "GKN")]
    #[case(PK2, "PK2")]
    #[case(PKL, "PKL")]
    #[case(ZAC, "ZAC")]
    #[case(ZFC, "ZFC")]
    #[case(ZLC, "ZLC")]
    #[case(ZM, "ZM")]
    #[case(ZMC, "ZMC")]
    #[case(AirMix, "Air.mix")]
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
    #[case(R404AMix, "R404A.mix")]
    #[case(R405A, "R405A.mix")]
    #[case(R406A, "R406A.mix")]
    #[case(R407A, "R407A.mix")]
    #[case(R407B, "R407B.mix")]
    #[case(R407CMix, "R407C.mix")]
    #[case(R407D, "R407D.mix")]
    #[case(R407E, "R407E.mix")]
    #[case(R407F, "R407F.mix")]
    #[case(R408A, "R408A.mix")]
    #[case(R409A, "R409A.mix")]
    #[case(R409B, "R409B.mix")]
    #[case(R410AMix, "R410A.mix")]
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
    #[case(R448A, "R448A.mix")]
    #[case(R449A, "R449A.mix")]
    #[case(R449B, "R449B.mix")]
    #[case(R450A, "R450A.mix")]
    #[case(R451A, "R451A.mix")]
    #[case(R451B, "R451B.mix")]
    #[case(R452A, "R452A.mix")]
    #[case(R453A, "R453A.mix")]
    #[case(R454A, "R454A.mix")]
    #[case(R454B, "R454B.mix")]
    #[case(R500, "R500.mix")]
    #[case(R501, "R501.mix")]
    #[case(R502, "R502.mix")]
    #[case(R503, "R503.mix")]
    #[case(R504, "R504.mix")]
    #[case(R507AMix, "R507A.mix")]
    #[case(R508A, "R508A.mix")]
    #[case(R508B, "R508B.mix")]
    #[case(R509A, "R509A.mix")]
    #[case(R510A, "R510A.mix")]
    #[case(R511A, "R511A.mix")]
    #[case(R512A, "R512A.mix")]
    #[case(R513A, "R513A.mix")]
    #[case(TypicalNaturalGas, "TypicalNaturalGas.mix")]
    fn substance_as_ref_always_returns_expected_str(
        #[case] substance: Substance,
        #[case] expected: &str,
    ) {
        assert_eq!(substance.as_ref(), expected);
    }

    //noinspection SpellCheckingInspection
    #[rstest]
    #[case(vec!["Acetone"], Acetone)]
    #[case(vec!["Air"], Air)]
    #[case(vec!["R729"], R729)]
    #[case(vec!["Ammonia", "NH3"], Ammonia)]
    #[case(vec!["R717"], R717)]
    #[case(vec!["Argon", "Ar"], Argon)]
    #[case(vec!["R740"], R740)]
    #[case(vec!["Benzene"], Benzene)]
    #[case(vec!["1-Butene", "1Butene" ,  "Butene"], Butene)]
    #[case(vec!["CarbonDioxide", "CO2"], CarbonDioxide)]
    #[case(vec!["R744"], R744)]
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
    #[case(vec!["R170"], R170)]
    #[case(vec!["Ethanol", "C2H6O"], Ethanol)]
    #[case(vec!["EthylBenzene", "EBENZENE"], EthylBenzene)]
    #[case(vec!["Ethylene"], Ethylene)]
    #[case(vec!["R1150"], R1150)]
    #[case(vec!["EthyleneOxide"], EthyleneOxide)]
    #[case(vec!["Fluorine"], Fluorine)]
    #[case(vec!["HeavyWater", "D2O"], HeavyWater)]
    #[case(vec!["Helium", "He"], Helium)]
    #[case(vec!["R704"], R704)]
    #[case(vec!["HFE143m", "HFE-143m"], HFE143m)]
    #[case(vec!["RE143a"], RE143a)]
    #[case(vec!["Hydrogen", "H2"], Hydrogen)]
    #[case(vec!["R702"], R702)]
    #[case(vec!["HydrogenChloride", "HCl"], HydrogenChloride)]
    #[case(vec!["HydrogenSulfide", "H2S"], HydrogenSulfide)]
    #[case(vec!["Isobutane", "IBUTANE"], Isobutane)]
    #[case(vec!["R600a"], R600a)]
    #[case(vec!["Isobutene", "IBUTENE"], Isobutene)]
    #[case(vec!["Isohexane", "IHEXANE"], Isohexane)]
    #[case(vec!["Isopentane", "IPENTANE"], Isopentane)]
    #[case(vec!["R601a"], R601a)]
    #[case(vec!["Krypton"], Krypton)]
    #[case(vec!["MD2M", "Decamethyltetrasiloxane"], MD2M)]
    #[case(vec!["MD3M", "Dodecamethylpentasiloxane"], MD3M)]
    #[case(vec!["MD4M", "Tetradecamethylhexasiloxane"], MD4M)]
    #[case(vec!["MDM", "Octamethyltrisiloxane"], MDM)]
    #[case(vec!["Methane", "CH4" ,  "n-C1H4"], Methane)]
    #[case(vec!["R50"], R50)]
    #[case(vec!["Methanol"], Methanol)]
    #[case(vec!["MethylLinoleate", "MLINOLEA"], MethylLinoleate)]
    #[case(vec!["MethylLinolenate", "MLINOLEN"], MethylLinolenate)]
    #[case(vec!["MethylOleate", "MOLEATE"], MethylOleate)]
    #[case(vec!["MethylPalmitate", "MPALMITA"], MethylPalmitate)]
    #[case(vec!["MethylStearate", "MSTEARAT"], MethylStearate)]
    #[case(vec!["MM", "Hexamethyldisiloxane"], MM)]
    #[case(vec!["m-Xylene", "mXylene" ,  "MC8H10"], mXylene)]
    #[case(vec!["n-Butane", "nButane" ,  "Butane" ,  "NC4H10" ,  "n-C4H10"], nButane)]
    #[case(vec!["R600"], R600)]
    #[case(vec!["n-Decane", "nDecane" ,  "Decane" ,  "NC10H22" ,  "n-C10H22"], nDecane)]
    #[case(vec!["n-Dodecane", "nDodecane" ,  "Dodecane" ,  "NC12H26" ,  "n-C12H26"], nDodecane)]
    #[case(vec!["Neon"], Neon)]
    #[case(vec!["R720"], R720)]
    #[case(vec!["Neopentane"], Neopentane)]
    #[case(vec!["n-Heptane", "nHeptane" ,  "Heptane" ,  "NC7H16" ,  "n-C7H16"], nHeptane)]
    #[case(vec!["n-Hexane", "nHexane" ,  "Hexane" ,  "NC6H14" ,  "n-C6H14"], nHexane)]
    #[case(vec!["Nitrogen", "N2"], Nitrogen)]
    #[case(vec!["R728"], R728)]
    #[case(vec!["NitrousOxide", "N2O"], NitrousOxide)]
    #[case(vec!["n-Nonane", "nNonane" ,  "Nonane" ,  "NC9H20" ,  "n-C9H20"], nNonane)]
    #[case(vec!["n-Octane", "nOctane" ,  "Octane" ,  "NC8H18" ,  "n-C8H18"], nOctane)]
    #[case(vec!["Novec649", "Novec1230"], Novec649)]
    #[case(vec!["n-Pentane", "nPentane" ,  "Pentane" ,  "NC5H12" ,  "n-C5H12"], nPentane)]
    #[case(vec!["R601"], R601)]
    #[case(vec!["n-Propane", "nPropane" ,  "Propane" ,  "C3H8" ,  "NC3H8" ,  "n-C3H8"], nPropane)]
    #[case(vec!["R290"], R290)]
    #[case(vec!["n-Undecane", "nUndecane" ,  "Undecane" ,  "NC11H24" ,  "n-C11H24"], nUndecane)]
    #[case(vec!["OrthoDeuterium"], Orthodeuterium)]
    #[case(vec!["OrthoHydrogen"], Orthohydrogen)]
    #[case(vec!["Oxygen", "O2"], Oxygen)]
    #[case(vec!["R732"], R732)]
    #[case(vec!["o-Xylene", "oXylene" ,  "OC8H10"], oXylene)]
    #[case(vec!["ParaDeuterium"], Paradeuterium)]
    #[case(vec!["ParaHydrogen"], Parahydrogen)]
    #[case(vec!["Propylene"], Propylene)]
    #[case(vec!["R1270"], R1270)]
    #[case(vec!["Propyne"], Propyne)]
    #[case(vec!["p-Xylene", "pXylene" ,  "PC8H10"], pXylene)]
    #[case(vec!["R11"], R11)]
    #[case(vec!["R113"], R113)]
    #[case(vec!["R114"], R114)]
    #[case(vec!["R115"], R115)]
    #[case(vec!["R116"], R116)]
    #[case(vec!["R12"], R12)]
    #[case(vec!["R123"], R123)]
    #[case(vec!["R1233zd(E)", "R1233zdE"], R1233zdE)]
    #[case(vec!["R1234yf"], R1234yf)]
    #[case(vec!["R1234ze(E)", "R1234zeE"], R1234zeE)]
    #[case(vec!["R1234ze(Z)", "R1234zeZ"], R1234zeZ)]
    #[case(vec!["R124"], R124)]
    #[case(vec!["R1243zf"], R1243zf)]
    #[case(vec!["R125"], R125)]
    #[case(vec!["R13"], R13)]
    #[case(vec!["R134a"], R134a)]
    #[case(vec!["R13I1", "CF3I"], R13I1)]
    #[case(vec!["R14"], R14)]
    #[case(vec!["R141b"], R141b)]
    #[case(vec!["R142b"], R142b)]
    #[case(vec!["R143a"], R143a)]
    #[case(vec!["R152a"], R152a)]
    #[case(vec!["R161"], R161)]
    #[case(vec!["R21"], R21)]
    #[case(vec!["R218"], R218)]
    #[case(vec!["R22"], R22)]
    #[case(vec!["R227ea"], R227ea)]
    #[case(vec!["R23"], R23)]
    #[case(vec!["R236ea"], R236ea)]
    #[case(vec!["R236fa"], R236fa)]
    #[case(vec!["R245ca"], R245ca)]
    #[case(vec!["R245fa"], R245fa)]
    #[case(vec!["R32"], R32)]
    #[case(vec!["R365mfc"], R365mfc)]
    #[case(vec!["R40"], R40)]
    #[case(vec!["R404A"], R404A)]
    #[case(vec!["R407C"], R407C)]
    #[case(vec!["R41"], R41)]
    #[case(vec!["R410A"], R410A)]
    #[case(vec!["R507A"], R507A)]
    #[case(vec!["RC318"], RC318)]
    #[case(vec!["SES36"], SES36)]
    #[case(vec!["SulfurDioxide", "SO2"], SulfurDioxide)]
    #[case(vec!["R764"], R764)]
    #[case(vec!["SulfurHexafluoride", "SF6"], SulfurHexafluoride)]
    #[case(vec!["R846"], R846)]
    #[case(vec!["Toluene"], Toluene)]
    #[case(vec!["trans-2-Butene", "T2BUTENE"], trans2Butene)]
    #[case(vec!["Water", "H2O"], Water)]
    #[case(vec!["R718"], R718)]
    #[case(vec!["Xenon", "Xe"], Xenon)]
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
    #[case(vec!["WaterIncomp", "IncompWater"], WaterIncomp)]
    #[case(vec!["XLT"], XLT)]
    #[case(vec!["XLT2"], XLT2)]
    #[case(vec!["ZS10"], ZS10)]
    #[case(vec!["ZS25"], ZS25)]
    #[case(vec!["ZS40"], ZS40)]
    #[case(vec!["ZS45"], ZS45)]
    #[case(vec!["ZS55"], ZS55)]
    #[case(vec!["FRE"], FRE)]
    #[case(vec!["IceEA"], IceEA)]
    #[case(vec!["IceNA"], IceNA)]
    #[case(vec!["IcePG"], IcePG)]
    #[case(vec!["LiBr"], LiBr)]
    #[case(vec!["MAM"], MAM)]
    #[case(vec!["MAM2"], MAM2)]
    #[case(vec!["MCA"], MCA)]
    #[case(vec!["MCA2"], MCA2)]
    #[case(vec!["MEA"], MEA)]
    #[case(vec!["MEA2"], MEA2)]
    #[case(vec!["MEG"], MEG)]
    #[case(vec!["MEG2"], MEG2)]
    #[case(vec!["MGL"], MGL)]
    #[case(vec!["MGL2"], MGL2)]
    #[case(vec!["MITSW"], MITSW)]
    #[case(vec!["MKA"], MKA)]
    #[case(vec!["MKA2"], MKA2)]
    #[case(vec!["MKC"], MKC)]
    #[case(vec!["MKC2"], MKC2)]
    #[case(vec!["MKF"], MKF)]
    #[case(vec!["MLI"], MLI)]
    #[case(vec!["MMA"], MMA)]
    #[case(vec!["MMA2"], MMA2)]
    #[case(vec!["MMG"], MMG)]
    #[case(vec!["MMG2"], MMG2)]
    #[case(vec!["MNA"], MNA)]
    #[case(vec!["MNA2"], MNA2)]
    #[case(vec!["MPG"], MPG)]
    #[case(vec!["MPG2"], MPG2)]
    #[case(vec!["VCA"], VCA)]
    #[case(vec!["VKC"], VKC)]
    #[case(vec!["VMA"], VMA)]
    #[case(vec!["VMG"], VMG)]
    #[case(vec!["VNA"], VNA)]
    #[case(vec!["AEG"], AEG)]
    #[case(vec!["AKF"], AKF)]
    #[case(vec!["AL"], AL)]
    #[case(vec!["AN"], AN)]
    #[case(vec!["APG"], APG)]
    #[case(vec!["GKN"], GKN)]
    #[case(vec!["PK2"], PK2)]
    #[case(vec!["PKL"], PKL)]
    #[case(vec!["ZAC"], ZAC)]
    #[case(vec!["ZFC"], ZFC)]
    #[case(vec!["ZLC"], ZLC)]
    #[case(vec!["ZM"], ZM)]
    #[case(vec!["ZMC"], ZMC)]
    #[case(vec!["Air.mix", "AirMix" ,"Air-mix"], AirMix)]
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
    #[case(vec!["R404A.mix", "R404AMix" , "R404A-mix"], R404AMix)]
    #[case(vec!["R405A.mix", "R405A"], R405A)]
    #[case(vec!["R406A.mix", "R406A"], R406A)]
    #[case(vec!["R407A.mix", "R407A"], R407A)]
    #[case(vec!["R407B.mix", "R407B"], R407B)]
    #[case(vec!["R407C.mix", "R407CMix" , "R407C-mix"], R407CMix)]
    #[case(vec!["R407D.mix", "R407D"], R407D)]
    #[case(vec!["R407E.mix", "R407E"], R407E)]
    #[case(vec!["R407F.mix", "R407F"], R407F)]
    #[case(vec!["R408A.mix", "R408A"], R408A)]
    #[case(vec!["R409A.mix", "R409A"], R409A)]
    #[case(vec!["R409B.mix", "R409B"], R409B)]
    #[case(vec!["R410A.mix", "R410AMix" ,  "R410A-mix"], R410AMix)]
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
    #[case(vec!["R448A.mix", "R448A"], R448A)]
    #[case(vec!["R449A.mix", "R449A"], R449A)]
    #[case(vec!["R449B.mix", "R449B"], R449B)]
    #[case(vec!["R450A.mix", "R450A"], R450A)]
    #[case(vec!["R451A.mix", "R451A"], R451A)]
    #[case(vec!["R451B.mix", "R451B"], R451B)]
    #[case(vec!["R452A.mix", "R452A"], R452A)]
    #[case(vec!["R453A.mix", "R453A"], R453A)]
    #[case(vec!["R454A.mix", "R454A"], R454A)]
    #[case(vec!["R454B.mix", "R454B"], R454B)]
    #[case(vec!["R500.mix", "R500"], R500)]
    #[case(vec!["R501.mix", "R501"], R501)]
    #[case(vec!["R502.mix", "R502"], R502)]
    #[case(vec!["R503.mix", "R503"], R503)]
    #[case(vec!["R504.mix", "R504"], R504)]
    #[case(vec!["R507A.mix", "R507AMix" , "R507A-mix"], R507AMix)]
    #[case(vec!["R508A.mix", "R508A"], R508A)]
    #[case(vec!["R508B.mix", "R508B"], R508B)]
    #[case(vec!["R509A.mix", "R509A"], R509A)]
    #[case(vec!["R510A.mix", "R510A"], R510A)]
    #[case(vec!["R511A.mix", "R511A"], R511A)]
    #[case(vec!["R512A.mix", "R512A"], R512A)]
    #[case(vec!["R513A.mix", "R513A"], R513A)]
    #[case(vec!["TypicalNaturalGas.mix", "TypicalNaturalGas", "NaturalGas"], TypicalNaturalGas)]
    fn substance_from_valid_str_returns_ok(
        #[case] valid_values: Vec<&str>,
        #[case] expected: Substance,
    ) {
        for s in valid_values {
            assert_eq!(Substance::from_str(s).unwrap(), expected);
            assert_eq!(Substance::try_from(s).unwrap(), expected);
        }
    }

    #[rstest]
    #[case(Pure, "Pure")]
    #[case(PseudoPure, "PseudoPure")]
    #[case(IncompPure, "IncompPure")]
    #[case(IncompMassBasedBinaryMixture, "IncompMassBasedBinaryMixture")]
    #[case(IncompVolumeBasedBinaryMixture, "IncompVolumeBasedBinaryMixture")]
    #[case(PredefinedMixture, "PredefinedMixture")]
    fn substance_category_as_ref_always_returns_expected_str(
        #[case] category: SubstanceCategory,
        #[case] expected: &str,
    ) {
        assert_eq!(category.as_ref(), expected);
    }

    #[rstest]
    #[case("Pure", Pure)]
    #[case("PseudoPure", PseudoPure)]
    #[case("IncompPure", IncompPure)]
    #[case("IncompMassBasedBinaryMixture", IncompMassBasedBinaryMixture)]
    #[case("IncompVolumeBasedBinaryMixture", IncompVolumeBasedBinaryMixture)]
    #[case("PredefinedMixture", PredefinedMixture)]
    fn substance_category_from_valid_str_returns_ok(
        #[case] valid_value: &str,
        #[case] expected: SubstanceCategory,
    ) {
        assert_eq!(SubstanceCategory::from_str(valid_value).unwrap(), expected);
        assert_eq!(SubstanceCategory::try_from(valid_value).unwrap(), expected);
    }
}
