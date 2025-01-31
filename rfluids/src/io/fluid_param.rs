// cSpell:disable

use crate::io::try_from;
use strum_macros::{AsRefStr, EnumString, FromRepr};

/// CoolProp fluids input/output parameters.
///
/// # Examples
///
/// Conversion between [`&str`](str):
///
/// ```
/// use std::str::FromStr;
/// use rfluids::io::FluidParam;
///
/// assert_eq!(FluidParam::Conductivity.as_ref(), "conductivity");
/// assert_eq!(FluidParam::from_str("conductivity"), Ok(FluidParam::Conductivity));
/// assert_eq!(FluidParam::try_from("L"), Ok(FluidParam::Conductivity));
/// ```
///
/// Conversion between [`u8`]:
///
/// ```
/// use rfluids::io::FluidParam;
///
/// assert_eq!(u8::from(FluidParam::Conductivity), 46);
/// assert_eq!(FluidParam::try_from(46), Ok(FluidParam::Conductivity));
/// ```
///
/// Conversion between [`f64`]:
///
/// ```
/// use rfluids::io::FluidParam;
///
/// assert_eq!(FluidParam::try_from(46.0), Ok(FluidParam::Conductivity));
/// ```
///
/// Conversion between [`FluidInputPair`](crate::io::FluidInputPair):
///
/// ```
/// use rfluids::io::{FluidInputPair, FluidParam};
///
/// assert_eq!(
///     <(FluidParam, FluidParam)>::from(FluidInputPair::PT),
///     (FluidParam::P, FluidParam::T)
/// );
/// assert_eq!(
///     FluidInputPair::try_from((FluidParam::T, FluidParam::P)),
///     Ok(FluidInputPair::PT)
/// );
/// ```
///
/// # See also
///
/// - [CoolProp fluids input/output parameters _(only those for which the value in the "Trivial" column is "False")_](https://coolprop.github.io/CoolProp/coolprop/HighLevelAPI.html#parameter-table)
#[derive(AsRefStr, EnumString, FromRepr, Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[strum(ascii_case_insensitive)]
#[repr(u8)]
pub enum FluidParam {
    /// Temperature _(K)_.
    #[strum(to_string = "T")]
    T = 19,

    /// Pressure _(Pa)_.
    #[strum(to_string = "P")]
    P = 20,

    /// Vapor quality _(dimensionless, from 0 to 1)_.
    #[strum(to_string = "Q")]
    Q = 21,

    /// Reciprocal reduced temperature _(dimensionless,
    /// [`TCritical`](FluidTrivialParam::TCritical)/[`T`](FluidParam::T))_.
    #[strum(to_string = "Tau")]
    Tau = 22,

    /// Reduced density _(dimensionless,
    /// [`DMass`](FluidParam::DMass)/[`DMassCritical`](FluidTrivialParam::DMassCritical))_.
    #[strum(to_string = "Delta")]
    Delta = 23,

    /// Molar density _(mol/m³)_.
    #[strum(to_string = "Dmolar")]
    DMolar = 24,

    /// Molar specific enthalpy _(J/mol)_.
    #[strum(to_string = "Hmolar")]
    HMolar = 25,

    /// Molar specific entropy _(J/mol/K)_.
    #[strum(to_string = "Smolar")]
    SMolar = 26,

    /// Molar specific constant pressure specific heat _(J/mol/K)_.
    #[strum(to_string = "Cpmolar")]
    CpMolar = 27,

    /// Ideal gas molar specific constant pressure specific heat _(J/mol/K)_.
    #[strum(to_string = "Cp0molar")]
    Cp0Molar = 28,

    /// Molar specific constant volume specific heat _(J/mol/K)_.
    #[strum(to_string = "Cvmolar")]
    CvMolar = 29,

    /// Molar specific internal energy _(J/mol)_.
    #[strum(to_string = "Umolar")]
    UMolar = 30,

    /// Molar specific Gibbs energy _(J/mol)_.
    #[strum(to_string = "Gmolar")]
    GMolar = 31,

    /// Molar specific Helmholtz energy _(J/mol)_.
    #[strum(to_string = "Helmholtzmolar")]
    HelmholtzMolar = 32,

    /// Residual molar enthalpy _(J/mol/K)_.
    #[strum(to_string = "Hmolar_residual")]
    HMolarResidual = 33,

    /// Residual molar entropy _(J/mol/K)_.
    #[strum(to_string = "Smolar_residual")]
    SMolarResidual = 34,

    /// Residual molar Gibbs energy _(J/mol/K)_.
    #[strum(to_string = "Gmolar_residual")]
    GMolarResidual = 35,

    /// Mass density _(kg/m³)_.
    #[strum(to_string = "Dmass", serialize = "D")]
    DMass = 36,

    /// Mass specific enthalpy _(J/kg)_.
    #[strum(to_string = "Hmass", serialize = "H")]
    HMass = 37,

    /// Mass specific entropy _(J/kg/K)_.
    #[strum(to_string = "Smass", serialize = "S")]
    SMass = 38,

    /// Mass specific constant pressure specific heat _(J/kg/K)_.
    #[strum(to_string = "Cpmass", serialize = "C")]
    CpMass = 39,

    /// Ideal gas mass specific constant pressure specific heat _(J/kg/K)_.
    #[strum(to_string = "Cp0mass")]
    Cp0Mass = 40,

    /// Mass specific constant volume specific heat _(J/kg/K)_.
    #[strum(to_string = "Cvmass", serialize = "O")]
    CvMass = 41,

    /// Mass specific internal energy _(J/kg)_.
    #[strum(to_string = "Umass", serialize = "U")]
    UMass = 42,

    /// Mass specific Gibbs energy _(J/kg)_.
    #[strum(to_string = "Gmass", serialize = "G")]
    GMass = 43,

    /// Mass specific Helmholtz energy _(J/kg)_.
    #[strum(to_string = "Helmholtzmass")]
    HelmholtzMass = 44,

    /// Dynamic viscosity _(Pa*s)_.
    #[strum(to_string = "viscosity", serialize = "V")]
    DynamicViscosity = 45,

    /// Thermal conductivity _(W/m/K)_.
    #[strum(to_string = "conductivity", serialize = "L")]
    Conductivity = 46,

    /// Surface tension _(N/m)_.
    #[strum(to_string = "surface_tension", serialize = "I")]
    SurfaceTension = 47,

    /// Prandtl number _(dimensionless)_.
    #[strum(to_string = "Prandtl")]
    Prandtl = 48,

    /// Sound speed _(m/s)_.
    #[strum(
        to_string = "speed_sound",
        serialize = "speed_of_sound",
        serialize = "A"
    )]
    SoundSpeed = 49,

    /// Isothermal compressibility _(1/Pa)_.
    #[strum(to_string = "isothermal_compressibility")]
    IsothermalCompressibility = 50,

    /// Isobaric expansion coefficient _(1/K)_.
    #[strum(to_string = "isobaric_expansion_coefficient")]
    IsobaricExpansionCoefficient = 51,

    /// Isentropic expansion coefficient _(dimensionless)_.
    #[strum(to_string = "isentropic_expansion_coefficient")]
    IsentropicExpansionCoefficient = 52,

    /// Fundamental derivative of gas dynamics _(dimensionless)_.
    #[strum(to_string = "fundamental_derivative_of_gas_dynamics")]
    FundamentalDerivativeOfGasDynamics = 53,

    /// Residual Helmholtz energy _(dimensionless)_.
    #[strum(to_string = "alphar")]
    AlphaR = 54,

    /// Derivative of residual Helmholtz energy
    /// with _[`Tau`](FluidParam::Tau) (dimensionless)_.
    #[strum(to_string = "dalphar_dtau_constdelta")]
    DAlphaRDTauConstDelta = 55,

    /// Derivative of residual Helmholtz energy
    /// with _[`Delta`](FluidParam::Delta) (dimensionless)_.
    #[strum(to_string = "dalphar_ddelta_consttau")]
    DAlphaRDDeltaConstTau = 56,

    /// Ideal Helmholtz energy _(dimensionless)_.
    #[strum(to_string = "alpha0")]
    Alpha0 = 57,

    /// Derivative of ideal Helmholtz energy
    /// with _[`Tau`](FluidParam::Tau) (dimensionless)_.
    #[strum(to_string = "dalpha0_dtau_constdelta")]
    DAlpha0DTauConstDelta = 58,

    /// Derivative of ideal Helmholtz energy
    /// with _[`Delta`](FluidParam::Delta) (dimensionless)_.
    #[strum(to_string = "dalpha0_ddelta_consttau")]
    DAlpha0DDeltaConstTau = 59,

    /// Second derivative of ideal Helmholtz energy
    /// with _[`Delta`](FluidParam::Delta) (dimensionless)_.
    #[strum(to_string = "d2alpha0_ddelta2_consttau")]
    D2Alpha0DDelta2ConstTau = 60,

    /// Third derivative of ideal Helmholtz energy
    /// with _[`Delta`](FluidParam::Delta) (dimensionless)_.
    #[strum(to_string = "d3alpha0_ddelta3_consttau")]
    D3Alpha0DDelta3ConstTau = 61,

    /// Second virial coefficient _(dimensionless)_.
    #[strum(to_string = "Bvirial")]
    BVirial = 62,

    /// Third virial coefficient _(dimensionless)_.
    #[strum(to_string = "Cvirial")]
    CVirial = 63,

    /// Derivative of second virial coefficient
    /// with _[`T`](FluidParam::T) (dimensionless)_.
    #[strum(to_string = "dBvirial_dT")]
    DBVirialDT = 64,

    /// Derivative of third virial coefficient
    /// with _[`T`](FluidParam::T) (dimensionless)_.
    #[strum(to_string = "dCvirial_dT")]
    DCVirialDT = 65,

    /// Compressibility factor _(dimensionless)_.
    #[strum(to_string = "Z")]
    Z = 66,

    /// Phase identification parameter _(dimensionless)_.
    #[strum(to_string = "PIP")]
    PIP = 67,

    /// Phase index _(dimensionless)_.
    #[strum(to_string = "Phase")]
    Phase = 78,
}

impl From<FluidParam> for u8 {
    fn from(value: FluidParam) -> Self {
        value as u8
    }
}

impl TryFrom<u8> for FluidParam {
    type Error = strum::ParseError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        FluidParam::from_repr(value).ok_or(strum::ParseError::VariantNotFound)
    }
}

impl TryFrom<f64> for FluidParam {
    type Error = strum::ParseError;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        try_from(value)
    }
}

/// CoolProp fluids trivial output parameters.
///
/// # Examples
///
/// Conversion between [`&str`](str):
///
/// ```
/// use std::str::FromStr;
/// use rfluids::io::FluidTrivialParam;
///
/// assert_eq!(FluidTrivialParam::TMin.as_ref(), "T_min");
/// assert_eq!(FluidTrivialParam::from_str("T_min"), Ok(FluidTrivialParam::TMin));
/// assert_eq!(FluidTrivialParam::try_from("T_min"), Ok(FluidTrivialParam::TMin));
/// ```
///
/// Conversion between [`u8`]:
///
/// ```
/// use rfluids::io::FluidTrivialParam;
///
/// assert_eq!(u8::from(FluidTrivialParam::TMax), 15);
/// assert_eq!(FluidTrivialParam::try_from(15), Ok(FluidTrivialParam::TMax));
/// ```
///
/// Conversion between [`f64`]:
///
/// ```
/// use rfluids::io::FluidTrivialParam;
///
/// assert_eq!(FluidTrivialParam::try_from(15.0), Ok(FluidTrivialParam::TMax));
/// ```
///
/// # See also
///
/// - [CoolProp fluids input/output parameters _(only those for which the value in the "Trivial" column is "True")_](https://coolprop.github.io/CoolProp/coolprop/HighLevelAPI.html#parameter-table)
#[derive(AsRefStr, EnumString, FromRepr, Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[strum(ascii_case_insensitive)]
#[repr(u8)]
pub enum FluidTrivialParam {
    /// Molar gas constant _(J/mol/K)_.
    #[strum(to_string = "gas_constant")]
    GasConstant = 1,

    /// Molar mass _(kg/mol)_.
    #[strum(
        to_string = "molar_mass",
        serialize = "M",
        serialize = "molarmass",
        serialize = "molemass"
    )]
    MolarMass = 2,

    /// Acentric factor _(dimensionless)_.
    #[strum(to_string = "acentric_factor", serialize = "acentric")]
    AcentricFactor = 3,

    /// Reducing point molar density _(mol/m³)_.
    #[strum(to_string = "rhomolar_reducing")]
    DMolarReducing = 4,

    /// Critical point molar density _(mol/m³)_.
    #[strum(to_string = "rhomolar_critical")]
    DMolarCritical = 5,

    /// Reducing point temperature _(K)_.
    #[strum(to_string = "T_reducing")]
    TReducing = 6,

    /// Critical point temperature _(K)_.
    #[strum(to_string = "T_critical", serialize = "Tcrit")]
    TCritical = 7,

    /// Reducing point mass density _(kg/m³)_.
    #[strum(to_string = "rhomass_reducing")]
    DMassReducing = 8,

    /// Critical point mass density _(kg/m³)_.
    #[strum(to_string = "rhomass_critical", serialize = "rhocrit")]
    DMassCritical = 9,

    /// Critical point pressure _(Pa)_.
    #[strum(to_string = "P_critical", serialize = "Pcrit")]
    PCritical = 10,

    /// Reducing point pressure _(Pa)_.
    #[strum(to_string = "P_reducing")]
    PReducing = 11,

    /// Triple point temperature _(K)_.
    #[strum(to_string = "T_triple", serialize = "Ttriple")]
    TTriple = 12,

    /// Triple point pressure _(Pa)_.
    #[strum(to_string = "P_triple", serialize = "Ptriple")]
    PTriple = 13,

    /// Minimum temperature _(K)_.
    #[strum(to_string = "T_min", serialize = "Tmin")]
    TMin = 14,

    /// Maximum temperature _(K)_.
    #[strum(to_string = "T_max", serialize = "Tmax")]
    TMax = 15,

    /// Maximum pressure _(Pa)_.
    #[strum(to_string = "P_max", serialize = "Pmax")]
    PMax = 16,

    /// Minimum pressure _(Pa)_.
    #[strum(to_string = "P_min", serialize = "Pmin")]
    PMin = 17,

    /// Dipole moment (C*m).
    #[strum(to_string = "dipole_moment")]
    DipoleMoment = 18,

    /// Minimum fraction (mole, mass or volume) value
    /// for incompressible mixtures _(dimensionless, from 0 to 1)_.
    #[strum(to_string = "fraction_min")]
    MinFraction = 68,

    /// Maximum fraction (mole, mass or volume) value
    /// for incompressible mixtures _(dimensionless, from 0 to 1)_.
    #[strum(to_string = "fraction_max")]
    MaxFraction = 69,

    /// Freezing temperature for incompressible mixtures _(K)_.
    #[strum(to_string = "T_freeze")]
    TFreeze = 70,

    /// 20-year global warming potential _(dimensionless)_.
    #[strum(to_string = "GWP20")]
    GWP20 = 71,

    /// 100-year global warming potential _(dimensionless)_.
    #[strum(to_string = "GWP100")]
    GWP100 = 72,

    /// 500-year global warming potential _(dimensionless)_.
    #[strum(to_string = "GWP500")]
    GWP500 = 73,

    /// Flammability hazard index _(dimensionless)_.
    #[strum(to_string = "FH")]
    FH = 74,

    /// Health hazard index _(dimensionless)_.
    #[strum(to_string = "HH")]
    HH = 75,

    /// Physical hazard index _(dimensionless)_.
    #[strum(to_string = "PH")]
    PH = 76,

    /// Ozone depletion potential _(dimensionless)_.
    #[strum(to_string = "ODP")]
    ODP = 77,
}

impl From<FluidTrivialParam> for u8 {
    fn from(value: FluidTrivialParam) -> Self {
        value as u8
    }
}

impl TryFrom<u8> for FluidTrivialParam {
    type Error = strum::ParseError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        FluidTrivialParam::from_repr(value).ok_or(strum::ParseError::VariantNotFound)
    }
}

impl TryFrom<f64> for FluidTrivialParam {
    type Error = strum::ParseError;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        try_from(value)
    }
}

#[cfg(test)]
mod tests {
    use super::FluidParam::*;
    use super::FluidTrivialParam::*;
    use super::*;
    use rstest::*;
    use std::fmt::Debug;
    use std::str::FromStr;

    #[rstest]
    #[case(GasConstant, "gas_constant")]
    #[case(MolarMass, "molar_mass")]
    #[case(AcentricFactor, "acentric_factor")]
    #[case(DMolarReducing, "rhomolar_reducing")]
    #[case(DMolarCritical, "rhomolar_critical")]
    #[case(TReducing, "T_reducing")]
    #[case(TCritical, "T_critical")]
    #[case(DMassReducing, "rhomass_reducing")]
    #[case(DMassCritical, "rhomass_critical")]
    #[case(PCritical, "P_critical")]
    #[case(PReducing, "P_reducing")]
    #[case(TTriple, "T_triple")]
    #[case(PTriple, "P_triple")]
    #[case(TMin, "T_min")]
    #[case(TMax, "T_max")]
    #[case(PMax, "P_max")]
    #[case(PMin, "P_min")]
    #[case(DipoleMoment, "dipole_moment")]
    #[case(T, "T")]
    #[case(P, "P")]
    #[case(Q, "Q")]
    #[case(Tau, "Tau")]
    #[case(Delta, "Delta")]
    #[case(DMolar, "Dmolar")]
    #[case(HMolar, "Hmolar")]
    #[case(SMolar, "Smolar")]
    #[case(CpMolar, "Cpmolar")]
    #[case(Cp0Molar, "Cp0molar")]
    #[case(CvMolar, "Cvmolar")]
    #[case(UMolar, "Umolar")]
    #[case(GMolar, "Gmolar")]
    #[case(HelmholtzMolar, "Helmholtzmolar")]
    #[case(HMolarResidual, "Hmolar_residual")]
    #[case(SMolarResidual, "Smolar_residual")]
    #[case(GMolarResidual, "Gmolar_residual")]
    #[case(DMass, "Dmass")]
    #[case(HMass, "Hmass")]
    #[case(SMass, "Smass")]
    #[case(CpMass, "Cpmass")]
    #[case(Cp0Mass, "Cp0mass")]
    #[case(CvMass, "Cvmass")]
    #[case(UMass, "Umass")]
    #[case(GMass, "Gmass")]
    #[case(HelmholtzMass, "Helmholtzmass")]
    #[case(DynamicViscosity, "viscosity")]
    #[case(Conductivity, "conductivity")]
    #[case(SurfaceTension, "surface_tension")]
    #[case(Prandtl, "Prandtl")]
    #[case(SoundSpeed, "speed_sound")]
    #[case(IsothermalCompressibility, "isothermal_compressibility")]
    #[case(IsobaricExpansionCoefficient, "isobaric_expansion_coefficient")]
    #[case(IsentropicExpansionCoefficient, "isentropic_expansion_coefficient")]
    #[case(
        FundamentalDerivativeOfGasDynamics,
        "fundamental_derivative_of_gas_dynamics"
    )]
    #[case(AlphaR, "alphar")]
    #[case(DAlphaRDTauConstDelta, "dalphar_dtau_constdelta")]
    #[case(DAlphaRDDeltaConstTau, "dalphar_ddelta_consttau")]
    #[case(Alpha0, "alpha0")]
    #[case(DAlpha0DTauConstDelta, "dalpha0_dtau_constdelta")]
    #[case(DAlpha0DDeltaConstTau, "dalpha0_ddelta_consttau")]
    #[case(D2Alpha0DDelta2ConstTau, "d2alpha0_ddelta2_consttau")]
    #[case(D3Alpha0DDelta3ConstTau, "d3alpha0_ddelta3_consttau")]
    #[case(BVirial, "Bvirial")]
    #[case(CVirial, "Cvirial")]
    #[case(DBVirialDT, "dBvirial_dT")]
    #[case(DCVirialDT, "dCvirial_dT")]
    #[case(Z, "Z")]
    #[case(PIP, "PIP")]
    #[case(MinFraction, "fraction_min")]
    #[case(MaxFraction, "fraction_max")]
    #[case(TFreeze, "T_freeze")]
    #[case(GWP20, "GWP20")]
    #[case(GWP100, "GWP100")]
    #[case(GWP500, "GWP500")]
    #[case(FH, "FH")]
    #[case(HH, "HH")]
    #[case(PH, "PH")]
    #[case(ODP, "ODP")]
    #[case(Phase, "Phase")]
    fn as_ref_returns_expected_str(#[case] param: impl AsRef<str>, #[case] expected: &str) {
        assert_eq!(param.as_ref(), expected);
    }

    #[rstest]
    #[case(vec!["gas_constant"], GasConstant)]
    #[case(vec!["molar_mass", "M", "molarmass", "molemass"], MolarMass)]
    #[case(vec!["acentric_factor", "acentric"], AcentricFactor)]
    #[case(vec!["rhomolar_reducing"], DMolarReducing)]
    #[case(vec!["rhomolar_critical"], DMolarCritical)]
    #[case(vec!["T_reducing"], TReducing)]
    #[case(vec!["T_critical", "Tcrit"], TCritical)]
    #[case(vec!["rhomass_reducing"], DMassReducing)]
    #[case(vec!["rhomass_critical", "rhocrit"], DMassCritical)]
    #[case(vec!["P_critical", "Pcrit"], PCritical)]
    #[case(vec!["P_reducing"], PReducing)]
    #[case(vec!["T_triple", "Ttriple"], TTriple)]
    #[case(vec!["P_triple", "Ptriple"], PTriple)]
    #[case(vec!["T_min", "Tmin"], TMin)]
    #[case(vec!["T_max", "Tmax"], TMax)]
    #[case(vec!["P_max", "Pmax"], PMax)]
    #[case(vec!["P_min", "Pmin"], PMin)]
    #[case(vec!["dipole_moment"], DipoleMoment)]
    #[case(vec!["T"], T)]
    #[case(vec!["P"], P)]
    #[case(vec!["Q"], Q)]
    #[case(vec!["Tau"], Tau)]
    #[case(vec!["Delta"], Delta)]
    #[case(vec!["Dmolar"], DMolar)]
    #[case(vec!["Hmolar"], HMolar)]
    #[case(vec!["Smolar"], SMolar)]
    #[case(vec!["Cpmolar"], CpMolar)]
    #[case(vec!["Cp0molar"], Cp0Molar)]
    #[case(vec!["Cvmolar"], CvMolar)]
    #[case(vec!["Umolar"], UMolar)]
    #[case(vec!["Gmolar"], GMolar)]
    #[case(vec!["Helmholtzmolar"], HelmholtzMolar)]
    #[case(vec!["Hmolar_residual"], HMolarResidual)]
    #[case(vec!["Smolar_residual"], SMolarResidual)]
    #[case(vec!["Gmolar_residual"], GMolarResidual)]
    #[case(vec!["Dmass", "D"], DMass)]
    #[case(vec!["Hmass", "H"], HMass)]
    #[case(vec!["Smass", "S"], SMass)]
    #[case(vec!["Cpmass", "C"], CpMass)]
    #[case(vec!["Cp0mass"], Cp0Mass)]
    #[case(vec!["Cvmass"], CvMass)]
    #[case(vec!["O"], CvMass)]
    #[case(vec!["Umass", "U"], UMass)]
    #[case(vec!["Gmass", "G"], GMass)]
    #[case(vec!["Helmholtzmass"], HelmholtzMass)]
    #[case(vec!["viscosity", "V"], DynamicViscosity)]
    #[case(vec!["conductivity", "L"], Conductivity)]
    #[case(vec!["surface_tension", "I"], SurfaceTension)]
    #[case(vec!["Prandtl"], Prandtl)]
    #[case(vec!["speed_sound", "speed_of_sound", "A"], SoundSpeed)]
    #[case(vec!["isothermal_compressibility"], IsothermalCompressibility)]
    #[case(vec!["isobaric_expansion_coefficient"], IsobaricExpansionCoefficient)]
    #[case(
        vec!["isentropic_expansion_coefficient"],
        IsentropicExpansionCoefficient
    )]
    #[case(
        vec!["fundamental_derivative_of_gas_dynamics"],
        FundamentalDerivativeOfGasDynamics
    )]
    #[case(vec!["alphar"], AlphaR)]
    #[case(vec!["dalphar_dtau_constdelta"], DAlphaRDTauConstDelta)]
    #[case(vec!["dalphar_ddelta_consttau"], DAlphaRDDeltaConstTau)]
    #[case(vec!["alpha0"], Alpha0)]
    #[case(vec!["dalpha0_dtau_constdelta"], DAlpha0DTauConstDelta)]
    #[case(vec!["dalpha0_ddelta_consttau"], DAlpha0DDeltaConstTau)]
    #[case(vec!["d2alpha0_ddelta2_consttau"], D2Alpha0DDelta2ConstTau)]
    #[case(vec!["d3alpha0_ddelta3_consttau"], D3Alpha0DDelta3ConstTau)]
    #[case(vec!["Bvirial"], BVirial)]
    #[case(vec!["Cvirial"], CVirial)]
    #[case(vec!["dBvirial_dT"], DBVirialDT)]
    #[case(vec!["dCvirial_dT"], DCVirialDT)]
    #[case(vec!["Z"], Z)]
    #[case(vec!["PIP"], PIP)]
    #[case(vec!["fraction_min"], MinFraction)]
    #[case(vec!["fraction_max"], MaxFraction)]
    #[case(vec!["T_freeze"], TFreeze)]
    #[case(vec!["GWP20"], GWP20)]
    #[case(vec!["GWP100"], GWP100)]
    #[case(vec!["GWP500"], GWP500)]
    #[case(vec!["FH"], FH)]
    #[case(vec!["HH"], HH)]
    #[case(vec!["PH"], PH)]
    #[case(vec!["ODP"], ODP)]
    #[case(vec!["Phase"], Phase)]
    fn from_valid_str_returns_ok<'a, T>(#[case] valid_values: Vec<&'a str>, #[case] expected: T)
    where
        T: FromStr<Err = strum::ParseError>
            + TryFrom<&'a str, Error = strum::ParseError>
            + Debug
            + Copy
            + Eq
            + PartialEq,
    {
        for s in valid_values {
            assert_eq!(T::from_str(s), Ok(expected));
            assert_eq!(T::try_from(s), Ok(expected));
        }
    }

    #[rstest]
    #[case("")]
    #[case("Hello, World!")]
    fn from_invalid_str_returns_err(#[case] invalid_value: &str) {
        assert!(FluidParam::from_str(invalid_value).is_err());
        assert!(FluidParam::try_from(invalid_value).is_err());
        assert!(FluidTrivialParam::from_str(invalid_value).is_err());
        assert!(FluidTrivialParam::try_from(invalid_value).is_err());
    }

    #[rstest]
    #[case(GasConstant, 1)]
    #[case(MolarMass, 2)]
    #[case(AcentricFactor, 3)]
    #[case(DMolarReducing, 4)]
    #[case(DMolarCritical, 5)]
    #[case(TReducing, 6)]
    #[case(TCritical, 7)]
    #[case(DMassReducing, 8)]
    #[case(DMassCritical, 9)]
    #[case(PCritical, 10)]
    #[case(PReducing, 11)]
    #[case(TTriple, 12)]
    #[case(PTriple, 13)]
    #[case(TMin, 14)]
    #[case(TMax, 15)]
    #[case(PMax, 16)]
    #[case(PMin, 17)]
    #[case(DipoleMoment, 18)]
    #[case(T, 19)]
    #[case(P, 20)]
    #[case(Q, 21)]
    #[case(Tau, 22)]
    #[case(Delta, 23)]
    #[case(DMolar, 24)]
    #[case(HMolar, 25)]
    #[case(SMolar, 26)]
    #[case(CpMolar, 27)]
    #[case(Cp0Molar, 28)]
    #[case(CvMolar, 29)]
    #[case(UMolar, 30)]
    #[case(GMolar, 31)]
    #[case(HelmholtzMolar, 32)]
    #[case(HMolarResidual, 33)]
    #[case(SMolarResidual, 34)]
    #[case(GMolarResidual, 35)]
    #[case(DMass, 36)]
    #[case(HMass, 37)]
    #[case(SMass, 38)]
    #[case(CpMass, 39)]
    #[case(Cp0Mass, 40)]
    #[case(CvMass, 41)]
    #[case(UMass, 42)]
    #[case(GMass, 43)]
    #[case(HelmholtzMass, 44)]
    #[case(DynamicViscosity, 45)]
    #[case(Conductivity, 46)]
    #[case(SurfaceTension, 47)]
    #[case(Prandtl, 48)]
    #[case(SoundSpeed, 49)]
    #[case(IsothermalCompressibility, 50)]
    #[case(IsobaricExpansionCoefficient, 51)]
    #[case(IsentropicExpansionCoefficient, 52)]
    #[case(FundamentalDerivativeOfGasDynamics, 53)]
    #[case(AlphaR, 54)]
    #[case(DAlphaRDTauConstDelta, 55)]
    #[case(DAlphaRDDeltaConstTau, 56)]
    #[case(Alpha0, 57)]
    #[case(DAlpha0DTauConstDelta, 58)]
    #[case(DAlpha0DDeltaConstTau, 59)]
    #[case(D2Alpha0DDelta2ConstTau, 60)]
    #[case(D3Alpha0DDelta3ConstTau, 61)]
    #[case(BVirial, 62)]
    #[case(CVirial, 63)]
    #[case(DBVirialDT, 64)]
    #[case(DCVirialDT, 65)]
    #[case(Z, 66)]
    #[case(PIP, 67)]
    #[case(MinFraction, 68)]
    #[case(MaxFraction, 69)]
    #[case(TFreeze, 70)]
    #[case(GWP20, 71)]
    #[case(GWP100, 72)]
    #[case(GWP500, 73)]
    #[case(FH, 74)]
    #[case(HH, 75)]
    #[case(PH, 76)]
    #[case(ODP, 77)]
    #[case(Phase, 78)]
    fn u8_from_param_returns_expected_value(#[case] param: impl Into<u8>, #[case] expected: u8) {
        assert_eq!(param.into(), expected);
    }

    #[rstest]
    #[case(1, GasConstant)]
    #[case(2, MolarMass)]
    #[case(3, AcentricFactor)]
    #[case(4, DMolarReducing)]
    #[case(5, DMolarCritical)]
    #[case(6, TReducing)]
    #[case(7, TCritical)]
    #[case(8, DMassReducing)]
    #[case(9, DMassCritical)]
    #[case(10, PCritical)]
    #[case(11, PReducing)]
    #[case(12, TTriple)]
    #[case(13, PTriple)]
    #[case(14, TMin)]
    #[case(15, TMax)]
    #[case(16, PMax)]
    #[case(17, PMin)]
    #[case(18, DipoleMoment)]
    #[case(19, T)]
    #[case(20, P)]
    #[case(21, Q)]
    #[case(22, Tau)]
    #[case(23, Delta)]
    #[case(24, DMolar)]
    #[case(25, HMolar)]
    #[case(26, SMolar)]
    #[case(27, CpMolar)]
    #[case(28, Cp0Molar)]
    #[case(29, CvMolar)]
    #[case(30, UMolar)]
    #[case(31, GMolar)]
    #[case(32, HelmholtzMolar)]
    #[case(33, HMolarResidual)]
    #[case(34, SMolarResidual)]
    #[case(35, GMolarResidual)]
    #[case(36, DMass)]
    #[case(37, HMass)]
    #[case(38, SMass)]
    #[case(39, CpMass)]
    #[case(40, Cp0Mass)]
    #[case(41, CvMass)]
    #[case(42, UMass)]
    #[case(43, GMass)]
    #[case(44, HelmholtzMass)]
    #[case(45, DynamicViscosity)]
    #[case(46, Conductivity)]
    #[case(47, SurfaceTension)]
    #[case(48, Prandtl)]
    #[case(49, SoundSpeed)]
    #[case(50, IsothermalCompressibility)]
    #[case(51, IsobaricExpansionCoefficient)]
    #[case(52, IsentropicExpansionCoefficient)]
    #[case(53, FundamentalDerivativeOfGasDynamics)]
    #[case(54, AlphaR)]
    #[case(55, DAlphaRDTauConstDelta)]
    #[case(56, DAlphaRDDeltaConstTau)]
    #[case(57, Alpha0)]
    #[case(58, DAlpha0DTauConstDelta)]
    #[case(59, DAlpha0DDeltaConstTau)]
    #[case(60, D2Alpha0DDelta2ConstTau)]
    #[case(61, D3Alpha0DDelta3ConstTau)]
    #[case(62, BVirial)]
    #[case(63, CVirial)]
    #[case(64, DBVirialDT)]
    #[case(65, DCVirialDT)]
    #[case(66, Z)]
    #[case(67, PIP)]
    #[case(68, MinFraction)]
    #[case(69, MaxFraction)]
    #[case(70, TFreeze)]
    #[case(71, GWP20)]
    #[case(72, GWP100)]
    #[case(73, GWP500)]
    #[case(74, FH)]
    #[case(75, HH)]
    #[case(76, PH)]
    #[case(77, ODP)]
    #[case(78, Phase)]
    fn try_from_valid_u8_or_f64_returns_ok<T>(#[case] valid_value: u8, #[case] expected: T)
    where
        T: TryFrom<u8, Error = strum::ParseError>
            + TryFrom<f64, Error = strum::ParseError>
            + Debug
            + Copy
            + Eq
            + PartialEq,
    {
        assert_eq!(T::try_from(valid_value), Ok(expected));
        assert_eq!(T::try_from(valid_value as f64), Ok(expected));
    }

    #[rstest]
    #[case(254)]
    #[case(255)]
    fn try_from_invalid_u8_returns_err(#[case] invalid_value: u8) {
        assert!(FluidParam::try_from(invalid_value).is_err());
        assert!(FluidTrivialParam::try_from(invalid_value).is_err());
    }

    #[rstest]
    #[case(-1.0)]
    #[case(255.0)]
    #[case(100e3)]
    fn try_from_invalid_f64_returns_err(#[case] invalid_value: f64) {
        assert!(FluidParam::try_from(invalid_value).is_err());
        assert!(FluidTrivialParam::try_from(invalid_value).is_err());
    }
}
