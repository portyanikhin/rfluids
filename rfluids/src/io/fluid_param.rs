// cSpell:disable

use super::try_from;

/// `CoolProp` fluids input/output parameters.
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
/// assert_eq!(FluidParam::Conductivity.as_ref(), "conductivity");
/// assert_eq!(FluidParam::from_str("conductivity"), Ok(FluidParam::Conductivity));
/// assert_eq!(FluidParam::try_from("L"), Ok(FluidParam::Conductivity));
/// ```
///
/// Conversion between [`u8`]:
///
/// ```
/// use rfluids::prelude::*;
///
/// assert_eq!(u8::from(FluidParam::Conductivity), 52);
/// assert_eq!(FluidParam::try_from(52), Ok(FluidParam::Conductivity));
/// ```
///
/// Conversion between [`f64`]:
///
/// ```
/// use rfluids::prelude::*;
///
/// assert_eq!(FluidParam::try_from(52.0), Ok(FluidParam::Conductivity));
/// ```
///
/// Conversion between [`FluidInputPair`](crate::io::FluidInputPair):
///
/// ```
/// use rfluids::prelude::*;
///
/// assert_eq!(
///     <(FluidParam, FluidParam)>::from(FluidInputPair::PT),
///     (FluidParam::P, FluidParam::T)
/// );
/// assert_eq!(FluidInputPair::try_from((FluidParam::T, FluidParam::P)), Ok(FluidInputPair::PT));
/// ```
///
/// # See Also
///
/// - [CoolProp fluids input/output parameters _(only those for which the value in the "Trivial" column is "False")_](https://coolprop.github.io/CoolProp/coolprop/HighLevelAPI.html#parameter-table)
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    PartialEq,
    strum_macros::AsRefStr,
    strum_macros::EnumString,
    strum_macros::FromRepr,
    strum_macros::IntoStaticStr,
)]
#[strum(ascii_case_insensitive)]
#[repr(u8)]
pub enum FluidParam {
    /// Temperature **\[K\]**.
    #[strum(to_string = "T")]
    T = 19,

    /// Pressure **\[Pa\]**.
    #[strum(to_string = "P")]
    P = 20,

    /// Vapor quality **\[dimensionless, from 0 to 1\]**.
    #[strum(to_string = "Q")]
    Q = 21,

    /// Reciprocal reduced temperature = [`TCritical`](FluidTrivialParam::TCritical)
    /// [`T`](FluidParam::T) **\[dimensionless\]**.
    #[strum(to_string = "Tau")]
    Tau = 22,

    /// Reduced density = [`DMass`](FluidParam::DMass)/
    /// [`DMassCritical`](FluidTrivialParam::DMassCritical) **\[dimensionless\]**.
    #[strum(to_string = "Delta")]
    Delta = 23,

    /// Molar density **\[mol/m³\]**.
    #[strum(to_string = "Dmolar")]
    DMolar = 24,

    /// Molar specific enthalpy **\[J/mol\]**.
    #[strum(to_string = "Hmolar")]
    HMolar = 25,

    /// Molar specific entropy **\[J/mol/K\]**.
    #[strum(to_string = "Smolar")]
    SMolar = 26,

    /// Molar specific heat at constant pressure **\[J/mol/K\]**.
    #[strum(to_string = "Cpmolar")]
    CpMolar = 27,

    /// Ideal gas molar specific heat at constant pressure **\[J/mol/K\]**.
    #[strum(to_string = "Cp0molar")]
    Cp0Molar = 28,

    /// Molar specific heat at constant volume **\[J/mol/K\]**.
    #[strum(to_string = "Cvmolar")]
    CvMolar = 29,

    /// Molar specific internal energy **\[J/mol\]**.
    #[strum(to_string = "Umolar")]
    UMolar = 30,

    /// Molar specific Gibbs energy **\[J/mol\]**.
    #[strum(to_string = "Gmolar")]
    GMolar = 31,

    /// Molar specific Helmholtz energy **\[J/mol\]**.
    #[strum(to_string = "Helmholtzmolar")]
    HelmholtzMolar = 32,

    /// Residual molar specific enthalpy **\[J/mol\]**.
    #[strum(to_string = "Hmolar_residual")]
    HMolarResidual = 33,

    /// Residual molar specific entropy **\[J/mol/K\]**.
    #[strum(to_string = "Smolar_residual")]
    SMolarResidual = 34,

    /// Residual molar specific Gibbs energy **\[J/mol\]**.
    #[strum(to_string = "Gmolar_residual")]
    GMolarResidual = 35,

    /// Ideal gas molar specific enthalpy **\[J/mol\]**.
    #[strum(to_string = "Hmolar_idealgas")]
    HMolarIdealGas = 36,

    /// Ideal gas molar specific entropy **\[J/mol/K\]**.
    #[strum(to_string = "Smolar_idealgas")]
    SMolarIdealGas = 37,

    /// Ideal gas molar specific internal energy **\[J/mol\]**.
    #[strum(to_string = "Umolar_idealgas")]
    UMolarIdealGas = 38,

    /// Mass density **\[kg/m³\]**.
    #[strum(to_string = "Dmass", serialize = "D")]
    DMass = 39,

    /// Mass specific enthalpy **\[J/kg\]**.
    #[strum(to_string = "Hmass", serialize = "H")]
    HMass = 40,

    /// Mass specific entropy **\[J/kg/K\]**.
    #[strum(to_string = "Smass", serialize = "S")]
    SMass = 41,

    /// Mass specific heat at constant pressure **\[J/kg/K\]**.
    #[strum(to_string = "Cpmass", serialize = "C")]
    CpMass = 42,

    /// Ideal gas mass specific heat at constant pressure **\[J/kg/K\]**.
    #[strum(to_string = "Cp0mass")]
    Cp0Mass = 43,

    /// Mass specific heat at constant volume **\[J/kg/K\]**.
    #[strum(to_string = "Cvmass", serialize = "O")]
    CvMass = 44,

    /// Mass specific internal energy **\[J/kg\]**.
    #[strum(to_string = "Umass", serialize = "U")]
    UMass = 45,

    /// Mass specific Gibbs energy **\[J/kg\]**.
    #[strum(to_string = "Gmass", serialize = "G")]
    GMass = 46,

    /// Mass specific Helmholtz energy **\[J/kg\]**.
    #[strum(to_string = "Helmholtzmass")]
    HelmholtzMass = 47,

    /// Ideal gas mass specific enthalpy **\[J/kg\]**.
    #[strum(to_string = "Hmass_idealgas")]
    HMassIdealGas = 48,

    /// Ideal gas mass specific entropy **\[J/kg/K\]**.
    #[strum(to_string = "Smass_idealgas")]
    SMassIdealGas = 49,

    /// Ideal gas mass specific internal energy **\[J/kg\]**.
    #[strum(to_string = "Umass_idealgas")]
    UMassIdealGas = 50,

    /// Dynamic viscosity **\[Pa·s\]**.
    #[strum(to_string = "viscosity", serialize = "V")]
    DynamicViscosity = 51,

    /// Thermal conductivity **\[W/m/K\]**.
    #[strum(to_string = "conductivity", serialize = "L")]
    Conductivity = 52,

    /// Surface tension **\[N/m\]**.
    #[strum(to_string = "surface_tension", serialize = "I")]
    SurfaceTension = 53,

    /// Prandtl number **\[dimensionless\]**.
    #[strum(to_string = "Prandtl")]
    Prandtl = 54,

    /// Sound speed **\[m/s\]**.
    #[strum(to_string = "speed_sound", serialize = "speed_of_sound", serialize = "A")]
    SoundSpeed = 55,

    /// Isothermal compressibility **\[1/Pa\]**.
    #[strum(to_string = "isothermal_compressibility")]
    IsothermalCompressibility = 56,

    /// Isobaric expansion coefficient **\[1/K\]**.
    #[strum(to_string = "isobaric_expansion_coefficient")]
    IsobaricExpansionCoefficient = 57,

    /// Isentropic expansion coefficient **\[dimensionless\]**.
    #[strum(to_string = "isentropic_expansion_coefficient")]
    IsentropicExpansionCoefficient = 58,

    /// Fundamental derivative of gas dynamics **\[dimensionless\]**.
    #[strum(to_string = "fundamental_derivative_of_gas_dynamics")]
    FundamentalDerivativeOfGasDynamics = 59,

    /// Residual Helmholtz energy contribution **\[dimensionless\]**.
    #[strum(to_string = "alphar")]
    AlphaR = 60,

    /// Derivative of residual Helmholtz energy contribution
    /// with [`Tau`](FluidParam::Tau) **\[dimensionless\]**.
    #[strum(to_string = "dalphar_dtau_constdelta")]
    DAlphaRDTauConstDelta = 61,

    /// Derivative of residual Helmholtz energy contribution
    /// with [`Delta`](FluidParam::Delta) **\[dimensionless\]**.
    #[strum(to_string = "dalphar_ddelta_consttau")]
    DAlphaRDDeltaConstTau = 62,

    /// Ideal gas Helmholtz energy contribution **\[dimensionless\]**.
    #[strum(to_string = "alpha0")]
    Alpha0 = 63,

    /// Derivative of ideal gas Helmholtz energy contribution
    /// with [`Tau`](FluidParam::Tau) **\[dimensionless\]**.
    #[strum(to_string = "dalpha0_dtau_constdelta")]
    DAlpha0DTauConstDelta = 64,

    /// Derivative of ideal gas Helmholtz energy contribution
    /// with [`Delta`](FluidParam::Delta) **\[dimensionless\]**.
    #[strum(to_string = "dalpha0_ddelta_consttau")]
    DAlpha0DDeltaConstTau = 65,

    /// Second derivative of ideal gas Helmholtz energy contribution
    /// with [`Delta`](FluidParam::Delta) **\[dimensionless\]**.
    #[strum(to_string = "d2alpha0_ddelta2_consttau")]
    D2Alpha0DDelta2ConstTau = 66,

    /// Third derivative of ideal gas Helmholtz energy contribution
    /// with [`Delta`](FluidParam::Delta) **\[dimensionless\]**.
    #[strum(to_string = "d3alpha0_ddelta3_consttau")]
    D3Alpha0DDelta3ConstTau = 67,

    /// Second virial coefficient **\[dimensionless\]**.
    #[strum(to_string = "Bvirial")]
    BVirial = 68,

    /// Third virial coefficient **\[dimensionless\]**.
    #[strum(to_string = "Cvirial")]
    CVirial = 69,

    /// Derivative of second virial coefficient with [`T`](FluidParam::T) **\[dimensionless\]**.
    #[strum(to_string = "dBvirial_dT")]
    DBVirialDT = 70,

    /// Derivative of third virial coefficient with [`T`](FluidParam::T) **\[dimensionless\]**.
    #[strum(to_string = "dCvirial_dT")]
    DCVirialDT = 71,

    /// Compressibility factor **\[dimensionless\]**.
    #[strum(to_string = "Z")]
    Z = 72,

    /// Phase identification parameter **\[dimensionless\]**.
    #[strum(to_string = "PIP")]
    PIP = 73,

    /// Phase index **\[dimensionless\]**.
    #[strum(to_string = "Phase")]
    Phase = 84,
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

/// `CoolProp` fluids trivial output parameters.
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
/// assert_eq!(FluidTrivialParam::TMin.as_ref(), "T_min");
/// assert_eq!(FluidTrivialParam::from_str("T_min"), Ok(FluidTrivialParam::TMin));
/// assert_eq!(FluidTrivialParam::try_from("T_min"), Ok(FluidTrivialParam::TMin));
/// ```
///
/// Conversion between [`u8`]:
///
/// ```
/// use rfluids::prelude::*;
///
/// assert_eq!(u8::from(FluidTrivialParam::TMax), 15);
/// assert_eq!(FluidTrivialParam::try_from(15), Ok(FluidTrivialParam::TMax));
/// ```
///
/// Conversion between [`f64`]:
///
/// ```
/// use rfluids::prelude::*;
///
/// assert_eq!(FluidTrivialParam::try_from(15.0), Ok(FluidTrivialParam::TMax));
/// ```
///
/// # See Also
///
/// - [CoolProp fluids input/output parameters _(only those for which the value in the "Trivial" column is "True")_](https://coolprop.github.io/CoolProp/coolprop/HighLevelAPI.html#parameter-table)
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    PartialEq,
    strum_macros::AsRefStr,
    strum_macros::EnumString,
    strum_macros::FromRepr,
    strum_macros::IntoStaticStr,
)]
#[strum(ascii_case_insensitive)]
#[repr(u8)]
pub enum FluidTrivialParam {
    /// Molar gas constant **\[J/mol/K\]**.
    #[strum(to_string = "gas_constant")]
    GasConstant = 1,

    /// Molar mass **\[kg/mol\]**.
    #[strum(
        to_string = "molar_mass",
        serialize = "M",
        serialize = "molarmass",
        serialize = "molemass"
    )]
    MolarMass = 2,

    /// Acentric factor **\[dimensionless\]**.
    #[strum(to_string = "acentric_factor", serialize = "acentric")]
    AcentricFactor = 3,

    /// Reducing point molar density **\[mol/m³\]**.
    #[strum(to_string = "rhomolar_reducing")]
    DMolarReducing = 4,

    /// Critical point molar density **\[mol/m³\]**.
    #[strum(to_string = "rhomolar_critical")]
    DMolarCritical = 5,

    /// Reducing point temperature **\[K\]**.
    #[strum(to_string = "T_reducing")]
    TReducing = 6,

    /// Critical point temperature **\[K\]**.
    #[strum(to_string = "T_critical", serialize = "Tcrit")]
    TCritical = 7,

    /// Reducing point mass density **\[kg/m³\]**.
    #[strum(to_string = "rhomass_reducing")]
    DMassReducing = 8,

    /// Critical point mass density **\[kg/m³\]**.
    #[strum(to_string = "rhomass_critical", serialize = "rhocrit")]
    DMassCritical = 9,

    /// Critical point pressure **\[Pa\]**.
    #[strum(to_string = "P_critical", serialize = "Pcrit")]
    PCritical = 10,

    /// Reducing point pressure **\[Pa\]**.
    #[strum(to_string = "P_reducing")]
    PReducing = 11,

    /// Triple point temperature **\[K\]**.
    #[strum(to_string = "T_triple", serialize = "Ttriple")]
    TTriple = 12,

    /// Triple point pressure **\[Pa\]**.
    #[strum(to_string = "P_triple", serialize = "Ptriple")]
    PTriple = 13,

    /// Minimum temperature **\[K\]**.
    #[strum(to_string = "T_min", serialize = "Tmin")]
    TMin = 14,

    /// Maximum temperature **\[K\]**.
    #[strum(to_string = "T_max", serialize = "Tmax")]
    TMax = 15,

    /// Maximum pressure **\[Pa\]**.
    #[strum(to_string = "P_max", serialize = "Pmax")]
    PMax = 16,

    /// Minimum pressure **\[Pa\]**.
    #[strum(to_string = "P_min", serialize = "Pmin")]
    PMin = 17,

    /// Dipole moment **\[C*m\]**.
    #[strum(to_string = "dipole_moment")]
    DipoleMoment = 18,

    /// Minimum fraction _(mole, mass or volume)_ value for incompressible mixtures
    /// **\[dimensionless, from 0 to 1\]**.
    #[strum(to_string = "fraction_min")]
    MinFraction = 74,

    /// Maximum fraction _(mole, mass or volume)_ value for incompressible mixtures
    /// **\[dimensionless, from 0 to 1\]**.
    #[strum(to_string = "fraction_max")]
    MaxFraction = 75,

    /// Freezing temperature for incompressible mixtures **\[K\]**.
    #[strum(to_string = "T_freeze")]
    TFreeze = 76,

    /// 20-year global warming potential **\[dimensionless\]**.
    #[strum(to_string = "GWP20")]
    GWP20 = 77,

    /// 100-year global warming potential **\[dimensionless\]**.
    #[strum(to_string = "GWP100")]
    GWP100 = 78,

    /// 500-year global warming potential **\[dimensionless\]**.
    #[strum(to_string = "GWP500")]
    GWP500 = 79,

    /// Flammability hazard index **\[dimensionless\]**.
    #[strum(to_string = "FH")]
    FH = 80,

    /// Health hazard index **\[dimensionless\]**.
    #[strum(to_string = "HH")]
    HH = 81,

    /// Physical hazard index **\[dimensionless\]**.
    #[strum(to_string = "PH")]
    PH = 82,

    /// Ozone depletion potential **\[dimensionless\]**.
    #[strum(to_string = "ODP")]
    ODP = 83,
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
    use std::{fmt::Debug, str::FromStr};

    use rstest::*;

    use super::{FluidParam::*, FluidTrivialParam::*, *};

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
    #[case(HMolarIdealGas, "Hmolar_idealgas")]
    #[case(SMolarIdealGas, "Smolar_idealgas")]
    #[case(UMolarIdealGas, "Umolar_idealgas")]
    #[case(DMass, "Dmass")]
    #[case(HMass, "Hmass")]
    #[case(SMass, "Smass")]
    #[case(CpMass, "Cpmass")]
    #[case(Cp0Mass, "Cp0mass")]
    #[case(CvMass, "Cvmass")]
    #[case(UMass, "Umass")]
    #[case(GMass, "Gmass")]
    #[case(HelmholtzMass, "Helmholtzmass")]
    #[case(HMassIdealGas, "Hmass_idealgas")]
    #[case(SMassIdealGas, "Smass_idealgas")]
    #[case(UMassIdealGas, "Umass_idealgas")]
    #[case(DynamicViscosity, "viscosity")]
    #[case(Conductivity, "conductivity")]
    #[case(SurfaceTension, "surface_tension")]
    #[case(Prandtl, "Prandtl")]
    #[case(SoundSpeed, "speed_sound")]
    #[case(IsothermalCompressibility, "isothermal_compressibility")]
    #[case(IsobaricExpansionCoefficient, "isobaric_expansion_coefficient")]
    #[case(IsentropicExpansionCoefficient, "isentropic_expansion_coefficient")]
    #[case(FundamentalDerivativeOfGasDynamics, "fundamental_derivative_of_gas_dynamics")]
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
    fn as_str(#[case] sut: impl AsRef<str> + Into<&'static str> + Copy, #[case] expected: &str) {
        // When
        let str = sut.as_ref();
        let static_str: &'static str = sut.into();

        // Then
        assert_eq!(str, expected);
        assert_eq!(static_str, expected);
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
    #[case(vec!["Hmolar_idealgas"], HMolarIdealGas)]
    #[case(vec!["Smolar_idealgas"], SMolarIdealGas)]
    #[case(vec!["Umolar_idealgas"], UMolarIdealGas)]
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
    #[case(vec!["Hmass_idealgas"], HMassIdealGas)]
    #[case(vec!["Smass_idealgas"], SMassIdealGas)]
    #[case(vec!["Umass_idealgas"], UMassIdealGas)]
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
    fn from_valid_str<'a, T>(#[case] valid: Vec<&'a str>, #[case] expected: T)
    where
        T: FromStr<Err = strum::ParseError>
            + TryFrom<&'a str, Error = strum::ParseError>
            + Debug
            + Copy
            + Eq
            + PartialEq,
    {
        for s in valid {
            // When
            let res1 = T::from_str(s).unwrap();
            let res2 = T::try_from(s).unwrap();

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
        let res1 = FluidParam::from_str(invalid);
        let res2 = FluidParam::try_from(invalid);
        let res3 = FluidTrivialParam::from_str(invalid);
        let res4 = FluidTrivialParam::try_from(invalid);

        // Then
        assert!(res1.is_err());
        assert!(res2.is_err());
        assert!(res3.is_err());
        assert!(res4.is_err());
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
    #[case(HMolarIdealGas, 36)]
    #[case(SMolarIdealGas, 37)]
    #[case(UMolarIdealGas, 38)]
    #[case(DMass, 39)]
    #[case(HMass, 40)]
    #[case(SMass, 41)]
    #[case(CpMass, 42)]
    #[case(Cp0Mass, 43)]
    #[case(CvMass, 44)]
    #[case(UMass, 45)]
    #[case(GMass, 46)]
    #[case(HelmholtzMass, 47)]
    #[case(HMassIdealGas, 48)]
    #[case(SMassIdealGas, 49)]
    #[case(UMassIdealGas, 50)]
    #[case(DynamicViscosity, 51)]
    #[case(Conductivity, 52)]
    #[case(SurfaceTension, 53)]
    #[case(Prandtl, 54)]
    #[case(SoundSpeed, 55)]
    #[case(IsothermalCompressibility, 56)]
    #[case(IsobaricExpansionCoefficient, 57)]
    #[case(IsentropicExpansionCoefficient, 58)]
    #[case(FundamentalDerivativeOfGasDynamics, 59)]
    #[case(AlphaR, 60)]
    #[case(DAlphaRDTauConstDelta, 61)]
    #[case(DAlphaRDDeltaConstTau, 62)]
    #[case(Alpha0, 63)]
    #[case(DAlpha0DTauConstDelta, 64)]
    #[case(DAlpha0DDeltaConstTau, 65)]
    #[case(D2Alpha0DDelta2ConstTau, 66)]
    #[case(D3Alpha0DDelta3ConstTau, 67)]
    #[case(BVirial, 68)]
    #[case(CVirial, 69)]
    #[case(DBVirialDT, 70)]
    #[case(DCVirialDT, 71)]
    #[case(Z, 72)]
    #[case(PIP, 73)]
    #[case(MinFraction, 74)]
    #[case(MaxFraction, 75)]
    #[case(TFreeze, 76)]
    #[case(GWP20, 77)]
    #[case(GWP100, 78)]
    #[case(GWP500, 79)]
    #[case(FH, 80)]
    #[case(HH, 81)]
    #[case(PH, 82)]
    #[case(ODP, 83)]
    #[case(Phase, 84)]
    fn into_u8(#[case] sut: impl Into<u8>, #[case] expected: u8) {
        // When
        let res = sut.into();

        // Then
        assert_eq!(res, expected);
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
    #[case(36, HMolarIdealGas)]
    #[case(37, SMolarIdealGas)]
    #[case(38, UMolarIdealGas)]
    #[case(39, DMass)]
    #[case(40, HMass)]
    #[case(41, SMass)]
    #[case(42, CpMass)]
    #[case(43, Cp0Mass)]
    #[case(44, CvMass)]
    #[case(45, UMass)]
    #[case(46, GMass)]
    #[case(47, HelmholtzMass)]
    #[case(48, HMassIdealGas)]
    #[case(49, SMassIdealGas)]
    #[case(50, UMassIdealGas)]
    #[case(51, DynamicViscosity)]
    #[case(52, Conductivity)]
    #[case(53, SurfaceTension)]
    #[case(54, Prandtl)]
    #[case(55, SoundSpeed)]
    #[case(56, IsothermalCompressibility)]
    #[case(57, IsobaricExpansionCoefficient)]
    #[case(58, IsentropicExpansionCoefficient)]
    #[case(59, FundamentalDerivativeOfGasDynamics)]
    #[case(60, AlphaR)]
    #[case(61, DAlphaRDTauConstDelta)]
    #[case(62, DAlphaRDDeltaConstTau)]
    #[case(63, Alpha0)]
    #[case(64, DAlpha0DTauConstDelta)]
    #[case(65, DAlpha0DDeltaConstTau)]
    #[case(66, D2Alpha0DDelta2ConstTau)]
    #[case(67, D3Alpha0DDelta3ConstTau)]
    #[case(68, BVirial)]
    #[case(69, CVirial)]
    #[case(70, DBVirialDT)]
    #[case(71, DCVirialDT)]
    #[case(72, Z)]
    #[case(73, PIP)]
    #[case(74, MinFraction)]
    #[case(75, MaxFraction)]
    #[case(76, TFreeze)]
    #[case(77, GWP20)]
    #[case(78, GWP100)]
    #[case(79, GWP500)]
    #[case(80, FH)]
    #[case(81, HH)]
    #[case(82, PH)]
    #[case(83, ODP)]
    #[case(84, Phase)]
    fn try_from_valid_u8_or_f64<T>(#[case] valid: u8, #[case] expected: T)
    where
        T: TryFrom<u8, Error = strum::ParseError>
            + TryFrom<f64, Error = strum::ParseError>
            + Debug
            + Copy
            + Eq
            + PartialEq,
    {
        // When
        let res1 = T::try_from(valid).unwrap();
        let res2 = T::try_from(f64::from(valid)).unwrap();

        // Then
        assert_eq!(res1, expected);
        assert_eq!(res2, expected);
    }

    #[rstest]
    #[case(254)]
    #[case(255)]
    fn try_from_invalid_u8(#[case] invalid: u8) {
        // When
        let res1 = FluidParam::try_from(invalid);
        let res2 = FluidTrivialParam::try_from(invalid);

        // Then
        assert!(res1.is_err());
        assert!(res2.is_err());
    }

    #[rstest]
    #[case(-1.0)]
    #[case(255.0)]
    #[case(100e3)]
    fn try_from_invalid_f64(#[case] invalid: f64) {
        // When
        let res1 = FluidParam::try_from(invalid);
        let res2 = FluidTrivialParam::try_from(invalid);

        // Then
        assert!(res1.is_err());
        assert!(res2.is_err());
    }
}
