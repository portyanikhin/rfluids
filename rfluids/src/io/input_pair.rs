use crate::io::Param;

/// CoolProp input pairs.
///
/// # Examples
///
/// How to convert [`InputPair`] into [`u8`]:
///
/// ```
/// use rfluids::io::InputPair;
///
/// assert_eq!(u8::from(InputPair::PT), 9);
/// ```
///
/// How to parse [`InputPair`] from two [`Param`]s:
///
/// ```
/// use rfluids::io::{InputPair, Param};
///
/// assert_eq!(InputPair::try_from((Param::T, Param::P)), Ok(InputPair::PT));
/// ```
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum InputPair {
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

impl From<InputPair> for u8 {
    fn from(value: InputPair) -> Self {
        value as u8
    }
}

impl TryFrom<(Param, Param)> for InputPair {
    type Error = strum::ParseError;

    fn try_from(value: (Param, Param)) -> Result<Self, Self::Error> {
        match value {
            (Param::Q, Param::T) | (Param::T, Param::Q) => Ok(InputPair::QT),
            (Param::P, Param::Q) | (Param::Q, Param::P) => Ok(InputPair::PQ),
            (Param::Q, Param::SMolar) | (Param::SMolar, Param::Q) => Ok(InputPair::QSMolar),
            (Param::Q, Param::SMass) | (Param::SMass, Param::Q) => Ok(InputPair::QSMass),
            (Param::HMolar, Param::Q) | (Param::Q, Param::HMolar) => Ok(InputPair::HMolarQ),
            (Param::HMass, Param::Q) | (Param::Q, Param::HMass) => Ok(InputPair::HMassQ),
            (Param::DMolar, Param::Q) | (Param::Q, Param::DMolar) => Ok(InputPair::DMolarQ),
            (Param::DMass, Param::Q) | (Param::Q, Param::DMass) => Ok(InputPair::DMassQ),
            (Param::P, Param::T) | (Param::T, Param::P) => Ok(InputPair::PT),
            (Param::DMass, Param::T) | (Param::T, Param::DMass) => Ok(InputPair::DMassT),
            (Param::DMolar, Param::T) | (Param::T, Param::DMolar) => Ok(InputPair::DMolarT),
            (Param::HMolar, Param::T) | (Param::T, Param::HMolar) => Ok(InputPair::HMolarT),
            (Param::HMass, Param::T) | (Param::T, Param::HMass) => Ok(InputPair::HMassT),
            (Param::SMolar, Param::T) | (Param::T, Param::SMolar) => Ok(InputPair::SMolarT),
            (Param::SMass, Param::T) | (Param::T, Param::SMass) => Ok(InputPair::SMassT),
            (Param::T, Param::UMolar) | (Param::UMolar, Param::T) => Ok(InputPair::TUMolar),
            (Param::T, Param::UMass) | (Param::UMass, Param::T) => Ok(InputPair::TUMass),
            (Param::DMass, Param::P) | (Param::P, Param::DMass) => Ok(InputPair::DMassP),
            (Param::DMolar, Param::P) | (Param::P, Param::DMolar) => Ok(InputPair::DMolarP),
            (Param::HMass, Param::P) | (Param::P, Param::HMass) => Ok(InputPair::HMassP),
            (Param::HMolar, Param::P) | (Param::P, Param::HMolar) => Ok(InputPair::HMolarP),
            (Param::P, Param::SMass) | (Param::SMass, Param::P) => Ok(InputPair::PSMass),
            (Param::P, Param::SMolar) | (Param::SMolar, Param::P) => Ok(InputPair::PSMolar),
            (Param::P, Param::UMass) | (Param::UMass, Param::P) => Ok(InputPair::PUMass),
            (Param::P, Param::UMolar) | (Param::UMolar, Param::P) => Ok(InputPair::PUMolar),
            (Param::HMass, Param::SMass) | (Param::SMass, Param::HMass) => {
                Ok(InputPair::HMassSMass)
            }
            (Param::HMolar, Param::SMolar) | (Param::SMolar, Param::HMolar) => {
                Ok(InputPair::HMolarSMolar)
            }
            (Param::SMass, Param::UMass) | (Param::UMass, Param::SMass) => {
                Ok(InputPair::SMassUMass)
            }
            (Param::SMolar, Param::UMolar) | (Param::UMolar, Param::SMolar) => {
                Ok(InputPair::SMolarUMolar)
            }
            (Param::DMass, Param::HMass) | (Param::HMass, Param::DMass) => {
                Ok(InputPair::DMassHMass)
            }
            (Param::DMolar, Param::HMolar) | (Param::HMolar, Param::DMolar) => {
                Ok(InputPair::DMolarHMolar)
            }
            (Param::DMass, Param::SMass) | (Param::SMass, Param::DMass) => {
                Ok(InputPair::DMassSMass)
            }
            (Param::DMolar, Param::SMolar) | (Param::SMolar, Param::DMolar) => {
                Ok(InputPair::DMolarSMolar)
            }
            (Param::DMass, Param::UMass) | (Param::UMass, Param::DMass) => {
                Ok(InputPair::DMassUMass)
            }
            (Param::DMolar, Param::UMolar) | (Param::UMolar, Param::DMolar) => {
                Ok(InputPair::DMolarUMolar)
            }
            _ => Err(strum::ParseError::VariantNotFound),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::InputPair::*;
    use super::Param::*;
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
        #[case] input_pair: InputPair,
        #[case] expected: u8,
    ) {
        assert_eq!(u8::from(input_pair), expected);
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
        #[case] valid_params: (Param, Param),
        #[case] expected: InputPair,
    ) {
        assert_eq!(InputPair::try_from(valid_params), Ok(expected));
    }

    #[rstest]
    #[case((TCritical, CpMass))]
    #[case((Phase, DMolar))]
    #[case((GWP100, ODP))]
    fn try_from_two_invalid_params_returns_err(#[case] invalid_params: (Param, Param)) {
        assert!(InputPair::try_from(invalid_params).is_err());
    }
}
