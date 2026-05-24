// cSpell:disable

/// `CoolProp` pure or pseudo-pure substances.
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
/// assert_eq!(Pure::Water.as_ref(), "Water");
/// assert_eq!(Pure::from_str("Water"), Ok(Pure::Water));
/// assert_eq!(Pure::try_from("H2O"), Ok(Pure::Water));
/// ```
///
/// # See Also
///
/// - [Pure and Pseudo-Pure Substances](https://coolprop.org/fluid_properties/PurePseudoPure.html)
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    PartialEq,
    strum_macros::AsRefStr,
    strum_macros::EnumString,
    strum_macros::IntoStaticStr,
)]
#[strum(ascii_case_insensitive)]
#[cfg_attr(test, derive(strum_macros::EnumIter))]
pub enum Pure {
    /// Acetone.
    ///
    /// # See Also
    ///
    /// - [Acetone](https://coolprop.org/fluid_properties/fluids/Acetone.html)
    #[strum(to_string = "Acetone")]
    Acetone,

    /// Dry air.
    ///
    /// # See Also
    ///
    /// - [Dry air](https://coolprop.org/fluid_properties/fluids/Air.html)
    #[strum(to_string = "Air")]
    Air,

    /// Ammonia.
    ///
    /// # See Also
    ///
    /// - [Ammonia](https://coolprop.org/fluid_properties/fluids/Ammonia.html)
    #[strum(to_string = "Ammonia", serialize = "NH3")]
    Ammonia,

    /// Argon.
    ///
    /// # See Also
    ///
    /// - [Argon](https://coolprop.org/fluid_properties/fluids/Argon.html)
    #[strum(to_string = "Argon", serialize = "Ar")]
    Argon,

    /// Benzene.
    ///
    /// # See Also
    ///
    /// - [Benzene](https://coolprop.org/fluid_properties/fluids/Benzene.html)
    #[strum(to_string = "Benzene")]
    Benzene,

    /// 1-Butene.
    ///
    /// # See Also
    ///
    /// - [1-Butene](https://coolprop.org/fluid_properties/fluids/1-Butene.html)
    #[strum(to_string = "1-Butene", serialize = "1Butene", serialize = "Butene")]
    Butene,

    /// Carbon dioxide.
    ///
    /// # See Also
    ///
    /// - [Carbon dioxide](https://coolprop.org/fluid_properties/fluids/CarbonDioxide.html)
    #[strum(to_string = "CarbonDioxide", serialize = "CO2")]
    CarbonDioxide,

    /// Carbon monoxide.
    ///
    /// # See Also
    ///
    /// - [Carbon monoxide](https://coolprop.org/fluid_properties/fluids/CarbonMonoxide.html)
    #[strum(to_string = "CarbonMonoxide", serialize = "CO")]
    CarbonMonoxide,

    /// Carbonyl sulfide.
    ///
    /// # See Also
    ///
    /// - [Carbonyl sulfide](https://coolprop.org/fluid_properties/fluids/CarbonylSulfide.html)
    #[strum(to_string = "CarbonylSulfide", serialize = "COS")]
    CarbonylSulfide,

    /// cis-2-Butene.
    ///
    /// # See Also
    ///
    /// - [cis-2-Butene](https://coolprop.org/fluid_properties/fluids/cis-2-Butene.html)
    #[strum(to_string = "cis-2-Butene", serialize = "C2BUTENE")]
    cis2Butene,

    /// Cyclohexane.
    ///
    /// # See Also
    ///
    /// - [Cyclohexane](https://coolprop.org/fluid_properties/fluids/CycloHexane.html)
    #[strum(to_string = "Cyclohexane", serialize = "CYCLOHEX")]
    Cyclohexane,

    /// Cyclopentane.
    ///
    /// # See Also
    ///
    /// - [Cyclopentane](https://coolprop.org/fluid_properties/fluids/Cyclopentane.html)
    #[strum(to_string = "Cyclopentane", serialize = "CYCLOPEN")]
    Cyclopentane,

    /// Cyclopropane.
    ///
    /// # See Also
    ///
    /// - [Cyclopropane](https://coolprop.org/fluid_properties/fluids/CycloPropane.html)
    #[strum(to_string = "Cyclopropane", serialize = "CYCLOPRO")]
    Cyclopropane,

    /// D4 -- octamethylcyclotetrasiloxane.
    ///
    /// # See Also
    ///
    /// - [D4](https://coolprop.org/fluid_properties/fluids/D4.html)
    #[strum(to_string = "D4", serialize = "Octamethylcyclotetrasiloxane")]
    D4,

    /// D5 -- decamethylcyclopentasiloxane.
    ///
    /// # See Also
    ///
    /// - [D5](https://coolprop.org/fluid_properties/fluids/D5.html)
    #[strum(to_string = "D5", serialize = "Decamethylcyclopentasiloxane")]
    D5,

    /// D6 -- dodecamethylcyclohexasiloxane.
    ///
    /// # See Also
    ///
    /// - [D6](https://coolprop.org/fluid_properties/fluids/D6.html)
    #[strum(to_string = "D6", serialize = "Dodecamethylcyclohexasiloxane")]
    D6,

    /// Deuterium.
    ///
    /// # See Also
    ///
    /// - [Deuterium](https://coolprop.org/fluid_properties/fluids/Deuterium.html)
    #[strum(to_string = "Deuterium", serialize = "D2")]
    Deuterium,

    /// 1,2-Dichloroethane.
    ///
    /// # See Also
    ///
    /// - [Dichloroethane](https://coolprop.org/fluid_properties/fluids/Dichloroethane.html)
    #[strum(to_string = "Dichloroethane", serialize = "1,2-dichloroethane")]
    Dichloroethane,

    /// Diethyl ether.
    ///
    /// # See Also
    ///
    /// - [Diethyl ether](https://coolprop.org/fluid_properties/fluids/DiethylEther.html)
    #[strum(to_string = "DiethylEther", serialize = "DEE")]
    DiethylEther,

    /// Dimethyl carbonate.
    ///
    /// # See Also
    ///
    /// - [Dimethyl carbonate](https://coolprop.org/fluid_properties/fluids/DimethylCarbonate.html)
    #[strum(to_string = "DimethylCarbonate", serialize = "DMC")]
    DimethylCarbonate,

    /// Dimethyl ether.
    ///
    /// # See Also
    ///
    /// - [Dimethyl ether](https://coolprop.org/fluid_properties/fluids/DimethylEther.html)
    #[strum(to_string = "DimethylEther", serialize = "DME")]
    DimethylEther,

    /// Ethane.
    ///
    /// # See Also
    ///
    /// - [Ethane](https://coolprop.org/fluid_properties/fluids/Ethane.html)
    #[strum(to_string = "Ethane", serialize = "n-C2H6")]
    Ethane,

    /// Ethanol.
    ///
    /// # See Also
    ///
    /// - [Ethanol](https://coolprop.org/fluid_properties/fluids/Ethanol.html)
    #[strum(to_string = "Ethanol", serialize = "C2H6O")]
    Ethanol,

    /// Ethylbenzene.
    ///
    /// # See Also
    ///
    /// - [Ethylbenzene](https://coolprop.org/fluid_properties/fluids/EthylBenzene.html)
    #[strum(to_string = "EthylBenzene", serialize = "EBENZENE")]
    EthylBenzene,

    /// Ethylene.
    ///
    /// # See Also
    ///
    /// - [Ethylene](https://coolprop.org/fluid_properties/fluids/Ethylene.html)
    #[strum(to_string = "Ethylene")]
    Ethylene,

    /// Ethylene oxide.
    ///
    /// # See Also
    ///
    /// - [Ethylene oxide](https://coolprop.org/fluid_properties/fluids/EthyleneOxide.html)
    #[strum(to_string = "EthyleneOxide")]
    EthyleneOxide,

    /// Fluorine.
    ///
    /// # See Also
    ///
    /// - [Fluorine](https://coolprop.org/fluid_properties/fluids/Fluorine.html)
    #[strum(to_string = "Fluorine")]
    Fluorine,

    /// Heavy water.
    ///
    /// # See Also
    ///
    /// - [Heavy water](https://coolprop.org/fluid_properties/fluids/HeavyWater.html)
    #[strum(to_string = "HeavyWater", serialize = "D2O")]
    HeavyWater,

    /// Helium.
    ///
    /// # See Also
    ///
    /// - [Helium](https://coolprop.org/fluid_properties/fluids/Helium.html)
    #[strum(to_string = "Helium", serialize = "He")]
    Helium,

    /// HFE-143m -- trifluoromethyl methyl ether.
    ///
    /// # See Also
    ///
    /// - [Trifluoromethyl methyl ether](https://coolprop.org/fluid_properties/fluids/HFE143m.html)
    #[strum(to_string = "HFE143m", serialize = "HFE-143m")]
    HFE143m,

    /// Hydrogen.
    ///
    /// # See Also
    ///
    /// - [Hydrogen](https://coolprop.org/fluid_properties/fluids/Hydrogen.html)
    #[strum(to_string = "Hydrogen", serialize = "H2")]
    Hydrogen,

    /// Hydrogen chloride.
    ///
    /// # See Also
    ///
    /// - [Hydrogen chloride](https://coolprop.org/fluid_properties/fluids/HydrogenChloride.html)
    #[strum(to_string = "HydrogenChloride", serialize = "HCl")]
    HydrogenChloride,

    /// Hydrogen sulfide.
    ///
    /// # See Also
    ///
    /// - [Hydrogen sulfide](https://coolprop.org/fluid_properties/fluids/HydrogenSulfide.html)
    #[strum(to_string = "HydrogenSulfide", serialize = "H2S")]
    HydrogenSulfide,

    /// Isobutane.
    ///
    /// # See Also
    ///
    /// - [Isobutane](https://coolprop.org/fluid_properties/fluids/IsoButane.html)
    #[strum(to_string = "Isobutane", serialize = "IBUTANE")]
    Isobutane,

    /// Isobutene.
    ///
    /// # See Also
    ///
    /// - [Isobutene](https://coolprop.org/fluid_properties/fluids/IsoButene.html)
    #[strum(to_string = "Isobutene", serialize = "IBUTENE")]
    Isobutene,

    /// Isohexane.
    ///
    /// # See Also
    ///
    /// - [Isohexane](https://coolprop.org/fluid_properties/fluids/Isohexane.html)
    #[strum(to_string = "Isohexane", serialize = "IHEXANE")]
    Isohexane,

    /// Isopentane.
    ///
    /// # See Also
    ///
    /// - [Isopentane](https://coolprop.org/fluid_properties/fluids/Isopentane.html)
    #[strum(to_string = "Isopentane", serialize = "IPENTANE")]
    Isopentane,

    /// Krypton.
    ///
    /// # See Also
    ///
    /// - [Krypton](https://coolprop.org/fluid_properties/fluids/Krypton.html)
    #[strum(to_string = "Krypton")]
    Krypton,

    /// MD2M -- decamethyltetrasiloxane.
    ///
    /// # See Also
    ///
    /// - [Decamethyltetrasiloxane](https://coolprop.org/fluid_properties/fluids/MD2M.html)
    #[strum(to_string = "MD2M", serialize = "Decamethyltetrasiloxane")]
    MD2M,

    /// MD3M -- dodecamethylpentasiloxane.
    ///
    /// # See Also
    ///
    /// - [Dodecamethylpentasiloxane](https://coolprop.org/fluid_properties/fluids/MD3M.html)
    #[strum(to_string = "MD3M", serialize = "Dodecamethylpentasiloxane")]
    MD3M,

    /// MD4M -- tetradecamethylhexasiloxane.
    ///
    /// # See Also
    ///
    /// - [Tetradecamethylhexasiloxane](https://coolprop.org/fluid_properties/fluids/MD4M.html)
    #[strum(to_string = "MD4M", serialize = "Tetradecamethylhexasiloxane")]
    MD4M,

    /// MDM -- octamethyltrisiloxane.
    ///
    /// # See Also
    ///
    /// - [Octamethyltrisiloxane](https://coolprop.org/fluid_properties/fluids/MDM.html)
    #[strum(to_string = "MDM", serialize = "Octamethyltrisiloxane")]
    MDM,

    /// Methane.
    ///
    /// # See Also
    ///
    /// - [Methane](https://coolprop.org/fluid_properties/fluids/Methane.html)
    #[strum(to_string = "Methane", serialize = "CH4", serialize = "n-C1H4")]
    Methane,

    /// Methanol.
    ///
    /// # See Also
    ///
    /// - [Methanol](https://coolprop.org/fluid_properties/fluids/Methanol.html)
    #[strum(to_string = "Methanol")]
    Methanol,

    /// Methyl linoleate.
    ///
    /// # See Also
    ///
    /// - [Methyl linoleate](https://coolprop.org/fluid_properties/fluids/MethylLinoleate.html)
    #[strum(to_string = "MethylLinoleate", serialize = "MLINOLEA")]
    MethylLinoleate,

    /// Methyl linolenate.
    ///
    /// # See Also
    ///
    /// - [Methyl linolenate](https://coolprop.org/fluid_properties/fluids/MethylLinolenate.html)
    #[strum(to_string = "MethylLinolenate", serialize = "MLINOLEN")]
    MethylLinolenate,

    /// Methyl oleate.
    ///
    /// # See Also
    ///
    /// - [Methyl oleate](https://coolprop.org/fluid_properties/fluids/MethylOleate.html)
    #[strum(to_string = "MethylOleate", serialize = "MOLEATE")]
    MethylOleate,

    /// Methyl palmitate.
    ///
    /// # See Also
    ///
    /// - [Methyl palmitate](https://coolprop.org/fluid_properties/fluids/MethylPalmitate.html)
    #[strum(to_string = "MethylPalmitate", serialize = "MPALMITA")]
    MethylPalmitate,

    /// Methyl stearate.
    ///
    /// # See Also
    ///
    /// - [Methyl stearate](https://coolprop.org/fluid_properties/fluids/MethylStearate.html)
    #[strum(to_string = "MethylStearate", serialize = "MSTEARAT")]
    MethylStearate,

    /// MM -- hexamethyldisiloxane.
    ///
    /// # See Also
    ///
    /// - [Hexamethyldisiloxane](https://coolprop.org/fluid_properties/fluids/MM.html)
    #[strum(to_string = "MM", serialize = "Hexamethyldisiloxane")]
    MM,

    /// m-Xylene.
    ///
    /// # See Also
    ///
    /// - [m-Xylene](https://coolprop.org/fluid_properties/fluids/m-Xylene.html)
    #[strum(to_string = "m-Xylene", serialize = "mXylene", serialize = "MC8H10")]
    mXylene,

    /// n-Butane.
    ///
    /// # See Also
    ///
    /// - [n-Butane](https://coolprop.org/fluid_properties/fluids/n-Butane.html)
    #[strum(
        to_string = "n-Butane",
        serialize = "nButane",
        serialize = "Butane",
        serialize = "NC4H10",
        serialize = "n-C4H10"
    )]
    nButane,

    /// n-Decane.
    ///
    /// # See Also
    ///
    /// - [n-Decane](https://coolprop.org/fluid_properties/fluids/n-Decane.html)
    #[strum(
        to_string = "n-Decane",
        serialize = "nDecane",
        serialize = "Decane",
        serialize = "NC10H22",
        serialize = "n-C10H22"
    )]
    nDecane,

    /// n-Dodecane.
    ///
    /// # See Also
    ///
    /// - [n-Dodecane](https://coolprop.org/fluid_properties/fluids/n-Dodecane.html)
    #[strum(
        to_string = "n-Dodecane",
        serialize = "nDodecane",
        serialize = "Dodecane",
        serialize = "NC12H26",
        serialize = "n-C12H26"
    )]
    nDodecane,

    /// Neon.
    ///
    /// # See Also
    ///
    /// - [Neon](https://coolprop.org/fluid_properties/fluids/Neon.html)
    #[strum(to_string = "Neon", serialize = "Ne")]
    Neon,

    /// Neopentane.
    ///
    /// # See Also
    ///
    /// - [Neopentane](https://coolprop.org/fluid_properties/fluids/Neopentane.html)
    #[strum(to_string = "Neopentane")]
    Neopentane,

    /// n-Heptane.
    ///
    /// # See Also
    ///
    /// - [n-Heptane](https://coolprop.org/fluid_properties/fluids/n-Heptane.html)
    #[strum(
        to_string = "n-Heptane",
        serialize = "nHeptane",
        serialize = "Heptane",
        serialize = "NC7H16",
        serialize = "n-C7H16"
    )]
    nHeptane,

    /// n-Hexane.
    ///
    /// # See Also
    ///
    /// - [n-Hexane](https://coolprop.org/fluid_properties/fluids/n-Hexane.html)
    #[strum(
        to_string = "n-Hexane",
        serialize = "nHexane",
        serialize = "Hexane",
        serialize = "NC6H14",
        serialize = "n-C6H14"
    )]
    nHexane,

    /// Nitrogen.
    ///
    /// # See Also
    ///
    /// - [Nitrogen](https://coolprop.org/fluid_properties/fluids/Nitrogen.html)
    #[strum(to_string = "Nitrogen", serialize = "N2")]
    Nitrogen,

    /// Nitrous oxide.
    ///
    /// # See Also
    ///
    /// - [Nitrous oxide](https://coolprop.org/fluid_properties/fluids/NitrousOxide.html)
    #[strum(to_string = "NitrousOxide", serialize = "N2O")]
    NitrousOxide,

    /// n-Nonane.
    ///
    /// # See Also
    ///
    /// - [n-Nonane](https://coolprop.org/fluid_properties/fluids/n-Nonane.html)
    #[strum(
        to_string = "n-Nonane",
        serialize = "nNonane",
        serialize = "Nonane",
        serialize = "NC9H20",
        serialize = "n-C9H20"
    )]
    nNonane,

    /// n-Octane.
    ///
    /// # See Also
    ///
    /// - [n-Octane](https://coolprop.org/fluid_properties/fluids/n-Octane.html)
    #[strum(
        to_string = "n-Octane",
        serialize = "nOctane",
        serialize = "Octane",
        serialize = "NC8H18",
        serialize = "n-C8H18"
    )]
    nOctane,

    /// Novec 649 / Novec 1230 -- perfluoro(2-methyl-3-pentanone).
    ///
    /// # See Also
    ///
    /// - [Novec 649](https://coolprop.org/fluid_properties/fluids/Novec649.html)
    #[strum(to_string = "Novec649", serialize = "Novec1230")]
    Novec649,

    /// n-Pentane.
    ///
    /// # See Also
    ///
    /// - [n-Pentane](https://coolprop.org/fluid_properties/fluids/n-Pentane.html)
    #[strum(
        to_string = "n-Pentane",
        serialize = "nPentane",
        serialize = "Pentane",
        serialize = "NC5H12",
        serialize = "n-C5H12"
    )]
    nPentane,

    /// n-Propane.
    ///
    /// # See Also
    ///
    /// - [n-Propane](https://coolprop.org/fluid_properties/fluids/n-Propane.html)
    #[strum(
        to_string = "n-Propane",
        serialize = "nPropane",
        serialize = "Propane",
        serialize = "C3H8",
        serialize = "NC3H8",
        serialize = "n-C3H8"
    )]
    nPropane,

    /// n-Undecane.
    ///
    /// # See Also
    ///
    /// - [n-Undecane](https://coolprop.org/fluid_properties/fluids/n-Undecane.html)
    #[strum(
        to_string = "n-Undecane",
        serialize = "nUndecane",
        serialize = "Undecane",
        serialize = "NC11H24",
        serialize = "n-C11H24"
    )]
    nUndecane,

    /// Orthodeuterium.
    ///
    /// # See Also
    ///
    /// - [Orthodeuterium](https://coolprop.org/fluid_properties/fluids/OrthoDeuterium.html)
    #[strum(to_string = "OrthoDeuterium", serialize = "o-D2")]
    Orthodeuterium,

    /// Orthohydrogen.
    ///
    /// # See Also
    ///
    /// - [Orthohydrogen](https://coolprop.org/fluid_properties/fluids/OrthoHydrogen.html)
    #[strum(to_string = "OrthoHydrogen", serialize = "o-H2")]
    Orthohydrogen,

    /// Oxygen.
    ///
    /// # See Also
    ///
    /// - [Oxygen](https://coolprop.org/fluid_properties/fluids/Oxygen.html)
    #[strum(to_string = "Oxygen", serialize = "O2")]
    Oxygen,

    /// o-Xylene.
    ///
    /// # See Also
    ///
    /// - [o-Xylene](https://coolprop.org/fluid_properties/fluids/o-Xylene.html)
    #[strum(to_string = "o-Xylene", serialize = "oXylene", serialize = "OC8H10")]
    oXylene,

    /// Paradeuterium.
    ///
    /// # See Also
    ///
    /// - [Paradeuterium](https://coolprop.org/fluid_properties/fluids/ParaDeuterium.html)
    #[strum(to_string = "ParaDeuterium", serialize = "p-D2")]
    Paradeuterium,

    /// Parahydrogen.
    ///
    /// # See Also
    ///
    /// - [Parahydrogen](https://coolprop.org/fluid_properties/fluids/ParaHydrogen.html)
    #[strum(to_string = "ParaHydrogen", serialize = "p-H2")]
    Parahydrogen,

    /// Propylene.
    ///
    /// # See Also
    ///
    /// - [Propylene](https://coolprop.org/fluid_properties/fluids/Propylene.html)
    #[strum(to_string = "Propylene")]
    Propylene,

    /// Propyne.
    ///
    /// # See Also
    ///
    /// - [Propyne](https://coolprop.org/fluid_properties/fluids/Propyne.html)
    #[strum(to_string = "Propyne")]
    Propyne,

    /// p-Xylene.
    ///
    /// # See Also
    ///
    /// - [p-Xylene](https://coolprop.org/fluid_properties/fluids/p-Xylene.html)
    #[strum(to_string = "p-Xylene", serialize = "pXylene", serialize = "PC8H10")]
    pXylene,

    /// Solkatherm SES36 -- azeotropic refrigerant mixture of R365mfc and Galden HT 55.
    ///
    /// # See Also
    ///
    /// - [Solkatherm SES36](https://coolprop.org/fluid_properties/fluids/SES36.html)
    #[strum(to_string = "SES36")]
    SES36,

    /// Refrigerant R11 -- trichlorofluoromethane.
    ///
    /// # See Also
    ///
    /// - [R11](https://coolprop.org/fluid_properties/fluids/R11.html)
    #[strum(to_string = "R11")]
    R11,

    /// Refrigerant R12 -- dichlorodifluoromethane.
    ///
    /// # See Also
    ///
    /// - [R12](https://coolprop.org/fluid_properties/fluids/R12.html)
    #[strum(to_string = "R12")]
    R12,

    /// Refrigerant R13 -- chlorotrifluoromethane.
    ///
    /// # See Also
    ///
    /// - [R13](https://coolprop.org/fluid_properties/fluids/R13.html)
    #[strum(to_string = "R13")]
    R13,

    /// Refrigerant R13I1 -- trifluoroiodomethane.
    ///
    /// # See Also
    ///
    /// - [R13I1](https://coolprop.org/fluid_properties/fluids/R13I1.html)
    #[strum(to_string = "R13I1", serialize = "CF3I")]
    R13I1,

    /// Refrigerant R14 -- carbon tetrafluoride.
    ///
    /// # See Also
    ///
    /// - [R14](https://coolprop.org/fluid_properties/fluids/R14.html)
    #[strum(to_string = "R14")]
    R14,

    /// Refrigerant R21 -- dichlorofluoromethane.
    ///
    /// # See Also
    ///
    /// - [R21](https://coolprop.org/fluid_properties/fluids/R21.html)
    #[strum(to_string = "R21")]
    R21,

    /// Refrigerant R22 -- chlorodifluoromethane.
    ///
    /// # See Also
    ///
    /// - [R22](https://coolprop.org/fluid_properties/fluids/R22.html)
    #[strum(to_string = "R22")]
    R22,

    /// Refrigerant R23 -- trifluoromethane.
    ///
    /// # See Also
    ///
    /// - [R23](https://coolprop.org/fluid_properties/fluids/R23.html)
    #[strum(to_string = "R23")]
    R23,

    /// Refrigerant R32 -- difluoromethane.
    ///
    /// # See Also
    ///
    /// - [R32](https://coolprop.org/fluid_properties/fluids/R32.html)
    #[strum(to_string = "R32")]
    R32,

    /// Refrigerant R40 -- chloromethane.
    ///
    /// # See Also
    ///
    /// - [R40](https://coolprop.org/fluid_properties/fluids/R40.html)
    #[strum(to_string = "R40")]
    R40,

    /// Refrigerant R41 -- fluoromethane.
    ///
    /// # See Also
    ///
    /// - [R41](https://coolprop.org/fluid_properties/fluids/R41.html)
    #[strum(to_string = "R41")]
    R41,

    /// Refrigerant R50 -- methane.
    ///
    /// # See Also
    ///
    /// - [Methane](https://coolprop.org/fluid_properties/fluids/Methane.html)
    #[strum(to_string = "R50")]
    R50,

    /// Refrigerant R113 -- 1,1,2-trichloro-1,2,2-trifluoroethane.
    ///
    /// # See Also
    ///
    /// - [R113](https://coolprop.org/fluid_properties/fluids/R113.html)
    #[strum(to_string = "R113")]
    R113,

    /// Refrigerant R114 -- 1,2-dichloro-1,1,2,2-tetrafluoroethane.
    ///
    /// # See Also
    ///
    /// - [R114](https://coolprop.org/fluid_properties/fluids/R114.html)
    #[strum(to_string = "R114")]
    R114,

    /// Refrigerant R115 -- chloropentafluoroethane.
    ///
    /// # See Also
    ///
    /// - [R115](https://coolprop.org/fluid_properties/fluids/R115.html)
    #[strum(to_string = "R115")]
    R115,

    /// Refrigerant R116 -- hexafluoroethane.
    ///
    /// # See Also
    ///
    /// - [R116](https://coolprop.org/fluid_properties/fluids/R116.html)
    #[strum(to_string = "R116")]
    R116,

    /// Refrigerant R123 -- 2,2-dichloro-1,1,1-trifluoroethane.
    ///
    /// # See Also
    ///
    /// - [R123](https://coolprop.org/fluid_properties/fluids/R123.html)
    #[strum(to_string = "R123")]
    R123,

    /// Refrigerant R124 -- 2-chloro-1,1,1,2-tetrafluoroethane.
    ///
    /// # See Also
    ///
    /// - [R124](https://coolprop.org/fluid_properties/fluids/R124.html)
    #[strum(to_string = "R124")]
    R124,

    /// Refrigerant R125 -- pentafluoroethane.
    ///
    /// # See Also
    ///
    /// - [R125](https://coolprop.org/fluid_properties/fluids/R125.html)
    #[strum(to_string = "R125")]
    R125,

    /// Refrigerant R134a -- 1,1,1,2-tetrafluoroethane.
    ///
    /// # See Also
    ///
    /// - [R134a](https://coolprop.org/fluid_properties/fluids/R134a.html)
    #[strum(to_string = "R134a")]
    R134a,

    /// Refrigerant R141b -- 1,1-dichloro-1-fluoroethane.
    ///
    /// # See Also
    ///
    /// - [R141b](https://coolprop.org/fluid_properties/fluids/R141b.html)
    #[strum(to_string = "R141b")]
    R141b,

    /// Refrigerant R142b -- 1-chloro-1,1-difluoroethane.
    ///
    /// # See Also
    ///
    /// - [R142b](https://coolprop.org/fluid_properties/fluids/R142b.html)
    #[strum(to_string = "R142b")]
    R142b,

    /// Refrigerant R143a -- 1,1,1-trifluoroethane.
    ///
    /// # See Also
    ///
    /// - [R143a](https://coolprop.org/fluid_properties/fluids/R143a.html)
    #[strum(to_string = "R143a")]
    R143a,

    /// Refrigerant RE143a -- trifluoromethyl methyl ether.
    ///
    /// # See Also
    ///
    /// - [Trifluoromethyl methyl ether](https://coolprop.org/fluid_properties/fluids/HFE143m.html)
    #[strum(to_string = "RE143a")]
    RE143a,

    /// Refrigerant R152a -- 1,1-difluoroethane.
    ///
    /// # See Also
    ///
    /// - [R152a](https://coolprop.org/fluid_properties/fluids/R152A.html)
    #[strum(to_string = "R152a")]
    R152a,

    /// Refrigerant R161 -- fluoroethane.
    ///
    /// # See Also
    ///
    /// - [R161](https://coolprop.org/fluid_properties/fluids/R161.html)
    #[strum(to_string = "R161")]
    R161,

    /// Refrigerant R170 -- ethane.
    ///
    /// # See Also
    ///
    /// - [Ethane](https://coolprop.org/fluid_properties/fluids/Ethane.html)
    #[strum(to_string = "R170")]
    R170,

    /// Refrigerant R218 -- octafluoropropane.
    ///
    /// # See Also
    ///
    /// - [R218](https://coolprop.org/fluid_properties/fluids/R218.html)
    #[strum(to_string = "R218")]
    R218,

    /// Refrigerant R227ea -- 1,1,1,2,3,3,3-heptafluoropropane.
    ///
    /// # See Also
    ///
    /// - [R227ea](https://coolprop.org/fluid_properties/fluids/R227EA.html)
    #[strum(to_string = "R227ea")]
    R227ea,

    /// Refrigerant R236ea -- 1,1,1,2,3,3-hexafluoropropane.
    ///
    /// # See Also
    ///
    /// - [R236ea](https://coolprop.org/fluid_properties/fluids/R236EA.html)
    #[strum(to_string = "R236ea")]
    R236ea,

    /// Refrigerant R236fa -- 1,1,1,3,3,3-hexafluoropropane.
    ///
    /// # See Also
    ///
    /// - [R236fa](https://coolprop.org/fluid_properties/fluids/R236FA.html)
    #[strum(to_string = "R236fa")]
    R236fa,

    /// Refrigerant R245ca -- 1,1,2,2,3-pentafluoropropane.
    ///
    /// # See Also
    ///
    /// - [R245ca](https://coolprop.org/fluid_properties/fluids/R245ca.html)
    #[strum(to_string = "R245ca")]
    R245ca,

    /// Refrigerant R245fa -- 1,1,1,3,3-pentafluoropropane.
    ///
    /// # See Also
    ///
    /// - [R245fa](https://coolprop.org/fluid_properties/fluids/R245fa.html)
    #[strum(to_string = "R245fa")]
    R245fa,

    /// Refrigerant R290 -- n-propane.
    ///
    /// # See Also
    ///
    /// - [n-Propane](https://coolprop.org/fluid_properties/fluids/n-Propane.html)
    #[strum(to_string = "R290")]
    R290,

    /// Refrigerant RC318 -- octafluorocyclobutane.
    ///
    /// # See Also
    ///
    /// - [Octafluorocyclobutane](https://coolprop.org/fluid_properties/fluids/RC318.html)
    #[strum(to_string = "RC318")]
    RC318,

    /// Refrigerant R365mfc -- 1,1,1,3,3-pentafluorobutane.
    ///
    /// # See Also
    ///
    /// - [R365mfc](https://coolprop.org/fluid_properties/fluids/R365MFC.html)
    #[strum(to_string = "R365mfc")]
    R365mfc,

    /// Pseudo-pure refrigerant blend R404A.
    ///
    /// # See Also
    ///
    /// - [R404A](https://coolprop.org/fluid_properties/fluids/R404A.html)
    #[strum(to_string = "R404A")]
    R404A,

    /// Pseudo-pure refrigerant blend R407C.
    ///
    /// # See Also
    ///
    /// - [R407C](https://coolprop.org/fluid_properties/fluids/R407C.html)
    #[strum(to_string = "R407C")]
    R407C,

    /// Pseudo-pure refrigerant blend R410A.
    ///
    /// # See Also
    ///
    /// - [R410A](https://coolprop.org/fluid_properties/fluids/R410A.html)
    #[strum(to_string = "R410A")]
    R410A,

    /// Pseudo-pure refrigerant blend R507A.
    ///
    /// # See Also
    ///
    /// - [R507A](https://coolprop.org/fluid_properties/fluids/R507A.html)
    #[strum(to_string = "R507A")]
    R507A,

    /// Refrigerant R600 -- n-butane.
    ///
    /// # See Also
    ///
    /// - [n-Butane](https://coolprop.org/fluid_properties/fluids/n-Butane.html)
    #[strum(to_string = "R600")]
    R600,

    /// Refrigerant R600a -- isobutane.
    ///
    /// # See Also
    ///
    /// - [Isobutane](https://coolprop.org/fluid_properties/fluids/IsoButane.html)
    #[strum(to_string = "R600a")]
    R600a,

    /// Refrigerant R601 -- n-pentane.
    ///
    /// # See Also
    ///
    /// - [n-Pentane](https://coolprop.org/fluid_properties/fluids/n-Pentane.html)
    #[strum(to_string = "R601")]
    R601,

    /// Refrigerant R601a -- isopentane.
    ///
    /// # See Also
    ///
    /// - [Isopentane](https://coolprop.org/fluid_properties/fluids/Isopentane.html)
    #[strum(to_string = "R601a")]
    R601a,

    /// Refrigerant R702 -- hydrogen.
    ///
    /// # See Also
    ///
    /// - [Hydrogen](https://coolprop.org/fluid_properties/fluids/Hydrogen.html)
    #[strum(to_string = "R702")]
    R702,

    /// Refrigerant R704 -- helium.
    ///
    /// # See Also
    ///
    /// - [Helium](https://coolprop.org/fluid_properties/fluids/Helium.html)
    #[strum(to_string = "R704")]
    R704,

    /// Refrigerant R717 -- ammonia.
    ///
    /// # See Also
    ///
    /// - [Ammonia](https://coolprop.org/fluid_properties/fluids/Ammonia.html)
    #[strum(to_string = "R717")]
    R717,

    /// Refrigerant R718 -- water.
    ///
    /// # See Also
    ///
    /// - [Water](https://coolprop.org/fluid_properties/fluids/Water.html)
    #[strum(to_string = "R718")]
    R718,

    /// Refrigerant R720 -- neon.
    ///
    /// # See Also
    ///
    /// - [Neon](https://coolprop.org/fluid_properties/fluids/Neon.html)
    #[strum(to_string = "R720")]
    R720,

    /// Refrigerant R728 -- nitrogen.
    ///
    /// # See Also
    ///
    /// - [Nitrogen](https://coolprop.org/fluid_properties/fluids/Nitrogen.html)
    #[strum(to_string = "R728")]
    R728,

    /// Refrigerant R729 -- air.
    ///
    /// # See Also
    ///
    /// - [Air](https://coolprop.org/fluid_properties/fluids/Air.html)
    #[strum(to_string = "R729")]
    R729,

    /// Refrigerant R732 -- oxygen.
    ///
    /// # See Also
    ///
    /// - [Oxygen](https://coolprop.org/fluid_properties/fluids/Oxygen.html)
    #[strum(to_string = "R732")]
    R732,

    /// Refrigerant R740 -- argon.
    ///
    /// # See Also
    ///
    /// - [Argon](https://coolprop.org/fluid_properties/fluids/Argon.html)
    #[strum(to_string = "R740")]
    R740,

    /// Refrigerant R744 -- carbon dioxide.
    ///
    /// # See Also
    ///
    /// - [Carbon dioxide](https://coolprop.org/fluid_properties/fluids/CarbonDioxide.html)
    #[strum(to_string = "R744")]
    R744,

    /// Refrigerant R764 -- sulfur dioxide.
    ///
    /// # See Also
    ///
    /// - [Sulfur dioxide](https://coolprop.org/fluid_properties/fluids/SulfurDioxide.html)
    #[strum(to_string = "SulfurDioxide", serialize = "R764")]
    R764,

    /// Refrigerant R846 -- sulfur hexafluoride.
    ///
    /// # See Also
    ///
    /// - [Sulfur hexafluoride](https://coolprop.org/fluid_properties/fluids/SulfurHexafluoride.html)
    #[strum(to_string = "SulfurHexafluoride", serialize = "R846")]
    R846,

    /// Refrigerant R1150 -- ethylene.
    ///
    /// # See Also
    ///
    /// - [Ethylene](https://coolprop.org/fluid_properties/fluids/Ethylene.html)
    #[strum(to_string = "R1150")]
    R1150,

    /// Refrigerant R1233zd(E) -- trans-1-chloro-3,3,3-trifluoropropene.
    ///
    /// # See Also
    ///
    /// - [R1233zd(E)](https://coolprop.org/fluid_properties/fluids/R1233zd%28E%29.html)
    #[strum(to_string = "R1233zd(E)", serialize = "R1233zdE")]
    R1233zdE,

    /// Refrigerant R1234yf -- 2,3,3,3-tetrafluoropropene.
    ///
    /// # See Also
    ///
    /// - [R1234yf](https://coolprop.org/fluid_properties/fluids/R1234yf.html)
    #[strum(to_string = "R1234yf")]
    R1234yf,

    /// Refrigerant R1234ze(E) -- trans-1,3,3,3-tetrafluoropropene.
    ///
    /// # See Also
    ///
    /// - [R1234ze(E)](https://coolprop.org/fluid_properties/fluids/R1234ze%28E%29.html)
    #[strum(to_string = "R1234ze(E)", serialize = "R1234zeE")]
    R1234zeE,

    /// Refrigerant R1234ze(Z) -- cis-1,3,3,3-tetrafluoropropene.
    ///
    /// # See Also
    ///
    /// - [R1234ze(Z)](https://coolprop.org/fluid_properties/fluids/R1234ze%28Z%29.html)
    #[strum(to_string = "R1234ze(Z)", serialize = "R1234zeZ")]
    R1234zeZ,

    /// Refrigerant R1243zf -- 3,3,3-trifluoropropene.
    ///
    /// # See Also
    ///
    /// - [R1243zf](https://coolprop.org/fluid_properties/fluids/R1243zf.html)
    #[strum(to_string = "R1243zf")]
    R1243zf,

    /// Refrigerant R1270 -- propylene.
    ///
    /// # See Also
    ///
    /// - [Propylene](https://coolprop.org/fluid_properties/fluids/Propylene.html)
    #[strum(to_string = "R1270")]
    R1270,

    /// Refrigerant R1336mzz(E) -- trans-1,1,1,4,4,4-hexafluoro-2-butene.
    ///
    /// # See Also
    ///
    /// - [R1336mzz(E)](https://coolprop.org/fluid_properties/fluids/R1336mzz%28E%29.html)
    #[strum(to_string = "R1336mzz(E)", serialize = "R1336mzzE")]
    R1336mzzE,

    /// Sulfur dioxide.
    ///
    /// # See Also
    ///
    /// - [Sulfur dioxide](https://coolprop.org/fluid_properties/fluids/SulfurDioxide.html)
    #[strum(to_string = "SulfurDioxide", serialize = "SO2")]
    SulfurDioxide,

    /// Sulfur hexafluoride.
    ///
    /// # See Also
    ///
    /// - [Sulfur hexafluoride](https://coolprop.org/fluid_properties/fluids/SulfurHexafluoride.html)
    #[strum(to_string = "SulfurHexafluoride", serialize = "SF6")]
    SulfurHexafluoride,

    /// Toluene.
    ///
    /// # See Also
    ///
    /// - [Toluene](https://coolprop.org/fluid_properties/fluids/Toluene.html)
    #[strum(to_string = "Toluene")]
    Toluene,

    /// trans-2-Butene.
    ///
    /// # See Also
    ///
    /// - [trans-2-Butene](https://coolprop.org/fluid_properties/fluids/trans-2-Butene.html)
    #[strum(to_string = "trans-2-Butene", serialize = "T2BUTENE")]
    trans2Butene,

    /// Water.
    ///
    /// # See Also
    ///
    /// - [Water](https://coolprop.org/fluid_properties/fluids/Water.html)
    #[strum(to_string = "Water", serialize = "H2O")]
    Water,

    /// Xenon.
    ///
    /// # See Also
    ///
    /// - [Xenon](https://coolprop.org/fluid_properties/fluids/Xenon.html)
    #[strum(to_string = "Xenon", serialize = "Xe")]
    Xenon,
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use rstest::*;

    use super::{Pure::*, *};

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
    fn as_str(#[case] sut: Pure, #[case] expected: &str) {
        // When
        let str = sut.as_ref();
        let static_str: &'static str = sut.into();

        // Then
        assert_eq!(str, expected);
        assert_eq!(static_str, expected);
    }

    #[rstest]
    #[case(vec!["Acetone"], Acetone)]
    #[case(vec!["Air"], Air)]
    #[case(vec!["Ammonia", "NH3"], Ammonia)]
    #[case(vec!["Argon", "Ar"], Argon)]
    #[case(vec!["Benzene"], Benzene)]
    #[case(vec!["1-Butene", "1Butene",  "Butene"], Butene)]
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
    #[case(vec!["Methane", "CH4",  "n-C1H4"], Methane)]
    #[case(vec!["Methanol"], Methanol)]
    #[case(vec!["MethylLinoleate", "MLINOLEA"], MethylLinoleate)]
    #[case(vec!["MethylLinolenate", "MLINOLEN"], MethylLinolenate)]
    #[case(vec!["MethylOleate", "MOLEATE"], MethylOleate)]
    #[case(vec!["MethylPalmitate", "MPALMITA"], MethylPalmitate)]
    #[case(vec!["MethylStearate", "MSTEARAT"], MethylStearate)]
    #[case(vec!["MM", "Hexamethyldisiloxane"], MM)]
    #[case(vec!["m-Xylene", "mXylene",  "MC8H10"], mXylene)]
    #[case(vec!["n-Butane", "nButane",  "Butane",  "NC4H10",  "n-C4H10"], nButane)]
    #[case(vec!["n-Decane", "nDecane",  "Decane",  "NC10H22",  "n-C10H22"], nDecane)]
    #[case(vec!["n-Dodecane", "nDodecane",  "Dodecane",  "NC12H26",  "n-C12H26"], nDodecane)]
    #[case(vec!["Neon"], Neon)]
    #[case(vec!["Neopentane"], Neopentane)]
    #[case(vec!["n-Heptane", "nHeptane",  "Heptane",  "NC7H16",  "n-C7H16"], nHeptane)]
    #[case(vec!["n-Hexane", "nHexane",  "Hexane",  "NC6H14",  "n-C6H14"], nHexane)]
    #[case(vec!["Nitrogen", "N2"], Nitrogen)]
    #[case(vec!["NitrousOxide", "N2O"], NitrousOxide)]
    #[case(vec!["n-Nonane", "nNonane",  "Nonane",  "NC9H20",  "n-C9H20"], nNonane)]
    #[case(vec!["n-Octane", "nOctane",  "Octane",  "NC8H18",  "n-C8H18"], nOctane)]
    #[case(vec!["Novec649", "Novec1230"], Novec649)]
    #[case(vec!["n-Pentane", "nPentane",  "Pentane",  "NC5H12",  "n-C5H12"], nPentane)]
    #[case(vec!["n-Propane", "nPropane",  "Propane",  "C3H8",  "NC3H8",  "n-C3H8"], nPropane)]
    #[case(vec!["n-Undecane", "nUndecane",  "Undecane",  "NC11H24",  "n-C11H24"], nUndecane)]
    #[case(vec!["OrthoDeuterium"], Orthodeuterium)]
    #[case(vec!["OrthoHydrogen"], Orthohydrogen)]
    #[case(vec!["Oxygen", "O2"], Oxygen)]
    #[case(vec!["o-Xylene", "oXylene",  "OC8H10"], oXylene)]
    #[case(vec!["ParaDeuterium"], Paradeuterium)]
    #[case(vec!["ParaHydrogen"], Parahydrogen)]
    #[case(vec!["Propylene"], Propylene)]
    #[case(vec!["Propyne"], Propyne)]
    #[case(vec!["p-Xylene", "pXylene",  "PC8H10"], pXylene)]
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
    fn from_valid_str(#[case] valid: Vec<&str>, #[case] expected: Pure) {
        for s in valid {
            // When
            let res1 = Pure::from_str(s).unwrap();
            let res2 = Pure::try_from(s).unwrap();

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
        let res1 = Pure::from_str(invalid);
        let res2 = Pure::try_from(invalid);

        // Then
        assert!(res1.is_err());
        assert!(res2.is_err());
    }
}
