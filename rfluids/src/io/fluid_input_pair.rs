use crate::io::FluidParam;

/// CoolProp input pairs.
///
/// # Examples
///
/// How to convert [`FluidInputPair`] to [`u8`]:
///
/// ```
/// use rfluids::io::FluidInputPair;
///
/// assert_eq!(u8::from(FluidInputPair::PT), 9);
/// ```
///
/// How to convert [`FluidInputPair`] to two [`FluidParam`]s:
///
/// ```
/// use rfluids::io::{FluidInputPair, FluidParam};
///
/// assert_eq!(
///     <(FluidParam, FluidParam)>::from(FluidInputPair::PT),
///     (FluidParam::P, FluidParam::T)
/// );
/// ```
///
/// How to parse [`FluidInputPair`] from two [`FluidParam`]s:
///
/// ```
/// use rfluids::io::{FluidInputPair, FluidParam};
///
/// assert_eq!(
///     FluidInputPair::try_from((FluidParam::T, FluidParam::P)),
///     Ok(FluidInputPair::PT)
/// );
/// ```
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum FluidInputPair {
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

impl From<FluidInputPair> for u8 {
    fn from(value: FluidInputPair) -> Self {
        value as u8
    }
}

impl From<FluidInputPair> for (FluidParam, FluidParam) {
    fn from(value: FluidInputPair) -> Self {
        match value {
            FluidInputPair::QT => (FluidParam::Q, FluidParam::T),
            FluidInputPair::PQ => (FluidParam::P, FluidParam::Q),
            FluidInputPair::QSMolar => (FluidParam::Q, FluidParam::SMolar),
            FluidInputPair::QSMass => (FluidParam::Q, FluidParam::SMass),
            FluidInputPair::HMolarQ => (FluidParam::HMolar, FluidParam::Q),
            FluidInputPair::HMassQ => (FluidParam::HMass, FluidParam::Q),
            FluidInputPair::DMolarQ => (FluidParam::DMolar, FluidParam::Q),
            FluidInputPair::DMassQ => (FluidParam::DMass, FluidParam::Q),
            FluidInputPair::PT => (FluidParam::P, FluidParam::T),
            FluidInputPair::DMassT => (FluidParam::DMass, FluidParam::T),
            FluidInputPair::DMolarT => (FluidParam::DMolar, FluidParam::T),
            FluidInputPair::HMolarT => (FluidParam::HMolar, FluidParam::T),
            FluidInputPair::HMassT => (FluidParam::HMass, FluidParam::T),
            FluidInputPair::SMolarT => (FluidParam::SMolar, FluidParam::T),
            FluidInputPair::SMassT => (FluidParam::SMass, FluidParam::T),
            FluidInputPair::TUMolar => (FluidParam::T, FluidParam::UMolar),
            FluidInputPair::TUMass => (FluidParam::T, FluidParam::UMass),
            FluidInputPair::DMassP => (FluidParam::DMass, FluidParam::P),
            FluidInputPair::DMolarP => (FluidParam::DMolar, FluidParam::P),
            FluidInputPair::HMassP => (FluidParam::HMass, FluidParam::P),
            FluidInputPair::HMolarP => (FluidParam::HMolar, FluidParam::P),
            FluidInputPair::PSMass => (FluidParam::P, FluidParam::SMass),
            FluidInputPair::PSMolar => (FluidParam::P, FluidParam::SMolar),
            FluidInputPair::PUMass => (FluidParam::P, FluidParam::UMass),
            FluidInputPair::PUMolar => (FluidParam::P, FluidParam::UMolar),
            FluidInputPair::HMassSMass => (FluidParam::HMass, FluidParam::SMass),
            FluidInputPair::HMolarSMolar => (FluidParam::HMolar, FluidParam::SMolar),
            FluidInputPair::SMassUMass => (FluidParam::SMass, FluidParam::UMass),
            FluidInputPair::SMolarUMolar => (FluidParam::SMolar, FluidParam::UMolar),
            FluidInputPair::DMassHMass => (FluidParam::DMass, FluidParam::HMass),
            FluidInputPair::DMolarHMolar => (FluidParam::DMolar, FluidParam::HMolar),
            FluidInputPair::DMassSMass => (FluidParam::DMass, FluidParam::SMass),
            FluidInputPair::DMolarSMolar => (FluidParam::DMolar, FluidParam::SMolar),
            FluidInputPair::DMassUMass => (FluidParam::DMass, FluidParam::UMass),
            FluidInputPair::DMolarUMolar => (FluidParam::DMolar, FluidParam::UMolar),
        }
    }
}

impl TryFrom<(FluidParam, FluidParam)> for FluidInputPair {
    type Error = strum::ParseError;

    fn try_from(value: (FluidParam, FluidParam)) -> Result<Self, Self::Error> {
        match value {
            (FluidParam::Q, FluidParam::T) | (FluidParam::T, FluidParam::Q) => {
                Ok(FluidInputPair::QT)
            }
            (FluidParam::P, FluidParam::Q) | (FluidParam::Q, FluidParam::P) => {
                Ok(FluidInputPair::PQ)
            }
            (FluidParam::Q, FluidParam::SMolar) | (FluidParam::SMolar, FluidParam::Q) => {
                Ok(FluidInputPair::QSMolar)
            }
            (FluidParam::Q, FluidParam::SMass) | (FluidParam::SMass, FluidParam::Q) => {
                Ok(FluidInputPair::QSMass)
            }
            (FluidParam::HMolar, FluidParam::Q) | (FluidParam::Q, FluidParam::HMolar) => {
                Ok(FluidInputPair::HMolarQ)
            }
            (FluidParam::HMass, FluidParam::Q) | (FluidParam::Q, FluidParam::HMass) => {
                Ok(FluidInputPair::HMassQ)
            }
            (FluidParam::DMolar, FluidParam::Q) | (FluidParam::Q, FluidParam::DMolar) => {
                Ok(FluidInputPair::DMolarQ)
            }
            (FluidParam::DMass, FluidParam::Q) | (FluidParam::Q, FluidParam::DMass) => {
                Ok(FluidInputPair::DMassQ)
            }
            (FluidParam::P, FluidParam::T) | (FluidParam::T, FluidParam::P) => {
                Ok(FluidInputPair::PT)
            }
            (FluidParam::DMass, FluidParam::T) | (FluidParam::T, FluidParam::DMass) => {
                Ok(FluidInputPair::DMassT)
            }
            (FluidParam::DMolar, FluidParam::T) | (FluidParam::T, FluidParam::DMolar) => {
                Ok(FluidInputPair::DMolarT)
            }
            (FluidParam::HMolar, FluidParam::T) | (FluidParam::T, FluidParam::HMolar) => {
                Ok(FluidInputPair::HMolarT)
            }
            (FluidParam::HMass, FluidParam::T) | (FluidParam::T, FluidParam::HMass) => {
                Ok(FluidInputPair::HMassT)
            }
            (FluidParam::SMolar, FluidParam::T) | (FluidParam::T, FluidParam::SMolar) => {
                Ok(FluidInputPair::SMolarT)
            }
            (FluidParam::SMass, FluidParam::T) | (FluidParam::T, FluidParam::SMass) => {
                Ok(FluidInputPair::SMassT)
            }
            (FluidParam::T, FluidParam::UMolar) | (FluidParam::UMolar, FluidParam::T) => {
                Ok(FluidInputPair::TUMolar)
            }
            (FluidParam::T, FluidParam::UMass) | (FluidParam::UMass, FluidParam::T) => {
                Ok(FluidInputPair::TUMass)
            }
            (FluidParam::DMass, FluidParam::P) | (FluidParam::P, FluidParam::DMass) => {
                Ok(FluidInputPair::DMassP)
            }
            (FluidParam::DMolar, FluidParam::P) | (FluidParam::P, FluidParam::DMolar) => {
                Ok(FluidInputPair::DMolarP)
            }
            (FluidParam::HMass, FluidParam::P) | (FluidParam::P, FluidParam::HMass) => {
                Ok(FluidInputPair::HMassP)
            }
            (FluidParam::HMolar, FluidParam::P) | (FluidParam::P, FluidParam::HMolar) => {
                Ok(FluidInputPair::HMolarP)
            }
            (FluidParam::P, FluidParam::SMass) | (FluidParam::SMass, FluidParam::P) => {
                Ok(FluidInputPair::PSMass)
            }
            (FluidParam::P, FluidParam::SMolar) | (FluidParam::SMolar, FluidParam::P) => {
                Ok(FluidInputPair::PSMolar)
            }
            (FluidParam::P, FluidParam::UMass) | (FluidParam::UMass, FluidParam::P) => {
                Ok(FluidInputPair::PUMass)
            }
            (FluidParam::P, FluidParam::UMolar) | (FluidParam::UMolar, FluidParam::P) => {
                Ok(FluidInputPair::PUMolar)
            }
            (FluidParam::HMass, FluidParam::SMass) | (FluidParam::SMass, FluidParam::HMass) => {
                Ok(FluidInputPair::HMassSMass)
            }
            (FluidParam::HMolar, FluidParam::SMolar) | (FluidParam::SMolar, FluidParam::HMolar) => {
                Ok(FluidInputPair::HMolarSMolar)
            }
            (FluidParam::SMass, FluidParam::UMass) | (FluidParam::UMass, FluidParam::SMass) => {
                Ok(FluidInputPair::SMassUMass)
            }
            (FluidParam::SMolar, FluidParam::UMolar) | (FluidParam::UMolar, FluidParam::SMolar) => {
                Ok(FluidInputPair::SMolarUMolar)
            }
            (FluidParam::DMass, FluidParam::HMass) | (FluidParam::HMass, FluidParam::DMass) => {
                Ok(FluidInputPair::DMassHMass)
            }
            (FluidParam::DMolar, FluidParam::HMolar) | (FluidParam::HMolar, FluidParam::DMolar) => {
                Ok(FluidInputPair::DMolarHMolar)
            }
            (FluidParam::DMass, FluidParam::SMass) | (FluidParam::SMass, FluidParam::DMass) => {
                Ok(FluidInputPair::DMassSMass)
            }
            (FluidParam::DMolar, FluidParam::SMolar) | (FluidParam::SMolar, FluidParam::DMolar) => {
                Ok(FluidInputPair::DMolarSMolar)
            }
            (FluidParam::DMass, FluidParam::UMass) | (FluidParam::UMass, FluidParam::DMass) => {
                Ok(FluidInputPair::DMassUMass)
            }
            (FluidParam::DMolar, FluidParam::UMolar) | (FluidParam::UMolar, FluidParam::DMolar) => {
                Ok(FluidInputPair::DMolarUMolar)
            }
            _ => Err(strum::ParseError::VariantNotFound),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::FluidInputPair::*;
    use super::FluidParam::*;
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(QT, 1)]
    #[case(PQ, 2)]
    #[case(QSMolar, 3)]
    #[case(QSMass, 4)]
    #[case(HMolarQ, 5)]
    #[case(HMassQ, 6)]
    #[case(DMolarQ, 7)]
    #[case(DMassQ, 8)]
    #[case(PT, 9)]
    #[case(DMassT, 10)]
    #[case(DMolarT, 11)]
    #[case(HMolarT, 12)]
    #[case(HMassT, 13)]
    #[case(SMolarT, 14)]
    #[case(SMassT, 15)]
    #[case(TUMolar, 16)]
    #[case(TUMass, 17)]
    #[case(DMassP, 18)]
    #[case(DMolarP, 19)]
    #[case(HMassP, 20)]
    #[case(HMolarP, 21)]
    #[case(PSMass, 22)]
    #[case(PSMolar, 23)]
    #[case(PUMass, 24)]
    #[case(PUMolar, 25)]
    #[case(HMassSMass, 26)]
    #[case(HMolarSMolar, 27)]
    #[case(SMassUMass, 28)]
    #[case(SMolarUMolar, 29)]
    #[case(DMassHMass, 30)]
    #[case(DMolarHMolar, 31)]
    #[case(DMassSMass, 32)]
    #[case(DMolarSMolar, 33)]
    #[case(DMassUMass, 34)]
    #[case(DMolarUMolar, 35)]
    fn u8_from_input_pair_always_returns_expected_value(
        #[case] input_pair: FluidInputPair,
        #[case] expected: u8,
    ) {
        assert_eq!(u8::from(input_pair), expected);
    }

    #[rstest]
    #[case(QT , (Q, T))]
    #[case(PQ , (P, Q))]
    #[case(QSMolar , (Q, SMolar))]
    #[case(QSMass , (Q, SMass))]
    #[case(HMolarQ , (HMolar, Q))]
    #[case(HMassQ , (HMass, Q))]
    #[case(DMolarQ , (DMolar, Q))]
    #[case(DMassQ , (DMass, Q))]
    #[case(PT , (P, T))]
    #[case(DMassT , (DMass, T))]
    #[case(DMolarT , (DMolar, T))]
    #[case(HMolarT , (HMolar, T))]
    #[case(HMassT , (HMass, T))]
    #[case(SMolarT , (SMolar, T))]
    #[case(SMassT , (SMass, T))]
    #[case(TUMolar , (T, UMolar))]
    #[case(TUMass , (T, UMass))]
    #[case(DMassP , (DMass, P))]
    #[case(DMolarP , (DMolar, P))]
    #[case(HMassP , (HMass, P))]
    #[case(HMolarP , (HMolar, P))]
    #[case(PSMass , (P, SMass))]
    #[case(PSMolar , (P, SMolar))]
    #[case(PUMass , (P, UMass))]
    #[case(PUMolar , (P, UMolar))]
    #[case(HMassSMass , (HMass, SMass))]
    #[case(HMolarSMolar , (HMolar, SMolar))]
    #[case(SMassUMass , (SMass, UMass))]
    #[case(SMolarUMolar , (SMolar, UMolar))]
    #[case(DMassHMass , (DMass, HMass))]
    #[case(DMolarHMolar , (DMolar, HMolar))]
    #[case(DMassSMass , (DMass, SMass))]
    #[case(DMolarSMolar , (DMolar, SMolar))]
    #[case(DMassUMass , (DMass, UMass))]
    #[case(DMolarUMolar , (DMolar, UMolar))]
    fn two_params_from_input_pair_always_returns_expected_value(
        #[case] input_pair: FluidInputPair,
        #[case] expected: (FluidParam, FluidParam),
    ) {
        assert_eq!(<(FluidParam, FluidParam)>::from(input_pair), expected);
    }

    #[rstest]
    #[case((Q, T), QT)]
    #[case((T, Q), QT)]
    #[case((P, Q), PQ)]
    #[case((Q, P), PQ)]
    #[case((Q, SMolar), QSMolar)]
    #[case((SMolar, Q), QSMolar)]
    #[case((Q, SMass), QSMass)]
    #[case((SMass, Q), QSMass)]
    #[case((HMolar, Q), HMolarQ)]
    #[case((Q, HMolar), HMolarQ)]
    #[case((HMass, Q), HMassQ)]
    #[case((Q, HMass), HMassQ)]
    #[case((DMolar, Q), DMolarQ)]
    #[case((Q, DMolar), DMolarQ)]
    #[case((DMass, Q), DMassQ)]
    #[case((Q, DMass), DMassQ)]
    #[case((P, T), PT)]
    #[case((T, P), PT)]
    #[case((DMass, T),DMassT)]
    #[case((T, DMass), DMassT)]
    #[case((DMolar, T), DMolarT)]
    #[case((T, DMolar), DMolarT)]
    #[case((HMolar, T), HMolarT)]
    #[case((T, HMolar), HMolarT)]
    #[case((HMass, T), HMassT)]
    #[case((T, HMass), HMassT)]
    #[case((SMolar, T), SMolarT)]
    #[case((T, SMolar), SMolarT)]
    #[case((SMass, T), SMassT)]
    #[case((T, SMass), SMassT)]
    #[case((T, UMolar), TUMolar)]
    #[case((UMolar, T), TUMolar)]
    #[case((T, UMass), TUMass)]
    #[case((UMass, T), TUMass)]
    #[case((DMass, P), DMassP)]
    #[case((P, DMass), DMassP)]
    #[case((DMolar, P), DMolarP)]
    #[case((P, DMolar), DMolarP)]
    #[case((HMass, P), HMassP)]
    #[case((P, HMass), HMassP)]
    #[case((HMolar, P), HMolarP)]
    #[case((P, HMolar), HMolarP)]
    #[case((P, SMass), PSMass)]
    #[case((SMass, P), PSMass)]
    #[case((P, SMolar), PSMolar)]
    #[case((SMolar, P), PSMolar)]
    #[case((P, UMass), PUMass)]
    #[case((UMass, P), PUMass)]
    #[case((P, UMolar), PUMolar)]
    #[case((UMolar, P), PUMolar)]
    #[case((HMass, SMass), HMassSMass)]
    #[case((SMass, HMass), HMassSMass)]
    #[case((HMolar, SMolar), HMolarSMolar)]
    #[case((SMolar, HMolar), HMolarSMolar)]
    #[case((SMass, UMass), SMassUMass)]
    #[case((UMass, SMass), SMassUMass)]
    #[case((SMolar, UMolar), SMolarUMolar)]
    #[case((UMolar, SMolar), SMolarUMolar)]
    #[case((DMass, HMass), DMassHMass)]
    #[case((HMass, DMass), DMassHMass)]
    #[case((DMolar, HMolar), DMolarHMolar)]
    #[case((HMolar, DMolar), DMolarHMolar)]
    #[case((DMass, SMass), DMassSMass)]
    #[case((SMass, DMass), DMassSMass)]
    #[case((DMolar, SMolar), DMolarSMolar)]
    #[case((SMolar, DMolar), DMolarSMolar)]
    #[case((DMass, UMass), DMassUMass)]
    #[case((UMass, DMass), DMassUMass)]
    #[case((DMolar, UMolar), DMolarUMolar)]
    #[case((UMolar, DMolar), DMolarUMolar)]
    fn try_from_two_valid_params_returns_ok(
        #[case] valid_params: (FluidParam, FluidParam),
        #[case] expected: FluidInputPair,
    ) {
        assert_eq!(FluidInputPair::try_from(valid_params), Ok(expected));
    }

    #[rstest]
    #[case((TCritical, CpMass))]
    #[case((Phase, DMolar))]
    #[case((GWP100, ODP))]
    fn try_from_two_invalid_params_returns_err(#[case] invalid_params: (FluidParam, FluidParam)) {
        assert!(FluidInputPair::try_from(invalid_params).is_err());
    }
}
