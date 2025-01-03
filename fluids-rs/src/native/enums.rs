use crate::native::common::CoolPropError;
use std::str::FromStr;

/// CoolProp input pairs
/// (for use in [`AbstractState::update`](crate::native::AbstractState::update)).
///
/// # Examples
///
/// How to get [`InputPair`] from two [`Parameter`]`s`:
///
/// ```
/// use fluids_rs::native::{InputPair, Parameter};
///
/// let result = InputPair::try_from((Parameter::T, Parameter::P)).unwrap();
/// assert_eq!(result, InputPair::PT);
/// ```
///
/// If specified parameters has no matching [`InputPair`]`s`, a [`CoolPropError`] is returned:
/// ```
/// use fluids_rs::native::{InputPair, Parameter};
///
/// let result = InputPair::try_from((Parameter::TCritical, Parameter::CpMass));
/// assert!(result.is_err());
/// assert_eq!(
///     result.unwrap_err().to_string(),
///     "Specified parameters ('TCritical', 'CpMass') has no matching input pairs!"
/// );
/// ```
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum InputPair {
    /// Default _(invalid)_ value.
    Invalid = 0,

    /// Vapor quality _(dimensionless, from 0 to 1)_, temperature _(K)_.
    QT = 1,

    /// Pressure _(Pa)_, vapor quality _(dimensionless, from 0 to 1)_.
    PQ = 2,

    /// Vapor quality _(dimensionless, from 0 to 1)_, molar specific entropy _(J/mol/K)_.
    QSMolar = 3,

    /// Vapor quality _(dimensionless, from 0 to 1)_, mass specific entropy _(J/kg/K)_.
    QSMass = 4,

    /// Molar specific enthalpy _(J/mol)_, vapor quality _(dimensionless, from 0 to 1)_.
    HMolarQ = 5,

    /// Mass specific enthalpy _(J/kg)_, vapor quality _(dimensionless, from 0 to 1)_.
    HMassQ = 6,

    /// Molar density _(mol/m3)_, vapor quality _(dimensionless, from 0 to 1)_.
    DMolarQ = 7,

    /// Mass density _(kg/m3)_, vapor quality _(dimensionless, from 0 to 1)_.
    DMassQ = 8,

    /// Pressure _(Pa)_, temperature _(K)_.
    PT = 9,

    /// Mass density _(kg/m3)_, temperature _(K)_.
    DMassT = 10,

    /// Molar density _(mol/m3)_, temperature _(K)_.
    DMolarT = 11,

    /// Molar specific enthalpy _(J/mol)_, temperature _(K)_.
    HMolarT = 12,

    /// Mass specific enthalpy _(J/kg)_, temperature _(K)_.
    HMassT = 13,

    /// Molar specific entropy _(J/mol/K)_, temperature _(K)_.
    SMolarT = 14,

    /// Mass specific entropy _(J/kg/K)_, temperature _(K)_.
    SMassT = 15,

    /// Temperature _(K)_, molar specific internal energy _(J/mol)_.
    TUMolar = 16,

    /// Temperature _(K)_, mass specific internal energy _(J/kg)_.
    TUMass = 17,

    /// Mass density _(kg/m3)_, pressure _(Pa)_.
    DMassP = 18,

    /// Molar density _(mol/m3)_, pressure _(Pa)_.
    DMolarP = 19,

    /// Mass specific enthalpy _(J/kg)_, pressure _(Pa)_.
    HMassP = 20,

    /// Molar specific enthalpy _(J/mol)_, pressure _(Pa)_.
    HMolarP = 21,

    /// Pressure _(Pa)_, mass specific entropy _(J/kg/K)_.
    PSMass = 22,

    /// Pressure _(Pa)_, molar specific entropy _(J/mol/K)_.
    PSMolar = 23,

    /// Pressure _(Pa)_, mass specific internal energy _(J/kg)_.
    PUMass = 24,

    /// Pressure _(Pa)_, molar specific internal energy _(J/mol)_.
    PUMolar = 25,

    /// Mass specific enthalpy _(J/kg)_, mass specific entropy _(J/kg/K)_.
    HMassSMass = 26,

    /// Molar specific enthalpy _(J/mol)_, molar specific entropy _(J/mol/K)_.
    HMolarSMolar = 27,

    /// Mass specific entropy _(J/kg/K)_, mass specific internal energy _(J/kg)_.
    SMassUMass = 28,

    /// Molar specific entropy _(J/mol/K)_, molar specific internal energy _(J/mol)_.
    SMolarUMolar = 29,

    /// Mass density _(kg/m3)_, mass specific enthalpy _(J/kg)_.
    DMassHMass = 30,

    /// Molar density _(mol/m3)_, molar specific enthalpy _(J/mol)_.
    DMolarHMolar = 31,

    /// Mass density _(kg/m3)_, mass specific entropy _(J/kg/K)_.
    DMassSMass = 32,

    /// Molar density _(mol/m3)_, molar specific entropy _(J/mol/K)_.
    DMolarSMolar = 33,

    /// Mass density _(kg/m3)_, mass specific internal energy _(J/kg)_.
    DMassUMass = 34,

    /// Molar density _(mol/m3)_, molar specific internal energy _(J/mol)_.
    DMolarUMolar = 35,
}

impl TryFrom<(Parameter, Parameter)> for InputPair {
    type Error = CoolPropError;

    fn try_from(value: (Parameter, Parameter)) -> Result<Self, Self::Error> {
        match value {
            (Parameter::Invalid, Parameter::Invalid) => Ok(InputPair::Invalid),
            (Parameter::Q, Parameter::T) | (Parameter::T, Parameter::Q) => Ok(InputPair::QT),
            (Parameter::P, Parameter::Q) | (Parameter::Q, Parameter::P) => Ok(InputPair::PQ),
            (Parameter::Q, Parameter::SMolar) | (Parameter::SMolar, Parameter::Q) => {
                Ok(InputPair::QSMolar)
            }
            (Parameter::Q, Parameter::SMass) | (Parameter::SMass, Parameter::Q) => {
                Ok(InputPair::QSMass)
            }
            (Parameter::HMolar, Parameter::Q) | (Parameter::Q, Parameter::HMolar) => {
                Ok(InputPair::HMolarQ)
            }
            (Parameter::HMass, Parameter::Q) | (Parameter::Q, Parameter::HMass) => {
                Ok(InputPair::HMassQ)
            }
            (Parameter::DMolar, Parameter::Q) | (Parameter::Q, Parameter::DMolar) => {
                Ok(InputPair::DMolarQ)
            }
            (Parameter::DMass, Parameter::Q) | (Parameter::Q, Parameter::DMass) => {
                Ok(InputPair::DMassQ)
            }
            (Parameter::P, Parameter::T) | (Parameter::T, Parameter::P) => Ok(InputPair::PT),
            (Parameter::DMass, Parameter::T) | (Parameter::T, Parameter::DMass) => {
                Ok(InputPair::DMassT)
            }
            (Parameter::DMolar, Parameter::T) | (Parameter::T, Parameter::DMolar) => {
                Ok(InputPair::DMolarT)
            }
            (Parameter::HMolar, Parameter::T) | (Parameter::T, Parameter::HMolar) => {
                Ok(InputPair::HMolarT)
            }
            (Parameter::HMass, Parameter::T) | (Parameter::T, Parameter::HMass) => {
                Ok(InputPair::HMassT)
            }
            (Parameter::SMolar, Parameter::T) | (Parameter::T, Parameter::SMolar) => {
                Ok(InputPair::SMolarT)
            }
            (Parameter::SMass, Parameter::T) | (Parameter::T, Parameter::SMass) => {
                Ok(InputPair::SMassT)
            }
            (Parameter::T, Parameter::UMolar) | (Parameter::UMolar, Parameter::T) => {
                Ok(InputPair::TUMolar)
            }
            (Parameter::T, Parameter::UMass) | (Parameter::UMass, Parameter::T) => {
                Ok(InputPair::TUMass)
            }
            (Parameter::DMass, Parameter::P) | (Parameter::P, Parameter::DMass) => {
                Ok(InputPair::DMassP)
            }
            (Parameter::DMolar, Parameter::P) | (Parameter::P, Parameter::DMolar) => {
                Ok(InputPair::DMolarP)
            }
            (Parameter::HMass, Parameter::P) | (Parameter::P, Parameter::HMass) => {
                Ok(InputPair::HMassP)
            }
            (Parameter::HMolar, Parameter::P) | (Parameter::P, Parameter::HMolar) => {
                Ok(InputPair::HMolarP)
            }
            (Parameter::P, Parameter::SMass) | (Parameter::SMass, Parameter::P) => {
                Ok(InputPair::PSMass)
            }
            (Parameter::P, Parameter::SMolar) | (Parameter::SMolar, Parameter::P) => {
                Ok(InputPair::PSMolar)
            }
            (Parameter::P, Parameter::UMass) | (Parameter::UMass, Parameter::P) => {
                Ok(InputPair::PUMass)
            }
            (Parameter::P, Parameter::UMolar) | (Parameter::UMolar, Parameter::P) => {
                Ok(InputPair::PUMolar)
            }
            (Parameter::HMass, Parameter::SMass) | (Parameter::SMass, Parameter::HMass) => {
                Ok(InputPair::HMassSMass)
            }
            (Parameter::HMolar, Parameter::SMolar) | (Parameter::SMolar, Parameter::HMolar) => {
                Ok(InputPair::HMolarSMolar)
            }
            (Parameter::SMass, Parameter::UMass) | (Parameter::UMass, Parameter::SMass) => {
                Ok(InputPair::SMassUMass)
            }
            (Parameter::SMolar, Parameter::UMolar) | (Parameter::UMolar, Parameter::SMolar) => {
                Ok(InputPair::SMolarUMolar)
            }
            (Parameter::DMass, Parameter::HMass) | (Parameter::HMass, Parameter::DMass) => {
                Ok(InputPair::DMassHMass)
            }
            (Parameter::DMolar, Parameter::HMolar) | (Parameter::HMolar, Parameter::DMolar) => {
                Ok(InputPair::DMolarHMolar)
            }
            (Parameter::DMass, Parameter::SMass) | (Parameter::SMass, Parameter::DMass) => {
                Ok(InputPair::DMassSMass)
            }
            (Parameter::DMolar, Parameter::SMolar) | (Parameter::SMolar, Parameter::DMolar) => {
                Ok(InputPair::DMolarSMolar)
            }
            (Parameter::DMass, Parameter::UMass) | (Parameter::UMass, Parameter::DMass) => {
                Ok(InputPair::DMassUMass)
            }
            (Parameter::DMolar, Parameter::UMolar) | (Parameter::UMolar, Parameter::DMolar) => {
                Ok(InputPair::DMolarUMolar)
            }
            (input1, input2) => Err(CoolPropError(format!(
                "Specified parameters ('{:?}', '{:?}') has no matching input pairs!",
                input1, input2
            ))),
        }
    }
}

/// CoolProp input/output parameters
/// (for use in [`AbstractState::keyed_output`](crate::native::AbstractState::keyed_output)).
///
/// # Examples
///
/// How to convert two [`Parameter`]`s` into [`InputPair`]:
///
/// ```
/// use fluids_rs::native::{InputPair, Parameter};
///
/// let result: InputPair = (Parameter::T, Parameter::P).try_into().unwrap();
/// assert_eq!(result, InputPair::PT);
/// ```
///
/// If specified parameters has no matching [`InputPair`]`s`, a [`CoolPropError`] is returned:
/// ```
/// use fluids_rs::native::{CoolPropError, InputPair, Parameter};
///
/// let result: Result<InputPair, CoolPropError> =
///     (Parameter::TCritical, Parameter::CpMass).try_into();
/// assert!(result.is_err());
/// assert_eq!(
///     result.unwrap_err().to_string(),
///     "Specified parameters ('TCritical', 'CpMass') has no matching input pairs!"
/// );
/// ```
///
/// # See also
///
/// [CoolProp inputs/outputs](https://coolprop.github.io/CoolProp/coolprop/HighLevelAPI.html#parameter-table)
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Parameter {
    /// Default _(invalid)_ value.
    Invalid = 0,

    /// Molar gas constant _(J/mol/K)_.
    GasConstant = 1,

    /// Molar mass _(kg/mol)_.
    MolarMass = 2,

    /// Acentric factor _(dimensionless)_.
    AcentricFactor = 3,

    /// Reducing point molar density _(mol/m3)_.
    DMolarReducing = 4,

    /// Critical point molar density _(mol/m3)_.
    DMolarCritical = 5,

    /// Reducing point temperature _(K)_.
    TReducing = 6,

    /// Critical point temperature _(K)_.
    TCritical = 7,

    /// Reducing point mass density _(kg/m3)_.
    DMassReducing = 8,

    /// Critical point mass density _(kg/m3)_.
    DMassCritical = 9,

    /// Critical point pressure _(Pa)_.
    PCritical = 10,

    /// Reducing point pressure _(Pa)_.
    PReducing = 11,

    /// Triple point temperature _(K)_.
    TTriple = 12,

    /// Triple point pressure _(Pa)_.
    PTriple = 13,

    /// Minimum temperature _(K)_.
    TMin = 14,

    /// Maximum temperature _(K)_.
    TMax = 15,

    /// Maximum pressure _(Pa)_.
    PMax = 16,

    /// Minimum pressure _(Pa)_.
    PMin = 17,

    /// Dipole moment (C*m).
    DipoleMoment = 18,

    /// Temperature _(K)_.
    T = 19,

    /// Pressure _(Pa)_.
    P = 20,

    /// Vapor quality _(dimensionless, from 0 to 1)_.
    Q = 21,

    /// Reciprocal reduced temperature _(dimensionless,
    /// [`TCritical`](Parameter::TCritical)`/`[`T`](Parameter::T))_.
    Tau = 22,

    /// Reduced density _(dimensionless,
    /// [`DMass`](Parameter::DMass)`/`[`DMassCritical`](Parameter::DMassCritical))_.
    Delta = 23,

    /// Molar density _(mol/m3)_.
    DMolar = 24,

    /// Molar specific enthalpy _(J/mol)_.
    HMolar = 25,

    /// Molar specific entropy _(J/mol/K)_.
    SMolar = 26,

    /// Molar specific constant pressure specific heat _(J/mol/K)_.
    CpMolar = 27,

    /// Ideal gas molar specific constant pressure specific heat _(J/mol/K)_.
    Cp0Molar = 28,

    /// Molar specific constant volume specific heat _(J/mol/K)_.
    CvMolar = 29,

    /// Molar specific internal energy _(J/mol)_.
    UMolar = 30,

    /// Molar specific Gibbs energy _(J/mol)_.
    GMolar = 31,

    /// Molar specific Helmholtz energy _(J/mol)_.
    HelmholtzMolar = 32,

    /// Residual molar enthalpy _(J/mol/K)_.
    HMolarResidual = 33,

    /// Residual molar entropy _(J/mol/K)_.
    SMolarResidual = 34,

    /// Residual molar Gibbs energy _(J/mol/K)_.
    GMolarResidual = 35,

    /// Mass density _(kg/m3)_.
    DMass = 36,

    /// Mass specific enthalpy _(J/kg)_.
    HMass = 37,

    /// Mass specific entropy _(J/kg/K)_.
    SMass = 38,

    /// Mass specific constant pressure specific heat _(J/kg/K)_.
    CpMass = 39,

    /// Ideal gas mass specific constant pressure specific heat _(J/kg/K)_.
    Cp0Mass = 40,

    /// Mass specific constant volume specific heat _(J/kg/K)_.
    CvMass = 41,

    /// Mass specific internal energy _(J/kg)_.
    UMass = 42,

    /// Mass specific Gibbs energy _(J/kg)_.
    GMass = 43,

    /// Mass specific Helmholtz energy _(J/kg)_.
    HelmholtzMass = 44,

    /// Dynamic viscosity _(Pa*s)_.
    DynamicViscosity = 45,

    /// Thermal conductivity _(W/m/K)_.
    Conductivity = 46,

    /// Surface tension _(N/m)_.
    SurfaceTension = 47,

    /// Prandtl number _(dimensionless)_.
    Prandtl = 48,

    /// Sound speed _(m/s)_.
    SoundSpeed = 49,

    /// Isothermal compressibility _(1/Pa)_.
    IsothermalCompressibility = 50,

    /// Isobaric expansion coefficient _(1/K)_.
    IsobaricExpansionCoefficient = 51,

    /// Isentropic expansion coefficient _(dimensionless)_.
    IsentropicExpansionCoefficient = 52,

    /// Fundamental derivative of gas dynamics _(dimensionless)_.
    FundamentalDerivativeOfGasDynamics = 53,

    /// Residual Helmholtz energy _(dimensionless)_.
    AlphaR = 54,

    /// Derivative of residual Helmholtz energy
    /// with _[`Tau`](Parameter::Tau) (dimensionless)_.
    DAlphaRDTauConstDelta = 55,

    /// Derivative of residual Helmholtz energy
    /// with _[`Delta`](Parameter::Delta) (dimensionless)_.
    DAlphaRDDeltaConstTau = 56,

    /// Ideal Helmholtz energy _(dimensionless)_.
    Alpha0 = 57,

    /// Derivative of ideal Helmholtz energy
    /// with _[`Tau`](Parameter::Tau) (dimensionless)_.
    DAlpha0DTauConstDelta = 58,

    /// Derivative of ideal Helmholtz energy
    /// with _[`Delta`](Parameter::Delta) (dimensionless)_.
    DAlpha0DDeltaConstTau = 59,

    /// Second derivative of ideal Helmholtz energy
    /// with _[`Delta`](Parameter::Delta) (dimensionless)_.
    D2Alpha0DDelta2ConstTau = 60,

    /// Third derivative of ideal Helmholtz energy
    /// with _[`Delta`](Parameter::Delta) (dimensionless)_.
    D3Alpha0DDelta3ConstTau = 61,

    /// Second virial coefficient _(dimensionless)_.
    BVirial = 62,

    /// Third virial coefficient _(dimensionless)_.
    CVirial = 63,

    /// Derivative of second virial coefficient
    /// with _[`T`](Parameter::T) (dimensionless)_.
    DBVirialDT = 64,

    /// Derivative of third virial coefficient
    /// with _[`T`](Parameter::T) (dimensionless)_.
    DCVirialDT = 65,

    /// Compressibility factor _(dimensionless)_.
    Z = 66,

    /// Phase identification parameter _(dimensionless)_.
    PIP = 67,

    /// Minimum fraction (mole, mass or volume) value
    /// for incompressible mixtures _(dimensionless, from 0 to 1)_.
    MinFraction = 68,

    /// Maximum fraction (mole, mass or volume) value
    /// for incompressible mixtures _(dimensionless, from 0 to 1)_.
    MaxFraction = 69,

    /// Freezing temperature for incompressible mixtures _(K)_.
    TFreeze = 70,

    /// 20-year global warming potential _(dimensionless)_.
    GWP20 = 71,

    /// 100-year global warming potential _(dimensionless)_.
    GWP100 = 72,

    /// 500-year global warming potential _(dimensionless)_.
    GWP500 = 73,

    /// Flammability hazard index _(dimensionless)_.
    FH = 74,

    /// Health hazard index _(dimensionless)_.
    HH = 75,

    /// Physical hazard index _(dimensionless)_.
    PH = 76,

    /// Ozone depletion potential _(dimensionless)_.
    ODP = 77,

    /// Phase index _(dimensionless)_.
    Phase = 78,

    /// Undefined _(reserve)_ value.
    Undefined = 79,
}

impl From<Parameter> for &'static str {
    //noinspection SpellCheckingInspection
    fn from(value: Parameter) -> Self {
        match value {
            Parameter::Invalid => "invalid",
            Parameter::GasConstant => "gas_constant",
            Parameter::MolarMass => "molar_mass",
            Parameter::AcentricFactor => "acentric_factor",
            Parameter::DMolarReducing => "rhomolar_reducing",
            Parameter::DMolarCritical => "rhomolar_critical",
            Parameter::TReducing => "T_reducing",
            Parameter::TCritical => "T_critical",
            Parameter::DMassReducing => "rhomass_reducing",
            Parameter::DMassCritical => "rhomass_critical",
            Parameter::PCritical => "P_critical",
            Parameter::PReducing => "P_reducing",
            Parameter::TTriple => "T_triple",
            Parameter::PTriple => "P_triple",
            Parameter::TMin => "T_min",
            Parameter::TMax => "T_max",
            Parameter::PMax => "P_max",
            Parameter::PMin => "P_min",
            Parameter::DipoleMoment => "dipole_moment",
            Parameter::T => "T",
            Parameter::P => "P",
            Parameter::Q => "Q",
            Parameter::Tau => "Tau",
            Parameter::Delta => "Delta",
            Parameter::DMolar => "Dmolar",
            Parameter::HMolar => "Hmolar",
            Parameter::SMolar => "Smolar",
            Parameter::CpMolar => "Cpmolar",
            Parameter::Cp0Molar => "Cp0molar",
            Parameter::CvMolar => "Cvmolar",
            Parameter::UMolar => "Umolar",
            Parameter::GMolar => "Gmolar",
            Parameter::HelmholtzMolar => "Helmholtzmolar",
            Parameter::HMolarResidual => "Hmolar_residual",
            Parameter::SMolarResidual => "Smolar_residual",
            Parameter::GMolarResidual => "Gmolar_residual",
            Parameter::DMass => "Dmass",
            Parameter::HMass => "Hmass",
            Parameter::SMass => "Smass",
            Parameter::CpMass => "Cpmass",
            Parameter::Cp0Mass => "Cp0mass",
            Parameter::CvMass => "Cvmass",
            Parameter::UMass => "Umass",
            Parameter::GMass => "Gmass",
            Parameter::HelmholtzMass => "Helmholtzmass",
            Parameter::DynamicViscosity => "viscosity",
            Parameter::Conductivity => "conductivity",
            Parameter::SurfaceTension => "surface_tension",
            Parameter::Prandtl => "Prandtl",
            Parameter::SoundSpeed => "speed_sound",
            Parameter::IsothermalCompressibility => "isothermal_compressibility",
            Parameter::IsobaricExpansionCoefficient => "isobaric_expansion_coefficient",
            Parameter::IsentropicExpansionCoefficient => "isentropic_expansion_coefficient",
            Parameter::FundamentalDerivativeOfGasDynamics => {
                "fundamental_derivative_of_gas_dynamics"
            }
            Parameter::AlphaR => "alphar",
            Parameter::DAlphaRDTauConstDelta => "dalphar_dtau_constdelta",
            Parameter::DAlphaRDDeltaConstTau => "dalphar_ddelta_consttau",
            Parameter::Alpha0 => "alpha0",
            Parameter::DAlpha0DTauConstDelta => "dalpha0_dtau_constdelta",
            Parameter::DAlpha0DDeltaConstTau => "dalpha0_ddelta_consttau",
            Parameter::D2Alpha0DDelta2ConstTau => "d2alpha0_ddelta2_consttau",
            Parameter::D3Alpha0DDelta3ConstTau => "d3alpha0_ddelta3_consttau",
            Parameter::BVirial => "Bvirial",
            Parameter::CVirial => "Cvirial",
            Parameter::DBVirialDT => "dBvirial_dT",
            Parameter::DCVirialDT => "dCvirial_dT",
            Parameter::Z => "Z",
            Parameter::PIP => "PIP",
            Parameter::MinFraction => "fraction_min",
            Parameter::MaxFraction => "fraction_max",
            Parameter::TFreeze => "T_freeze",
            Parameter::GWP20 => "GWP20",
            Parameter::GWP100 => "GWP100",
            Parameter::GWP500 => "GWP500",
            Parameter::FH => "FH",
            Parameter::HH => "HH",
            Parameter::PH => "PH",
            Parameter::ODP => "ODP",
            Parameter::Phase => "Phase",
            Parameter::Undefined => "undefined_parameter",
        }
    }
}

impl FromStr for Parameter {
    type Err = CoolPropError;

    //noinspection SpellCheckingInspection
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "invalid" => Ok(Parameter::Invalid),
            "gas_constant" => Ok(Parameter::GasConstant),
            "molar_mass" | "m" | "molarmass" | "molemass" => Ok(Parameter::MolarMass),
            "acentric_factor" | "acentric" => Ok(Parameter::AcentricFactor),
            "rhomolar_reducing" => Ok(Parameter::DMolarReducing),
            "rhomolar_critical" => Ok(Parameter::DMolarCritical),
            "t_reducing" => Ok(Parameter::TReducing),
            "t_critical" | "tcrit" => Ok(Parameter::TCritical),
            "rhomass_reducing" => Ok(Parameter::DMassReducing),
            "rhomass_critical" | "rhocrit" => Ok(Parameter::DMassCritical),
            "p_critical" | "pcrit" => Ok(Parameter::PCritical),
            "p_reducing" => Ok(Parameter::PReducing),
            "t_triple" | "ttriple" => Ok(Parameter::TTriple),
            "p_triple" | "ptriple" => Ok(Parameter::PTriple),
            "t_min" | "tmin" => Ok(Parameter::TMin),
            "t_max" | "tmax" => Ok(Parameter::TMax),
            "p_max" | "pmax" => Ok(Parameter::PMax),
            "p_min" | "pmin" => Ok(Parameter::PMin),
            "dipole_moment" => Ok(Parameter::DipoleMoment),
            "t" => Ok(Parameter::T),
            "p" => Ok(Parameter::P),
            "q" => Ok(Parameter::Q),
            "tau" => Ok(Parameter::Tau),
            "delta" => Ok(Parameter::Delta),
            "dmolar" => Ok(Parameter::DMolar),
            "hmolar" => Ok(Parameter::HMolar),
            "smolar" => Ok(Parameter::SMolar),
            "cpmolar" => Ok(Parameter::CpMolar),
            "cp0molar" => Ok(Parameter::Cp0Molar),
            "cvmolar" => Ok(Parameter::CvMolar),
            "umolar" => Ok(Parameter::UMolar),
            "gmolar" => Ok(Parameter::GMolar),
            "helmholtzmolar" => Ok(Parameter::HelmholtzMolar),
            "hmolar_residual" => Ok(Parameter::HMolarResidual),
            "smolar_residual" => Ok(Parameter::SMolarResidual),
            "gmolar_residual" => Ok(Parameter::GMolarResidual),
            "dmass" | "d" => Ok(Parameter::DMass),
            "hmass" | "h" => Ok(Parameter::HMass),
            "smass" | "s" => Ok(Parameter::SMass),
            "cpmass" | "c" => Ok(Parameter::CpMass),
            "cp0mass" => Ok(Parameter::Cp0Mass),
            "cvmass" | "o" => Ok(Parameter::CvMass),
            "umass" | "u" => Ok(Parameter::UMass),
            "gmass" | "g" => Ok(Parameter::GMass),
            "helmholtzmass" => Ok(Parameter::HelmholtzMass),
            "viscosity" | "v" => Ok(Parameter::DynamicViscosity),
            "conductivity" | "l" => Ok(Parameter::Conductivity),
            "surface_tension" | "i" => Ok(Parameter::SurfaceTension),
            "prandtl" => Ok(Parameter::Prandtl),
            "speed_sound" | "speed_of_sound" | "a" => Ok(Parameter::SoundSpeed),
            "isothermal_compressibility" => Ok(Parameter::IsothermalCompressibility),
            "isobaric_expansion_coefficient" => Ok(Parameter::IsobaricExpansionCoefficient),
            "isentropic_expansion_coefficient" => Ok(Parameter::IsentropicExpansionCoefficient),
            "fundamental_derivative_of_gas_dynamics" => {
                Ok(Parameter::FundamentalDerivativeOfGasDynamics)
            }
            "alphar" => Ok(Parameter::AlphaR),
            "dalphar_dtau_constdelta" => Ok(Parameter::DAlphaRDTauConstDelta),
            "dalphar_ddelta_consttau" => Ok(Parameter::DAlphaRDDeltaConstTau),
            "alpha0" => Ok(Parameter::Alpha0),
            "dalpha0_dtau_constdelta" => Ok(Parameter::DAlpha0DTauConstDelta),
            "dalpha0_ddelta_consttau" => Ok(Parameter::DAlpha0DDeltaConstTau),
            "d2alpha0_ddelta2_consttau" => Ok(Parameter::D2Alpha0DDelta2ConstTau),
            "d3alpha0_ddelta3_consttau" => Ok(Parameter::D3Alpha0DDelta3ConstTau),
            "bvirial" => Ok(Parameter::BVirial),
            "cvirial" => Ok(Parameter::CVirial),
            "dbvirial_dt" => Ok(Parameter::DBVirialDT),
            "dcvirial_dt" => Ok(Parameter::DCVirialDT),
            "z" => Ok(Parameter::Z),
            "pip" => Ok(Parameter::PIP),
            "fraction_min" => Ok(Parameter::MinFraction),
            "fraction_max" => Ok(Parameter::MaxFraction),
            "t_freeze" => Ok(Parameter::TFreeze),
            "gwp20" => Ok(Parameter::GWP20),
            "gwp100" => Ok(Parameter::GWP100),
            "gwp500" => Ok(Parameter::GWP500),
            "fh" => Ok(Parameter::FH),
            "hh" => Ok(Parameter::HH),
            "ph" => Ok(Parameter::PH),
            "odp" => Ok(Parameter::ODP),
            "phase" => Ok(Parameter::Phase),
            "undefined_parameter" | "undefined" => Ok(Parameter::Undefined),
            _ => Err(CoolPropError(format!(
                "'{}' has no matching parameters!",
                s
            ))),
        }
    }
}

impl TryFrom<&str> for Parameter {
    type Error = CoolPropError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Parameter::from_str(value)
    }
}

/// Phase states of fluids and mixtures.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Phase {
    /// Liquid _([`P`](Parameter::P) `<` [`PCritical`](Parameter::PCritical) `&`
    /// [`T`](Parameter::T) `<` [`TCritical`](Parameter::TCritical); above saturation)_.
    Liquid = 0,

    /// Supercritical fluid _([`P`](Parameter::P) `>` [`PCritical`](Parameter::PCritical) `&`
    /// [`T`](Parameter::T) `>` [`TCritical`](Parameter::TCritical))_.
    Supercritical = 1,

    /// Supercritical gas _([`P`](Parameter::P) `<` [`PCritical`](Parameter::PCritical) `&`
    /// [`T`](Parameter::T) `>` [`TCritical`](Parameter::TCritical))_.
    SupercriticalGas = 2,

    /// Supercritical liquid _([`P`](Parameter::P) `>` [`PCritical`](Parameter::PCritical) `&`
    /// [`T`](Parameter::T) `<` [`TCritical`](Parameter::TCritical))_.
    SupercriticalLiquid = 3,

    /// Critical point _([`P`](Parameter::P) `=` [`PCritical`](Parameter::PCritical) `&`
    /// [`T`](Parameter::T) `=` [`TCritical`](Parameter::TCritical))_.
    CriticalPoint = 4,

    /// Gas _([`P`](Parameter::P) `<` [`PCritical`](Parameter::PCritical) `&`
    /// [`T`](Parameter::T) `<` [`TCritical`](Parameter::TCritical); below saturation)_.
    Gas = 5,

    /// Two-phase fluid _([`P`](Parameter::P) `<` [`PCritical`](Parameter::PCritical) `&`
    /// [`T`](Parameter::T) `<` [`TCritical`](Parameter::TCritical); mixed liquid/gas)_.
    TwoPhase = 6,

    /// Unknown phase.
    Unknown = 7,

    /// CoolProp to determine phase.
    NotImposed = 8,
}

impl From<Phase> for &'static str {
    //noinspection SpellCheckingInspection
    fn from(value: Phase) -> Self {
        match value {
            Phase::Liquid => "phase_liquid",
            Phase::Supercritical => "phase_supercritical",
            Phase::SupercriticalGas => "phase_supercritical_gas",
            Phase::SupercriticalLiquid => "phase_supercritical_liquid",
            Phase::CriticalPoint => "phase_critical_point",
            Phase::Gas => "phase_gas",
            Phase::TwoPhase => "phase_twophase",
            Phase::Unknown => "phase_unknown",
            Phase::NotImposed => "phase_not_imposed",
        }
    }
}

impl FromStr for Phase {
    type Err = CoolPropError;

    //noinspection SpellCheckingInspection
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "phase_liquid" | "liquid" => Ok(Phase::Liquid),
            "phase_supercritical" | "supercritical" => Ok(Phase::Supercritical),
            "phase_supercritical_gas" | "supercritical_gas" | "supercriticalgas" => {
                Ok(Phase::SupercriticalGas)
            }
            "phase_supercritical_liquid" | "supercritical_liquid" | "supercriticalliquid" => {
                Ok(Phase::SupercriticalLiquid)
            }
            "phase_critical_point" | "critical_point" | "criticalpoint" => Ok(Phase::CriticalPoint),
            "phase_gas" | "gas" => Ok(Phase::Gas),
            "phase_twophase" | "phase_two_phase" | "two_phase" | "twophase" => Ok(Phase::TwoPhase),
            "phase_unknown" | "unknown" => Ok(Phase::Unknown),
            "phase_not_imposed" | "not_imposed" | "notimposed" => Ok(Phase::NotImposed),
            _ => Err(CoolPropError(format!(
                "'{}' has no matching phase state!",
                s
            ))),
        }
    }
}

impl TryFrom<&str> for Phase {
    type Error = CoolPropError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Phase::from_str(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case((Parameter::Invalid, Parameter::Invalid), InputPair::Invalid)]
    #[case((Parameter::Q, Parameter::T), InputPair::QT)]
    #[case((Parameter::T, Parameter::Q), InputPair::QT)]
    #[case((Parameter::P, Parameter::Q), InputPair::PQ)]
    #[case((Parameter::Q, Parameter::P), InputPair::PQ)]
    #[case((Parameter::Q, Parameter::SMolar), InputPair::QSMolar)]
    #[case((Parameter::SMolar, Parameter::Q), InputPair::QSMolar)]
    #[case((Parameter::Q, Parameter::SMass), InputPair::QSMass)]
    #[case((Parameter::SMass, Parameter::Q), InputPair::QSMass)]
    #[case((Parameter::HMolar, Parameter::Q), InputPair::HMolarQ)]
    #[case((Parameter::Q, Parameter::HMolar), InputPair::HMolarQ)]
    #[case((Parameter::HMass, Parameter::Q), InputPair::HMassQ)]
    #[case((Parameter::Q, Parameter::HMass), InputPair::HMassQ)]
    #[case((Parameter::DMolar, Parameter::Q), InputPair::DMolarQ)]
    #[case((Parameter::Q, Parameter::DMolar), InputPair::DMolarQ)]
    #[case((Parameter::DMass, Parameter::Q), InputPair::DMassQ)]
    #[case((Parameter::Q, Parameter::DMass), InputPair::DMassQ)]
    #[case((Parameter::P, Parameter::T), InputPair::PT)]
    #[case((Parameter::T, Parameter::P), InputPair::PT)]
    #[case((Parameter::DMass, Parameter::T), InputPair::DMassT)]
    #[case((Parameter::T, Parameter::DMass), InputPair::DMassT)]
    #[case((Parameter::DMolar, Parameter::T), InputPair::DMolarT)]
    #[case((Parameter::T, Parameter::DMolar), InputPair::DMolarT)]
    #[case((Parameter::HMolar, Parameter::T), InputPair::HMolarT)]
    #[case((Parameter::T, Parameter::HMolar), InputPair::HMolarT)]
    #[case((Parameter::HMass, Parameter::T), InputPair::HMassT)]
    #[case((Parameter::T, Parameter::HMass), InputPair::HMassT)]
    #[case((Parameter::SMolar, Parameter::T), InputPair::SMolarT)]
    #[case((Parameter::T, Parameter::SMolar), InputPair::SMolarT)]
    #[case((Parameter::SMass, Parameter::T), InputPair::SMassT)]
    #[case((Parameter::T, Parameter::SMass), InputPair::SMassT)]
    #[case((Parameter::T, Parameter::UMolar), InputPair::TUMolar)]
    #[case((Parameter::UMolar, Parameter::T), InputPair::TUMolar)]
    #[case((Parameter::T, Parameter::UMass), InputPair::TUMass)]
    #[case((Parameter::UMass, Parameter::T), InputPair::TUMass)]
    #[case((Parameter::DMass, Parameter::P), InputPair::DMassP)]
    #[case((Parameter::P, Parameter::DMass), InputPair::DMassP)]
    #[case((Parameter::DMolar, Parameter::P), InputPair::DMolarP)]
    #[case((Parameter::P, Parameter::DMolar), InputPair::DMolarP)]
    #[case((Parameter::HMass, Parameter::P), InputPair::HMassP)]
    #[case((Parameter::P, Parameter::HMass), InputPair::HMassP)]
    #[case((Parameter::HMolar, Parameter::P), InputPair::HMolarP)]
    #[case((Parameter::P, Parameter::HMolar), InputPair::HMolarP)]
    #[case((Parameter::P, Parameter::SMass), InputPair::PSMass)]
    #[case((Parameter::SMass, Parameter::P), InputPair::PSMass)]
    #[case((Parameter::P, Parameter::SMolar), InputPair::PSMolar)]
    #[case((Parameter::SMolar, Parameter::P), InputPair::PSMolar)]
    #[case((Parameter::P, Parameter::UMass), InputPair::PUMass)]
    #[case((Parameter::UMass, Parameter::P), InputPair::PUMass)]
    #[case((Parameter::P, Parameter::UMolar), InputPair::PUMolar)]
    #[case((Parameter::UMolar, Parameter::P), InputPair::PUMolar)]
    #[case((Parameter::HMass, Parameter::SMass), InputPair::HMassSMass)]
    #[case((Parameter::SMass, Parameter::HMass), InputPair::HMassSMass)]
    #[case((Parameter::HMolar, Parameter::SMolar), InputPair::HMolarSMolar)]
    #[case((Parameter::SMolar, Parameter::HMolar), InputPair::HMolarSMolar)]
    #[case((Parameter::SMass, Parameter::UMass), InputPair::SMassUMass)]
    #[case((Parameter::UMass, Parameter::SMass), InputPair::SMassUMass)]
    #[case((Parameter::SMolar, Parameter::UMolar), InputPair::SMolarUMolar)]
    #[case((Parameter::UMolar, Parameter::SMolar), InputPair::SMolarUMolar)]
    #[case((Parameter::DMass, Parameter::HMass), InputPair::DMassHMass)]
    #[case((Parameter::HMass, Parameter::DMass), InputPair::DMassHMass)]
    #[case((Parameter::DMolar, Parameter::HMolar), InputPair::DMolarHMolar)]
    #[case((Parameter::HMolar, Parameter::DMolar), InputPair::DMolarHMolar)]
    #[case((Parameter::DMass, Parameter::SMass), InputPair::DMassSMass)]
    #[case((Parameter::SMass, Parameter::DMass), InputPair::DMassSMass)]
    #[case((Parameter::DMolar, Parameter::SMolar), InputPair::DMolarSMolar)]
    #[case((Parameter::SMolar, Parameter::DMolar), InputPair::DMolarSMolar)]
    #[case((Parameter::DMass, Parameter::UMass), InputPair::DMassUMass)]
    #[case((Parameter::UMass, Parameter::DMass), InputPair::DMassUMass)]
    #[case((Parameter::DMolar, Parameter::UMolar), InputPair::DMolarUMolar)]
    #[case((Parameter::UMolar, Parameter::DMolar), InputPair::DMolarUMolar)]
    fn input_pair_try_from_two_valid_parameters_returns_ok(
        #[case] valid_parameters: (Parameter, Parameter),
        #[case] expected: InputPair,
    ) {
        let result = InputPair::try_from(valid_parameters);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected);
    }

    #[rstest]
    #[case((Parameter::TCritical, Parameter::CpMass))]
    #[case((Parameter::Phase, Parameter::DMolar))]
    #[case((Parameter::GWP100, Parameter::ODP))]
    fn input_pair_try_from_two_invalid_parameters_returns_err(
        #[case] invalid_parameters: (Parameter, Parameter),
    ) {
        let result = InputPair::try_from(invalid_parameters);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            format!(
                "Specified parameters ('{:?}', '{:?}') has no matching input pairs!",
                invalid_parameters.0, invalid_parameters.1
            )
        );
    }

    //noinspection SpellCheckingInspection
    #[rstest]
    #[case(Parameter::Invalid, "invalid")]
    #[case(Parameter::GasConstant, "gas_constant")]
    #[case(Parameter::MolarMass, "molar_mass")]
    #[case(Parameter::AcentricFactor, "acentric_factor")]
    #[case(Parameter::DMolarReducing, "rhomolar_reducing")]
    #[case(Parameter::DMolarCritical, "rhomolar_critical")]
    #[case(Parameter::TReducing, "T_reducing")]
    #[case(Parameter::TCritical, "T_critical")]
    #[case(Parameter::DMassReducing, "rhomass_reducing")]
    #[case(Parameter::DMassCritical, "rhomass_critical")]
    #[case(Parameter::PCritical, "P_critical")]
    #[case(Parameter::PReducing, "P_reducing")]
    #[case(Parameter::TTriple, "T_triple")]
    #[case(Parameter::PTriple, "P_triple")]
    #[case(Parameter::TMin, "T_min")]
    #[case(Parameter::TMax, "T_max")]
    #[case(Parameter::PMax, "P_max")]
    #[case(Parameter::PMin, "P_min")]
    #[case(Parameter::DipoleMoment, "dipole_moment")]
    #[case(Parameter::T, "T")]
    #[case(Parameter::P, "P")]
    #[case(Parameter::Q, "Q")]
    #[case(Parameter::Tau, "Tau")]
    #[case(Parameter::Delta, "Delta")]
    #[case(Parameter::DMolar, "Dmolar")]
    #[case(Parameter::HMolar, "Hmolar")]
    #[case(Parameter::SMolar, "Smolar")]
    #[case(Parameter::CpMolar, "Cpmolar")]
    #[case(Parameter::Cp0Molar, "Cp0molar")]
    #[case(Parameter::CvMolar, "Cvmolar")]
    #[case(Parameter::UMolar, "Umolar")]
    #[case(Parameter::GMolar, "Gmolar")]
    #[case(Parameter::HelmholtzMolar, "Helmholtzmolar")]
    #[case(Parameter::HMolarResidual, "Hmolar_residual")]
    #[case(Parameter::SMolarResidual, "Smolar_residual")]
    #[case(Parameter::GMolarResidual, "Gmolar_residual")]
    #[case(Parameter::DMass, "Dmass")]
    #[case(Parameter::HMass, "Hmass")]
    #[case(Parameter::SMass, "Smass")]
    #[case(Parameter::CpMass, "Cpmass")]
    #[case(Parameter::Cp0Mass, "Cp0mass")]
    #[case(Parameter::CvMass, "Cvmass")]
    #[case(Parameter::UMass, "Umass")]
    #[case(Parameter::GMass, "Gmass")]
    #[case(Parameter::HelmholtzMass, "Helmholtzmass")]
    #[case(Parameter::DynamicViscosity, "viscosity")]
    #[case(Parameter::Conductivity, "conductivity")]
    #[case(Parameter::SurfaceTension, "surface_tension")]
    #[case(Parameter::Prandtl, "Prandtl")]
    #[case(Parameter::SoundSpeed, "speed_sound")]
    #[case(Parameter::IsothermalCompressibility, "isothermal_compressibility")]
    #[case(
        Parameter::IsobaricExpansionCoefficient,
        "isobaric_expansion_coefficient"
    )]
    #[case(
        Parameter::IsentropicExpansionCoefficient,
        "isentropic_expansion_coefficient"
    )]
    #[case(
        Parameter::FundamentalDerivativeOfGasDynamics,
        "fundamental_derivative_of_gas_dynamics"
    )]
    #[case(Parameter::AlphaR, "alphar")]
    #[case(Parameter::DAlphaRDTauConstDelta, "dalphar_dtau_constdelta")]
    #[case(Parameter::DAlphaRDDeltaConstTau, "dalphar_ddelta_consttau")]
    #[case(Parameter::Alpha0, "alpha0")]
    #[case(Parameter::DAlpha0DTauConstDelta, "dalpha0_dtau_constdelta")]
    #[case(Parameter::DAlpha0DDeltaConstTau, "dalpha0_ddelta_consttau")]
    #[case(Parameter::D2Alpha0DDelta2ConstTau, "d2alpha0_ddelta2_consttau")]
    #[case(Parameter::D3Alpha0DDelta3ConstTau, "d3alpha0_ddelta3_consttau")]
    #[case(Parameter::BVirial, "Bvirial")]
    #[case(Parameter::CVirial, "Cvirial")]
    #[case(Parameter::DBVirialDT, "dBvirial_dT")]
    #[case(Parameter::DCVirialDT, "dCvirial_dT")]
    #[case(Parameter::Z, "Z")]
    #[case(Parameter::PIP, "PIP")]
    #[case(Parameter::MinFraction, "fraction_min")]
    #[case(Parameter::MaxFraction, "fraction_max")]
    #[case(Parameter::TFreeze, "T_freeze")]
    #[case(Parameter::GWP20, "GWP20")]
    #[case(Parameter::GWP100, "GWP100")]
    #[case(Parameter::GWP500, "GWP500")]
    #[case(Parameter::FH, "FH")]
    #[case(Parameter::HH, "HH")]
    #[case(Parameter::PH, "PH")]
    #[case(Parameter::ODP, "ODP")]
    #[case(Parameter::Phase, "Phase")]
    #[case(Parameter::Undefined, "undefined_parameter")]
    fn parameter_into_static_str_always_returns_expected_value(
        #[case] parameter: Parameter,
        #[case] expected: &'static str,
    ) {
        let result: &'static str = parameter.into();
        assert_eq!(result, expected);
    }

    //noinspection SpellCheckingInspection
    #[rstest]
    #[case("invalid", Parameter::Invalid)]
    #[case("gas_constant", Parameter::GasConstant)]
    #[case("molar_mass", Parameter::MolarMass)]
    #[case("M", Parameter::MolarMass)]
    #[case("molarmass", Parameter::MolarMass)]
    #[case("molemass", Parameter::MolarMass)]
    #[case("acentric_factor", Parameter::AcentricFactor)]
    #[case("acentric", Parameter::AcentricFactor)]
    #[case("rhomolar_reducing", Parameter::DMolarReducing)]
    #[case("rhomolar_critical", Parameter::DMolarCritical)]
    #[case("T_reducing", Parameter::TReducing)]
    #[case("T_critical", Parameter::TCritical)]
    #[case("Tcrit", Parameter::TCritical)]
    #[case("rhomass_reducing", Parameter::DMassReducing)]
    #[case("rhomass_critical", Parameter::DMassCritical)]
    #[case("rhocrit", Parameter::DMassCritical)]
    #[case("P_critical", Parameter::PCritical)]
    #[case("Pcrit", Parameter::PCritical)]
    #[case("P_reducing", Parameter::PReducing)]
    #[case("T_triple", Parameter::TTriple)]
    #[case("Ttriple", Parameter::TTriple)]
    #[case("P_triple", Parameter::PTriple)]
    #[case("Ptriple", Parameter::PTriple)]
    #[case("T_min", Parameter::TMin)]
    #[case("Tmin", Parameter::TMin)]
    #[case("T_max", Parameter::TMax)]
    #[case("Tmax", Parameter::TMax)]
    #[case("P_max", Parameter::PMax)]
    #[case("Pmax", Parameter::PMax)]
    #[case("P_min", Parameter::PMin)]
    #[case("Pmin", Parameter::PMin)]
    #[case("dipole_moment", Parameter::DipoleMoment)]
    #[case("T", Parameter::T)]
    #[case("P", Parameter::P)]
    #[case("Q", Parameter::Q)]
    #[case("Tau", Parameter::Tau)]
    #[case("Delta", Parameter::Delta)]
    #[case("Dmolar", Parameter::DMolar)]
    #[case("Hmolar", Parameter::HMolar)]
    #[case("Smolar", Parameter::SMolar)]
    #[case("Cpmolar", Parameter::CpMolar)]
    #[case("Cp0molar", Parameter::Cp0Molar)]
    #[case("Cvmolar", Parameter::CvMolar)]
    #[case("Umolar", Parameter::UMolar)]
    #[case("Gmolar", Parameter::GMolar)]
    #[case("Helmholtzmolar", Parameter::HelmholtzMolar)]
    #[case("Hmolar_residual", Parameter::HMolarResidual)]
    #[case("Smolar_residual", Parameter::SMolarResidual)]
    #[case("Gmolar_residual", Parameter::GMolarResidual)]
    #[case("Dmass", Parameter::DMass)]
    #[case("D", Parameter::DMass)]
    #[case("Hmass", Parameter::HMass)]
    #[case("H", Parameter::HMass)]
    #[case("Smass", Parameter::SMass)]
    #[case("S", Parameter::SMass)]
    #[case("Cpmass", Parameter::CpMass)]
    #[case("C", Parameter::CpMass)]
    #[case("Cp0mass", Parameter::Cp0Mass)]
    #[case("Cvmass", Parameter::CvMass)]
    #[case("O", Parameter::CvMass)]
    #[case("Umass", Parameter::UMass)]
    #[case("U", Parameter::UMass)]
    #[case("Gmass", Parameter::GMass)]
    #[case("G", Parameter::GMass)]
    #[case("Helmholtzmass", Parameter::HelmholtzMass)]
    #[case("viscosity", Parameter::DynamicViscosity)]
    #[case("V", Parameter::DynamicViscosity)]
    #[case("conductivity", Parameter::Conductivity)]
    #[case("L", Parameter::Conductivity)]
    #[case("surface_tension", Parameter::SurfaceTension)]
    #[case("I", Parameter::SurfaceTension)]
    #[case("Prandtl", Parameter::Prandtl)]
    #[case("speed_sound", Parameter::SoundSpeed)]
    #[case("speed_of_sound", Parameter::SoundSpeed)]
    #[case("A", Parameter::SoundSpeed)]
    #[case("isothermal_compressibility", Parameter::IsothermalCompressibility)]
    #[case(
        "isobaric_expansion_coefficient",
        Parameter::IsobaricExpansionCoefficient
    )]
    #[case(
        "isentropic_expansion_coefficient",
        Parameter::IsentropicExpansionCoefficient
    )]
    #[case(
        "fundamental_derivative_of_gas_dynamics",
        Parameter::FundamentalDerivativeOfGasDynamics
    )]
    #[case("alphar", Parameter::AlphaR)]
    #[case("dalphar_dtau_constdelta", Parameter::DAlphaRDTauConstDelta)]
    #[case("dalphar_ddelta_consttau", Parameter::DAlphaRDDeltaConstTau)]
    #[case("alpha0", Parameter::Alpha0)]
    #[case("dalpha0_dtau_constdelta", Parameter::DAlpha0DTauConstDelta)]
    #[case("dalpha0_ddelta_consttau", Parameter::DAlpha0DDeltaConstTau)]
    #[case("d2alpha0_ddelta2_consttau", Parameter::D2Alpha0DDelta2ConstTau)]
    #[case("d3alpha0_ddelta3_consttau", Parameter::D3Alpha0DDelta3ConstTau)]
    #[case("Bvirial", Parameter::BVirial)]
    #[case("Cvirial", Parameter::CVirial)]
    #[case("dBvirial_dT", Parameter::DBVirialDT)]
    #[case("dCvirial_dT", Parameter::DCVirialDT)]
    #[case("Z", Parameter::Z)]
    #[case("PIP", Parameter::PIP)]
    #[case("fraction_min", Parameter::MinFraction)]
    #[case("fraction_max", Parameter::MaxFraction)]
    #[case("T_freeze", Parameter::TFreeze)]
    #[case("GWP20", Parameter::GWP20)]
    #[case("GWP100", Parameter::GWP100)]
    #[case("GWP500", Parameter::GWP500)]
    #[case("FH", Parameter::FH)]
    #[case("HH", Parameter::HH)]
    #[case("PH", Parameter::PH)]
    #[case("ODP", Parameter::ODP)]
    #[case("Phase", Parameter::Phase)]
    #[case("undefined_parameter", Parameter::Undefined)]
    #[case("undefined", Parameter::Undefined)]
    fn parameter_from_valid_str_returns_ok(#[case] s: &str, #[case] expected: Parameter) {
        let mut result = Parameter::from_str(s);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected);
        result = Parameter::try_from(s);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected);
    }

    #[rstest]
    #[case("")]
    #[case("Hello, World!")]
    fn parameter_from_invalid_str_returns_err(#[case] s: &str) {
        let result = Parameter::from_str(s);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            format!("'{}' has no matching parameters!", s)
        );
    }

    //noinspection SpellCheckingInspection
    #[rstest]
    #[case(Phase::Liquid, "phase_liquid")]
    #[case(Phase::Supercritical, "phase_supercritical")]
    #[case(Phase::SupercriticalGas, "phase_supercritical_gas")]
    #[case(Phase::SupercriticalLiquid, "phase_supercritical_liquid")]
    #[case(Phase::CriticalPoint, "phase_critical_point")]
    #[case(Phase::Gas, "phase_gas")]
    #[case(Phase::TwoPhase, "phase_twophase")]
    #[case(Phase::Unknown, "phase_unknown")]
    #[case(Phase::NotImposed, "phase_not_imposed")]
    fn phase_into_static_str_always_returns_expected_value(
        #[case] phase: Phase,
        #[case] expected: &'static str,
    ) {
        let result: &'static str = phase.into();
        assert_eq!(result, expected);
    }

    //noinspection SpellCheckingInspection
    #[rstest]
    #[case("phase_liquid", Phase::Liquid)]
    #[case("liquid", Phase::Liquid)]
    #[case("phase_supercritical", Phase::Supercritical)]
    #[case("supercritical", Phase::Supercritical)]
    #[case("phase_supercritical_gas", Phase::SupercriticalGas)]
    #[case("supercritical_gas", Phase::SupercriticalGas)]
    #[case("supercriticalgas", Phase::SupercriticalGas)]
    #[case("phase_supercritical_liquid", Phase::SupercriticalLiquid)]
    #[case("supercritical_liquid", Phase::SupercriticalLiquid)]
    #[case("supercriticalliquid", Phase::SupercriticalLiquid)]
    #[case("phase_critical_point", Phase::CriticalPoint)]
    #[case("critical_point", Phase::CriticalPoint)]
    #[case("criticalpoint", Phase::CriticalPoint)]
    #[case("phase_gas", Phase::Gas)]
    #[case("gas", Phase::Gas)]
    #[case("phase_twophase", Phase::TwoPhase)]
    #[case("phase_two_phase", Phase::TwoPhase)]
    #[case("two_phase", Phase::TwoPhase)]
    #[case("twophase", Phase::TwoPhase)]
    #[case("phase_unknown", Phase::Unknown)]
    #[case("unknown", Phase::Unknown)]
    #[case("phase_not_imposed", Phase::NotImposed)]
    #[case("not_imposed", Phase::NotImposed)]
    #[case("notimposed", Phase::NotImposed)]
    fn phase_from_valid_str_returns_ok(#[case] s: &str, #[case] expected: Phase) {
        let mut result = Phase::from_str(s);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected);
        result = Phase::try_from(s);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected);
    }

    #[rstest]
    #[case("")]
    #[case("Hello, World!")]
    fn phase_from_invalid_str_returns_err(#[case] s: &str) {
        let result = Phase::from_str(s);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            format!("'{}' has no matching phase state!", s)
        );
    }
}
