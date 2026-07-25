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
/// assert_eq!(u8::from(FluidParam::SMass), 42);
/// assert_eq!(FluidParam::try_from(42), Ok(FluidParam::SMass));
/// ```
///
/// Conversion between [`f64`]:
///
/// ```
/// use rfluids::prelude::*;
///
/// assert_eq!(FluidParam::try_from(42.0), Ok(FluidParam::SMass));
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
/// - [CoolProp Fluids Input/Output Parameters](https://coolprop.org/coolprop/HighLevelAPI.html#parameter-table)
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

    /// Mole-based vapor quality **\[dimensionless, from 0 to 1\]**.
    #[strum(to_string = "Q")]
    Q = 21,

    /// Mass-based vapor quality **\[dimensionless, from 0 to 1\]**.
    #[strum(to_string = "Qmass")]
    QMass = 22,

    /// Reciprocal reduced temperature = [`TCritical`](FluidTrivialParam::TCritical)
    /// [`T`](FluidParam::T) **\[dimensionless\]**.
    #[strum(to_string = "Tau")]
    Tau = 23,

    /// Reduced density = [`DMass`](FluidParam::DMass)/
    /// [`DMassCritical`](FluidTrivialParam::DMassCritical) **\[dimensionless\]**.
    #[strum(to_string = "Delta")]
    Delta = 24,

    /// Molar density **\[mol/m³\]**.
    #[strum(to_string = "Dmolar")]
    DMolar = 25,

    /// Molar specific enthalpy **\[J/mol\]**.
    #[strum(to_string = "Hmolar")]
    HMolar = 26,

    /// Molar specific entropy **\[J/mol/K\]**.
    #[strum(to_string = "Smolar")]
    SMolar = 27,

    /// Molar specific heat at constant pressure **\[J/mol/K\]**.
    #[strum(to_string = "Cpmolar")]
    CpMolar = 28,

    /// Ideal gas molar specific heat at constant pressure **\[J/mol/K\]**.
    #[strum(to_string = "Cp0molar")]
    Cp0Molar = 29,

    /// Molar specific heat at constant volume **\[J/mol/K\]**.
    #[strum(to_string = "Cvmolar")]
    CvMolar = 30,

    /// Molar specific internal energy **\[J/mol\]**.
    #[strum(to_string = "Umolar")]
    UMolar = 31,

    /// Molar specific Gibbs energy **\[J/mol\]**.
    #[strum(to_string = "Gmolar")]
    GMolar = 32,

    /// Molar specific Helmholtz energy **\[J/mol\]**.
    #[strum(to_string = "Helmholtzmolar")]
    HelmholtzMolar = 33,

    /// Residual molar specific enthalpy **\[J/mol\]**.
    #[strum(to_string = "Hmolar_residual")]
    HMolarResidual = 34,

    /// Residual molar specific entropy **\[J/mol/K\]**.
    #[strum(to_string = "Smolar_residual")]
    SMolarResidual = 35,

    /// Residual molar specific Gibbs energy **\[J/mol\]**.
    #[strum(to_string = "Gmolar_residual")]
    GMolarResidual = 36,

    /// Ideal gas molar specific enthalpy **\[J/mol\]**.
    #[strum(to_string = "Hmolar_idealgas")]
    HMolarIdealGas = 37,

    /// Ideal gas molar specific entropy **\[J/mol/K\]**.
    #[strum(to_string = "Smolar_idealgas")]
    SMolarIdealGas = 38,

    /// Ideal gas molar specific internal energy **\[J/mol\]**.
    #[strum(to_string = "Umolar_idealgas")]
    UMolarIdealGas = 39,

    /// Mass density **\[kg/m³\]**.
    #[strum(to_string = "Dmass", serialize = "D")]
    DMass = 40,

    /// Mass specific enthalpy **\[J/kg\]**.
    #[strum(to_string = "Hmass", serialize = "H")]
    HMass = 41,

    /// Mass specific entropy **\[J/kg/K\]**.
    #[strum(to_string = "Smass", serialize = "S")]
    SMass = 42,

    /// Mass specific heat at constant pressure **\[J/kg/K\]**.
    #[strum(to_string = "Cpmass", serialize = "C")]
    CpMass = 43,

    /// Ideal gas mass specific heat at constant pressure **\[J/kg/K\]**.
    #[strum(to_string = "Cp0mass")]
    Cp0Mass = 44,

    /// Mass specific heat at constant volume **\[J/kg/K\]**.
    #[strum(to_string = "Cvmass", serialize = "O")]
    CvMass = 45,

    /// Mass specific internal energy **\[J/kg\]**.
    #[strum(to_string = "Umass", serialize = "U")]
    UMass = 46,

    /// Mass specific Gibbs energy **\[J/kg\]**.
    #[strum(to_string = "Gmass", serialize = "G")]
    GMass = 47,

    /// Mass specific Helmholtz energy **\[J/kg\]**.
    #[strum(to_string = "Helmholtzmass")]
    HelmholtzMass = 48,

    /// Ideal gas mass specific enthalpy **\[J/kg\]**.
    #[strum(to_string = "Hmass_idealgas")]
    HMassIdealGas = 49,

    /// Ideal gas mass specific entropy **\[J/kg/K\]**.
    #[strum(to_string = "Smass_idealgas")]
    SMassIdealGas = 50,

    /// Ideal gas mass specific internal energy **\[J/kg\]**.
    #[strum(to_string = "Umass_idealgas")]
    UMassIdealGas = 51,

    /// Dynamic viscosity **\[Pa·s\]**.
    #[strum(to_string = "viscosity", serialize = "V")]
    DynamicViscosity = 52,

    /// Thermal conductivity **\[W/m/K\]**.
    #[strum(to_string = "conductivity", serialize = "L")]
    Conductivity = 53,

    /// Surface tension **\[N/m\]**.
    #[strum(to_string = "surface_tension", serialize = "I")]
    SurfaceTension = 54,

    /// Prandtl number **\[dimensionless\]**.
    #[strum(to_string = "Prandtl")]
    Prandtl = 55,

    /// Sound speed **\[m/s\]**.
    #[strum(to_string = "speed_sound", serialize = "speed_of_sound", serialize = "A")]
    SoundSpeed = 56,

    /// Isothermal compressibility **\[1/Pa\]**.
    #[strum(to_string = "isothermal_compressibility")]
    IsothermalCompressibility = 57,

    /// Isobaric expansion coefficient **\[1/K\]**.
    #[strum(to_string = "isobaric_expansion_coefficient")]
    IsobaricExpansionCoefficient = 58,

    /// Isentropic expansion coefficient **\[dimensionless\]**.
    #[strum(to_string = "isentropic_expansion_coefficient")]
    IsentropicExpansionCoefficient = 59,

    /// Fundamental derivative of gas dynamics **\[dimensionless\]**.
    #[strum(to_string = "fundamental_derivative_of_gas_dynamics")]
    FundamentalDerivativeOfGasDynamics = 60,

    /// Residual Helmholtz energy contribution **\[dimensionless\]**.
    #[strum(to_string = "alphar")]
    AlphaR = 61,

    /// Derivative of residual Helmholtz energy contribution
    /// with [`Tau`](FluidParam::Tau) **\[dimensionless\]**.
    #[strum(to_string = "dalphar_dtau_constdelta")]
    DAlphaRDTauConstDelta = 62,

    /// Derivative of residual Helmholtz energy contribution
    /// with [`Delta`](FluidParam::Delta) **\[dimensionless\]**.
    #[strum(to_string = "dalphar_ddelta_consttau")]
    DAlphaRDDeltaConstTau = 63,

    /// Ideal gas Helmholtz energy contribution **\[dimensionless\]**.
    #[strum(to_string = "alpha0")]
    Alpha0 = 64,

    /// Derivative of ideal gas Helmholtz energy contribution
    /// with [`Tau`](FluidParam::Tau) **\[dimensionless\]**.
    #[strum(to_string = "dalpha0_dtau_constdelta")]
    DAlpha0DTauConstDelta = 65,

    /// Derivative of ideal gas Helmholtz energy contribution
    /// with [`Delta`](FluidParam::Delta) **\[dimensionless\]**.
    #[strum(to_string = "dalpha0_ddelta_consttau")]
    DAlpha0DDeltaConstTau = 66,

    /// Second derivative of ideal gas Helmholtz energy contribution
    /// with [`Delta`](FluidParam::Delta) **\[dimensionless\]**.
    #[strum(to_string = "d2alpha0_ddelta2_consttau")]
    D2Alpha0DDelta2ConstTau = 67,

    /// Third derivative of ideal gas Helmholtz energy contribution
    /// with [`Delta`](FluidParam::Delta) **\[dimensionless\]**.
    #[strum(to_string = "d3alpha0_ddelta3_consttau")]
    D3Alpha0DDelta3ConstTau = 68,

    /// Second virial coefficient **\[dimensionless\]**.
    #[strum(to_string = "Bvirial")]
    BVirial = 69,

    /// Third virial coefficient **\[dimensionless\]**.
    #[strum(to_string = "Cvirial")]
    CVirial = 70,

    /// Derivative of second virial coefficient with [`T`](FluidParam::T) **\[dimensionless\]**.
    #[strum(to_string = "dBvirial_dT")]
    DBVirialDT = 71,

    /// Derivative of third virial coefficient with [`T`](FluidParam::T) **\[dimensionless\]**.
    #[strum(to_string = "dCvirial_dT")]
    DCVirialDT = 72,

    /// Compressibility factor **\[dimensionless\]**.
    #[strum(to_string = "Z")]
    Z = 73,

    /// Phase identification parameter **\[dimensionless\]**.
    #[strum(to_string = "PIP")]
    PIP = 74,

    /// Phase index **\[dimensionless\]**.
    #[strum(to_string = "Phase")]
    Phase = 85,
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
/// - [CoolProp Fluids Input/Output Parameters](https://coolprop.org/coolprop/HighLevelAPI.html#parameter-table)
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
    MinFraction = 75,

    /// Maximum fraction _(mole, mass or volume)_ value for incompressible mixtures
    /// **\[dimensionless, from 0 to 1\]**.
    #[strum(to_string = "fraction_max")]
    MaxFraction = 76,

    /// Freezing temperature for incompressible mixtures **\[K\]**.
    #[strum(to_string = "T_freeze")]
    TFreeze = 77,

    /// 20-year global warming potential **\[dimensionless\]**.
    #[strum(to_string = "GWP20")]
    GWP20 = 78,

    /// 100-year global warming potential **\[dimensionless\]**.
    #[strum(to_string = "GWP100")]
    GWP100 = 79,

    /// 500-year global warming potential **\[dimensionless\]**.
    #[strum(to_string = "GWP500")]
    GWP500 = 80,

    /// Flammability hazard index **\[dimensionless\]**.
    #[strum(to_string = "FH")]
    FH = 81,

    /// Health hazard index **\[dimensionless\]**.
    #[strum(to_string = "HH")]
    HH = 82,

    /// Physical hazard index **\[dimensionless\]**.
    #[strum(to_string = "PH")]
    PH = 83,

    /// Ozone depletion potential **\[dimensionless\]**.
    #[strum(to_string = "ODP")]
    ODP = 84,
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
    #[case(QMass, "Qmass")]
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
    #[case(vec!["Qmass"], QMass)]
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
    #[case(QMass, 22)]
    #[case(Tau, 23)]
    #[case(Delta, 24)]
    #[case(DMolar, 25)]
    #[case(HMolar, 26)]
    #[case(SMolar, 27)]
    #[case(CpMolar, 28)]
    #[case(Cp0Molar, 29)]
    #[case(CvMolar, 30)]
    #[case(UMolar, 31)]
    #[case(GMolar, 32)]
    #[case(HelmholtzMolar, 33)]
    #[case(HMolarResidual, 34)]
    #[case(SMolarResidual, 35)]
    #[case(GMolarResidual, 36)]
    #[case(HMolarIdealGas, 37)]
    #[case(SMolarIdealGas, 38)]
    #[case(UMolarIdealGas, 39)]
    #[case(DMass, 40)]
    #[case(HMass, 41)]
    #[case(SMass, 42)]
    #[case(CpMass, 43)]
    #[case(Cp0Mass, 44)]
    #[case(CvMass, 45)]
    #[case(UMass, 46)]
    #[case(GMass, 47)]
    #[case(HelmholtzMass, 48)]
    #[case(HMassIdealGas, 49)]
    #[case(SMassIdealGas, 50)]
    #[case(UMassIdealGas, 51)]
    #[case(DynamicViscosity, 52)]
    #[case(Conductivity, 53)]
    #[case(SurfaceTension, 54)]
    #[case(Prandtl, 55)]
    #[case(SoundSpeed, 56)]
    #[case(IsothermalCompressibility, 57)]
    #[case(IsobaricExpansionCoefficient, 58)]
    #[case(IsentropicExpansionCoefficient, 59)]
    #[case(FundamentalDerivativeOfGasDynamics, 60)]
    #[case(AlphaR, 61)]
    #[case(DAlphaRDTauConstDelta, 62)]
    #[case(DAlphaRDDeltaConstTau, 63)]
    #[case(Alpha0, 64)]
    #[case(DAlpha0DTauConstDelta, 65)]
    #[case(DAlpha0DDeltaConstTau, 66)]
    #[case(D2Alpha0DDelta2ConstTau, 67)]
    #[case(D3Alpha0DDelta3ConstTau, 68)]
    #[case(BVirial, 69)]
    #[case(CVirial, 70)]
    #[case(DBVirialDT, 71)]
    #[case(DCVirialDT, 72)]
    #[case(Z, 73)]
    #[case(PIP, 74)]
    #[case(MinFraction, 75)]
    #[case(MaxFraction, 76)]
    #[case(TFreeze, 77)]
    #[case(GWP20, 78)]
    #[case(GWP100, 79)]
    #[case(GWP500, 80)]
    #[case(FH, 81)]
    #[case(HH, 82)]
    #[case(PH, 83)]
    #[case(ODP, 84)]
    #[case(Phase, 85)]
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
    #[case(22, QMass)]
    #[case(23, Tau)]
    #[case(24, Delta)]
    #[case(25, DMolar)]
    #[case(26, HMolar)]
    #[case(27, SMolar)]
    #[case(28, CpMolar)]
    #[case(29, Cp0Molar)]
    #[case(30, CvMolar)]
    #[case(31, UMolar)]
    #[case(32, GMolar)]
    #[case(33, HelmholtzMolar)]
    #[case(34, HMolarResidual)]
    #[case(35, SMolarResidual)]
    #[case(36, GMolarResidual)]
    #[case(37, HMolarIdealGas)]
    #[case(38, SMolarIdealGas)]
    #[case(39, UMolarIdealGas)]
    #[case(40, DMass)]
    #[case(41, HMass)]
    #[case(42, SMass)]
    #[case(43, CpMass)]
    #[case(44, Cp0Mass)]
    #[case(45, CvMass)]
    #[case(46, UMass)]
    #[case(47, GMass)]
    #[case(48, HelmholtzMass)]
    #[case(49, HMassIdealGas)]
    #[case(50, SMassIdealGas)]
    #[case(51, UMassIdealGas)]
    #[case(52, DynamicViscosity)]
    #[case(53, Conductivity)]
    #[case(54, SurfaceTension)]
    #[case(55, Prandtl)]
    #[case(56, SoundSpeed)]
    #[case(57, IsothermalCompressibility)]
    #[case(58, IsobaricExpansionCoefficient)]
    #[case(59, IsentropicExpansionCoefficient)]
    #[case(60, FundamentalDerivativeOfGasDynamics)]
    #[case(61, AlphaR)]
    #[case(62, DAlphaRDTauConstDelta)]
    #[case(63, DAlphaRDDeltaConstTau)]
    #[case(64, Alpha0)]
    #[case(65, DAlpha0DTauConstDelta)]
    #[case(66, DAlpha0DDeltaConstTau)]
    #[case(67, D2Alpha0DDelta2ConstTau)]
    #[case(68, D3Alpha0DDelta3ConstTau)]
    #[case(69, BVirial)]
    #[case(70, CVirial)]
    #[case(71, DBVirialDT)]
    #[case(72, DCVirialDT)]
    #[case(73, Z)]
    #[case(74, PIP)]
    #[case(75, MinFraction)]
    #[case(76, MaxFraction)]
    #[case(77, TFreeze)]
    #[case(78, GWP20)]
    #[case(79, GWP100)]
    #[case(80, GWP500)]
    #[case(81, FH)]
    #[case(82, HH)]
    #[case(83, PH)]
    #[case(84, ODP)]
    #[case(85, Phase)]
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
